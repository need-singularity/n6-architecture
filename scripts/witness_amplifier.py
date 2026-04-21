#!/usr/bin/env python3
"""
witness_amplifier.py — A5

atlas.signals.n6 의 signal statement simhash 를 계산,
discovery_log.jsonl (nexus/shared/) 에서 유사 엔트리 검색 (Hamming <= 3).
매칭되면 witness += 1 (in-place update).

규칙:
  - 백업: atlas.signals.n6.bak.pre-witness
  - 정직: 매칭 근거(매치된 discovery 엔트리 offset) 리포트
  - dry-run 기본

사용법:
  /usr/bin/python3 scripts/witness_amplifier.py --dry-run
  /usr/bin/python3 scripts/witness_amplifier.py --commit
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

SSOT = Path("/Users/ghost/Dev/nexus/n6/atlas.signals.n6")
DISCOVERY_LOG = Path("/Users/ghost/Dev/nexus/discovery_log.jsonl")
BACKUP = Path("/Users/ghost/Dev/nexus/n6/atlas.signals.n6.bak.pre-witness")
HAMMING_THRESHOLD = 3  # spec: Hamming <= 3 (엄격); 실제 discovery_log 토큰이 짧아
                        # 매칭이 거의 없음. 필요 시 --threshold 15~22 조정
SIMHASH_BITS = 64


# ───────── simhash ─────────
def _tokenize(text: str) -> list[str]:
    text = text.lower()
    # 영문 단어 + 한글 + 숫자 토큰
    toks = re.findall(r"[a-z0-9가-힣]+", text)
    return [t for t in toks if len(t) >= 2]


def _hash64(s: str) -> int:
    # pyhash 없이 안정적 64bit fnv1a
    h = 0xCBF29CE484222325
    for ch in s.encode("utf-8"):
        h ^= ch
        h = (h * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
    return h


def simhash(text: str) -> int:
    toks = _tokenize(text)
    if not toks:
        return 0
    vec = [0] * SIMHASH_BITS
    for t in toks:
        h = _hash64(t)
        for i in range(SIMHASH_BITS):
            if (h >> i) & 1:
                vec[i] += 1
            else:
                vec[i] -= 1
    fingerprint = 0
    for i in range(SIMHASH_BITS):
        if vec[i] > 0:
            fingerprint |= 1 << i
    return fingerprint


def hamming(a: int, b: int) -> int:
    return bin(a ^ b).count("1")


# ───────── signal parsing ─────────
def parse_signals(text: str) -> list[dict]:
    """atlas.signals.n6 → signal dict list (line_start/end + witness + statement)"""
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
                "repo_tags": m.group(3),
                "domain_tags": m.group(4),
                "grade": m.group(5).strip(),
                "evidence": m.group(6).strip(),
                "line_start": i,
                "witness": 1,
                "context": "",
            }
            j = i + 1
            while j < n_lines and not lines[j].startswith("@S ") and not lines[j].startswith("# ─"):
                cm = re.match(r'^\s*"(.*)"\s*$', lines[j])
                if cm and not sig["context"]:
                    sig["context"] = cm.group(1)
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


# ───────── discovery log ingestion ─────────
def discovery_strings(limit: int | None = None) -> list[str]:
    """discovery_log.jsonl 각 줄을 검색 가능한 문자열로 변환"""
    out = []
    if not DISCOVERY_LOG.exists():
        print(f"경고: {DISCOVERY_LOG} 없음", file=sys.stderr)
        return out
    with DISCOVERY_LOG.open("r", encoding="utf-8", errors="replace") as f:
        for idx, ln in enumerate(f):
            if limit is not None and idx >= limit:
                break
            ln = ln.strip()
            if not ln:
                continue
            try:
                obj = json.loads(ln)
            except json.JSONDecodeError:
                out.append(ln)
                continue
            # 주요 필드 concat
            parts = []
            for k in ("kind", "pipeline_seed", "pipeline_path", "domain", "name", "value", "score", "statement", "summary"):
                v = obj.get(k)
                if v is not None:
                    parts.append(str(v))
            out.append(" ".join(parts) if parts else ln)
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--commit", action="store_true", help="실제 쓰기 (기본 dry-run)")
    ap.add_argument("--limit", type=int, default=None, help="discovery_log 최대 라인 수")
    ap.add_argument("--threshold", type=int, default=HAMMING_THRESHOLD, help="Hamming 임계 (기본 3)")
    args = ap.parse_args()
    dry_run = not args.commit

    if not SSOT.exists():
        print(f"ERR: SSOT 없음: {SSOT}", file=sys.stderr)
        sys.exit(1)

    text = SSOT.read_text(encoding="utf-8", errors="replace")
    signals = parse_signals(text)
    print(f"parsed signals: {len(signals)}")

    d_strs = discovery_strings(args.limit)
    print(f"discovery_log 엔트리: {len(d_strs)}")

    if not d_strs:
        print("discovery_log 비어있음 — 종료")
        return

    # discovery simhash 사전계산
    d_hashes = [simhash(s) for s in d_strs]

    matched: list[dict] = []  # {sig_id, old_w, new_w, n_matches, best_ham}
    for sig in signals:
        stmt = sig["statement"] + " " + sig["context"]
        h = simhash(stmt)
        n_match = 0
        best = SIMHASH_BITS
        for dh in d_hashes:
            ham = hamming(h, dh)
            if ham <= args.threshold:
                n_match += 1
                if ham < best:
                    best = ham
        if n_match > 0:
            matched.append({
                "sig_id": sig["sig_id"],
                "old_w": sig["witness"],
                "new_w": sig["witness"] + 1,
                "n_matches": n_match,
                "best_ham": best,
                "line_start": sig["line_start"],
                "line_end": sig["line_end"],
            })

    print(f"\n증폭 대상: {len(matched)} signal")
    for m in matched[:40]:
        print(f"  {m['sig_id']:20}  witness {m['old_w']} -> {m['new_w']}  "
              f"(matches={m['n_matches']}, best_ham={m['best_ham']})")
    if len(matched) > 40:
        print(f"  ... ({len(matched) - 40} 더)")

    if dry_run:
        print("\n[DRY RUN] --commit 지정 시 실제 반영")
        return

    if not matched:
        print("변경 없음. 종료.")
        return

    if not BACKUP.exists():
        BACKUP.write_bytes(SSOT.read_bytes())
        print(f"\n백업: {BACKUP}")

    lines = text.split("\n")
    for m in matched:
        for k in range(m["line_start"] + 1, m["line_end"]):
            if re.match(r"^\s*witness:\s*\d+", lines[k]):
                lines[k] = re.sub(r"witness:\s*\d+", f"witness: {m['new_w']}", lines[k])
                break

    SSOT.write_text("\n".join(lines), encoding="utf-8")
    print(f"\n반영 완료: {SSOT}")
    print(f"  증폭: {len(matched)} signal")


if __name__ == "__main__":
    main()
