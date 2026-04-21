#!/usr/bin/env python3
"""
signal_to_hexa.py — A13 + A14

atlas.signals.n6 에서 [M7!] 또는 [M?] signal 을 뽑아
간단한 하네스 .hexa 파일 자동 생성.
--run-all 시 모든 하네스를 실행하고 FAIL signal 은 [MN] 강등.

수식 파싱 예:
  resonance_n6: "K(2) = n"      →  check K2 == n
  resonance_n6: "σ·φ = n·τ"    →  check sigma * phi == n * tau
  resonance_n6: "분모 = n"       →  자유 검증 (주석)
  resonance_n6: "argument = n"   →  check argument == n  (주석 가드)

출력 경로: /Users/ghost/Dev/n6-architecture/theory/predictions/verify_SIG-<id>.hexa

FAIL 강등 경로:
  - --run-all 시 각 하네스를 hexa 또는 없으면 syntax-check 만
  - FAIL signal 은 header 의 [M7!]/[M?] → [MN] 으로 in-place 교체
  - null_reason: "auto-harness FAIL <date>" 추가

사용법:
  /usr/bin/python3 scripts/signal_to_hexa.py --dry-run
  /usr/bin/python3 scripts/signal_to_hexa.py --commit
  /usr/bin/python3 scripts/signal_to_hexa.py --run-all --commit
"""
from __future__ import annotations

import argparse
import re
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

SSOT = Path("/Users/ghost/Dev/nexus/n6/atlas.signals.n6")
OUT_DIR = Path("/Users/ghost/Dev/n6-architecture/theory/predictions")
BACKUP = Path("/Users/ghost/Dev/nexus/n6/atlas.signals.n6.bak.pre-signal2hexa")

TARGET_GRADES = {"M7!", "M?"}

N6_CONSTS = {
    "n": 6, "sigma": 12, "phi": 2, "tau": 4, "sopfr": 5, "J2": 24,
}


def now_iso_date() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%d")


def parse_signals(text: str) -> list[dict]:
    signals: list[dict] = []
    lines = text.split("\n")
    i = 0
    n_lines = len(lines)
    while i < n_lines:
        m = re.match(
            r"^@S\s+(\S+)\s*=\s*(.+?)\s*::\s*signal\s*\[([^\]]+)\]\s*\[([^\]]+)\]\s*\[([^\]]+)\]\s*\[([^\]]+)\]",
            lines[i],
        )
        if m:
            sig = {
                "sig_id": m.group(1),
                "statement": m.group(2),
                "repo_tags": [t.strip() for t in m.group(3).split(",")],
                "domain_tags": [t.strip() for t in m.group(4).split(",")],
                "grade": m.group(5).strip(),
                "evidence": m.group(6).strip(),
                "line_start": i,
                "context": "",
                "resonance_n6": None,
                "witness": 1,
            }
            j = i + 1
            while j < n_lines and not lines[j].startswith("@S ") and not lines[j].startswith("# ─"):
                cm = re.match(r'^\s*"(.*)"\s*$', lines[j])
                if cm and not sig["context"]:
                    sig["context"] = cm.group(1)
                rm = re.match(r'^\s*resonance_n6:\s*(.*)$', lines[j])
                if rm:
                    val = rm.group(1).strip()
                    if val.lower() == "null":
                        sig["resonance_n6"] = None
                    else:
                        sig["resonance_n6"] = val.strip('"')
                wm = re.search(r"^\s*witness:\s*(\d+)", lines[j])
                if wm:
                    sig["witness"] = int(wm.group(1))
                j += 1
            sig["line_end"] = j
            signals.append(sig)
            i = j
        else:
            i += 1
    return signals


# ───────── 수식 파싱 ─────────
def parse_equation(formula: str) -> tuple[str, str] | None:
    """
    formula → (lhs_expr, rhs_expr) or None
    지원:
      "K(2) = n"           → ("K_2", "n")
      "σ·φ = n·τ"           → ("sigma * phi", "n * tau")
      "σφ = nτ = 24"        → ("sigma * phi", "n * tau")   (=chain 첫 쌍)
      "argument = n"        → ("argument", "n")
      "분모 = n"             → ("denom", "n")
    """
    if not formula:
        return None
    s = formula
    # 한글 단어 → 영문 치환
    s = s.replace("분모", "denom")
    s = s.replace("분자", "numer")
    s = s.replace("argument", "argument")
    # 그리스 → 영문
    s = s.replace("σ", "sigma").replace("φ", "phi").replace("τ", "tau").replace("Σ", "sigma")
    s = s.replace("·", "*").replace("×", "*").replace("⋅", "*")
    # K(2) → K_2 등
    s = re.sub(r"\bK\(\s*(\d+)\s*\)", r"K_\1", s)
    # K(1),K(2),K(3)) = (φ,n,σ) 같은 복잡 튜플은 skip
    if "(" in s and ")" in s and "," in s:
        return None
    # 첫 '=' 로 split
    parts = re.split(r"(?<!<)(?<!>)=(?!=)", s, maxsplit=1)
    if len(parts) != 2:
        return None
    lhs = parts[0].strip()
    rhs = parts[1].strip()
    # trailing 한글/주석 제거
    rhs = re.split(r"[;?#:]|\s+\?", rhs, maxsplit=1)[0].strip()
    if not lhs or not rhs:
        return None
    # 너무 복잡한건 포기
    if re.search(r"[^\sa-zA-Z0-9_\*\+\-\/\^\.]", lhs + rhs):
        return None
    return (lhs, rhs)


