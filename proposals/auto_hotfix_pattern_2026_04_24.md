# AUTO: Hotfix pattern audit — 2026-04-24

**source**: Phase 8 propose_from_gaps (hotfix_commit_count_30d=5 ≥ 5)

The last 30 days include 5 commits whose subject matches one of
`workaround | hotfix | emergency | regression | revert`. Review whether
these cluster around a single module/file; if they do, promote the
underlying cause to a ROI item.

## Action checklist

- [ ] `git log --since='30 days ago' --grep='workaround\|hotfix\|emergency\|regression\|revert' --name-only` — inspect.
- [ ] Identify the top-3 hottest paths.
- [ ] If one module shows ≥3 hits: open a root-cause ROI item and link back here.
- [ ] Otherwise mark this audit closed (no clustering).

## Resolution

<!-- fill in when this audit closes -->

## Meta

- Generated automatically. Delete or fill in manually.
- Re-running `bin/n6_meta propose` on a different day creates a new
  audit file; same-day runs are idempotent.
