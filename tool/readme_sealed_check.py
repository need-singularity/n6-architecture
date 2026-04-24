#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
readme_sealed_check.py — enforce own#14 (readme-sealed-required).

Computes SHA-256 of repo-root README.md and compares it against the sealed
hash stored in README.md.sealed.hash. On first run (no sealed file) the
file is initialised and the run exits 0 with a warning, per the spec.

Also syncs the hash into .readme-rules.json scopes.root.sealed_hash when
that field is present or writable — preserving it as an additional SSOT
pointer per own#14 decl.

Exit codes: 0 = match (or init), 1 = mismatch, 2 = internal error.
Flags    : --update = recompute + overwrite sealed hash (manual re-seal).
"""

from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parent.parent
README_PATH = REPO_ROOT / "README.md"
SEALED_PATH = REPO_ROOT / "README.md.sealed.hash"
RULES_PATH = REPO_ROOT / ".readme-rules.json"


def compute_sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(65536), b""):
            h.update(chunk)
    return h.hexdigest()


def read_sealed_hash(path: Path) -> str | None:
    """Return a 64-char hex digest from the sealed file, or None if missing/invalid.

    The existing file may contain legacy content (e.g. '1208768509:75127') that is
    not a SHA-256 — treat that as 'no canonical sealed hash yet' and allow init.
    """
    if not path.is_file():
        return None
    raw = path.read_text(encoding="utf-8", errors="replace").strip()
    # Accept bare 64-hex or 'sha256:<hex>' prefix.
    token = raw.split()[0] if raw else ""
    if token.startswith("sha256:"):
        token = token.split(":", 1)[1]
    if len(token) == 64 and all(c in "0123456789abcdefABCDEF" for c in token):
        return token.lower()
    return None


def write_sealed_hash(path: Path, digest: str) -> None:
    path.write_text(f"sha256:{digest}\n", encoding="utf-8")


def sync_rules_field(digest: str) -> bool:
    """Write sealed_hash into .readme-rules.json scopes.root if possible.

    Returns True if the file was updated, False if no change was needed or file absent.
    Never raises on failure — rules-file sync is best-effort.
    """
    if not RULES_PATH.is_file():
        return False
    try:
        data = json.loads(RULES_PATH.read_text(encoding="utf-8"))
    except Exception:
        return False
    if not isinstance(data, dict):
        return False
    scopes = data.get("scopes")
    if not isinstance(scopes, dict):
        return False
    root = scopes.get("root")
    if not isinstance(root, dict):
        return False
    if root.get("sealed_hash") == digest:
        return False
    root["sealed_hash"] = digest
    try:
        RULES_PATH.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        return True
    except Exception:
        return False


def main(argv: list[str]) -> int:
    if not README_PATH.is_file():
        print(f"[sealed-check] FATAL: {README_PATH} missing — own#14 cannot run", file=sys.stderr)
        return 2

    update_mode = "--update" in argv[1:]

    current = compute_sha256(README_PATH)
    sealed = read_sealed_hash(SEALED_PATH)

    # First-run / init mode: sealed file missing or non-SHA256 legacy content.
    if sealed is None:
        write_sealed_hash(SEALED_PATH, current)
        changed = sync_rules_field(current)
        print(f"[sealed-check] INIT: wrote sealed hash sha256:{current[:12]}... -> {SEALED_PATH.name}")
        if changed:
            print(f"[sealed-check] INIT: synced sealed_hash into .readme-rules.json scopes.root")
        print(f"[sealed-check] warn: first-run initialisation, not blocking. Re-run to enforce.")
        return 0

    if update_mode:
        write_sealed_hash(SEALED_PATH, current)
        sync_rules_field(current)
        print(f"[sealed-check] UPDATE: new sealed sha256:{current[:12]}... (was {sealed[:12]}...)")
        return 0

    if current == sealed:
        # Keep .readme-rules.json in sync if the field drifted.
        synced = sync_rules_field(current)
        status = "SYNCED .readme-rules.json" if synced else "clean"
        print(f"[sealed-check] MATCH sha256:{current[:12]}... ({status})")
        return 0

    print(f"[sealed-check] FAIL: README.md sha256 drift", file=sys.stderr)
    print(f"  current : sha256:{current}", file=sys.stderr)
    print(f"  sealed  : sha256:{sealed}", file=sys.stderr)
    print(f"  remedy  : re-seal via `python3 tool/readme_sealed_check.py --update`", file=sys.stderr)
    return 1


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv))
    except Exception as exc:  # pragma: no cover
        print(f"[sealed-check] FATAL: {exc}", file=sys.stderr)
        sys.exit(2)
