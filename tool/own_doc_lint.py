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
import subprocess
import sys
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parent.parent
OWN_FILE = REPO_ROOT / ".own"
REPORT_PATH = REPO_ROOT / "reports" / "n6_own_doc_lint.json"
# own#1 HARD-block allowlist sidecar — loaded lazily by check_rule_1.
# (own#4/#6/#12 HARD-block legacy allowlists are inline below — < 50 entries
#  each, see OWN4/OWN6/OWN12_LEGACY_ALLOWLIST.)
OWN1_ALLOWLIST_PATH = REPO_ROOT / "tool" / "own1_legacy_allowlist.json"

# HARD-block rule set. Promoted in two waves:
#   2026-04-24 wave 1 — own#1 (per user 'hard go' directive)
#   2026-04-24 wave 2 — own#2/#3/#4/#5/#6/#7/#8/#9/#10/#11/#12/#16
#                       (per user 'hard!!!' aggressive promotion directive)
# Any HARD violation causes overall exit 1 regardless of other rules.
# Rules with active violations on promotion day were grandfathered via
# OWN{4,6,12}_LEGACY_ALLOWLIST inline sets below — new files outside
# those allowlists must comply immediately.
HARD_RULES: set[int] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 16, 29}

# Target rule IDs — the 14 DOC-ONLY rules this linter enforces.
TARGET_RULES = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 16, 29]
# Note: own#14 (readme-sealed) lives in readme_sealed_check.py.
# own#13/#15/#17..#21 already have runtime verifiers (ouroboros detector, .doc-rules, hexa tools).

# Classification — which of the 14 DOC-ONLY rules we can auto-lint.
# Phase 2 (2026-04-24): extended heuristic coverage to own#2/#5/#8/#9/#10/#12.
# These checks are intentionally conservative — false positives are preferred
# over false negatives and each flag is a review signal, not a hard fact.
AUTO_VERIFIABLE = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 16, 29}
MANUAL_ONLY: set[int] = set()
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
                # Keys that should always be lists (own 29 multi-section spec etc.)
                LIST_KEYS = {"mandatory-sections", "optional-sections"}
                if key in LIST_KEYS:
                    if key not in current:
                        current[key] = []
                    current[key].append(value)
                elif key in current and isinstance(current[key], str):
                    # Multiple fields with same name (e.g. decl) — string concat.
                    current[key] = current[key] + " " + value
                elif key in current and isinstance(current[key], list):
                    current[key].append(value)
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


# own#1 doc-english-required — HARD block (promoted 2026-04-24 via 'hard go').
#
# Scope (matches .own own#1 decl): repo-root public docs + full decl path set
# (domains/, theory/, reports/, experiments/, papers/, bridge/, n6shared/,
# proposals/) + CONTRIBUTING.md.
# Threshold: CJK density 0% — not even a single Korean/Japanese/Han character
# in a new .md file under these paths. Matches own#17's standard for root
# READMEs; now extended project-wide to the documented corpus + proposals +
# contributor docs (2026-04-24 'hard!!! 모두 english only' directive).
#
# Legacy files (>1k existing CJK-containing .md docs as of promotion date)
# are grandfathered via OWN1_LEGACY_ALLOWLIST — a frozen sidecar JSON that
# must be pruned (not grown) over time. Pattern reuses own#5 OWN5_LEGACY_ALLOWLIST
# approach. The allowlist is FROZEN and SHRINK-ONLY: existing entries may be
# removed once the underlying file is translated, but no new grandfathers may
# be added. New CJK .md files under own#1 decl scope will fail the HARD check
# and block merges.
CJK_RE = re.compile(r"[぀-ヿ㐀-䶿一-鿿가-힯]")

# own#1 decl scope directories (relative to repo root). These are the
# directories where .md files must be English-only going forward.
# Scope expanded 2026-04-24 (user 'hard!!! 모두 english only' directive) —
# proposals/ added so contributor proposals also fall under HARD English-only.
OWN1_SCOPE_DIRS = (
    "domains",
    "theory",
    "reports",
    "experiments",
    "papers",
    "bridge",
    "n6shared",
    "proposals",
)
# Root-level docs that own#1 also enforces. Root README.md is governed by
# own#17 (public-readme-english-only) and is intentionally excluded here to
# avoid double-jeopardy. CONTRIBUTING.md is added 2026-04-24 per the
# 'hard!!! 모두 english only' directive — contributor-facing root docs are
# project scope, not just internal corpus.
OWN1_ROOT_DOCS: tuple[str, ...] = ("CONTRIBUTING.md",)


def _load_own1_allowlist() -> set[str]:
    """Load the frozen grandfather list of legacy CJK .md files.

    Sidecar: tool/own1_legacy_allowlist.json (generated 2026-04-24 from pre-scan
    of CJK-containing .md files in own#1 decl scope). Missing file is treated
    as an empty set — the HARD check then flags everything, which is the
    intended fail-loud behaviour if the allowlist is accidentally removed.

    Policy verification (advisory, log-only — does not fail the lint):
      * `_meta.policy` should mention FROZEN or SHRINK-ONLY. Anything else is a
        sign the allowlist policy doc has been weakened — emit a warning.
      * `_meta.count` should match the actual entry count. A mismatch suggests
        an un-recorded mutation (likely an unsanctioned addition under the
        shrink-only policy); warn but do not fail (operational flexibility —
        the lint must keep running so that translation work can land).
    The shrink-only policy is enforced socially / via PR review; these are
    observability hooks, not gates.
    """
    if not OWN1_ALLOWLIST_PATH.is_file():
        return set()
    try:
        data = json.loads(OWN1_ALLOWLIST_PATH.read_text(encoding="utf-8"))
        entries = data.get("allowlist", []) if isinstance(data, dict) else []
        meta = data.get("_meta", {}) if isinstance(data, dict) else {}
        # Policy presence check — log-only.
        policy = str(meta.get("policy", "")).upper()
        if "FROZEN" not in policy and "SHRINK-ONLY" not in policy and "SHRINK_ONLY" not in policy:
            print(
                "[own_doc_lint] WARN own1 allowlist _meta.policy missing FROZEN/SHRINK-ONLY marker — "
                "review tool/own1_legacy_allowlist.json",
                file=sys.stderr,
            )
        # Count parity check — log-only.
        declared = meta.get("count")
        actual = len(entries) if isinstance(entries, list) else 0
        if isinstance(declared, int) and declared != actual:
            print(
                f"[own_doc_lint] WARN own1 allowlist _meta.count={declared} but actual entries={actual} — "
                f"possible un-recorded mutation (shrink-only policy)",
                file=sys.stderr,
            )
        return {str(p) for p in entries if isinstance(p, str) and p.strip()}
    except Exception:
        return set()


