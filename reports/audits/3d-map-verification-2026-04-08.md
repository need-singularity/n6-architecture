# 3D Reality Map Deployment Verification -- 2026-04-08

## Summary

- **URL**: https://need-singularity.github.io/n6-architecture/
- **Render success**: yes (WebGL canvas OK, Three.js r175)
- **Deployed node count**: 247 (EXACT 228 / CLOSE 7 / MISS 12, EXACT 92.3%)
- **Local HEAD node count**: 276
- **Mismatch**: deployment is serving an older reality_map.json

## File locations

- HTML: `docs/index.html` (689 lines, Three.js r175 ESM CDN)
- Data: `docs/reality_map.json` (276 nodes, 49 edges, includes _meta/thread_edges/parent_edges/sibling_edges)
- Reference mode: `fetch("reality_map.json")` relative path (normal)

## GitHub Pages state

- Repo: `need-singularity/n6-architecture`
- Page title: "n=6 Reality Map 3D v6.0 -- WebGL"
- Canvas rendering: normal (L0~L5 spheres, causal edges, X/Y/Z axis labels, HUD all working)
- Console errors: 1 (likely minor, e.g. favicon; no render impact)

## Integrity check

| Item | Local HEAD | Deployed | Match |
|------|----------|--------|------|
| nodes | 276 | 247 | mismatch |
| EXACT% | (needs recomputation) | 92.3% | - |
| Last commit | 26f79544 (2026-04-07 23:35) | not reflected | - |

Branch: `main`, in sync with origin (`up to date`).
HEAD commit (`2cf9141d feat: WebGL 3D map + ...`) has been pushed, but the GitHub Pages build appears to not have refreshed yet.

## Recommended actions

1. Check GitHub Actions Pages build status (`gh run list --workflow=pages-build-deployment`)
2. Force rebuild: push empty commit or re-save Pages settings
3. Cache-bypass check: revisit with `?v=276` querystring
4. After build, re-verify node count matches 276

## Screenshot

`n6-3dmap-2026-04-08.png` (Playwright capture, 1119x1062, viewport)

## Referenced files

- `$N6_ARCH/docs/index.html`
- `$N6_ARCH/docs/reality_map.json`
- `$N6_ARCH/n6-3dmap-2026-04-08.png`
