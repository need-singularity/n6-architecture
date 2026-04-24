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

# Target rule IDs — the 14 DOC-ONLY rules this linter enforces.
TARGET_RULES = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 16]
# Note: own#14 (readme-sealed) lives in readme_sealed_check.py.
# own#13/#15/#17..#21 already have runtime verifiers (ouroboros detector, .doc-rules, hexa tools).

# Classification — which of the 14 DOC-ONLY rules we can auto-lint.
# Phase 2 (2026-04-24): extended heuristic coverage to own#2/#5/#8/#9/#10/#12.
# These checks are intentionally conservative — false positives are preferred
# over false negatives and each flag is a review signal, not a hard fact.
AUTO_VERIFIABLE = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 16}
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


def check_rule_5_theory_report_separation() -> list[dict[str, str]]:
    violations: list[dict[str, str]] = []
    theory_root = REPO_ROOT / "theory"
    if theory_root.is_dir():
        for md in _walk_md(theory_root):
            if md.name in _THEORY_FILENAME_WHITELIST:
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
                try:
                    text = md.read_text(encoding="utf-8", errors="replace")
                except Exception:
                    continue
                if not _MISS_TOKEN_RE.search(text):
                    violations.append({
                        "rule": "own#12",
                        "path": str(md.relative_to(REPO_ROOT)),
                        "detail": (
                            "no MISS / 실패 조건 / failure-criteria section detected — "
                            "declare miss_criteria before data collection"
                        ),
                    })
    return violations


# -----------------------------------------------------------------------------
# Orchestrator
# -----------------------------------------------------------------------------

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
