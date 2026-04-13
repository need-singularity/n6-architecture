#!/usr/bin/env bash
# ──────────────────────────────────────────────────────────
# verify_batch.sh — verify 파일 일괄 실행 + mtime 캐시
# 용도: 변경된 verify 파일만 선별 실행, 결과를 캐시에 기록
# 캐시: .verify_cache.json (리포 루트)
# 사용: bash n6shared/tools/verify_batch.sh [--all] [--dry-run]
#   --all     : mtime 무관 전체 재실행
#   --dry-run : 실행 대상만 표시, 실제 실행 안 함
# ──────────────────────────────────────────────────────────

set -euo pipefail

# 리포 루트 자동 탐지
export REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
export CACHE_FILE="${REPO_ROOT}/.verify_cache.json"
export HEXA_BIN="${HOME}/.cargo/bin/hexa"

# 옵션 파싱
RUN_ALL=false
DRY_RUN=false
for arg in "$@"; do
  case "$arg" in
    --all)     RUN_ALL=true ;;
    --dry-run) DRY_RUN=true ;;
    *)         echo "[오류] 알 수 없는 옵션: $arg"; exit 1 ;;
  esac
done
export RUN_ALL
export DRY_RUN

# hexa 바이너리 확인
if [ ! -x "$HEXA_BIN" ]; then
  # 로컬 빌드 fallback
  if [ -x "/tmp/hexa_rs_build/hexa" ]; then
    HEXA_BIN="/tmp/hexa_rs_build/hexa"
  else
    echo "[경고] hexa 바이너리 없음 — 결과는 mtime 갱신만 기록"
    HEXA_BIN=""
  fi
fi

# 캐시 파일 존재 확인
if [ ! -f "$CACHE_FILE" ]; then
  echo "[오류] 캐시 파일 없음: $CACHE_FILE"
  echo "       먼저 캐시를 생성하세요."
  exit 1
fi

# python3 으로 캐시 비교 + 실행 대상 선별
python3 << 'PYEOF'
import json, os, subprocess, sys, time

repo_root = os.environ["REPO_ROOT"]
cache_path = os.environ["CACHE_FILE"]
hexa_bin = os.environ.get("HEXA_BIN", "")
run_all = os.environ.get("RUN_ALL", "false") == "true"
dry_run = os.environ.get("DRY_RUN", "false") == "true"

# 캐시 읽기
with open(cache_path) as f:
    cache = json.load(f)

entries = cache.get("entries", {})
now_iso = time.strftime("%Y-%m-%dT%H:%M:%S")

# 통계
total = 0
skip = 0
run_targets = []
missing = 0

for rel_path, info in sorted(entries.items()):
    abs_path = os.path.join(repo_root, rel_path)
    total += 1

    # 파일 존재 확인
    if not os.path.isfile(abs_path):
        missing += 1
        continue

    st = os.stat(abs_path)
    cur_mtime = int(st.st_mtime)
    cur_size = st.st_size

    # mtime/size 비교 — 변경 여부 판단
    if not run_all:
        if cur_mtime == info.get("mtime") and cur_size == info.get("size"):
            if info.get("last_result") is not None:
                skip += 1
                continue

    run_targets.append((rel_path, abs_path, cur_mtime, cur_size))

# 실행
passed = 0
failed = 0
errors = 0
skipped = skip

print(f"=== verify 배치 실행 ===")
print(f"  등록 파일: {total}개")
print(f"  누락 파일: {missing}개")
print(f"  mtime 미변경 (skip): {skip}개")
print(f"  실행 대상: {len(run_targets)}개")
if dry_run:
    print(f"  모드: --dry-run (실행 안 함)")
print()

for i, (rel_path, abs_path, cur_mtime, cur_size) in enumerate(run_targets, 1):
    tag = f"[{i}/{len(run_targets)}]"

    if dry_run:
        print(f"  {tag} 대상: {rel_path}")
        continue

    # hexa 실행 (있으면)
    result_str = "SKIP"
    if hexa_bin and abs_path.endswith(".hexa"):
        try:
            proc = subprocess.run(
                [hexa_bin, abs_path],
                capture_output=True, text=True, timeout=30
            )
            if proc.returncode == 0:
                result_str = "PASS"
                passed += 1
            else:
                result_str = "FAIL"
                failed += 1
        except subprocess.TimeoutExpired:
            result_str = "TIMEOUT"
            errors += 1
        except Exception as e:
            result_str = f"ERROR"
            errors += 1
    elif abs_path.endswith(".py"):
        try:
            proc = subprocess.run(
                ["python3", abs_path],
                capture_output=True, text=True, timeout=60
            )
            if proc.returncode == 0:
                result_str = "PASS"
                passed += 1
            else:
                result_str = "FAIL"
                failed += 1
        except subprocess.TimeoutExpired:
            result_str = "TIMEOUT"
            errors += 1
        except Exception as e:
            result_str = f"ERROR"
            errors += 1
    else:
        # hexa 없이 mtime만 갱신
        result_str = "RECORDED"
        passed += 1

    # 캐시 갱신
    entries[rel_path]["mtime"] = cur_mtime
    entries[rel_path]["size"] = cur_size
    entries[rel_path]["last_result"] = result_str
    entries[rel_path]["last_run"] = now_iso

    status_mark = {"PASS": "+", "FAIL": "X", "TIMEOUT": "T", "ERROR": "!", "RECORDED": "~"}.get(result_str, "?")
    print(f"  {tag} [{status_mark}] {result_str}: {rel_path}")

# 캐시 저장
if not dry_run:
    cache["entries"] = entries
    cache["last_batch_run"] = now_iso
    cache["last_batch_stats"] = {
        "total": total,
        "skip": skipped,
        "run": len(run_targets),
        "pass": passed,
        "fail": failed,
        "error": errors,
        "missing": missing
    }
    with open(cache_path, "w") as f:
        json.dump(cache, f, indent=2, ensure_ascii=False)

# 요약 출력
print()
print(f"=== 요약 ===")
print(f"  전체: {total}개")
print(f"  SKIP (미변경): {skipped}개")
print(f"  PASS: {passed}개")
print(f"  FAIL: {failed}개")
print(f"  ERROR: {errors}개")
print(f"  누락: {missing}개")
if not dry_run:
    print(f"  캐시 갱신: {cache_path}")

# 실패 시 종료코드 1
if failed > 0 or errors > 0:
    sys.exit(1)
PYEOF
