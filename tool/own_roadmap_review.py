#!/usr/bin/env python3
"""own#19 — weekly roadmap review reminder. LEGACY SHIM.

This Python file was migrated to tool/own_roadmap_review.hexa per raw 9
hexa-only mandate (commit chain post 04f85f01). This shim preserves the
existing CI / cron invocation path while delegating to the canonical hexa
implementation.

Migration date: 2026-04-29
Canonical impl: tool/own_roadmap_review.hexa
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
        print("[own#19 shim] hexa binary not found in PATH — fallback impossible")
        return 2
    hexa_path = repo_root / "tool" / "own_roadmap_review.hexa"
    if not hexa_path.is_file():
        print(f"[own#19 shim] missing canonical impl: {hexa_path}")
        return 2
    result = subprocess.run([hexa_bin, str(hexa_path)] + sys.argv[1:])
    return result.returncode


if __name__ == "__main__":
    sys.exit(main())
