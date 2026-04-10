#!/usr/bin/env bash
set -euo pipefail

# NEXUS-6 Autonomous Growth — Full Installation
# ================================================
# Usage: ./install_autonomous.sh [--uninstall]
#
# Installs ALL automation layers:
#   1. launchd daemon (heartbeat — always running, every 30min)
#   2. cron jobs (scheduled growth cycles + daily report + weekly push)
#   3. git hooks (measure on every commit)
#   4. daily report generator (linked)
#   5. auto-push after successful growth (linked)
#
# Idempotent: safe to run multiple times.
# --uninstall: removes launchd, cron entries, git hook.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
NEXUS_HOME="$HOME/.nexus"
PLIST_LABEL="com.nexus.growth"
PLIST_PATH="$HOME/Library/LaunchAgents/${PLIST_LABEL}.plist"
HOOK_PATH="$REPO_ROOT/.git/hooks/post-commit"
CRON_MARKER="# NEXUS-6 Autonomous Growth"

# ── Colors ────────────────────────────────────────────────────────────
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m'

info()  { echo -e "${GREEN}[+]${NC} $*"; }
warn()  { echo -e "${YELLOW}[!]${NC} $*"; }
error() { echo -e "${RED}[x]${NC} $*"; }
header(){ echo -e "${CYAN}=== $* ===${NC}"; }

# ══════════════════════════════════════════════════════════════════════
# Uninstall
# ══════════════════════════════════════════════════════════════════════

do_uninstall() {
    header "NEXUS-6 Autonomous Growth — Uninstall"

    # 1. launchd
    if launchctl list 2>/dev/null | grep -q "$PLIST_LABEL"; then
        info "Unloading launchd agent..."
        launchctl unload "$PLIST_PATH" 2>/dev/null || true
    fi
    if [[ -f "$PLIST_PATH" ]]; then
        rm -f "$PLIST_PATH"
        info "Removed $PLIST_PATH"
    else
        info "launchd plist not found (already clean)"
    fi

    # 2. cron
    if crontab -l 2>/dev/null | grep -q "$CRON_MARKER"; then
        info "Removing NEXUS-6 cron entries..."
        crontab -l 2>/dev/null | grep -v "NEXUS-6\|nexus_growth_daemon\|growth_daily_report\|nexus.*push" | crontab -
        info "Cron entries removed"
    else
        info "No NEXUS-6 cron entries found (already clean)"
    fi

    # 3. git hook
    if [[ -f "$HOOK_PATH" ]] && grep -q "NEXUS-6" "$HOOK_PATH" 2>/dev/null; then
        # If the hook is ONLY nexus-6, remove it; otherwise strip our lines
        local line_count
        line_count=$(wc -l < "$HOOK_PATH" | tr -d ' ')
        local nexus_lines
        nexus_lines=$(grep -c "NEXUS-6\|nexus\|measure\.sh" "$HOOK_PATH" 2>/dev/null || echo "0")
        if [[ "$nexus_lines" -ge $((line_count - 2)) ]]; then
            rm -f "$HOOK_PATH"
            info "Removed post-commit hook"
        else
            grep -v "NEXUS-6\|nexus\|measure\.sh" "$HOOK_PATH" > "${HOOK_PATH}.tmp"
            mv "${HOOK_PATH}.tmp" "$HOOK_PATH"
            chmod +x "$HOOK_PATH"
            info "Stripped NEXUS-6 lines from post-commit hook"
        fi
    else
        info "No NEXUS-6 git hook found (already clean)"
    fi

    echo ""
    info "Uninstall complete. Logs remain in $NEXUS_HOME/ (remove manually if desired)."
    exit 0
}

# ══════════════════════════════════════════════════════════════════════
# Install
# ══════════════════════════════════════════════════════════════════════

if [[ "${1:-}" == "--uninstall" ]]; then
    do_uninstall
fi

header "NEXUS-6 Autonomous Growth — Installation"
echo ""
echo "  Repo:       $REPO_ROOT"
echo "  Scripts:    $SCRIPT_DIR"
echo "  State dir:  $NEXUS_HOME"
echo ""

# ── 0. Create state directory ─────────────────────────────────────────
mkdir -p "$NEXUS_HOME/reports"
info "State directory: $NEXUS_HOME"

# ── 1. launchd daemon ────────────────────────────────────────────────
header "Layer 3a: launchd daemon (every 30 minutes)"

mkdir -p "$HOME/Library/LaunchAgents"

cat > "$PLIST_PATH" << 'PLIST_EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.nexus.growth</string>
    <key>ProgramArguments</key>
    <array>
        <string>/bin/bash</string>
        <string>SCRIPT_DIR_PLACEHOLDER/nexus_growth_daemon.sh</string>
        <string>--max-cycles</string>
        <string>1</string>
    </array>
    <key>StartInterval</key>
    <integer>1800</integer>
    <key>WorkingDirectory</key>
    <string>SCRIPT_DIR_PLACEHOLDER</string>
    <key>StandardOutPath</key>
    <string>HOME_PLACEHOLDER/.nexus/growth.log</string>
    <key>StandardErrorPath</key>
    <string>HOME_PLACEHOLDER/.nexus/growth-error.log</string>
    <key>EnvironmentVariables</key>
    <dict>
        <key>PATH</key>
        <string>HOME_PLACEHOLDER/.local/bin:HOME_PLACEHOLDER/.cargo/bin:/usr/local/bin:/usr/bin:/bin</string>
        <key>CLAUDE_CLI</key>
        <string>HOME_PLACEHOLDER/.local/bin/claude</string>
    </dict>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>
PLIST_EOF