OWN1_LEGACY_ALLOWLIST: set[str] = _load_own1_allowlist()


def check_rule_1_doc_english(rules: dict[int, dict[str, Any]]) -> list[dict[str, str]]:
    """own#1 HARD enforcement — 0% CJK in .md under decl scope (new files).

    Legacy files listed in OWN1_LEGACY_ALLOWLIST are grandfathered (no flag).
    Any other .md with one or more CJK chars is a violation — this is HARD,
    violations trigger process exit 1 via main().
    """
    violations: list[dict[str, str]] = []
    seen: set[Path] = set()

    # Root-level public docs (missing-file check retained for own#17 overlap).
    for name in OWN1_ROOT_DOCS:
        path = REPO_ROOT / name
        if not path.is_file():
            violations.append({
                "rule": "own#1",
                "path": name,
                "detail": "required English-primary doc is missing",
            })
            continue
        seen.add(path.resolve())
        rel = str(path.relative_to(REPO_ROOT))
        cjk_count = _count_cjk(path)
        if cjk_count > 0 and rel not in OWN1_LEGACY_ALLOWLIST:
            violations.append({
                "rule": "own#1",
                "path": rel,
                "detail": f"CJK present ({cjk_count} chars) — English-only required",
            })

    # Full own#1 decl scope — every .md under the seven listed roots.
    for scope in OWN1_SCOPE_DIRS:
        root = REPO_ROOT / scope
        if not root.is_dir():
            continue
        for md in _walk_md(root):
            resolved = md.resolve()
            if resolved in seen:
                continue
            seen.add(resolved)
            rel = str(md.relative_to(REPO_ROOT))
            if rel in OWN1_LEGACY_ALLOWLIST:
                # Grandfathered legacy file — acknowledged exception.
                continue
            cjk_count = _count_cjk(md)
            if cjk_count > 0:
                violations.append({
                    "rule": "own#1",
                    "path": rel,
                    "detail": (
                        f"CJK present ({cjk_count} chars) — new .md must be "
                        f"English-only (own#1 HARD since 2026-04-24)"
                    ),
                })
    return violations


def _count_cjk(path: Path) -> int:
    try:
        text = path.read_text(encoding="utf-8", errors="replace")
    except Exception:
        return 0
    return len(CJK_RE.findall(text))


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

# Legacy domain bodies grandfathered when own#4 was promoted to HARD on
# 2026-04-24. Each entry was an existing canonical doc that pre-dated the
# 15-section template requirement. New domain bodies (or edits that drop
# sections from non-allowlisted files) must comply with §1..§15 immediately.
# Frozen set — entries should be REMOVED (after compliance fix), never added.
OWN4_LEGACY_ALLOWLIST: set[str] = {
    "domains/culture/ethnomusicology/ethnomusicology.md",
    "domains/infra/forensic-science/forensic-science.md",
    "domains/life/gastrointestinal-medicine/gastrointestinal-medicine.md",
    "domains/life/music-therapy/music-therapy.md",
    "domains/life/nuclear-medicine/nuclear-medicine.md",
    "domains/life/radiation-biology/radiation-biology.md",
    "domains/life/sleep-medicine/sleep-medicine.md",
    "domains/life/tibetan-medicine/tibetan-medicine.md",
    "domains/life/urban-farming/urban-farming.md",
    "domains/physics/computational-fluid-dynamics/computational-fluid-dynamics.md",
    "domains/space/astrobiology/astrobiology.md",
    "domains/space/astrodynamics/astrodynamics.md",
    "domains/space/space-medicine/space-medicine.md",
}


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
            rel = str(body.relative_to(REPO_ROOT))
            if rel in OWN4_LEGACY_ALLOWLIST:
                # Grandfathered — pre-HARD-promotion legacy file.
                continue
            text = body.read_text(encoding="utf-8", errors="replace")
            present = {int(m.group(1)) for m in SECTION_HEAD_RE.finditer(text)}
            missing = [n for n in range(1, 16) if n not in present]
            if missing:
                violations.append({
                    "rule": "own#4",
                    "path": rel,
                    "detail": f"missing sections: {missing}",
                })
    return violations


# own#6 paper-verify-embedded — papers/*.md must contain an embedded ```python block.
#
# Legacy papers grandfathered when own#6 was promoted to HARD on 2026-04-24.
# Entries are existing papers/ docs that pre-dated the embedded-verify mandate
# (or were drafted under the old verify-script-sibling pattern). New papers
# under papers/ must inline a ```python verify block immediately. Frozen set.
OWN6_LEGACY_ALLOWLIST: set[str] = {
    "papers/M10star-21-unified-theorem-2026-04-15.md",
    "papers/bernoulli-18-arxiv-stub-2026-04-15.md",
    "papers/bernoulli-b6-sign-2026-04-22.md",
    "papers/consciousness-measurement-protocol-2026-04-15.md",
    "papers/consciousness-red-team-n6-failure-2026-04-15.md",
    "papers/doob-talagrand-tau-2026-04-22.md",
    "papers/embody-p10-1-l13-l14-unified-spec-2026-04-15.md",
    "papers/embody-p11-2-nanobot-gen2-2026-04-15.md",
    "papers/embody-p12-2-quantum-sensor-design-2026-04-15.md",
    "papers/embody-p13-1-qkd-6state-design-2026-04-15.md",
    "papers/enriques-abelian-6fold-link-2026-04-22.md",
    "papers/lemmas-A3-A4-conditional-2026-04-15.md",
    "papers/monte-carlo-control-e-2026-04-22.md",
    "papers/n=6-convergence-80-domains-2026-04-19.md",
    "papers/plunnecke-6-2026-04-22.md",
    "papers/yang-mills-beta0-rewriting-2026-04-22.md",
}


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
        rel = str(path.relative_to(REPO_ROOT))
        is_grandfathered = rel in OWN6_LEGACY_ALLOWLIST
        text = path.read_text(encoding="utf-8", errors="replace")
        if "```python" not in text and not is_grandfathered:
            violations.append({
                "rule": "own#6",
                "path": rel,
                "detail": "paper missing embedded ```python verify block",
            })
        # Separate verify scripts forbidden — applies to ALL papers, including
        # grandfathered ones (the prohibition itself is independent of the
        # embedded-block requirement).
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
# Phase-2 heuristic checkers (own#2/#5/#8/#9/#10/#12).
# These trade some false-positive risk for 100% automation coverage — every
# flag below is a "review signal" the maintainer should eyeball, not a hard
# factual violation. Tune thresholds as the corpus stabilises.
# -----------------------------------------------------------------------------


