#!/usr/bin/env bash
# tool/zenodo/gen_tarball.sh
#
# Generate the Lean 4 source tarball for the Zenodo supplementary upload
# (option (b) of cycle-8 §2 item 8).
#
# Idempotent: re-running overwrites tool/zenodo/lean4-n6-mechverif-cycle12.tar.gz
# deterministically. We DO NOT include .lake/ build artifacts (the
# Mathlib pin in lake-manifest.json is sufficient for reproducibility).

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ZENODO_DIR="${REPO_ROOT}/tool/zenodo"
OUT="${ZENODO_DIR}/lean4-n6-mechverif-cycle12.tar.gz"

cd "$REPO_ROOT"

# Use --exclude to skip build cache. The tarball contains only the
# source-of-truth Lean files + the toolchain + lakefile + manifest.
tar --exclude='lean4-n6/.lake' \
    --exclude='lean4-n6/build' \
    --exclude='*.olean' \
    --exclude='.DS_Store' \
    -czf "$OUT" \
    lean4-n6/N6/MechVerif/AX1.lean \
    lean4-n6/N6/MechVerif/AX2.lean \
    lean4-n6/N6/MechVerif/MKBridge.lean \
    lean4-n6/N6/MechVerif/Foundation/Strand.lean \
    lean4-n6/N6/MechVerif/Foundation/Axioms.lean \
    lean4-n6/lean-toolchain \
    lean4-n6/lakefile.lean \
    lean4-n6/lake-manifest.json \
    lean4-n6/Main.lean

echo "[gen_tarball.sh] wrote $OUT"
ls -lh "$OUT"
