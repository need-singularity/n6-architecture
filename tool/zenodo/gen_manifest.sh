#!/usr/bin/env bash
# tool/zenodo/gen_manifest.sh
#
# Generate SHA-256 manifest of all artifacts that will be uploaded to
# Zenodo. Idempotent: re-running overwrites the manifest deterministically.
#
# Output: tool/zenodo/manifest.sha256
# Format: each line is "<sha256>  <relative-path>" (sha256sum/shasum -a 256
#         compatible).

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ZENODO_DIR="${REPO_ROOT}/tool/zenodo"
MANIFEST="${ZENODO_DIR}/manifest.sha256"

# Pick the right SHA-256 binary on macOS vs Linux.
if command -v sha256sum >/dev/null 2>&1; then
  SHA256="sha256sum"
else
  SHA256="shasum -a 256"
fi

# Files in the manifest (deposit artifacts + reproducibility helpers).
FILES=(
  "papers/hexa-weave-formal-mechanical-w2-2026-04-28.md"
  "lean4-n6/N6/MechVerif/AX1.lean"
  "lean4-n6/N6/MechVerif/AX2.lean"
  "lean4-n6/N6/MechVerif/MKBridge.lean"
  "lean4-n6/N6/MechVerif/Foundation/Strand.lean"
  "lean4-n6/N6/MechVerif/Foundation/Axioms.lean"
  "lean4-n6/lean-toolchain"
  "lean4-n6/lakefile.lean"
  "lean4-n6/lake-manifest.json"
  "tool/zenodo/metadata.json"
  "tool/zenodo/README_zenodo.md"
  "tool/zenodo/requirements.txt"
  "tool/zenodo/verify_paper_block.py"
  "tool/zenodo/deposit.sh"
  "tool/zenodo/gen_manifest.sh"
  "tool/zenodo/gen_tarball.sh"
  "tool/zenodo/USER_INPUT_CHECKLIST.md"
  "proposals/hexa_weave_zenodo_auto_prep_2026_04_28.md"
  "design/kick/2026-04-28_zenodo-auto-prep_omega_cycle.json"
  "design/kick/2026-04-28_zenodo-deposit-prep-cycle14_omega_cycle.json"
)

# Optional: include compiled PDF + Lean tarball if present (deposit-time only).
OPTIONAL=(
  "papers/hexa-weave-formal-mechanical-w2-2026-04-28.pdf"
  "tool/zenodo/lean4-n6-mechverif-cycle12.tar.gz"
)

cd "$REPO_ROOT"

{
  echo "# HEXA-WEAVE Zenodo deposit SHA-256 manifest"
  echo "# Generated: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "# Tool: tool/zenodo/gen_manifest.sh"
  echo "#"
  echo "# Format: <sha256>  <relative-path>"
  echo "# Compatible with: sha256sum -c  /  shasum -a 256 -c"
  echo "#"
  for f in "${FILES[@]}"; do
    if [[ -f "$f" ]]; then
      $SHA256 "$f"
    else
      echo "# MISSING: $f" >&2
    fi
  done
  for f in "${OPTIONAL[@]}"; do
    if [[ -f "$f" ]]; then
      $SHA256 "$f"
    else
      echo "# OPTIONAL_NOT_PRESENT: $f"
    fi
  done
} > "$MANIFEST"

echo "[gen_manifest.sh] wrote $MANIFEST"
echo "[gen_manifest.sh] $(wc -l < "$MANIFEST") lines (including comment header)."