# own#2 n6-master-identity — constants σ(6)=12, φ(6)=2, τ(6)=4, J₂=24 must be
# consistent wherever they are declared as standalone equalities. We extract
# `NAME(6) = VALUE` or `J₂ = VALUE` patterns and report any file that contradicts
# the canonical value. Multi-term products (σ·J₂=288) and expressions with
# additional operators are excluded to avoid false positives.
IDENTITY_CANONICAL = {
    "sigma(6)": 12,
    "phi(6)": 2,
    "tau(6)": 4,
    "J2": 24,
}

# Single-term identity-constant declarations. Excludes lines containing '·', '*',
# '+', '-', '/' between the symbol and the '=' to skip product/composite forms
# like "σ·J₂=288" or "σ²=144". We also require the captured number to be
# *terminal* within its term — if followed by ' + ', ' - ', ' * ', '·', '×',
# ' / ', '^' the match is a mid-expression coincidence (e.g. 'σ(6)=1+2+3+6')
# and is discarded.
_IDENT_SINGLE_RE = re.compile(
    r"(?P<name>σ\(6\)|sigma\(6\)|φ\(6\)|phi\(6\)|τ\(6\)|tau\(6\)|J[₂2])"
    r"\s*=\s*(?P<value>\d{1,4})\b"
)
_IDENT_COMPOSITE_GUARD_RE = re.compile(r"[·*×/+\-]")
# After the numeric value — reject if followed by an arithmetic continuation.
_IDENT_TERM_CONT_RE = re.compile(r"^\s*[+\-*/·×^]")


def _normalise_ident_name(raw: str) -> str:
    mapping = {
        "σ(6)": "sigma(6)", "sigma(6)": "sigma(6)",
        "φ(6)": "phi(6)",   "phi(6)": "phi(6)",
        "τ(6)": "tau(6)",   "tau(6)": "tau(6)",
        "J₂": "J2",         "J2": "J2",
    }
    return mapping.get(raw, raw)


def check_rule_2_master_identity() -> list[dict[str, str]]:
    violations: list[dict[str, str]] = []
    # Scan chip-* domain bodies + CLAUDE.md + root docs (per .own decl scope).
    scan_targets: list[Path] = [
        REPO_ROOT / "CLAUDE.md",
        REPO_ROOT / "README.md",
    ]
    chip_root = REPO_ROOT / "domains" / "compute"
    if chip_root.is_dir():
        for d in sorted(chip_root.iterdir()):
            if d.is_dir() and d.name.startswith("chip-"):
                for f in d.glob("*.md"):
                    scan_targets.append(f)
    theory_proofs = REPO_ROOT / "theory" / "proofs"
    if theory_proofs.is_dir():
        for f in sorted(theory_proofs.glob("theorem-r*.md")):
            scan_targets.append(f)

    for path in scan_targets:
        if not path.is_file():
            continue
        try:
            text = path.read_text(encoding="utf-8", errors="replace")
        except Exception:
            continue
        for lineno, line in enumerate(text.splitlines(), start=1):
            # Skip lines with composite operators around the symbol — these are
            # derived expressions (σ·φ=24, σ·J₂=288, σ²=144, etc.) not base
            # identity assertions.
            for m in _IDENT_SINGLE_RE.finditer(line):
                # Look at the 12 chars before the match for composite operators.
                prefix = line[max(0, m.start() - 12):m.start()]
                if _IDENT_COMPOSITE_GUARD_RE.search(prefix):
                    continue
                # Reject mid-expression coincidences: if the captured number
                # is immediately followed by an arithmetic operator (+ - * · × / ^)
                # the RHS is a sum/product, not a standalone identity constant.
                tail = line[m.end():]
                if _IDENT_TERM_CONT_RE.match(tail):
                    continue
                name = _normalise_ident_name(m.group("name"))
                value = int(m.group("value"))
                canonical = IDENTITY_CANONICAL.get(name)
                if canonical is not None and value != canonical:
                    violations.append({
                        "rule": "own#2",
                        "path": str(path.relative_to(REPO_ROOT)),
                        "detail": (
                            f"line {lineno}: {name}={value} contradicts canonical "
                            f"{name}={canonical} (context: {line.strip()[:80]!r})"
                        ),
                    })
    return violations


# own#5 theory-report-separation — theory/ should host permanent theory (no
# date/version suffix) while reports/ carries timestamped snapshots. We flag
# theory/**/*.md filenames matching typical report-suffix patterns.
_DATE_SUFFIX_RE = re.compile(
    r"(?:^|[-_])(?:20\d{2}[-_]\d{2}[-_]\d{2}|20\d{6}|v\d+[-_]report|session[-_]\d+)"
    r"(?:\.md)?$",
    re.IGNORECASE,
)
_THEORY_FILENAME_WHITELIST = {"README.md", "CLAUDE.md", "INDEX.md", "_INDEX.md"}

