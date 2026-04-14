#!/usr/bin/env bash
# ============================================================
# n6-architecture 상위 6편 논문 pandoc PDF 빌드 스크립트
# ============================================================
# 생성일: 2026-04-14
# 태스크: PAPER-P5-1
# 필수 환경: pandoc 3.9+, xelatex (TeX Live 2026), xeCJK, hyperxmp
# 폰트: Apple SD Gothic Neo (macOS 내장), Menlo
# 사용법: bash papers/pandoc_templates/build_top6.sh
# ============================================================

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$PROJECT_ROOT"

OUTDIR="papers/pandoc_templates/output"
HEADER="papers/pandoc_templates/_pandoc_header.yaml"
BIB="papers/pandoc_templates/skeleton.bib"

# 공통 pandoc 인수 (폰트 오버라이드 포함)
COMMON_ARGS=(
  --metadata-file="$HEADER"
  --bibliography="$BIB"
  --pdf-engine=xelatex
  -V 'mainfont=Apple SD Gothic Neo'
  -V 'sansfont=Apple SD Gothic Neo'
  -V 'monofont=Menlo'
  -V 'CJKmainfont=Apple SD Gothic Neo'
  -V 'CJKsansfont=Apple SD Gothic Neo'
  -V 'CJKmonofont=Menlo'
  -V 'keywords='
)

mkdir -p "$OUTDIR"

# 상위 6편 빌드 목록: (입력md, venue_yaml, 출력pdf)
declare -a PAPERS=(
  "papers/n6-dance-choreography-paper.md|papers/pandoc_templates/venue_nature_comms.yaml|n6-dance-choreography-paper.pdf"
  "papers/n6-writing-systems-paper.md|papers/pandoc_templates/venue_nature_comms.yaml|n6-writing-systems-paper.pdf"
  "papers/n6-wine-enology-paper.md|papers/pandoc_templates/venue_nature_comms.yaml|n6-wine-enology-paper.pdf"
  "papers/n6-carbon-capture-paper.md|papers/pandoc_templates/venue_nature_comms.yaml|n6-carbon-capture-paper.pdf"
  "papers/n6-gravity-wave-paper.md|papers/pandoc_templates/venue_prl.yaml|n6-gravity-wave-paper.pdf"
  "papers/n6-aquaculture-paper.md|papers/pandoc_templates/venue_nature_comms.yaml|n6-aquaculture-paper.pdf"
)

SUCCESS=0
FAIL=0

for entry in "${PAPERS[@]}"; do
  IFS='|' read -r INPUT VENUE OUTPUT <<< "$entry"
  echo "빌드 중: $INPUT -> $OUTDIR/$OUTPUT"
  if pandoc "${COMMON_ARGS[@]}" \
    --metadata-file="$VENUE" \
    "$INPUT" \
    -o "$OUTDIR/$OUTPUT" 2>/dev/null; then
    pages=$(pdfinfo "$OUTDIR/$OUTPUT" 2>/dev/null | grep "Pages:" | awk '{print $2}')
    size=$(ls -lh "$OUTDIR/$OUTPUT" | awk '{print $5}')
    echo "  성공: ${pages}p / ${size}"
    SUCCESS=$((SUCCESS + 1))
  else
    echo "  실패!"
    FAIL=$((FAIL + 1))
  fi
done

echo ""
echo "=== 빌드 완료 ==="
echo "성공: $SUCCESS / 실패: $FAIL / 전체: ${#PAPERS[@]}"
