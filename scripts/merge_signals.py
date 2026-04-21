#!/usr/bin/env python3
"""
staging/atlas.signals.staging.{nx,n6arch,an}.n6 → atlas.signals.n6 merge
+ cross-repo 자동 매칭 (수치·키워드 공명 탐지)
"""
from __future__ import annotations
import re
from pathlib import Path
from datetime import datetime, timezone

NEXUS_N6 = Path("/Users/ghost/Dev/nexus/n6")
SSOT = NEXUS_N6 / "atlas.signals.n6"
STAGE_DIR = NEXUS_N6 / "staging"
BACKUP = NEXUS_N6 / "atlas.signals.n6.bak.pre-merge"

STAGES = [
    ("[NX] — Agent A 흡수 (nexus 전체)", STAGE_DIR / "atlas.signals.staging.nx.n6"),
    ("[N6] — Agent B 흡수 (n6-arch 전체)", STAGE_DIR / "atlas.signals.staging.n6arch.n6"),
    ("[AN] — Agent C 흡수 (anima 전체)", STAGE_DIR / "atlas.signals.staging.an.n6"),
]


def parse_signals(text: str) -> list[dict]:
    signals = []
    lines = text.split("\n")
    i = 0
    while i < len(lines):
        m = re.match(
            r"@S\s+(\S+)\s*=\s*(.+?)\s*::\s*signal\s*\[([^\]]+)\]\s*\[([^\]]+)\]\s*\[([^\]]+)\]\s*\[([^\]]+)\]",
            lines[i],
        )
        if m:
            sig = {
                "sig_id": m.group(1),
                "statement": m.group(2),
                "repo_tags": set(t.strip() for t in m.group(3).split(",")),
                "domain_tags": set(t.strip() for t in m.group(4).split(",")),
                "grade": m.group(5).strip(),
                "evidence": m.group(6).strip(),
                "start": i,
                "context": "",
                "text_bulk": lines[i],
            }
            j = i + 1
            while j < len(lines) and not lines[j].startswith("@S ") and not lines[j].startswith("# ─"):
                sig["text_bulk"] += "\n" + lines[j]
                if not sig["context"]:
                    cm = re.match(r'\s*"(.*)"', lines[j])
                    if cm:
                        sig["context"] = cm.group(1)
                j += 1
            sig["end"] = j
            signals.append(sig)
            i = j
        else:
            i += 1
    return signals


def extract_numerics(text: str) -> set[str]:
    """수치 패턴 토큰화"""
    tokens = set()
    for m in re.finditer(r"\d+\.\d+", text):
        tokens.add(m.group())
    for m in re.finditer(r"\b\d+pp\b", text):
        tokens.add(m.group())
    for m in re.finditer(r"\b\d+×", text):
        tokens.add(m.group())
    for m in re.finditer(r"\b\d+x\b", text, re.IGNORECASE):
        tokens.add(m.group().lower())
    for m in re.finditer(r"σ[=≈]\s*\d+(\.\d+)?", text):
        tokens.add(m.group().replace(" ", ""))
    for m in re.finditer(r"\d+%", text):
        tokens.add(m.group())
    return tokens


def extract_keywords(text: str) -> set[str]:
    """의미있는 키워드 추출"""
    kw = set()
    low = text.lower()
    for k in ["σφ=nτ", "stochastic resonance", "resonance", "universal", "bernoulli",
             "completeness", "완전수", "millennium", "ouroboros", "sweet spot",
             "peak", "threshold", "bell", "hebbian", "ratchet", "kuramoto",
             "primitive", "hub", "cross-domain", "perfect6", "golden"]:
        if k.lower() in low:
            kw.add(k.lower())
    return kw


def find_cross_matches(all_signals: list[dict]) -> dict[str, set[str]]:
    """signal 간 수치·키워드 공명 탐지"""
    features = {}
    for s in all_signals:
        txt = s["statement"] + " " + s["context"]
        features[s["sig_id"]] = {
            "nums": extract_numerics(txt),
            "keys": extract_keywords(txt),
            "repos": s["repo_tags"],
        }

    matches: dict[str, set[str]] = {s["sig_id"]: set() for s in all_signals}
    sig_list = list(all_signals)
    for i in range(len(sig_list)):
        a = sig_list[i]
        fa = features[a["sig_id"]]
        for j in range(i + 1, len(sig_list)):
            b = sig_list[j]
            fb = features[b["sig_id"]]
            repo_diff = (fa["repos"] - {"CROSS"}) != (fb["repos"] - {"CROSS"})
            if not repo_diff:
                continue
            num_overlap = fa["nums"] & fb["nums"]
            key_overlap = fa["keys"] & fb["keys"]
            score = len(num_overlap) * 2 + len(key_overlap)
            if score >= 3:
                matches[a["sig_id"]].add(b["sig_id"])
                matches[b["sig_id"]].add(a["sig_id"])
    return matches