# Legacy theory snapshot files grandfathered in 2026-04-24. These were created
# before own#5 was tightened to forbid date/version suffixes under theory/.
# Rather than risk breaking existing cross-links (papers, atlas, external
# citations), we whitelist them as acknowledged exceptions. Any NEW dated file
# must land under reports/ — this set is frozen.
#
# Paths are relative to repo root and must match exactly (str comparison).
OWN5_LEGACY_ALLOWLIST: set[str] = {
    "theory/proofs/transcend-p11-3-ouroboros-b2-proof-2026-04-15.md",
    "theory/proofs/formal-p13-1-bsd-n6-2026-04-15.md",
    "theory/proofs/formal-p11-2-hodge-n6-2026-04-15.md",
    "theory/proofs/n6-boundary-metatheory-2026-04-14.md",
    "theory/proofs/mk4-trident-final-verdict-2026-04-15.md",
    "theory/proofs/l11-l15-quantum-nuclear-mapping-2026-04-14.md",
    "theory/proofs/fisher-ouroboros-reformulation-2026-04-15.md",
    "theory/proofs/ouroboros-alpha-universality-2026-04-15.md",
    "theory/proofs/attractor-meta-theorem-2026-04-11.md",
    "theory/proofs/attractor-meta-theorem-extended-2026-04-14.md",
    "theory/proofs/bernoulli-boundary-2026-04-11.md",
    "theory/proofs/formal-p12-2-cy3-hodge-retry-2026-04-15.md",
    "theory/proofs/formal-p10-1-riemann-sigma-tau-2026-04-15.md",
    "theory/proofs/mk4-theorem-candidates-2026-04-14.md",
    "theory/proofs/formal-p11-1-selberg-ingham-2026-04-15.md",
    "theory/proofs/formal-p12-1-conrey-gonek-6th-moment-2026-04-15.md",
    "theory/preprints/millennium-v3-preprint-draft-2026-04-16.md",
    "theory/roadmap-v2/millennium-v4-design-2026-04-16.md",
    "theory/roadmap-v2/millennium-v3-design-2026-04-15.md",
}


def check_rule_5_theory_report_separation() -> list[dict[str, str]]:
    violations: list[dict[str, str]] = []
    theory_root = REPO_ROOT / "theory"
    if theory_root.is_dir():
        for md in _walk_md(theory_root):
            if md.name in _THEORY_FILENAME_WHITELIST:
                continue
            rel = str(md.relative_to(REPO_ROOT))
            if rel in OWN5_LEGACY_ALLOWLIST:
                # Legacy snapshot — grandfathered exception. New files under
                # theory/ must not carry date/version suffixes.
                continue
            # Stem without extension for suffix matching.
            stem = md.stem
            if _DATE_SUFFIX_RE.search(stem):
                violations.append({
                    "rule": "own#5",
                    "path": str(md.relative_to(REPO_ROOT)),
                    "detail": (
                        "theory/ file carries date/version suffix — move timestamped "
                        "snapshot to reports/ or rename to a permanent theory title"
                    ),
                })
    return violations


# own#8 lens-ssot-external — lens references in .md should point at the external
# nexus SSOT ($NEXUS/shared/lenses/...) rather than the repo-local deprecated
# bridge/src/telescope/lenses/ path. We flag two classes of suspect references:
#   (a) any <bridge/src/telescope/lenses/*.rs> mention in .md (legacy citation)
#   (b) markdown links of the form `[...](lenses/...)` that resolve to an
#       internal relative path instead of the external SSOT.
_LEGACY_LENS_REF_RE = re.compile(r"bridge/src/telescope/lenses/[A-Za-z0-9_./-]+\.rs")
_INTERNAL_LENS_LINK_RE = re.compile(r"\]\((?:\./)?lenses/[^\)]+\)")
_EXTERNAL_LENS_OK_RE = re.compile(r"\$NEXUS/shared/lenses|nexus/shared/lenses")


def check_rule_8_lens_ssot_external() -> list[dict[str, str]]:
    violations: list[dict[str, str]] = []
    # Scope: repo-internal .md under /bridge, /theory, /papers, /domains, root.
    scan_roots = [
        REPO_ROOT,
        REPO_ROOT / "bridge",
        REPO_ROOT / "theory",
        REPO_ROOT / "papers",
        REPO_ROOT / "domains",
    ]
    seen: set[Path] = set()
    for root in scan_roots:
        if not root.is_dir():
            continue
        for md in _walk_md(root):
            if md in seen:
                continue
            seen.add(md)
            try:
                text = md.read_text(encoding="utf-8", errors="replace")
            except Exception:
                continue
            for m in _LEGACY_LENS_REF_RE.finditer(text):
                # Determine line number.
                lineno = text.count("\n", 0, m.start()) + 1
                violations.append({
                    "rule": "own#8",
                    "path": str(md.relative_to(REPO_ROOT)),
                    "detail": (
                        f"line {lineno}: legacy Rust-lens reference '{m.group(0)}' — "
                        f"replace with $NEXUS/shared/lenses/<domain>_<topic>.hexa"
                    ),
                })
            for m in _INTERNAL_LENS_LINK_RE.finditer(text):
                lineno = text.count("\n", 0, m.start()) + 1
                snippet = m.group(0)
                # Skip if the same line/snippet already cites the external SSOT.
                line_start = text.rfind("\n", 0, m.start()) + 1
                line_end = text.find("\n", m.end())
                line_text = text[line_start: line_end if line_end >= 0 else len(text)]
                if _EXTERNAL_LENS_OK_RE.search(line_text):
                    continue
                violations.append({
                    "rule": "own#8",
                    "path": str(md.relative_to(REPO_ROOT)),
                    "detail": (
                        f"line {lineno}: relative lens link {snippet!r} bypasses "
                        f"$NEXUS/shared/lenses/ SSOT"
                    ),
                })
    return violations


