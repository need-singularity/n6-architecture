#!/usr/bin/env python3
"""HEXA-WEAVE paper §7.1 (a) embedded verify block — standalone re-runner.

Mirror of the Python block embedded in
    papers/hexa-weave-formal-mechanical-w2-2026-04-28.md
    §7.1 (a) "Python sympy block"
extracted into a runnable file so a Zenodo reviewer can reproduce the
[2, 1000] sweep yielding {6} without parsing the markdown.

Usage:
    python3 tool/zenodo/verify_paper_block.py

Expected output (verbatim, recorded 2026-04-28 on Mac M2):
    AX-1 verify-embedded PASS:
      sigma(6)=12, phi(6)=2, tau(6)=4
      bounded [2,50]    solutions: [6]
      extended [2,1000] solutions: [6]
      spec corrigendum: ax1_eq(1) = True; n=1 forces n >= 2 quantifier amend

Exit codes:
    0  -- PASS (sympy reproduces the embedded block, sweep yields [6])
    1  -- IMPORT FAIL (sympy missing or wrong version)
    2  -- ASSERTION FAIL (sweep yielded something other than [6]; raw 71
          falsifier F-W6-ZEN-13-2 fired; deposit must NOT proceed)
"""
from __future__ import annotations

import sys


def main() -> int:
    try:
        from sympy import divisor_sigma, totient, divisor_count  # type: ignore
    except ImportError as e:
        print(f"verify_paper_block: IMPORT FAIL -- sympy missing: {e}",
              file=sys.stderr)
        return 1

    def ax1_eq(n: int) -> bool:
        """AX1Eq n := sigma(n) * phi(n) == n * tau(n)."""
        return divisor_sigma(n, 1) * totient(n) == n * divisor_count(n)

    try:
        # Reverse direction: AX1Eq 6 holds.
        assert ax1_eq(6) is True, "AX1 reverse direction: AX1Eq 6 must hold"
        assert divisor_sigma(6, 1) == 12, "sigma(6) = 12"
        assert totient(6) == 2, "phi(6) = 2"
        assert divisor_count(6) == 4, "tau(6) = 4"
        assert 12 * 2 == 24 == 6 * 4, "n6 master identity at n=6"

        # Bounded forward direction (cycle-7 hardened to [2, 50]).
        solutions_bounded = [n for n in range(2, 51) if ax1_eq(n)]
        assert solutions_bounded == [6], (
            f"AX1 bounded forward [2,50] expected [6], got {solutions_bounded}"
        )

        # Spec corrigendum surface at n=1.
        assert ax1_eq(1) is True, "Spec corrigendum: AX1Eq 1 holds trivially"

        # Extended sweep [2, 1000] -> {6}.
        solutions_extended = [n for n in range(2, 1001) if ax1_eq(n)]
        assert solutions_extended == [6], (
            f"AX1 extended sweep [2,1000] expected [6], got {solutions_extended}"
        )
    except AssertionError as e:
        print(f"verify_paper_block: ASSERTION FAIL -- {e}", file=sys.stderr)
        print("F-W6-ZEN-13-2 falsifier FIRED. DO NOT proceed with Zenodo deposit.",
              file=sys.stderr)
        return 2

    print("AX-1 verify-embedded PASS:")
    print(f"  sigma(6)={divisor_sigma(6, 1)}, "
          f"phi(6)={totient(6)}, "
          f"tau(6)={divisor_count(6)}")
    print(f"  bounded [2,50]    solutions: {solutions_bounded}")
    print(f"  extended [2,1000] solutions: {solutions_extended}")
    print(f"  spec corrigendum: ax1_eq(1) = {ax1_eq(1)}; "
          f"n=1 forces n >= 2 quantifier amend")
    return 0


if __name__ == "__main__":
    sys.exit(main())
