# Claude Code Cache/Limit Bug Analysis and Response

> Investigated: 2026-04-02
> Related issues: anthropics/claude-code#41788, #40524, #34629
> Analysis repo: https://github.com/ArkNill/claude-code-cache-analysis

---

## Symptoms

- Max 20 plan ($200/mo) 5-hour limit exhausted in **~70 minutes**
- **4~5x** faster token drain than normal
- Occurs between v2.1.88~v2.1.89, reproducible

---

## Two bugs (independent)

### Bug 1: Sentinel substitution bug (standalone binary only)

| Item | Description |
|------|-------------|
| **Scope** | standalone binary only (npm install unaffected) |
| **Cause** | Bun fork's `cch=00000` sentinel substitution breaks the prefix within messages |
| **Effect** | Cache-read ratio pinned at **4~17%**, unrecoverable |
| **Version** | Confirmed on v2.1.89 |
| **Fix** | Switch to npm install or update to v2.1.90 |

### Bug 2: Resume cache breakage (v2.1.69+)

| Item | Description |
|------|-------------|
| **Scope** | All install methods (when `--resume` is used) |
| **Cause** | `deferred_tools_delta` (introduced in v2.1.69) alters `messages[0]` structure |
| **Mechanism** | fresh: messages[0]=13.4KB (deferred_tools+MCP+skills) vs resume: messages[0]=352B -> prefix match fails |
| **Effect** | Entire history (200K~500K tokens) re-authored each turn -> O(n) cost |
| **Fix** | Avoid `--resume`, or pin to v2.1.68 |

---

## Cache operation principle

```
Normal behavior:
  Turn 1: cache_read=312,377  cache_create=1,944  (initial cache write)
  Turn 2: cache_read=314,321  cache_create=493    (cache reuse, append delta)
  Turn 3: cache_read=314,814  cache_create=172    (cache reuse)
  -> cached-token cost = 10% of input tokens

On bug:
  Turn N:   cache_read=216,204  cache_create=10,504   <- cache starts breaking
  Turn N+1: cache_read=216,204  cache_create=11,815   <- cache cannot extend, rewrites
  Turn N+5: cache_read=11,428   cache_create=224,502  <- only system prompt cached
  -> 200K+ tokens fully billed each turn
```

---

## Drain-accelerating actions

| Action | Risk | Description |
|--------|------|-------------|
| `--resume` | RED | Full conversation replay = billable input (500K+ tokens/run) |
| `/dream`, `/insights` | RED | Background API calls, silent token drain |
| 2+ concurrent terminals | YELLOW | No cache sharing across sessions -> ~2x drain |
| File-rewriting slash commands | YELLOW | 20~27% token consumption per invocation |
| Sub-agent (Haiku) | YELLOW | Cache-read 0%, measured 317K tokens over 31 calls |
| v2.1.89 standalone | RED | Sentinel bug + terminal content disappearance |

---

## Resolution/mitigation

### Immediate (client)

1. **Use npm install** (instead of standalone binary)
   ```bash
   npm install -g @anthropic-ai/claude-code@2.1.90
   ```

2. **Disable auto-updater**
   ```json
   // ~/.claude/settings.json
   { "env": { "DISABLE_AUTOUPDATER": "1" } }
   ```

3. **Avoid `--resume`** — start a new session instead

4. **Limit to one concurrent terminal**

5. **Periodic cache cleanup**
   ```bash
   find ~/.claude/file-history -mtime +7 -type f -delete
   find ~/.claude/paste-cache -mtime +7 -type f -delete
   find ~/.claude/session-env -mtime +7 -type f -delete
   ```

6. **Slim down CLAUDE.md** — it is sent as the system prompt each turn, so its size is direct cost

### Cache monitoring

- Set a transparent proxy in `ANTHROPIC_BASE_URL` -> realtime cache_read/cache_create tracking
- Normal: cache-read ratio 80%+
- Anomaly: session restart recommended if below 40%

### Version guide

| Version | Status |
|---------|--------|
| v2.1.68 or older | OK Bug 2 absent (deferred_tools_delta not yet introduced) |
| v2.1.69~v2.1.88 | WARN Bug 2 present (when --resume) |
| v2.1.89 standalone | RED Bug 1 + Bug 2 simultaneously |
| v2.1.90 | OK Bug 1 fixed, Bug 2 partially mitigated |

---

## Normal cache performance (v2.1.90 baseline)

| Install method | Steady session | Sub-agent cold start |
|----------------|---------------|----------------------|
| npm (Node.js) | 95~99.8% cache read | 79~87% |
| standalone | 95~99.7% | 47~67% (-> 94~99% after warm-up) |

---

## Referenced issues

- anthropics/claude-code#41788 — rate limit exhausted in 70 min (main issue)
- anthropics/claude-code#40524 — Bug 1: conversation history cache invalidation
- anthropics/claude-code#34629 — Bug 2: resume cache break (deferred_tools_delta)
- anthropics/claude-code#42260 — `--resume` token bomb
- anthropics/claude-code#42244 — v2.1.89 terminal content disappearance
- anthropics/claude-code#41249 — limit exhausted in under 1 hour
- anthropics/claude-code#38357 — 5~10x faster drain

---

## Anthropic response status

- For 2+ months, **no official response** on rate-limit issues (as of 2026-04-02)
- Server-side: even after cache patches, drain is still faster than 2~3 weeks prior -> suspected limit recomputation
- Community: ArkNill's analysis repo is tracking independently