# own#9 lens-auto-absorb-atlas — recent lens-touching commits must correspond
# to atlas.n6 growth in lens-originated entries. We compare (1) count of
# lens-tagged atlas entries and (2) recent git commits mentioning 'lens' in
# the subject. If there are fresh lens commits but atlas.n6 has zero lens
# markers, emit a warning. This is a best-effort signal — false positives are
# possible when a lens commit is purely refactor/cleanup.
_ATLAS_LENS_PATTERN_RE = re.compile(r"\blens(?:_id|_origin|:)|source\s*=\s*lens", re.IGNORECASE)
_ATLAS_INLINE_LENS_RE = re.compile(r"[-_/]lens\b", re.IGNORECASE)


def _git_log_subjects(since_days: int = 30, grep: str = "lens") -> list[str]:
    try:
        result = subprocess.run(
            [
                "git", "-C", str(REPO_ROOT),
                "log",
                f"--since={since_days} days ago",
                f"--grep={grep}",
                "-i",
                "--pretty=format:%s",
            ],
            capture_output=True, text=True, timeout=10, check=False,
        )
        if result.returncode != 0:
            return []
        return [ln for ln in result.stdout.splitlines() if ln.strip()]
    except Exception:
        return []


def check_rule_9_lens_absorb_atlas() -> list[dict[str, str]]:
    violations: list[dict[str, str]] = []
    atlas_path = REPO_ROOT / "atlas" / "atlas.n6"
    # Fallback to symlink-style locations.
    if not atlas_path.is_file():
        alt = REPO_ROOT / "n6shared" / "atlas.n6"
        if alt.is_file():
            atlas_path = alt
    if not atlas_path.is_file():
        violations.append({
            "rule": "own#9",
            "path": "atlas/atlas.n6",
            "detail": "atlas.n6 SSOT not found — cannot verify lens absorption",
        })
        return violations
    try:
        atlas_text = atlas_path.read_text(encoding="utf-8", errors="replace")
    except Exception as exc:
        violations.append({
            "rule": "own#9",
            "path": str(atlas_path.relative_to(REPO_ROOT)),
            "detail": f"atlas.n6 unreadable: {exc}",
        })
        return violations
    lens_hits = len(_ATLAS_LENS_PATTERN_RE.findall(atlas_text))
    inline_lens_hits = len(_ATLAS_INLINE_LENS_RE.findall(atlas_text))
    total_lens_signal = lens_hits + inline_lens_hits

    recent_lens_commits = _git_log_subjects(since_days=30, grep="lens")
    if recent_lens_commits and total_lens_signal == 0:
        violations.append({
            "rule": "own#9",
            "path": str(atlas_path.relative_to(REPO_ROOT)),
            "detail": (
                f"{len(recent_lens_commits)} lens-tagged commits in last 30d but "
                f"atlas.n6 contains 0 lens markers — absorption may have stalled"
            ),
        })
    # Independent invariant: atlas entries referencing lens must be non-zero
    # whenever a lens_registry exists. Emit a soft flag if lens registry has
    # entries but atlas is silent.
    lens_registry = REPO_ROOT / "n6shared" / "config" / "lens_registry.json"
    if lens_registry.is_file() and total_lens_signal == 0:
        try:
            reg_text = lens_registry.read_text(encoding="utf-8")
            if '"lens' in reg_text or "lens_id" in reg_text:
                violations.append({
                    "rule": "own#9",
                    "path": str(lens_registry.relative_to(REPO_ROOT)),
                    "detail": (
                        "lens_registry.json declares lenses but atlas.n6 has "
                        "no lens-originated entries"
                    ),
                })
        except Exception:
            pass
    return violations


# own#10 rust-lens-ban-legacy — any new .rs file under bridge/src/telescope/ or
# a top-level lenses/ path is forbidden. Current-state scan (file existence)
# plus rolling-90-day git-log scan.
_LENS_RS_PATH_RE = re.compile(
    r"^(?:bridge/src/telescope/(?:lenses/)?[^/\s]+\.rs|lenses/[^\s]+\.rs)$"
)


def _git_recent_rs_files(since_days: int = 90) -> set[str]:
    try:
        result = subprocess.run(
            [
                "git", "-C", str(REPO_ROOT),
                "log",
                f"--since={since_days} days ago",
                "--name-only",
                "--pretty=format:",
                "--diff-filter=A",  # additions only
            ],
            capture_output=True, text=True, timeout=15, check=False,
        )
        if result.returncode != 0:
            return set()
        return {ln.strip() for ln in result.stdout.splitlines() if ln.strip()}
    except Exception:
        return set()


def check_rule_10_rust_lens_ban() -> list[dict[str, str]]:
    violations: list[dict[str, str]] = []
    # (a) present-state scan.
    tele_dir = REPO_ROOT / "bridge" / "src" / "telescope"
    if tele_dir.is_dir():
        for rs in tele_dir.rglob("*.rs"):
            violations.append({
                "rule": "own#10",
                "path": str(rs.relative_to(REPO_ROOT)),
                "detail": "legacy Rust lens present in bridge/src/telescope — freeze / migrate to HEXA",
            })
    lenses_dir = REPO_ROOT / "lenses"
    if lenses_dir.is_dir():
        for rs in lenses_dir.rglob("*.rs"):
            violations.append({
                "rule": "own#10",
                "path": str(rs.relative_to(REPO_ROOT)),
                "detail": "legacy Rust lens present under /lenses — freeze / migrate to HEXA",
            })
    # (b) rolling 90-day git addition scan — catches even deleted files.
    recent = _git_recent_rs_files(since_days=90)
    for candidate in sorted(recent):
        if _LENS_RS_PATH_RE.match(candidate):
            violations.append({
                "rule": "own#10",
                "path": candidate,
                "detail": "new Rust lens added in rolling 90-day window (git history)",
            })
    return violations


# own#12 miss-criteria-declared — every experiment doc should declare MISS
# (failure) criteria ahead of data collection. We scan top-level + first-level
# experiment files for explicit markers; absence is a review signal.
_MISS_TOKEN_RE = re.compile(
    r"miss[ _-]criteria"
    r"|실패\s*(?:조건|기준)"
    r"|failure\s+criteria"
    r"|##\s*MISS\b"
    r"|###\s+MISS\b"
    r"|MISS\s*:\s*",
    re.IGNORECASE,
)
_EXPERIMENT_META_WHITELIST = {"README.md", "CLAUDE.md", "INDEX.md"}