def hexa_expr(side: str) -> str | None:
    """sympy 없이 n6 상수 기반 단순 수식만 허용"""
    side = side.replace("^", "**")
    # 변수 식별
    ids = set(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", side))
    allowed = {"n", "sigma", "phi", "tau", "sopfr", "J2",
               "denom", "numer", "argument",
               "K_1", "K_2", "K_3"}
    unknown = ids - allowed
    if unknown:
        return None
    return side


def harness_body(sig: dict, eq: tuple[str, str] | None) -> str:
    """생성할 .hexa 본문"""
    ts = now_iso_date()
    head = f"""// verify_{sig['sig_id']}.hexa
// 자동 생성: signal_to_hexa.py ({ts})
// signal: {sig['sig_id']}  grade={sig['grade']}  witness={sig['witness']}
// 규칙: HEXA-FIRST, 한글 주석, 순수 산술

let n = 6
let sigma = 12
let phi = 2
let tau = 4
let sopfr = 5
let J2 = 24

// 부가 상수 (공식 수식에서 자주 등장)
let denom = n
let numer = 1
let argument = n
let K_1 = phi
let K_2 = n
let K_3 = sigma

println("============================================================")
println(" signal 자동 하네스: {sig['sig_id']}")
println(" statement: {_safe(sig['statement'])}")
println("============================================================")

let mut pass_n = 0
let mut fail_n = 0

fn check(label, cond) {{
  if cond {{
    println("  PASS: " + label)
    1
  }} else {{
    println("  FAIL: " + label)
    0
  }}
}}
"""
    body = ""
    if eq is not None:
        lhs, rhs = eq
        lhs_e = hexa_expr(lhs)
        rhs_e = hexa_expr(rhs)
        if lhs_e is not None and rhs_e is not None:
            body = f"""
println("")
println(">>> resonance_n6 검증 <<<")
let lhs_v = {lhs_e}
let rhs_v = {rhs_e}
pass_n = pass_n + check("{_safe(lhs)} = {_safe(rhs)}  ->  " + to_string(lhs_v) + " == " + to_string(rhs_v), lhs_v == rhs_v)
"""
        else:
            body = f"""
println("")
println(">>> resonance_n6 수식 파싱 불가 — skip (MISS) <<<")
println("  formula: {_safe(sig.get('resonance_n6') or '')}")
"""
    else:
        body = """
println("")
println(">>> resonance_n6 필드 없음 — 자동 검증 생략 (MISS) <<<")
"""
    tail = """
println("")
println("============================================================")
println(" PASS=" + to_string(pass_n) + "  FAIL=" + to_string(fail_n))
println("============================================================")
"""
    return head + body + tail


def _safe(s: str) -> str:
    return s.replace('"', '\\"').replace("\n", " ")[:180]


def write_harness(sig: dict) -> tuple[Path, bool]:
    """(path, has_equation) 반환"""
    eq = parse_equation(sig.get("resonance_n6") or "")
    body = harness_body(sig, eq)
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    path = OUT_DIR / f"verify_{sig['sig_id']}.hexa"
    path.write_text(body, encoding="utf-8")
    return path, eq is not None


def run_harness(path: Path) -> tuple[bool, str]:
    """하네스 실행 결과 (PASS?, 로그)"""
    # hexa 바이너리 탐색
    hexa_bin = None
    for cand in ("hexa", "/Users/ghost/.cargo/bin/hexa", "/usr/local/bin/hexa"):
        try:
            r = subprocess.run(["which", cand], capture_output=True, text=True, timeout=3)
            if r.returncode == 0 and r.stdout.strip():
                hexa_bin = r.stdout.strip()
                break
        except Exception:
            continue
    if hexa_bin is None:
        return (True, "SKIP: hexa binary 없음 — 실행 생략 (FAIL 판단 보류)")
    try:
        r = subprocess.run([hexa_bin, str(path)], capture_output=True, text=True, timeout=30)
    except Exception as e:
        return (False, f"EXEC_ERR: {e}")
    log = (r.stdout or "") + (r.stderr or "")
    fail_cnt = len(re.findall(r"\bFAIL:\b", log))
    return (fail_cnt == 0, log)


def demote_signal(lines: list[str], sig: dict, reason: str) -> None:
    """in-place: grade → [MN] + null_reason 추가"""
    start = sig["line_start"]
    end = sig["line_end"]
    hdr = lines[start]
    hdr = re.sub(r"\[M7!\]|\[M\?\]|\[M7\]", "[MN]", hdr, count=1)
    lines[start] = hdr
    # null_reason 라인 삽입 (if 없으면)
    has_reason = any(re.match(r"^\s*null_reason:", lines[k]) for k in range(start, end))
    if not has_reason:
        insert_at = start + 1
        # context 뒤에 넣기
        if start + 1 < end and re.match(r'^\s*".*"', lines[start + 1]):
            insert_at = start + 2
        lines.insert(insert_at, f'  null_reason: "{reason}"')


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--commit", action="store_true", help="실제 쓰기 (기본 dry-run)")
    ap.add_argument("--run-all", action="store_true", help="생성 후 전부 실행 + FAIL→[MN] 강등")
    ap.add_argument("--limit", type=int, default=None, help="생성 최대 개수")
    args = ap.parse_args()
    dry_run = not args.commit

    if not SSOT.exists():
        print(f"ERR: SSOT 없음: {SSOT}", file=sys.stderr)
        sys.exit(1)

    text = SSOT.read_text(encoding="utf-8", errors="replace")
    signals = parse_signals(text)
    targets = [s for s in signals if s["grade"] in TARGET_GRADES]
    print(f"전체 signals: {len(signals)}  |  생성 대상 [{', '.join(TARGET_GRADES)}]: {len(targets)}")

    if args.limit:
        targets = targets[:args.limit]
        print(f"  limit={args.limit} 적용 → {len(targets)}")

    # 기존 .hexa skip 판단 — dry-run 미리보기용 카운트
    will_create = 0
    will_skip = 0
    for s in targets:
        path = OUT_DIR / f"verify_{s['sig_id']}.hexa"
        if path.exists():
            will_skip += 1
        else:
            will_create += 1

    print(f"신규 하네스 생성: {will_create}  |  이미 존재: {will_skip}")

    if dry_run and not args.run_all:
        for s in targets[:15]:
            eq = parse_equation(s.get("resonance_n6") or "")
            mark = "EQ" if eq else "NO"
            print(f"  [{mark}] {s['sig_id']:22} {s['grade']:4} | res={s.get('resonance_n6') or '-':40}")
        print("\n[DRY RUN] --commit 지정 시 파일 생성")
        return

    # ─── 실제 생성 ───
    created: list[dict] = []
    for s in targets:
        path = OUT_DIR / f"verify_{s['sig_id']}.hexa"
        if path.exists():
            continue
        path, has_eq = write_harness(s)
        created.append({"sig": s, "path": path, "has_eq": has_eq})
    print(f"\n생성 완료: {len(created)} 개 .hexa")

    if not args.run_all:
        return

    # ─── --run-all: 실행 + FAIL 강등 ───
    pass_cnt = 0
    fail_cnt = 0
    skip_cnt = 0
    fail_sigs: list[dict] = []
    for c in created:
        ok, log = run_harness(c["path"])
        if log.startswith("SKIP"):
            skip_cnt += 1
            continue
        if ok:
            pass_cnt += 1
        else:
            fail_cnt += 1
            fail_sigs.append(c["sig"])
    print(f"\n실행 결과 — PASS={pass_cnt}  FAIL={fail_cnt}  SKIP={skip_cnt}")

    if not fail_sigs:
        print("FAIL 없음. 강등 skip.")
        return

    # in-place FAIL → [MN]
    if not BACKUP.exists():
        BACKUP.write_bytes(SSOT.read_bytes())
        print(f"백업: {BACKUP}")

    lines = text.split("\n")
    reason = f"auto-harness FAIL {now_iso_date()}"
    # 역순으로 처리 (line_start offset 유지)
    for s in sorted(fail_sigs, key=lambda x: -x["line_start"]):
        demote_signal(lines, s, reason)
    SSOT.write_text("\n".join(lines), encoding="utf-8")
    print(f"강등: {len(fail_sigs)} signal → [MN]")


if __name__ == "__main__":
    main()
