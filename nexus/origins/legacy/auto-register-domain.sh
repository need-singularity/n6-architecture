#!/bin/bash
# auto-register-domain.sh — goal.md 프론트매터 → dse-map.toml 등록 + 커밋 + 푸시
# 사용법: tools/auto-register-domain.sh docs/<domain>/goal.md
set -eo pipefail

GOAL="${1:?사용법: $0 docs/<domain>/goal.md}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

DOMAIN=$(echo "$GOAL" | sed 's|.*/docs/||;s|/goal\.md||')

if [ ! -f "$GOAL" ]; then
  echo "오류: $GOAL 파일 없음" >&2
  exit 1
fi

# 프론트매터 파싱 (--- 블록 사이)
META=$(/usr/bin/python3 << PYEOF
import sys, re, shlex

with open("$GOAL") as f:
    content = f.read()

m = re.search(r'^---\s*\n(.*?)\n---', content, re.DOTALL)
if not m:
    print("NO_FRONTMATTER")
    sys.exit(0)

lines = m.group(1).strip().split('\n')
for line in lines:
    if ':' in line:
        key, val = line.split(':', 1)
        key = key.strip().upper().replace('-', '_')
        val = val.strip()
        # bash-safe 출력: 값을 작은따옴표로 감싸되 내부 작은따옴표 이스케이프
        safe_val = val.replace("'", "'\\''")
        print(f"{key}='{safe_val}'")
PYEOF
)

if [ "$META" = "NO_FRONTMATTER" ]; then
  echo "경고: $GOAL에 프론트매터(---) 없음. dse-map 등록 건너뜀."
  echo "프론트매터 예시:"
  echo "---"
  echo "domain: mouse"
  echo "bt_start: 1115"
  echo "bt_count: 10"
  echo "alien_level: 10"
  echo "constants_exact: 25"
  echo "constants_count: 25"
  echo "levels: [\"센서\", \"입력부\", \"기구부\", \"통신부\", \"프로토콜\"]"
  echo "cross_dse: [\"chip-architecture\", \"ergonomics\"]"
  echo "note: \"φ=2 이진입력, sopfr=5 버튼...\""
  echo "---"
  # 프론트매터 없어도 커밋+푸시는 진행
  SKIP_DSE=1
else
  eval "$META"
  SKIP_DSE=0
fi

# 1) dse-map.toml 등록 (섹션 없으면 생성)
DSE_FILE="docs/dse-map.toml"
if [ "${SKIP_DSE:-0}" = "0" ] && [ -f "$DSE_FILE" ]; then
  if grep -q "^\[${DOMAIN}\]" "$DSE_FILE"; then
    echo "✓ dse-map.toml: [$DOMAIN] 이미 존재 — 건너뜀"
  else
    echo "" >> "$DSE_FILE"
    cat >> "$DSE_FILE" << TOML_EOF

[${DOMAIN}]
goal = true
dse = "done"
alien_level = ${ALIEN_LEVEL:-10}
levels = ${LEVELS:-[]}
cross_dse = ${CROSS_DSE:-[]}
note = "${NOTE:-}"
TOML_EOF
    echo "✓ dse-map.toml: [$DOMAIN] 신규 등록"
  fi
fi

# 2) 변경 파일 수집
CHANGED_FILES=()
[ -f "docs/${DOMAIN}/goal.md" ] && CHANGED_FILES+=("docs/${DOMAIN}/goal.md")
[ -d "docs/${DOMAIN}" ] && CHANGED_FILES+=("docs/${DOMAIN}/")
[ -f "$DSE_FILE" ] && CHANGED_FILES+=("$DSE_FILE")
[ -f "docs/atlas-constants.md" ] && CHANGED_FILES+=("docs/atlas-constants.md")
[ -f "docs/breakthrough-theorems.md" ] && CHANGED_FILES+=("docs/breakthrough-theorems.md")

# 3) 변경 사항 확인
if git diff --quiet -- "${CHANGED_FILES[@]}" && git diff --cached --quiet -- "${CHANGED_FILES[@]}" && [ -z "$(git ls-files --others --exclude-standard -- "${CHANGED_FILES[@]}" 2>/dev/null)" ]; then
  echo "변경 사항 없음 — 커밋 건너뜀"
  exit 0
fi

# 4) 커밋
BT_START="${BT_START:-?}"
BT_COUNT="${BT_COUNT:-?}"
EXACT="${CONSTANTS_EXACT:-?}"
TOTAL="${CONSTANTS_COUNT:-?}"

if [ "$BT_COUNT" != "?" ] && [ "$BT_START" != "?" ]; then
  BT_END=$((BT_START + BT_COUNT - 1))
  BT_RANGE="BT-${BT_START}~${BT_END}"
else
  BT_RANGE="BT-신규"
fi

COMMIT_MSG="feat: ${DOMAIN} 도메인 ${EXACT}/${TOTAL} EXACT 골화 — ${BT_RANGE}

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"

git add "${CHANGED_FILES[@]}"
git commit -m "$COMMIT_MSG"
echo "✓ 커밋 완료: ${DOMAIN}"

# 5) 푸시
git push
echo "✓ 푸시 완료"

echo ""
echo "═══════════════════════════════════"
echo "  도메인: ${DOMAIN}"
echo "  상수:   ${EXACT}/${TOTAL} EXACT"
echo "  BT:     ${BT_RANGE}"
echo "  DSE:    $([ "${SKIP_DSE:-0}" = "0" ] && echo '등록' || echo '건너뜀')"
echo "  커밋:   완료"
echo "  푸시:   완료"
echo "═══════════════════════════════════"
