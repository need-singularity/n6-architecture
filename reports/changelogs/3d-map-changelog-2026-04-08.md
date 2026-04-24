# 3D Reality Map v6.1 full-set change record

Date: 2026-04-08
Target: `docs/index.html` (Three.js r175 WebGL 3D visualization)
Prior version: v6.0 (2026-04-07, 247 nodes / 49 causal edges)

## Five added features

### 1. Causal-chain animation
- Top-right `>> causal chain` button toggle
- Auto-cycles up to 5 paths from `DATA.edges` (2.2-second interval)
- Pulse highlight on from/to nodes per step (scale 1.6x pulsation)
- Camera OrbitControls target smoothly moves to the midpoint of the two nodes
- On stop, pulse/target restored

### 2. BT back-reference click popup
- On node click, if `bt_refs` array exists, show top-center popup
- Shown: claim, id, BT tag (purple chip), measured, n6_expr, cause, source_url link
- Nodes without bt_refs preserve original behavior (open `source_url` in new tab)
- Of 247 nodes, 229 (92.7%) hold bt_refs -> popup works on nearly all nodes

### 3. origin color/size toggle
- Top-right `[ ] origin color` checkbox
- When ON, repaint every instance color by origin:
  - natural = `#4fc3f7` (cyan)
  - engineering = `#ff7043` (orange)
  - convention = `#b0bec5` (gray)
- When OFF, restore prior level/CHAIN colors
- Operates orthogonally to the search bar's origin filter

### 4. Z-axis 6-domain separation (n=6)
- Top-right `Z=6 domain` checkbox (default ON)
- Compress 7 levels into 6 buckets (L5_material + L5_bio = domain 5)
- Each domain placed on its Z slice (`DOMAIN_Z_GAP = 4 * SCALE_Z`)
- Domain distribution (247 nodes): [47, 20, 33, 15, 16, 116] — all 6 slots filled
- When toggled OFF, restore v6.0 existing thread x index Z coordinates
- On coordinate change, all edge BufferGeometry + arrow cones auto-rebuilt

### 5. WebGL performance optimization (InstancedMesh)
- InstancedMesh adopted already in v6.0 — v6.1 reinforces:
  - On node-coordinate update, single `instanceMatrix.needsUpdate` flag relocates everything
  - On origin-color repaint, single setColorAt + `instanceColor.needsUpdate` call
  - Raycaster fallback distance check keeps 60 FPS at 247 nodes
  - Same code path guaranteed for 1000+ node expansion

## New files

- `.github/workflows/reality_map_verify.yml` — CI verification
  - Node-count >= 200 threshold (regression prevention)
  - 6-domain distribution report
  - bt_refs-holding node count (BT back-reference UI dependency check)
  - Causal edge count
  - Triggers: changes to docs/reality_map.json, docs/index.html, this workflow
- `docs/3d-map-changelog-2026-04-08.md` — this document

## Regression-prevention check

- v6.0 search/filters (search-input, filter-grade, filter-origin) operate as-is
- v6.0 glow sprite EXACT/CHAIN pulsation preserved
- v6.0 thread/parent/sibling/causal four edge kinds all restored (rebuildEdges)
- v6.0 source_url click behavior preserved as fallback for nodes without bt_refs
- Single `index.html` preserved (no-split rule adhered to)
- JS syntax verification: `node --check` PASS

## Added line count

- `docs/index.html`: +about 200 lines (689 -> about 887)
- `.github/workflows/reality_map_verify.yml`: +71 lines
- `docs/3d-map-changelog-2026-04-08.md`: this file

## CI local pre-verification

```
nodes=247 exact=228 bt_refs=229 domains=[47, 20, 33, 15, 16, 116] edges=49
```
- 247 >= 200 threshold PASS
- No empty domain PASS
- bt_refs 92.7% coverage PASS