# Legacy experiment docs grandfathered when own#12 was promoted to HARD on
# 2026-04-24. These pre-date the explicit MISS-criteria-up-front mandate
# (CONTRIBUTING.md honesty charter #3). New experiment plan/report .md files
# under experiments/, reports/experiments/, theory/experiments/ must declare a
# MISS / failure-criteria section before data collection. Frozen set.
OWN12_LEGACY_ALLOWLIST: set[str] = {
    "experiments/anu_mc_verification/promote_candidate.md",
    "experiments/atlas_promotion_p5_report.md",
    "experiments/blowup/p4_breakthrough_attempt.md",
    "experiments/blowup/p4_dfs_5module_report.md",
    "experiments/chip-verify/boot_matrix_report.md",
    "experiments/chip-verify/n6_speak_integration_bench_report.md",
    "experiments/chip-verify/soc_bench_promotion_report.md",
    "experiments/chip-verify/stage0_rerun_report.md",
    "experiments/chip-verify/verify_chain.md",
    "experiments/conjecture_to_10star_20.md",
    "experiments/dse/arch_unified_fuse50_report.md",
    "experiments/dse/atlas_promotion_candidates_p2.md",
    "experiments/dse/atlas_promotion_manual_checklist_40.md",
    "experiments/dse/cross_dse_fusion_v2_design.md",
    "experiments/dse/cross_matrix_112x10_summary.md",
    "experiments/dse/cross_matrix_v3_summary.md",
    "experiments/dse/dse_400_expansion_plan.md",
    "experiments/dse/dse_500_expansion_plan.md",
    "experiments/dse/energy_pareto_sweep_plan.md",
    "experiments/dse/p2_alien_210_plan.md",
    "experiments/grover_n6_uniqueness/paper_snippet.md",
    "experiments/paper/bipartite_audit_top10.md",
    "experiments/paper/bipartite_v2_report.md",
    "experiments/paper_ranking_p3_top48.md",
    "experiments/red_team_cycle1.md",
    "experiments/tier1_experiment_protocols.md",
}


def check_rule_12_miss_criteria() -> list[dict[str, str]]:
    violations: list[dict[str, str]] = []
    roots = [
        REPO_ROOT / "experiments",
        REPO_ROOT / "reports" / "experiments",
        REPO_ROOT / "theory" / "experiments",
    ]
    # Only scan top-level + 1-deep experiment docs to stay fast + focused on
    # human-authored plan/report files (not auto-generated artefacts).
    for root in roots:
        if not root.is_dir():
            continue
        for entry in sorted(root.iterdir()):
            candidates: list[Path] = []
            if entry.is_file() and entry.suffix == ".md":
                candidates.append(entry)
            elif entry.is_dir() and not entry.name.startswith("."):
                for sub in entry.iterdir():
                    if sub.is_file() and sub.suffix == ".md":
                        candidates.append(sub)
            for md in candidates:
                if md.name in _EXPERIMENT_META_WHITELIST:
                    continue
                rel = str(md.relative_to(REPO_ROOT))
                if rel in OWN12_LEGACY_ALLOWLIST:
                    # Grandfathered — pre-HARD-promotion legacy doc.
                    continue
                try:
                    text = md.read_text(encoding="utf-8", errors="replace")
                except Exception:
                    continue
                if not _MISS_TOKEN_RE.search(text):
                    violations.append({
                        "rule": "own#12",
                        "path": rel,
                        "detail": (
                            "no MISS / 실패 조건 / failure-criteria section detected — "
                            "declare miss_criteria before data collection"
                        ),
                    })
    return violations


# -----------------------------------------------------------------------------
# Orchestrator
# -----------------------------------------------------------------------------

def _parse_own29_section_spec(line: str) -> tuple[str, str, int, str] | None:
    """Parse a single mandatory-sections / optional-sections value:
        '<key>=<heading>|min_rows=<N>|axis=<axis_slug>'
    Returns (key, heading, min_rows, axis_slug) or None if malformed.
    """
    parts = line.split("|")
    if len(parts) < 3:
        return None
    head_part = parts[0].strip()
    if "=" not in head_part:
        return None
    key, heading = head_part.split("=", 1)
    min_rows = 4
    axis = ""
    for p in parts[1:]:
        p = p.strip()
        if p.startswith("min_rows="):
            try:
                min_rows = int(p.split("=", 1)[1])
            except ValueError:
                pass
        elif p.startswith("axis="):
            axis = p.split("=", 1)[1].strip()
    return (key.strip(), heading.strip(), min_rows, axis)


