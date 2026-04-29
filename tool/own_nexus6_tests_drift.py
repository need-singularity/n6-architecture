#!/usr/bin/env python3
"""own_nexus6_tests_drift.py — own#21 drift checker. LEGACY SHIM.

Migrated to tool/own_nexus6_tests_drift.hexa per raw 9 hexa-only mandate.
This shim preserves CI invocation while delegating to canonical hexa impl.

Migration date: 2026-04-29
Canonical impl: tool/own_nexus6_tests_drift.hexa
"""
from __future__ import annotations

import shutil
import subprocess
import sys
from pathlib import Path


def main() -> int:
    repo_root = Path(__file__).resolve().parent.parent
    hexa_bin = shutil.which("hexa")
    if hexa_bin is None:
        print("[own#21 shim] hexa binary not found in PATH")
        return 2
    hexa_path = repo_root / "tool" / "own_nexus6_tests_drift.hexa"
    if not hexa_path.is_file():
        print(f"[own#21 shim] missing canonical impl: {hexa_path}")
        return 2
    result = subprocess.run([hexa_bin, str(hexa_path)] + sys.argv[1:])
    return result.returncode


if __name__ == "__main__":
    sys.exit(main())
