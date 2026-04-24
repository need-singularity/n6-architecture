# n6-architecture remote-dispatch cwd mapping

**status**: landed 2026-04-24 (patch in sister repo `nexus/scripts/bin/hexa_remote`)
**convergence ref**: `~/core/nexus/convergence/drill_stability.convergence` @id `N6_ARCH_REMOTE_DISPATCH_CWD_MAPPING`

## Symptom

Running `nexus drill` (or any heavy-compute subcmd: smash / free / absolute / meta-closure / hyperarithmetic) from `~/core/n6-architecture` aborts with:

```
hexa resolver: ⚠ heavy-compute (...run.hexa drill) remote dispatch failed (exit 64) — Mac 로컬 실행 시 stage0 SIGKILL 위험
NEXUS_REMOTE_DOWNGRADE {"heavy_compute":true,"cmd":"drill","hosts_tried":[],"reason":"remote_unavailable","fallback":"abort"}
NEXUS_REMOTE_ERROR {"cmd":"drill","hosts_tried":[],"reason":"all_timeout","action":"abort_to_prevent_oom","exit_code":74}
```

`hosts_tried` is empty — the resolver aborted *before* probing any host.

## Root cause

`~/core/nexus/scripts/bin/hexa_remote` at L160-215 maps local cwd → remote path. Only three sister repos were originally wired:

- `$HOME/{Dev,dev}/airgenome` → `$HOME/airgenome`
- `$HOME/{Dev,dev,core}/nexus` → `$HOME/Dev/nexus`
- `$HOME/{Dev,dev}/anima` → `$HOME/anima`

Any other cwd falls through to `*) exit 64 ;;`. The outer wrapper (`scripts/bin/hexa`) then sees exit 64 + Darwin + heavy-compute + no `HEXA_ALLOW_LOCAL_FALLBACK` → emits the abort-74. The empty `hosts_tried` is a side-effect of the early exit (no probe ever ran).

## Fix

Append a case branch for `$HOME/core/n6-architecture`:

```bash
"$HOME_LC/core/n6-architecture"|"$HOME_LC/core/n6-architecture"/*)
    for candidate in "$HOME/core/n6-architecture"; do
        [ -d "$candidate" ] && LOCAL_ROOT="$candidate" && break
    done
    REMOTE_ROOT='$HOME/core/n6-architecture'
    SYNC_EXCLUDES=(
        --exclude=.git/
        --exclude='*.log'
        --exclude=.DS_Store
        --exclude=__pycache__/
        --exclude=.claude/
        --exclude=.growth/
        --exclude=reports/history/
        --exclude=build/
    )
    ;;
```

Insertion point: between the anima case (`;;` at L213) and the fallthrough (`*)` at L214).

## Recovery procedure if reverted

The file has `uchg` (BSD user-immutable) flag set, which blocks edits even by the owner. If the patch disappears:

```bash
# verify symptom
~/.hx/bin/nexus drill --seed 'canary' 2>&1 | head -3
# expect: NEXUS_REMOTE_ERROR with hosts_tried:[] and exit 74

# verify flag + file state
ls -lO ~/core/nexus/scripts/bin/hexa_remote
# expect: ...  uchg 14756 ...  (may vary)

grep -c 'core/n6-architecture' ~/core/nexus/scripts/bin/hexa_remote
# expect: 3  (if 0: patch gone, re-apply below)

# re-apply (the Python block is idempotent — refuses to double-patch)
chflags nouchg ~/core/nexus/scripts/bin/hexa_remote
python3 <<'PY'
p = "/Users/ghost/core/nexus/scripts/bin/hexa_remote"
txt = open(p).read()
if '"$HOME_LC/core/n6-architecture"' in txt:
    print("already_patched"); exit(0)
needle = '        --exclude=.claude/\n    )\n    ;;\n  *)\n    exit 64 ;;'
insertion = '''        --exclude=.claude/
    )
    ;;
  "$HOME_LC/core/n6-architecture"|"$HOME_LC/core/n6-architecture"/*)
    for candidate in "$HOME/core/n6-architecture"; do
        [ -d "$candidate" ] && LOCAL_ROOT="$candidate" && break
    done
    REMOTE_ROOT='$HOME/core/n6-architecture'
    SYNC_EXCLUDES=(
        --exclude=.git/
        --exclude='*.log'
        --exclude=.DS_Store
        --exclude=__pycache__/
        --exclude=.claude/
        --exclude=.growth/
        --exclude=reports/history/
        --exclude=build/
    )
    ;;
  *)
    exit 64 ;;'''
assert needle in txt, "needle missing — inspect file manually"
open(p, "w").write(txt.replace(needle, insertion, 1))
print("patched")
PY
chflags uchg ~/core/nexus/scripts/bin/hexa_remote

# verify
~/.hx/bin/nexus drill --seed 'post-patch canary' 2>&1 | head -3
# expect: "hexa_remote: ubu1 에서 원격 실행 중" (no NEXUS_REMOTE_ERROR)
```

## Related upstream gaps

### 1. preflight SSH-alias resolution — **already fixed upstream (2026-04-22)**

`~/core/hexa-lang/gate/remote_preflight.hexa:234-246` already resolves aliases via `ssh -G <host> | grep ^hostname` before the `/dev/tcp` probe (`H=$(ssh -G ...); timeout ... bash -c "exec 3<>/dev/tcp/${H:-<alias>}/22"`). The documented bypass `HEXA_REMOTE_SKIP_PREFLIGHT=1` is now unnecessary. Verified 2026-04-24: `hexa run remote_preflight.hexa --hosts ubu1,ubu2,htz --json-only` returns `verdict:"ok"`.

### 2. UserPromptSubmit hook feedback loop — **fixed 2026-04-24 (hexa-lang e5c00a28)**

`~/core/hexa-lang/gate/claude_prompt_hook.hexa` enriches short user prompts with transcript context for keyword scanning. Agent task-notifications echoed back as `user`-type transcript entries carry `drill`/`smash` keywords that are not user intent. The upstream `sed_clean` can fail on truncated tags at the tail -40 boundary, re-triggering heavy-compute commands.

Fix: `~/core/hexa-lang/gate/prompt_scan.hexa` now defensively strips `<task-notification>`, `<system-reminder>`, `<command-NAME>` blocks (plus orphan-open variants and inner structural tags) BEFORE keyword matching. Independent of upstream sed_clean. Debug env: `PROMPT_SCAN_DEBUG_STRIP=1`.

Verification:
- `"drill 해줘!!!"` → fires `[CMD] drill` (preserved)
- `"fix!!! <task-notification>...drill...</task-notification>"` → `"fix!!!"` (loop broken)
- `"hi <system-reminder>you should drill!</system-reminder>"` → `"hi"` (loop broken)

## Why uchg matters

The immutable flag prevents casual overwrites — e.g., if someone re-syncs `scripts/bin/` from an upstream mirror without review. It is *not* a security boundary (owner can `chflags nouchg`), but it ensures edits require deliberate intent.

Scanners that verify the patch should check `grep -c 'core/n6-architecture' ~/core/nexus/scripts/bin/hexa_remote` equals 3 and fail loudly if not.
