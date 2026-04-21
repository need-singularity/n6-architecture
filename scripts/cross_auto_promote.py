#!/usr/bin/env python3
"""
cross_auto_promote.py — A10

atlas.signals.n6 의 각 signal 에서 수치를 정규식 추출.
다른 repo_tag signal 과
  - 수치 3+ 일치
  OR
  - 주요 키워드 2+ 일치 (σφ=nτ, stochastic resonance, ouroboros, completeness, ...)
이면 CROSS 태그 추가 + witness += 1.

규칙:
  - repo_tags 에 CROSS 이미 있으면 skip (witness 만 증분 가능)
  - self-reference 방지: 같은 repo 쌍은 CROSS 부여 안함
  - in-place update
  - dry-run 기본

사용법:
  /usr/bin/python3 scripts/cross_auto_promote.py --dry-run
  /usr/bin/python3 scripts/cross_auto_promote.py --commit
"""
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

SSOT = Path("/Users/ghost/Dev/nexus/n6/atlas.signals.n6")
BACKUP = Path("/Users/ghost/Dev/nexus/n6/atlas.signals.n6.bak.pre-cross-promote")

KEYWORDS = [
    "σφ=nτ", "sigma*phi=n*tau", "stochastic resonance", "ouroboros",
    "completeness", "완전수", "millennium", "universal", "bernoulli",
    "hebbian", "ratchet", "kuramoto", "bell", "kissing", "basel",
    "resonance", "threshold", "peak", "sweet spot", "primitive",
    "perfect6", "hub", "jacobi", "riemann", "ζ(2)", "ζ(4)", "ζ(6)",
]


def extract_numerics(text: str) -> set[str]:
    tokens: set[str] = set()
    for m in re.finditer(r"\d+\.\d+", text):
        tokens.add(m.group())
    for m in re.finditer(r"\b\d+pp\b", text):
        tokens.add(m.group())
    for m in re.finditer(r"\b\d+×", text):
        tokens.add(m.group())
    for m in re.finditer(r"\b\d+x\b", text, re.IGNORECASE):
        tokens.add(m.group().lower())
    for m in re.finditer(r"[σΣ]\s*[=≈]\s*\d+(\.\d+)?", text):
        tokens.add(m.group().replace(" ", ""))
    for m in re.finditer(r"\b\d+%", text):
        tokens.add(m.group())
    for m in re.finditer(r"\bp\s*=\s*\d+\.\d+", text):
        tokens.add(m.group().replace(" ", ""))
    return tokens


def extract_keywords(text: str) -> set[str]:
    low = text.lower()
    kws: set[str] = set()
    for k in KEYWORDS:
        if k.lower() in low:
            kws.add(k.lower())
    return kws


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
                "header_line": lines[i],
                "context": "",
                "witness": 1,
                "cross_repo": [],
            }
            j = i + 1
            while j < n_lines and not lines[j].startswith("@S ") and not lines[j].startswith("# ─"):
                cm = re.match(r'^\s*"(.*)"\s*$', lines[j])
                if cm and not sig["context"]:
                    sig["context"] = cm.group(1)
                wm = re.search(r"^\s*witness:\s*(\d+)", lines[j])
                if wm:
                    sig["witness"] = int(wm.group(1))
                crm = re.match(r"^\s*cross_repo:\s*\[(.*)\]\s*$", lines[j])
                if crm:
                    inner = crm.group(1).strip()
                    if inner:
                        sig["cross_repo"] = [t.strip() for t in inner.split(",") if t.strip()]
                j += 1
            sig["line_end"] = j
            signals.append(sig)
            i = j
        else:
            i += 1
    return signals


def primary_repos(repo_tags: list[str]) -> set[str]:
    """CROSS 제외한 실제 repo 집합"""
    return set(t for t in repo_tags if t in {"NX", "N6", "AN"})


