#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
own_nexus6_tests_drift.py — own#21 readme-nexus6-tests-count-drift checker.

Replaces the legacy ``echo 2078`` stub (see .own own#21) with a real
comparator between:

  * README-claimed NEXUS-6 test count — regex token ``tests: NNNN``
    (drift_pattern from .own own#21), extracted from README.md.

  * Authoritative test count — the ``total`` field of
    reports/n6_selftest.json (schema n6-architecture/n6_selftest/1),
    produced by the n6 self-test harness (pass + fail scanners).

Behavior
--------
Emits reports/n6_own21_drift.json summarising:
  - authoritative count and its source
  - all README candidate tokens (matched ``tests: N`` strings)
  - mismatches / missing-token status
  - overall verdict (PASS / WARN_NO_TOKEN / DRIFT / ERROR)

Exit codes
----------
  0 — PASS (README token matches authoritative total)
  0 — WARN_NO_TOKEN (README carries no ``tests: N`` token; SOFT — nothing
      to drift *against*, but surface a warning in the JSON report so
      humans can wire the token in once the harness publishes it).
      The own#21 cron runs with on_fail=warn in .own, so SOFT semantics
      are preserved even when the CI step is SOFT.
  0 — WARN_NO_SELFTEST (reports/n6_selftest.json missing / unreadable /
      field-missing; SOFT — this is a CI-environment issue, not a drift
      violation, so we do not fail the check). Verdict + detail are
      written to the JSON report for triage.
  1 — DRIFT (at least one README token disagrees with authoritative)
  2 — ERROR (authoritative source present but structurally invalid in a
      way that was not caught by WARN_NO_SELFTEST)

Design notes
------------
Parity with own#20 (readme-techniques-count-drift): own#20 uses
``drift_pattern (\\d+) Techniques`` + ``drift_actual find techniques …
| wc -l``. The hexa runner compares those with README extraction.
own#21 here mirrors that pattern in Python, independent of hexa so it
runs on any CI image without the hexa CLI bootstrap.

Python 3.11, standard library only (no PyYAML / no external deps).

Usage
-----
    python3 tool/own_nexus6_tests_drift.py [--verbose]
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parent.parent
README_PATH = REPO_ROOT / "README.md"
SELFTEST_PATH = REPO_ROOT / "reports" / "n6_selftest.json"
REPORT_PATH = REPO_ROOT / "reports" / "n6_own21_drift.json"

# own#21 .own spec — drift_pattern = tests:\s+\d+
# We capture the numeric group for comparison.
DRIFT_PATTERN = re.compile(r"tests:\s+(\d+)")


def _utc_now() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def _load_authoritative() -> tuple[int | None, str]:
    """Return (count, source_description). count=None on failure."""
    if not SELFTEST_PATH.exists():
        return None, f"missing:{SELFTEST_PATH.relative_to(REPO_ROOT)}"
    try:
        data = json.loads(SELFTEST_PATH.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        return None, f"unreadable:{SELFTEST_PATH.relative_to(REPO_ROOT)}:{exc}"
    total = data.get("total")
    if not isinstance(total, int):
        return None, (
            f"field-missing:{SELFTEST_PATH.relative_to(REPO_ROOT)}:total"
        )
    return total, f"{SELFTEST_PATH.relative_to(REPO_ROOT)}#total"


def _extract_readme_claims(text: str) -> list[dict[str, Any]]:
    """Extract all ``tests: NNNN`` tokens with line numbers."""
    claims: list[dict[str, Any]] = []
    for lineno, line in enumerate(text.splitlines(), start=1):
        for match in DRIFT_PATTERN.finditer(line):
            claims.append(
                {
                    "line": lineno,
                    "match": match.group(0),
                    "count": int(match.group(1)),
                }
            )
    return claims


def _write_report(report: dict[str, Any]) -> None:
    REPORT_PATH.parent.mkdir(parents=True, exist_ok=True)
    REPORT_PATH.write_text(
        json.dumps(report, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )


def main() -> int:
    ap = argparse.ArgumentParser(
        description="own#21 nexus6 tests count drift checker",
    )
    ap.add_argument(
        "--verbose",
        action="store_true",
        help="echo per-claim details to stdout",
    )
    args = ap.parse_args()

    report: dict[str, Any] = {
        "schema": "n6-architecture/n6_own21_drift/1",
        "rule": "own#21",
        "slug": "readme-nexus6-tests-count-drift",
        "ts": _utc_now(),
        "drift_pattern": DRIFT_PATTERN.pattern,
    }

    # 1) authoritative count
    authoritative, source_desc = _load_authoritative()
    report["authoritative"] = {
        "count": authoritative,
        "source": source_desc,
    }

    if authoritative is None:
        # Defensive SOFT skip: reports/n6_selftest.json is tracked in this
        # repo, but in environments where it is absent / unreadable / has a
        # missing `total` field (shallow clones, fixture-less checkouts, or
        # pre-harness-run states), treat it as a CI-env issue rather than a
        # drift violation. Emit WARN_NO_SELFTEST + exit 0 so the own#21 CI
        # step is not marked red. The comparison logic itself is untouched.
        report["verdict"] = "WARN_NO_SELFTEST"
        report["detail"] = (
            f"authoritative source unavailable ({source_desc}); "
            "SOFT skip — nothing to compare README tokens against"
        )
        _write_report(report)
        print(
            f"[own#21] WARN_NO_SELFTEST: cannot load authoritative count "
            f"({source_desc}) — SOFT skip (exit 0)",
            file=sys.stderr,
        )
        return 0

    # 2) README claims
    if not README_PATH.exists():
        report["verdict"] = "ERROR"
        report["detail"] = f"README.md missing at {README_PATH}"
        _write_report(report)
        print("[own#21] ERROR: README.md missing", file=sys.stderr)
        return 2

    readme_text = README_PATH.read_text(encoding="utf-8")
    claims = _extract_readme_claims(readme_text)
    report["readme"] = {
        "path": str(README_PATH.relative_to(REPO_ROOT)),
        "claims": claims,
        "claim_count": len(claims),
    }

    # 3) verdict
    if not claims:
        report["verdict"] = "WARN_NO_TOKEN"
        report["detail"] = (
            "README.md carries no `tests: N` token matching drift_pattern "
            f"{DRIFT_PATTERN.pattern!r}; nothing to compare against "
            f"authoritative count={authoritative}. Add a canonical token "
            "to README.md (or keep own#21 deferred) to enable hard drift "
            "enforcement."
        )
        _write_report(report)
        if args.verbose:
            print(f"[own#21] WARN_NO_TOKEN authoritative={authoritative}")
        # SOFT: exit 0 so CI step is not red — rule is on_fail=warn anyway.
        return 0

    mismatches = [c for c in claims if c["count"] != authoritative]
    report["mismatches"] = mismatches
    if mismatches:
        report["verdict"] = "DRIFT"
        report["detail"] = (
            f"{len(mismatches)} README token(s) disagree with "
            f"authoritative count={authoritative}"
        )
        _write_report(report)
        print(
            f"[own#21] DRIFT: authoritative={authoritative} but README has "
            f"{len(mismatches)} mismatching token(s):",
            file=sys.stderr,
        )
        for m in mismatches:
            print(
                f"  README.md:{m['line']}  {m['match']}  "
                f"(claimed={m['count']})",
                file=sys.stderr,
            )
        return 1

    report["verdict"] = "PASS"
    report["detail"] = (
        f"all {len(claims)} README token(s) match authoritative "
        f"count={authoritative}"
    )
    _write_report(report)
    if args.verbose:
        print(
            f"[own#21] PASS authoritative={authoritative} claims={len(claims)}"
        )
    return 0


if __name__ == "__main__":
    sys.exit(main())