# Replace placeholders with actual paths
sed -i '' "s|SCRIPT_DIR_PLACEHOLDER|${SCRIPT_DIR}|g" "$PLIST_PATH"
sed -i '' "s|HOME_PLACEHOLDER|${HOME}|g" "$PLIST_PATH"

info "Created $PLIST_PATH"
echo "  To activate: launchctl load $PLIST_PATH"

# ── 2. cron jobs ──────────────────────────────────────────────────────
header "Layer 3b: cron jobs"

# Build new cron block
CRON_BLOCK=$(cat << CRON_EOF
$CRON_MARKER
# Fast cycle: every 30min, 1 growth action
*/30 * * * * cd $SCRIPT_DIR && PATH="$HOME/.local/bin:$HOME/.cargo/bin:\$PATH" bash nexus_growth_daemon.sh --max-cycles 1 >> $NEXUS_HOME/cron-growth.log 2>&1
# Daily deep growth: 6 cycles at 3am
0 3 * * * cd $SCRIPT_DIR && PATH="$HOME/.local/bin:$HOME/.cargo/bin:\$PATH" bash nexus_growth_daemon.sh --max-cycles 6 >> $NEXUS_HOME/deep-growth.log 2>&1
# Daily report: 7am
0 7 * * * cd $SCRIPT_DIR && PATH="$HOME/.local/bin:$HOME/.cargo/bin:\$PATH" bash growth_daily_report.sh >> $NEXUS_HOME/daily-report.log 2>&1
# Weekly push: Sunday 4am
0 4 * * 0 cd $REPO_ROOT && git push origin main >> $NEXUS_HOME/push.log 2>&1
# END NEXUS-6
CRON_EOF
)

# Remove existing nexus entries, then append
EXISTING_CRON=$(crontab -l 2>/dev/null || echo "")
if echo "$EXISTING_CRON" | grep -q "$CRON_MARKER"; then
    # Strip old block
    CLEAN_CRON=$(echo "$EXISTING_CRON" | sed "/$CRON_MARKER/,/# END NEXUS-6/d")
else
    CLEAN_CRON="$EXISTING_CRON"
fi

# Append new block
echo "${CLEAN_CRON}
${CRON_BLOCK}" | crontab -

info "Cron jobs installed (4 entries)"
echo "  Verify with: crontab -l"

# ── 3. git post-commit hook ──────────────────────────────────────────
header "Layer 3c: git post-commit hook"

mkdir -p "$(dirname "$HOOK_PATH")"

HOOK_CONTENT=$(cat << HOOK_EOF
#!/usr/bin/env bash
# NEXUS-6: Auto-measure after every commit
cd "$SCRIPT_DIR"
bash measure.sh >> "$NEXUS_HOME/metrics-history.jsonl" 2>/dev/null &
HOOK_EOF
)

if [[ -f "$HOOK_PATH" ]]; then
    if grep -q "NEXUS-6" "$HOOK_PATH" 2>/dev/null; then
        info "post-commit hook already contains NEXUS-6 (skipping)"
    else
        # Append to existing hook
        echo "" >> "$HOOK_PATH"
        echo "$HOOK_CONTENT" >> "$HOOK_PATH"
        info "Appended NEXUS-6 to existing post-commit hook"
    fi
else
    echo "$HOOK_CONTENT" > "$HOOK_PATH"
    info "Created post-commit hook"
fi
chmod +x "$HOOK_PATH"

# ── 4. Ensure all scripts are executable ──────────────────────────────
header "Making scripts executable"

chmod +x "$SCRIPT_DIR/nexus_growth_daemon.sh" 2>/dev/null || true
chmod +x "$SCRIPT_DIR/growth_daily_report.sh" 2>/dev/null || true
chmod +x "$SCRIPT_DIR/growth_intelligence.sh" 2>/dev/null || true
chmod +x "$SCRIPT_DIR/growth_notify.sh" 2>/dev/null || true
chmod +x "$SCRIPT_DIR/measure.sh" 2>/dev/null || true
chmod +x "$SCRIPT_DIR/auto_grow.sh" 2>/dev/null || true
chmod +x "$SCRIPT_DIR/install_autonomous.sh" 2>/dev/null || true

info "All scripts marked executable"

# ── 5. Summary ────────────────────────────────────────────────────────
echo ""
header "Installation Complete"
echo ""
echo "  Installed components:"
echo "    [1] launchd plist  : $PLIST_PATH"
echo "    [2] cron jobs      : 4 entries (30min cycle, 3am deep, 7am report, Sun push)"
echo "    [3] git hook       : $HOOK_PATH"
echo "    [4] state dir      : $NEXUS_HOME/"
echo ""
echo "  To activate launchd (start autonomous growth):"
echo "    launchctl load $PLIST_PATH"
echo ""
echo "  To verify cron:"
echo "    crontab -l | grep nexus"
echo ""
echo "  To uninstall everything:"
echo "    $SCRIPT_DIR/install_autonomous.sh --uninstall"
echo ""
echo "  Logs:"
echo "    $NEXUS_HOME/growth.log           (launchd stdout)"
echo "    $NEXUS_HOME/growth-error.log     (launchd stderr)"
echo "    $NEXUS_HOME/cron-growth.log      (cron 30min cycles)"
echo "    $NEXUS_HOME/deep-growth.log      (cron 3am deep cycles)"
echo "    $NEXUS_HOME/daily-report.log     (daily report output)"
echo "    $NEXUS_HOME/metrics-history.jsonl (git hook measurements)"
echo "    $NEXUS_HOME/notifications.log    (growth notifications)"
echo ""
info "NEXUS-6 autonomous growth is ready. Run launchctl load to begin."