def rebuild_header(sig: dict) -> str:
    repo_tags = list(dict.fromkeys(sig["repo_tags"]))  # 중복 제거, 순서 유지
    if "CROSS" not in repo_tags:
        repo_tags = ["CROSS"] + repo_tags
    rt = ",".join(repo_tags)
    dt = ",".join(sig["domain_tags"])
    return (
        f"@S {sig['sig_id']} = {sig['statement']} :: signal "
        f"[{rt}] [{dt}] [{sig['grade']}] [{sig['evidence']}]"
    )


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--commit", action="store_true")
    ap.add_argument("--num-threshold", type=int, default=3)
    ap.add_argument("--key-threshold", type=int, default=2)
    args = ap.parse_args()
    dry_run = not args.commit

    if not SSOT.exists():
        print(f"ERR: SSOT 없음: {SSOT}", file=sys.stderr)
        sys.exit(1)

    text = SSOT.read_text(encoding="utf-8", errors="replace")
    signals = parse_signals(text)
    print(f"parsed signals: {len(signals)}")

    # 특성 추출
    for s in signals:
        txt = s["statement"] + " " + s["context"]
        s["nums"] = extract_numerics(txt)
        s["keys"] = extract_keywords(txt)

    # 쌍별 매칭 → promote 대상
    promote: dict[str, dict] = {}  # sig_id -> {matches, reason}
    n = len(signals)
    for i in range(n):
        a = signals[i]
        a_repos = primary_repos(a["repo_tags"])
        if not a_repos:
            continue
        for j in range(i + 1, n):
            b = signals[j]
            b_repos = primary_repos(b["repo_tags"])
            if not b_repos or a_repos == b_repos:
                continue  # 같은 단일 repo 쌍은 self-ref
            if not (a_repos - b_repos) and not (b_repos - a_repos):
                continue
            num_hit = len(a["nums"] & b["nums"])
            key_hit = len(a["keys"] & b["keys"])
            if num_hit >= args.num_threshold or key_hit >= args.key_threshold:
                for x, y in ((a, b), (b, a)):
                    if "CROSS" in x["repo_tags"]:
                        continue  # 이미 CROSS
                    entry = promote.setdefault(x["sig_id"], {
                        "sig": x,
                        "matches": [],
                    })
                    entry["matches"].append({
                        "other": y["sig_id"],
                        "num_hit": num_hit,
                        "key_hit": key_hit,
                    })

    print(f"\nCROSS 승격 대상: {len(promote)}")
    for sid, info in list(promote.items())[:30]:
        sig = info["sig"]
        ms = info["matches"]
        reasons = ", ".join(f"{m['other']}(n={m['num_hit']},k={m['key_hit']})" for m in ms[:3])
        print(f"  + {sid:20} [{','.join(sig['repo_tags']):10}] {len(ms)}매칭 | {reasons}")
    if len(promote) > 30:
        print(f"  ... ({len(promote) - 30} 더)")

    if dry_run:
        print("\n[DRY RUN] --commit 지정 시 실제 반영")
        return

    if not promote:
        print("변경 없음. 종료.")
        return

    if not BACKUP.exists():
        BACKUP.write_bytes(SSOT.read_bytes())
        print(f"\n백업: {BACKUP}")

    lines = text.split("\n")
    for sid, info in promote.items():
        sig = info["sig"]
        # header 교체 (CROSS tag 추가)
        lines[sig["line_start"]] = rebuild_header(sig)
        # witness 증분
        new_w = sig["witness"] + 1
        for k in range(sig["line_start"] + 1, sig["line_end"]):
            if re.match(r"^\s*witness:\s*\d+", lines[k]):
                lines[k] = re.sub(r"witness:\s*\d+", f"witness: {new_w}", lines[k])
                break

    SSOT.write_text("\n".join(lines), encoding="utf-8")
    print(f"\n반영 완료: {SSOT}")
    print(f"  CROSS 승격: {len(promote)}")


if __name__ == "__main__":
    main()