def main():
    if SSOT.exists():
        BACKUP.write_bytes(SSOT.read_bytes())
        print(f"백업: {BACKUP}")

    baseline = parse_signals(SSOT.read_text(encoding="utf-8"))
    print(f"기존 atlas.signals.n6: {len(baseline)} signals")

    existing_ids = set(s["sig_id"] for s in baseline)

    merged_blocks = []
    all_parsed = list(baseline)

    for header, stage_path in STAGES:
        if not stage_path.exists():
            print(f"  경고: {stage_path} 없음")
            continue
        text = stage_path.read_text(encoding="utf-8")
        parsed = parse_signals(text)
        new_signals = [s for s in parsed if s["sig_id"] not in existing_ids]
        skipped = len(parsed) - len(new_signals)
        print(f"  {header}: {len(parsed)} 개 → {len(new_signals)} 신규 ({skipped} 중복 skip)")
        for s in new_signals:
            existing_ids.add(s["sig_id"])
            all_parsed.append(s)
        block_lines = [f"\n# ─── {header} 병합 ({datetime.now(timezone.utc).isoformat()[:10]}) ───\n"]
        for s in new_signals:
            block_lines.append(s["text_bulk"])
        merged_blocks.append("\n".join(block_lines))

    with SSOT.open("a", encoding="utf-8") as f:
        for block in merged_blocks:
            f.write(block + "\n")

    print(f"\n병합 후 총 signals: {len(all_parsed)}")

    print("\n=== cross-repo 공명 탐지 ===")
    matches = find_cross_matches(all_parsed)
    cross_candidates = [(sid, ms) for sid, ms in matches.items() if ms]
    print(f"  공명 후보 signal 수: {len(cross_candidates)}")

    top10 = sorted(cross_candidates, key=lambda x: -len(x[1]))[:10]
    print(f"\n  상위 10 (매칭 signal 수 기준):")
    id_to_sig = {s["sig_id"]: s for s in all_parsed}
    for sid, ms in top10:
        s = id_to_sig[sid]
        repos = ",".join(sorted(s["repo_tags"]))
        print(f"    {sid:20} [{repos:12}] ({len(ms)} 매칭) — {s['statement'][:70]}")

    cross_report_path = NEXUS_N6 / "signals" / "cross_repo_candidates.txt"
    cross_report_path.parent.mkdir(parents=True, exist_ok=True)
    with cross_report_path.open("w", encoding="utf-8") as f:
        f.write("# Cross-repo 공명 후보 (수동 검토 대상)\n")
        f.write(f"# 생성: {datetime.now(timezone.utc).isoformat()}\n\n")
        for sid, ms in sorted(cross_candidates, key=lambda x: -len(x[1])):
            s = id_to_sig[sid]
            repos = ",".join(sorted(s["repo_tags"]))
            f.write(f"\n{sid} [{repos}] — {s['statement'][:100]}\n")
            for mid in sorted(ms):
                ms_s = id_to_sig.get(mid)
                if ms_s:
                    ms_repos = ",".join(sorted(ms_s["repo_tags"]))
                    f.write(f"  ↔ {mid} [{ms_repos}] — {ms_s['statement'][:80]}\n")
    print(f"\n  리포트: {cross_report_path}")

    print("\n=== 최종 통계 ===")
    from collections import Counter
    repo_counts = Counter()
    grade_counts = Counter()
    for s in all_parsed:
        for t in s["repo_tags"]:
            repo_counts[t] += 1
        grade_counts[s["grade"]] += 1
    print("리포:")
    for t, c in sorted(repo_counts.items(), key=lambda x: -x[1]):
        print(f"  [{t:6}] {c}")
    print("등급:")
    for g in ["M10*", "M10", "M9", "M7!", "M7", "M?", "MN"]:
        print(f"  [{g:4}] {grade_counts.get(g, 0)}")


if __name__ == "__main__":
    main()
