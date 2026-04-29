#!/usr/bin/env python3
"""
scripts/flatten_imports_py.py — pure-python fallback for hexa-lang's
tool/flatten_imports.hexa. Same contract: inline the `use "name"` graph
of a .hexa file into a single flat .hexa translation unit.

Activated by bin/n6_verify_run when hexa-lang's flatten_imports.hexa is
absent (risk mitigation per proposals/dup_derivation_consolidation_phase2_2026_04_24.md §4).

Resolution order for `use "NAME"`:
  1. caller-dir/NAME.hexa
  2. cwd/NAME.hexa
  3. cwd/self/NAME.hexa

Usage:
    python3 scripts/flatten_imports_py.py <target.hexa> <output.hexa>
"""
from __future__ import annotations
import os, re, sys
from pathlib import Path

USE_RE = re.compile(r'^\s*use\s+"([^"]+)"\s*$')


def resolve(name: str, caller_dir: Path, cwd: Path) -> Path | None:
    base = name[:-5] if name.endswith(".hexa") else name
    for cand in (caller_dir / f"{base}.hexa", cwd / f"{base}.hexa", cwd / "self" / f"{base}.hexa"):
        if cand.is_file():
            return cand.resolve()
    return None


def flatten(target: Path) -> str:
    cwd = Path.cwd()
    order: list[Path] = []
    seen: set[Path] = set()

    def dfs(path: Path) -> None:
        path = path.resolve()
        if path in seen:
            return
        seen.add(path)
        text = path.read_text()
        for line in text.splitlines():
            m = USE_RE.match(line)
            if not m:
                continue
            dep = resolve(m.group(1), path.parent, cwd)
            if dep is None:
                print(f"[flatten_imports_py] WARN: unresolved use \"{m.group(1)}\" in {path}", file=sys.stderr)
                continue
            dfs(dep)
        order.append(path)

    dfs(target.resolve())

    parts = [
        "// === FLATTENED by scripts/flatten_imports_py.py ===",
        f"// target: {target}",
        f"// files:  {len(order)}",
        "// order:  post-order DFS (leaves-first)",
        "",
    ]
    # Keep target first per hexa-lang convention (main must be parseable first).
    order.sort(key=lambda p: 0 if p == target.resolve() else 1)
    for path in order:
        parts.append(f"// ----- BEGIN {path.name} -----")
        stripped_lines: list[str] = []
        for line in path.read_text().splitlines():
            m = USE_RE.match(line)
            if m:
                stripped_lines.append(f'// [flatten_imports_py] stripped: use "{m.group(1)}"')
            else:
                stripped_lines.append(line)
        parts.append("\n".join(stripped_lines))
        parts.append(f"// ----- END {path.name} -----")
        parts.append("")
    return "\n".join(parts)


def main() -> int:
    if len(sys.argv) != 3:
        print("usage: flatten_imports_py.py <target.hexa> <output.hexa>", file=sys.stderr)
        return 2
    target = Path(sys.argv[1])
    output = Path(sys.argv[2])
    if not target.is_file():
        print(f"[flatten_imports_py] target not found: {target}", file=sys.stderr)
        return 2
    flat = flatten(target)
    output.write_text(flat)
    print(f"[flatten_imports_py] wrote {output} ({flat.count('BEGIN')} files)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
