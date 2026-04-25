#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
proposal_lint.py — naming + section-merge lint for proposals/*.md.

Two HARD rules:

  proposal#1 — filename naming by category
    category=identity     → kebab-case, NO date suffix
    category=operational  → snake_case + _YYYY_MM_DD suffix
    category=phased       → kebab-case + -YYYY-MM-DD suffix

    If frontmatter `category:` is missing, it is inferred from the filename:
      * filename matching <name>_YYYY_MM_DD.md  (snake_case + underscore date)  → operational
      * filename matching <name>-YYYY-MM-DD.md  (kebab-case + dash date)        → phased
      * filename matching pure kebab-case with no date                          → identity

  proposal#2 — umbrella consolidation
    If frontmatter declares `umbrella: <name>`, the file must NOT carry multiple
    H2 content sections whose first words differ. Such a file should consolidate
    those domains into a single `## <umbrella> (N bets / consolidated)` block.

Calling convention:
    python3 tool/proposal_lint.py                        # lint every proposals/*.md
    python3 tool/proposal_lint.py proposals/foo.md       # single file

Exit code: 0 when zero HARD violations, 1 otherwise.
Stdlib only — no third-party dependencies.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path
from typing import Iterable

REPO_ROOT = Path(__file__).resolve().parent.parent
PROPOSALS_DIR = REPO_ROOT / "proposals"

# -----------------------------------------------------------------------------
# Filename pattern detection
# -----------------------------------------------------------------------------

# Pure kebab-case identifier (lowercase letters / digits / single hyphens, no
# underscores, no date). Used for `identity` filenames.
_KEBAB_NO_DATE_RE = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
# Operational: snake_case stem ending with _YYYY_MM_DD. The leading stem itself
# uses snake_case (lowercase letters / digits / underscores; no hyphens).
_SNAKE_DATE_SUFFIX_RE = re.compile(
    r"^[a-z0-9]+(?:_[a-z0-9]+)*_\d{4}_\d{2}_\d{2}$"
)
# Phased: kebab-case stem ending with -YYYY-MM-DD.
_KEBAB_DATE_SUFFIX_RE = re.compile(
    r"^[a-z0-9]+(?:-[a-z0-9]+)*-\d{4}-\d{2}-\d{2}$"
)
# Loose date detection — used to flag "identity has a date suffix" violations.
_ANY_DATE_SUFFIX_RE = re.compile(r"[-_]\d{4}[-_]\d{2}[-_]\d{2}$")


def infer_category_from_filename(stem: str) -> str | None:
    """Return inferred category from filename stem, or None if unclassifiable."""
    if _SNAKE_DATE_SUFFIX_RE.match(stem):
        return "operational"
    if _KEBAB_DATE_SUFFIX_RE.match(stem):
        return "phased"
    if _KEBAB_NO_DATE_RE.match(stem):
        return "identity"
    return None


# -----------------------------------------------------------------------------
# Frontmatter parser — minimal YAML subset (key: value, one per line)
# -----------------------------------------------------------------------------

def parse_frontmatter(text: str) -> tuple[dict[str, str], str]:
    """Return ({key: value}, body) from a markdown text.

    Frontmatter is the YAML-style block delimited by lines containing only
    ``---`` at the very start of the file. Only top-level scalar `key: value`
    pairs are recognised — lists / nested mappings are ignored. The body is
    everything after the closing fence (or the entire text when no frontmatter).
    """
    if not text.startswith("---"):
        return {}, text
    lines = text.splitlines(keepends=True)
    if not lines or lines[0].strip() != "---":
        return {}, text
    fm: dict[str, str] = {}
    end_idx: int | None = None
    for i in range(1, len(lines)):
        if lines[i].strip() == "---":
            end_idx = i
            break
        # Match `key: value` (whitespace-insensitive). Allow leading whitespace
        # for resilience but ignore obviously-nested keys.
        m = re.match(r"^([A-Za-z_][A-Za-z0-9_-]*)\s*:\s*(.*?)\s*$", lines[i])
        if m:
            key, value = m.group(1), m.group(2)
            # Strip surrounding quotes if present.
            if len(value) >= 2 and value[0] == value[-1] and value[0] in ("'", '"'):
                value = value[1:-1]
            fm[key] = value
    if end_idx is None:
        return {}, text
    body = "".join(lines[end_idx + 1:])
    return fm, body


# -----------------------------------------------------------------------------
# H2 heading extraction + content/generic classification
# -----------------------------------------------------------------------------

_H2_RE = re.compile(r"^##\s+(.+?)\s*$", re.MULTILINE)

# Generic / structural H2 phrases that should NOT count as "domain content
# sections". Comparison is case-insensitive on a lowercase, parenthetical-
# stripped form of the heading.
_GENERIC_H2_PHRASES: tuple[str, ...] = (
    "executive summary",
    "closing",
    "risks",
    "open questions",
    "sequencing",
    "linked registries",
    "what would make this wrong",
    "top",  # captures "Top N", "Top 6 deployment priorities", etc.
    "why",  # captures "Why X matters", "Why now", etc.
)


def _strip_paren(text: str) -> str:
    """Drop trailing parenthetical and surrounding whitespace from a heading."""
    return re.sub(r"\s*\(.*\)\s*$", "", text).strip()


def _is_generic_heading(heading: str) -> bool:
    norm = _strip_paren(heading).lower()
    for phrase in _GENERIC_H2_PHRASES:
        # Match phrases as whole leading words — "Top 6 deployment priorities"
        # starts with "top", "Why now" starts with "why".
        if norm == phrase or norm.startswith(phrase + " ") or norm.startswith(phrase + "."):
            return True
    return False


def _first_word(heading: str) -> str:
    """Return the leading capitalized/identifier token of an H2 heading.

    Examples:
        'AI Business (219 ideas)' → 'AI'
        'Energy & Grid'           → 'Energy'
        'SA Applied Tech (...)'   → 'SA'
    """
    stripped = _strip_paren(heading)
    # Trim non-letter leading chars then split on whitespace.
    tokens = re.split(r"\s+", stripped.strip())
    return tokens[0] if tokens else ""


def extract_content_h2_first_words(body: str) -> list[str]:
    """Return first-word tokens for all non-generic H2 headings in body order."""
    words: list[str] = []
    for m in _H2_RE.finditer(body):
        heading = m.group(1)
        if _is_generic_heading(heading):
            continue
        fw = _first_word(heading)
        if fw:
            words.append(fw)
    return words


# -----------------------------------------------------------------------------
# Per-file linter
# -----------------------------------------------------------------------------

def lint_file(path: Path) -> list[dict[str, str]]:
    """Run proposal#1 + proposal#2 on a single .md file."""
    violations: list[dict[str, str]] = []
    rel = str(path.relative_to(REPO_ROOT))
    try:
        text = path.read_text(encoding="utf-8", errors="replace")
    except Exception as exc:
        violations.append({
            "rule": "proposal#1",
            "path": rel,
            "detail": f"unreadable file: {exc}",
        })
        return violations

    fm, body = parse_frontmatter(text)
    declared = fm.get("category") or None
    inferred = infer_category_from_filename(path.stem)
    category = declared or inferred

    # ---------- proposal#1 — filename naming ----------
    if category == "identity":
        if _ANY_DATE_SUFFIX_RE.search(path.stem):
            violations.append({
                "rule": "proposal#1",
                "path": rel,
                "detail": (
                    "category=identity but filename carries a date suffix — "
                    "use kebab-case with NO date (e.g. 'foo-bar.md')"
                ),
            })
        elif not _KEBAB_NO_DATE_RE.match(path.stem):
            violations.append({
                "rule": "proposal#1",
                "path": rel,
                "detail": (
                    f"category=identity expects pure kebab-case, got stem "
                    f"{path.stem!r}"
                ),
            })
    elif category == "operational":
        if not _SNAKE_DATE_SUFFIX_RE.match(path.stem):
            violations.append({
                "rule": "proposal#1",
                "path": rel,
                "detail": (
                    f"category=operational expects snake_case + _YYYY_MM_DD, "
                    f"got stem {path.stem!r}"
                ),
            })
    elif category == "phased":
        if not _KEBAB_DATE_SUFFIX_RE.match(path.stem):
            violations.append({
                "rule": "proposal#1",
                "path": rel,
                "detail": (
                    f"category=phased expects kebab-case + -YYYY-MM-DD, got "
                    f"stem {path.stem!r}"
                ),
            })
    else:
        # No category declared, no inference possible — flag as unclassifiable.
        violations.append({
            "rule": "proposal#1",
            "path": rel,
            "detail": (
                f"filename stem {path.stem!r} does not match any of "
                f"identity (kebab, no date) / operational (snake_YYYY_MM_DD) / "
                f"phased (kebab-YYYY-MM-DD); add frontmatter `category:`"
            ),
        })

    # Cross-check: when frontmatter category is declared AND inference also
    # returns a definite-and-different bucket, flag the mismatch. This catches
    # files like an identity-named file mistakenly labelled `category: phased`.
    if declared and inferred and declared != inferred:
        violations.append({
            "rule": "proposal#1",
            "path": rel,
            "detail": (
                f"frontmatter category={declared!r} contradicts filename "
                f"pattern (looks like {inferred!r})"
            ),
        })

    # ---------- proposal#2 — umbrella consolidation ----------
    domain_first_words = extract_content_h2_first_words(body)
    distinct = sorted(set(domain_first_words))
    umbrella = fm.get("umbrella") or None

    if len(domain_first_words) >= 2 and len(distinct) >= 2 and umbrella:
        violations.append({
            "rule": "proposal#2",
            "path": rel,
            "detail": (
                f"file declares `umbrella: {umbrella}` but has multiple H2 "
                f"content sections with differing first words "
                f"({', '.join(distinct)}). Consolidate into a single "
                f"`## {umbrella} Applied Tech (N bets / consolidated)` block."
            ),
        })

    return violations


# -----------------------------------------------------------------------------
# Orchestrator
# -----------------------------------------------------------------------------

def _is_readme_like(path: Path) -> bool:
    return path.stem.lower() in {"readme", "claude", "index", "_index"}


def _gather_targets(argv: list[str]) -> list[Path]:
    if len(argv) > 1:
        out: list[Path] = []
        for arg in argv[1:]:
            p = (REPO_ROOT / arg).resolve() if not Path(arg).is_absolute() else Path(arg).resolve()
            if not p.is_file():
                print(f"[proposal_lint] WARN: {arg} not found", file=sys.stderr)
                continue
            out.append(p)
        return out
    if not PROPOSALS_DIR.is_dir():
        return []
    return sorted(p for p in PROPOSALS_DIR.glob("*.md") if not _is_readme_like(p))


def _format_status(rule: str, found: Iterable[dict[str, str]]) -> str:
    matches = [v for v in found if v.get("rule") == rule]
    if not matches:
        return f"{rule} OK"
    reasons = "; ".join(v["detail"] for v in matches)
    return f"{rule} FAIL: {reasons}"


def main(argv: list[str]) -> int:
    targets = _gather_targets(argv)
    print(f"[proposal_lint] target: {len(targets)} files")
    all_violations: list[dict[str, str]] = []
    for path in targets:
        rel = path.relative_to(REPO_ROOT) if path.is_absolute() else path
        found = lint_file(path)
        all_violations.extend(found)
        print(f"  {rel}")
        print(f"    {_format_status('proposal#1', found)}")
        print(f"    {_format_status('proposal#2', found)}")
    hard = len(all_violations)  # both rules are HARD
    print(
        f"[proposal_lint] total violations: {len(all_violations)} "
        f"(HARD={hard})"
    )
    return 1 if hard else 0


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv))
    except Exception as exc:  # pragma: no cover
        print(f"[proposal_lint] FATAL: {exc}", file=sys.stderr)
        sys.exit(2)