def _check_one_toolkit_section(text: str, lines: list[str], heading: str,
                               min_rows: int, axis: str,
                               mandatory: bool) -> list[dict[str, str]]:
    """Run the 4-axis toolkit table check for one registered section.

    Returns a list of violation dicts. Empty list = PASS for this section.
    For optional sections: if heading absent AND no toolkit-table-shaped
    block found via heading, returns []; otherwise full check applies.
    """
    import re as _re
    out: list[dict[str, str]] = []

    # Locate the H2 heading line; section-scoped scan starts from there.
    heading_idx = -1
    for i, ln in enumerate(lines):
        if ln == heading:
            heading_idx = i
            break
    if heading_idx < 0:
        if mandatory:
            out.append({
                "rule": "own#29",
                "path": "README.md",
                "detail": f"[mandatory] missing heading line: '{heading}' (axis={axis})",
            })
        return out  # optional + no heading → silent pass

    # heading present → header row + min_rows + doc link checks all apply
    # axis (b): header row template — first cell is "🛸" (alien-grade reach,
    # cycle 29 strengthening 2026-04-29); final cell is "Doc"; second-to-last
    # is "<X> contrast" allowing the "AlphaFold" / "Mainstream" / "Classical"
    # etc. variation. We accept any exact `| 🛸 | Tool | One-liner | Everyday
    # analogy | What it does | <something> | Doc |` shape (7 columns). Search
    # is SECTION-SCOPED: only look at lines AFTER the heading and BEFORE the
    # next H1 / H2 boundary.
    header_re = _re.compile(
        r"^\|\s*🛸\s*\|\s*Tool\s*\|\s*One-liner\s*\|\s*Everyday analogy\s*\|\s*"
        r"What it does\s*\|\s*[^|]+\|\s*Doc\s*\|\s*$"
    )
    section_end = len(lines)
    for j in range(heading_idx + 1, len(lines)):
        ln_j = lines[j]
        # Stop scan at next H1 or H2 (markdown heading boundary)
        if ln_j.startswith("# ") or ln_j.startswith("## "):
            section_end = j
            break
    header_idx = -1
    for i in range(heading_idx + 1, section_end):
        if header_re.match(lines[i]):
            header_idx = i
            break
    if header_idx < 0:
        out.append({
            "rule": "own#29",
            "path": "README.md",
            "detail": f"[{axis}] heading '{heading}' present but no 7-column toolkit header row found within section (expect leading 🛸 column)",
        })
        return out

    # axes (c)+(d)
    data_start = header_idx + 2
    data_rows: list[str] = []
    for i in range(data_start, min(data_start + 50, len(lines))):
        ln = lines[i].strip()
        if not ln:
            break
        if not ln.startswith("|"):
            break
        data_rows.append(ln)

    if len(data_rows) < min_rows:
        out.append({
            "rule": "own#29",
            "path": "README.md",
            "detail": f"[{axis}] need >={min_rows} data rows under '{heading}'; found {len(data_rows)}",
        })

    if axis:
        doc_re = _re.compile(rf"\[doc\]\((domains/{_re.escape(axis)}/[^)]+)\)")
    else:
        doc_re = _re.compile(r"\[doc\]\((domains/[^)]+)\)")
    # cycle 29 strengthening 2026-04-29: first cell = 🛸 alien-grade reach
    # integer 0-10. Validate per data row.
    reach_re = _re.compile(r"^\|\s*(\d+)\s*\|")
    for row in data_rows:
        rm = reach_re.match(row)
        if rm is None:
            out.append({
                "rule": "own#29",
                "path": "README.md",
                "detail": f"[{axis}] toolkit row missing 🛸 reach integer in first cell: {row[:80]!r}",
            })
        else:
            try:
                reach_val = int(rm.group(1))
                if reach_val < 0 or reach_val > 10:
                    out.append({
                        "rule": "own#29",
                        "path": "README.md",
                        "detail": f"[{axis}] toolkit row 🛸 reach value out of range 0-10 (got {reach_val}): {row[:80]!r}",
                    })
            except ValueError:
                out.append({
                    "rule": "own#29",
                    "path": "README.md",
                    "detail": f"[{axis}] toolkit row 🛸 reach not an integer: {row[:80]!r}",
                })
        m = doc_re.search(row)
        if m is None:
            out.append({
                "rule": "own#29",
                "path": "README.md",
                "detail": f"[{axis}] toolkit row missing [doc](domains/{axis}/...) link: {row[:80]!r}",
            })
            continue
        link_path = m.group(1)
        if not (REPO_ROOT / link_path).is_file():
            out.append({
                "rule": "own#29",
                "path": "README.md",
                "detail": f"[{axis}] toolkit doc link unresolved: {link_path}",
            })

    return out


def check_rule_29_readme_friendly_toolkit(rules: dict[int, dict[str, Any]]) -> list[dict[str, str]]:
    """own#29 HARD enforcement — README.md per-section friendly toolkit
    table contract, generalized 2026-04-29 from Biology-only to multi-section.

    Reads `mandatory-sections` and `optional-sections` arrays from .own:
      <key>=<H2 heading>|min_rows=<N>|axis=<axis_slug>

    Mandatory: heading absent → violation.
    Optional: heading absent → silent pass; heading present → full 4-axis check.

    Per-section 5-axis check (cycle 29 strengthening 2026-04-29):
      (a) H2 heading line present in README.md
      (b) 7-column header row matching template `| 🛸 | Tool | One-liner |
          Everyday analogy | What it does | <Mainstream> contrast | Doc |`
      (c) >=min_rows data rows under the table block
      (d) each data row's first cell is integer 0-10 (🛸 alien-grade reach)
      (e) each data row's last cell contains `[doc](domains/<axis>/<slug>/<slug>.md)`
          link resolving to an existing file under repo root

    Established 2026-04-29 (cycle 25) via user 'lint 강화' + 'README 의 모든
    도메인 섹션들에 반영되도록 lint 수정' directive. Cycle 29 strengthening
    2026-04-29 prepended 🛸 alien-grade reach column per user '외계인 지수 /
    현재 도달지수 / 🛸 이모지 단독 / lint 반드시 필요'.
    """
    violations: list[dict[str, str]] = []
    readme_path = REPO_ROOT / "README.md"
    if not readme_path.is_file():
        return [{
            "rule": "own#29",
            "path": "README.md",
            "detail": "README.md missing at repo root",
        }]
    text = readme_path.read_text(encoding="utf-8", errors="replace")
    lines = text.splitlines()

    rule29 = rules.get(29, {})
    mandatory_specs = rule29.get("mandatory-sections", []) or []
    optional_specs = rule29.get("optional-sections", []) or []

    if not isinstance(mandatory_specs, list):
        mandatory_specs = [mandatory_specs]
    if not isinstance(optional_specs, list):
        optional_specs = [optional_specs]

    # Process each registered section
    for spec_line in mandatory_specs:
        parsed = _parse_own29_section_spec(spec_line)
        if parsed is None:
            violations.append({
                "rule": "own#29",
                "path": ".own",
                "detail": f"malformed mandatory-sections spec: '{spec_line}'",
            })
            continue
        _, heading, min_rows, axis = parsed
        violations.extend(_check_one_toolkit_section(text, lines, heading,
                                                     min_rows, axis,
                                                     mandatory=True))

    for spec_line in optional_specs:
        parsed = _parse_own29_section_spec(spec_line)
        if parsed is None:
            violations.append({
                "rule": "own#29",
                "path": ".own",
                "detail": f"malformed optional-sections spec: '{spec_line}'",
            })
            continue
        _, heading, min_rows, axis = parsed
        violations.extend(_check_one_toolkit_section(text, lines, heading,
                                                     min_rows, axis,
                                                     mandatory=False))

    return violations


