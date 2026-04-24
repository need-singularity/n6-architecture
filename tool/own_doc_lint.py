#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
own_doc_lint.py — DOC-ONLY own-rule enforcement for n6-architecture.

Enforces the 14 DOC-ONLY rules in .own (own#1..#12, #16 — excluding #13/#15/#17..#21
which either have existing runtime verifiers or sit outside code-enforceable scope).

SSOT       : repo-root .own (parsed here as informational metadata)
Output     : reports/n6_own_doc_lint.json
Exit code  : 0 when no violations, 1 on any violation, 2 on internal error.

Dependencies: Python 3.8+ standard library only.
"""

from __future__ import annotations

import json
import os
import re
import sys
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parent.parent
OWN_FILE = REPO_ROOT / ".own"
REPORT_PATH = REPO_ROOT / "reports" / "n6_own_doc_lint.json"

# Target rule IDs — the 14 DOC-ONLY rules this linter enforces.
TARGET_RULES = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 16]
# Note: own#14 (readme-sealed) lives in readme_sealed_check.py.
# own#13/#15/#17..#21 already have runtime verifiers (ouroboros detector, .doc-rules, hexa tools).

# Classification — which of the 14 DOC-ONLY rules we can auto-lint.
AUTO_VERIFIABLE = {1, 3, 4, 6, 7, 11, 16}
MANUAL_ONLY = {2, 5, 8, 9, 10, 12}
# Rationale captured in the report + final summary.


# -----------------------------------------------------------------------------
# .own parser — lightweight, no external deps.
# -----------------------------------------------------------------------------

def parse_own_file(path: Path) -> dict[int, dict[str, Any]]:
    """Return {rule_id: {slug, status, decl, scope, why, ...}} for every `own N ...` block."""
    if not path.is_file():
        return {}
    rules: dict[int, dict[str, Any]] = {}
    current: dict[str, Any] | None = None
    header_re = re.compile(r"^own\s+(\d+)\s+(\S+)\s+\"([^\"]*)\"\s*$")
    field_re = re.compile(r"^\s{2,}(\S+)\s+(.*)$")
    with path.open("r", encoding="utf-8") as fh:
        for raw in fh:
            line = raw.rstrip("\n")
            if not line.strip() or line.lstrip().startswith("#"):
                continue
            m_head = header_re.match(line)
            if m_head:
                if current is not None:
                    rules[current["id"]] = current
                current = {
                    "id": int(m_head.group(1)),
                    "status": m_head.group(2),
                    "title": m_head.group(3),
                }
                continue
            if current is None:
                continue
            m_field = field_re.match(line)
            if m_field:
                key, value = m_field.group(1), m_field.group(2).strip()
                # Multiple fields with same name (e.g. decl) — keep first, append later.
                if key in current and isinstance(current[key], str):
                    current[key] = current[key] + " " + value
                else:
                    current[key] = value
    if current is not None:
        rules[current["id"]] = current
    return rules


# -----------------------------------------------------------------------------
# Individual rule checkers. Each returns a list[dict] of violations (may be empty).
# Each violation dict: {"path": str, "detail": str}.
# -----------------------------------------------------------------------------

def _walk_md(root: Path, skip_dirs: set[str] | None = None) -> list[Path]:
    skip = skip_dirs or {".git", "node_modules", ".claude", "build", "target", "lean4-n6"}
    hits: list[Path] = []
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = [d for d in dirnames if d not in skip and not d.startswith(".")]
        for name in filenames:
            if name.endswith(".md"):
                hits.append(Path(dirpath) / name)
    return hits


# own#1 doc-english-required — scan for CJK in public .md docs (root + common dirs).
# Narrow scope: only flag high-signal docs to avoid noise from legacy Korean domain docs.
CJK_RE = re.compile(r"[぀-ヿ㐀-䶿一-鿿가-힯]")


def check_rule_1_doc_english(rules: dict[int, dict[str, Any]]) -> list[dict[str, str]]:
    violations: list[dict[str, str]] = []
    # Public-facing docs that must be English-primary per own#1 + own#17 overlap.
    candidates = [
        REPO_ROOT / "README.md",
        REPO_ROOT / "CONTRIBUTING.md",
        REPO_ROOT / "CLAUDE.md",
    ]
    for path in candidates:
        if not path.is_file():
            violations.append({
                "rule": "own#1",
                "path": str(path.relative_to(REPO_ROOT)),
                "detail": "required English-primary doc is missing",
            })
            continue
        text = path.read_text(encoding="utf-8", errors="replace")
        # Flag if CJK density > 5% of non-space chars (accepts stray terms, rejects body text).
        non_space = re.sub(r"\s+", "", text)
        if not non_space:
            continue
        cjk_chars = CJK_RE.findall(non_space)
        density = len(cjk_chars) / max(1, len(non_space))
        if density > 0.05:
            violations.append({
                "rule": "own#1",
                "path": str(path.relative_to(REPO_ROOT)),
                "detail": f"CJK density {density:.1%} exceeds 5% threshold ({len(cjk_chars)} CJK chars)",
            })
    return violations


# own#3 one-doc-per-domain — each domain dir should contain exactly one body doc
# matching <domain>.md (plus optional CLAUDE.md). Extra top-level .md files are flagged.
def check_rule_3_one_doc_per_domain() -> list[dict[str, str]]:
    violations: list[dict[str, str]] = []
    domains_root = REPO_ROOT / "domains"
    if not domains_root.is_dir():
        return violations
    for axis_dir in sorted(domains_root.iterdir()):
        if not axis_dir.is_dir() or axis_dir.name.startswith("_"):
            continue
        for domain_dir in sorted(axis_dir.iterdir()):
            if not domain_dir.is_dir() or domain_dir.name.startswith("_"):
                continue
            top_level_md = [
                p for p in domain_dir.iterdir()
                if p.is_file() and p.suffix == ".md"
            ]
            body_candidate = domain_dir / f"{domain_dir.name}.md"
            body_exists = body_candidate.is_file()
            extras = [
                p for p in top_level_md
                if p.name not in {f"{domain_dir.name}.md", "CLAUDE.md"}
            ]
            if not body_exists:
                violations.append({
                    "rule": "own#3",
                    "path": str(domain_dir.relative_to(REPO_ROOT)),
                    "detail": f"missing canonical body doc {domain_dir.name}.md",
                })
            for extra in extras:
                violations.append({
                    "rule": "own#3",
                    "path": str(extra.relative_to(REPO_ROOT)),
                    "detail": "extra top-level .md beyond {name}.md + CLAUDE.md",
                })
    return violations


# own#4 domain-15-section-template — body doc must contain §1..§15 section headers.
SECTION_HEAD_RE = re.compile(r"^\s*#{1,6}\s*§(\d+)\b", re.MULTILINE)


def check_rule_4_fifteen_sections() -> list[dict[str, str]]:
    violations: list[dict[str, str]] = []
    domains_root = REPO_ROOT / "domains"
    if not domains_root.is_dir():
        return violations
    for axis_dir in sorted(domains_root.iterdir()):
        if not axis_dir.is_dir() or axis_dir.name.startswith("_"):
            continue
        for domain_dir in sorted(axis_dir.iterdir()):
            if not domain_dir.is_dir() or domain_dir.name.startswith("_"):
                continue
            body = domain_dir / f"{domain_dir.name}.md"
            if not body.is_file():
                continue  # covered by rule#3
            text = body.read_text(encoding="utf-8", errors="replace")
            present = {int(m.group(1)) for m in SECTION_HEAD_RE.finditer(text)}
            missing = [n for n in range(1, 16) if n not in present]
            if missing:
                violations.append({
                    "rule": "own#4",
                    "path": str(body.relative_to(REPO_ROOT)),
                    "detail": f"missing sections: {missing}",
                })
    return violations


# own#6 paper-verify-embedded — papers/*.md must contain an embedded ```python block.
def check_rule_6_paper_verify_embedded() -> list[dict[str, str]]:
    violations: list[dict[str, str]] = []
    papers_root = REPO_ROOT / "papers"
    if not papers_root.is_dir():
        return violations
    for path in sorted(papers_root.glob("*.md")):
        # Skip README/CLAUDE/INDEX-type meta docs.
        name_lower = path.stem.lower()
        if name_lower in {"readme", "claude", "index", "_index"}:
            continue
        text = path.read_text(encoding="utf-8", errors="replace")
        if "```python" not in text:
            violations.append({
                "rule": "own#6",
                "path": str(path.relative_to(REPO_ROOT)),
                "detail": "paper missing embedded ```python verify block",
            })
        # Separate verify scripts forbidden.
        sibling_verify = list(path.parent.glob(f"{path.stem}_verify.*")) + \
                         list(path.parent.glob(f"{path.stem}-verify.*"))
        for sv in sibling_verify:
            violations.append({
                "rule": "own#6",
                "path": str(sv.relative_to(REPO_ROOT)),
                "detail": "separate verify file forbidden — inline into paper .md",
            })
    return violations


# own#7 registry-sync-on-tool-add — every bridge/origins/<name>/ must appear in bridge/_registry.json.
def check_rule_7_registry_sync() -> list[dict[str, str]]:
    violations: list[dict[str, str]] = []
    origins = REPO_ROOT / "bridge" / "origins"
    registry = REPO_ROOT / "bridge" / "_registry.json"
    if not origins.is_dir() or not registry.is_file():
        return violations
    try:
        reg_raw = registry.read_text(encoding="utf-8")
        reg_data = json.loads(reg_raw)
    except Exception as exc:
        violations.append({
            "rule": "own#7",
            "path": str(registry.relative_to(REPO_ROOT)),
            "detail": f"registry parse failed: {exc}",
        })
        return violations
    # Build a flat set of referenced origin dir names by scanning the raw text for
    # 'origins/<name>/' tokens — robust to nested schema shapes.
    ref_names = set(re.findall(r"origins/([^/\"]+)/", reg_raw))
    for origin_dir in sorted(origins.iterdir()):
        if not origin_dir.is_dir() or origin_dir.name.startswith("_") or origin_dir.name.startswith("."):
            continue
        if origin_dir.name not in ref_names:
            violations.append({
                "rule": "own#7",
                "path": str(origin_dir.relative_to(REPO_ROOT)),
                "detail": "bridge origin absent from bridge/_registry.json",
            })
    _ = reg_data  # kept for future structured validation
    return violations


# own#11 bt-solution-claim-ban — scan public docs for 'solved' / '해결' next to BT-5xx tokens.
BT_CLAIM_RE = re.compile(
    r"(BT[-\s]?5(4[1-37]|45|46|47))[^.\n]{0,120}?(solved|solution|resolved|해결)",
    re.IGNORECASE,
)


def check_rule_11_bt_solution_ban() -> list[dict[str, str]]:
    violations: list[dict[str, str]] = []
    scan_targets = [
        REPO_ROOT / "README.md",
        REPO_ROOT / "CONTRIBUTING.md",
    ]
    # Also scan papers/*.md — claims there would be public.
    papers_root = REPO_ROOT / "papers"
    if papers_root.is_dir():
        scan_targets.extend(sorted(papers_root.glob("*.md")))
    for path in scan_targets:
        if not path.is_file():
            continue
        text = path.read_text(encoding="utf-8", errors="replace")
        for m in BT_CLAIM_RE.finditer(text):
            violations.append({
                "rule": "own#11",
                "path": str(path.relative_to(REPO_ROOT)),
                "detail": f"Millennium solve-claim pattern: {m.group(0)[:80]!r}",
            })
    return violations


# own#16 md-domain-registry-required — domains/_index.json must list every <axis>/<name>.
def check_rule_16_domain_registry() -> list[dict[str, str]]:
    violations: list[dict[str, str]] = []
    domains_root = REPO_ROOT / "domains"
    index = domains_root / "_index.json"
    if not index.is_file():
        violations.append({
            "rule": "own#16",
            "path": "domains/_index.json",
            "detail": "canonical index missing",
        })
        return violations
    try:
        idx_text = index.read_text(encoding="utf-8")
        idx_data = json.loads(idx_text)  # noqa: F841 — structured access optional
    except Exception as exc:
        violations.append({
            "rule": "own#16",
            "path": "domains/_index.json",
            "detail": f"index parse failed: {exc}",
        })
        return violations
    # Each axis should also have its own sub-index.
    for axis_dir in sorted(domains_root.iterdir()):
        if not axis_dir.is_dir() or axis_dir.name.startswith("_"):
            continue
        sub_index = axis_dir / "_index.json"
        if not sub_index.is_file():
            violations.append({
                "rule": "own#16",
                "path": str(sub_index.relative_to(REPO_ROOT)),
                "detail": f"axis '{axis_dir.name}' missing _index.json sub-SSOT",
            })
            continue
        try:
            sub_text = sub_index.read_text(encoding="utf-8")
        except Exception as exc:
            violations.append({
                "rule": "own#16",
                "path": str(sub_index.relative_to(REPO_ROOT)),
                "detail": f"sub-index read failed: {exc}",
            })
            continue
        for domain_dir in sorted(axis_dir.iterdir()):
            if not domain_dir.is_dir() or domain_dir.name.startswith("_"):
                continue
            # Look for the domain name as a JSON-quoted token in the sub-index.
            if f'"{domain_dir.name}"' not in sub_text:
                violations.append({
                    "rule": "own#16",
                    "path": str(domain_dir.relative_to(REPO_ROOT)),
                    "detail": f"domain '{domain_dir.name}' not listed in {axis_dir.name}/_index.json",
                })
    return violations


# -----------------------------------------------------------------------------
# Orchestrator
# -----------------------------------------------------------------------------

CHECKERS = [
    ("own#1", lambda rules: check_rule_1_doc_english(rules)),
    ("own#3", lambda rules: check_rule_3_one_doc_per_domain()),
    ("own#4", lambda rules: check_rule_4_fifteen_sections()),
    ("own#6", lambda rules: check_rule_6_paper_verify_embedded()),
    ("own#7", lambda rules: check_rule_7_registry_sync()),
    ("own#11", lambda rules: check_rule_11_bt_solution_ban()),
    ("own#16", lambda rules: check_rule_16_domain_registry()),
]


def main(argv: list[str]) -> int:
    rules = parse_own_file(OWN_FILE)
    all_violations: list[dict[str, str]] = []
    per_rule_counts: dict[str, int] = {}
    for rule_id, fn in CHECKERS:
        try:
            found = fn(rules)
        except Exception as exc:  # defensive — no single checker should abort the rest
            found = [{"rule": rule_id, "path": "(checker)", "detail": f"internal error: {exc}"}]
        per_rule_counts[rule_id] = len(found)
        all_violations.extend(found)

    manual_rule_slugs = {
        f"own#{rid}": rules.get(rid, {}).get("slug", "(missing)")
        for rid in sorted(MANUAL_ONLY)
    }

    report = {
        "_meta": {
            "tool": "own_doc_lint.py",
            "ssot": ".own (repo-root)",
            "target_rules": [f"own#{n}" for n in TARGET_RULES],
            "auto_verifiable": [f"own#{n}" for n in sorted(AUTO_VERIFIABLE)],
            "manual_only": manual_rule_slugs,
            "note": "own#13/#15/#17..#21 handled by existing verifiers — not re-checked here.",
        },
        "summary": {
            "total_violations": len(all_violations),
            "per_rule": per_rule_counts,
        },
        "violations": all_violations,
    }

    REPORT_PATH.parent.mkdir(parents=True, exist_ok=True)
    REPORT_PATH.write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8")

    # Human-readable stdout summary.
    print(f"[own_doc_lint] target rules: {[f'own#{n}' for n in TARGET_RULES]}")
    print(f"[own_doc_lint] auto-verifiable: {[f'own#{n}' for n in sorted(AUTO_VERIFIABLE)]}")
    print(f"[own_doc_lint] manual-only    : {[f'own#{n}' for n in sorted(MANUAL_ONLY)]}")
    for rule_id, count in per_rule_counts.items():
        marker = "OK" if count == 0 else f"FAIL ({count})"
        print(f"  {rule_id:<8} {marker}")
    print(f"[own_doc_lint] report: {REPORT_PATH.relative_to(REPO_ROOT)}")
    print(f"[own_doc_lint] total violations: {len(all_violations)}")

    return 1 if all_violations else 0


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv))
    except Exception as exc:  # pragma: no cover
        print(f"[own_doc_lint] FATAL: {exc}", file=sys.stderr)
        sys.exit(2)
