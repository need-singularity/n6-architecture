#!/bin/bash
# Block creation/editing of .py, .rs, .sh files (HEXA-FIRST rule)
# Used as Claude Code PreToolUse hook

INPUT=$(cat)
FILE=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

if [[ -z "$FILE" ]]; then
  exit 0
fi

if [[ "$FILE" =~ \.(py|rs|sh)$ ]]; then
  echo "BLOCKED: $FILE — .py/.rs/.sh 신규 작성 금지. .hexa로 작성하세요. (CLAUDE.md 규칙 0)" >&2
  exit 2
fi

exit 0