CHECKERS = [
    ("own#1", lambda rules: check_rule_1_doc_english(rules)),
    ("own#2", lambda rules: check_rule_2_master_identity()),
    ("own#3", lambda rules: check_rule_3_one_doc_per_domain()),
    ("own#4", lambda rules: check_rule_4_fifteen_sections()),
    ("own#5", lambda rules: check_rule_5_theory_report_separation()),
    ("own#6", lambda rules: check_rule_6_paper_verify_embedded()),
    ("own#7", lambda rules: check_rule_7_registry_sync()),
    ("own#8", lambda rules: check_rule_8_lens_ssot_external()),
    ("own#9", lambda rules: check_rule_9_lens_absorb_atlas()),
    ("own#10", lambda rules: check_rule_10_rust_lens_ban()),
    ("own#11", lambda rules: check_rule_11_bt_solution_ban()),
    ("own#12", lambda rules: check_rule_12_miss_criteria()),
    ("own#16", lambda rules: check_rule_16_domain_registry()),
    ("own#29", lambda rules: check_rule_29_readme_friendly_toolkit(rules)),
]


def _parse_rule_filter(argv: list[str]) -> set[int] | None:
    """Parse `--rule N` (may repeat / be comma-separated). None = all rules."""
    wanted: set[int] = set()
    i = 1
    while i < len(argv):
        arg = argv[i]
        if arg in ("--rule", "-r"):
            i += 1
            if i >= len(argv):
                break
            for tok in argv[i].split(","):
                tok = tok.strip().lstrip("#").lstrip("own").lstrip("#")
                if tok.isdigit():
                    wanted.add(int(tok))
        elif arg.startswith("--rule="):
            for tok in arg.split("=", 1)[1].split(","):
                tok = tok.strip().lstrip("#").lstrip("own").lstrip("#")
                if tok.isdigit():
                    wanted.add(int(tok))
        i += 1
    return wanted or None


def main(argv: list[str]) -> int:
    rule_filter = _parse_rule_filter(argv)
    rules = parse_own_file(OWN_FILE)
    all_violations: list[dict[str, str]] = []
    per_rule_counts: dict[str, int] = {}
    for rule_id, fn in CHECKERS:
        # rule_id looks like "own#N" — extract the numeric id for filter match.
        rid_num = int(rule_id.split("#", 1)[1])
        if rule_filter is not None and rid_num not in rule_filter:
            continue
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

    # Separate HARD (own#1, promoted 2026-04-24) from SOFT rules for exit-code logic.
    hard_violations = [v for v in all_violations if v.get("rule") in {f"own#{n}" for n in HARD_RULES}]
    soft_violations = [v for v in all_violations if v not in hard_violations]

    report = {
        "_meta": {
            "tool": "own_doc_lint.py",
            "ssot": ".own (repo-root)",
            "target_rules": [f"own#{n}" for n in TARGET_RULES],
            "auto_verifiable": [f"own#{n}" for n in sorted(AUTO_VERIFIABLE)],
            "manual_only": manual_rule_slugs,
            "hard_rules": [f"own#{n}" for n in sorted(HARD_RULES)],
            "rule_filter": sorted(rule_filter) if rule_filter else None,
            "note": "own#13/#15/#17..#21 handled by existing verifiers — not re-checked here.",
        },
        "summary": {
            "total_violations": len(all_violations),
            "hard_violations": len(hard_violations),
            "soft_violations": len(soft_violations),
            "per_rule": per_rule_counts,
        },
        "violations": all_violations,
    }

    # Only write the full report when running the unfiltered pass — targeted
    # `--rule N` invocations are for CI / local dev and should not clobber the
    # canonical report with partial data.
    if rule_filter is None:
        REPORT_PATH.parent.mkdir(parents=True, exist_ok=True)
        REPORT_PATH.write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8")

    # Human-readable stdout summary.
    filter_label = (
        f"[own_doc_lint] rule filter : {sorted(rule_filter)}"
        if rule_filter else
        f"[own_doc_lint] rule filter : (none — full run)"
    )
    print(filter_label)
    print(f"[own_doc_lint] target rules: {[f'own#{n}' for n in TARGET_RULES]}")
    print(f"[own_doc_lint] auto-verifiable: {[f'own#{n}' for n in sorted(AUTO_VERIFIABLE)]}")
    print(f"[own_doc_lint] manual-only    : {[f'own#{n}' for n in sorted(MANUAL_ONLY)]}")
    print(f"[own_doc_lint] HARD rules     : {[f'own#{n}' for n in sorted(HARD_RULES)]}")
    for rule_id, count in per_rule_counts.items():
        marker = "OK" if count == 0 else f"FAIL ({count})"
        print(f"  {rule_id:<8} {marker}")
    if rule_filter is None:
        print(f"[own_doc_lint] report: {REPORT_PATH.relative_to(REPO_ROOT)}")
    print(
        f"[own_doc_lint] total violations: {len(all_violations)} "
        f"(HARD={len(hard_violations)}, SOFT={len(soft_violations)})"
    )

    # Exit policy (2026-04-24 'hard!!!' promotion):
    #   - HARD violations (any of the 13 rules in HARD_RULES) -> exit 1 (merge
    #     blocker). All auto-verifiable own rules are now HARD.
    #   - SOFT-only violations -> exit 1 preserved for backward compat (there
    #     are no remaining auto-verifiable SOFT rules at this time, but the
    #     branch is kept so the contract stays "any violation => exit 1").
    # Overall contract: zero violations => 0, else 1.
    # The HARD/SOFT split is exposed in the JSON report for triage dashboards.
    return 1 if all_violations else 0


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv))
    except Exception as exc:  # pragma: no cover
        print(f"[own_doc_lint] FATAL: {exc}", file=sys.stderr)
        sys.exit(2)
