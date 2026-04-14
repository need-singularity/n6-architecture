# 3D 현실 지도 배포 검증 — 2026-04-08

## 요약

- **URL**: https://need-singularity.github.io/n6-architecture/
- **렌더 성공**: 예 (WebGL 캔버스 정상, Three.js r175)
- **배포본 노드 수**: 247 (EXACT 228 / CLOSE 7 / MISS 12, EXACT 92.3%)
- **로컬 HEAD 노드 수**: 276
- **불일치**: 배포본이 구버전 reality_map.json을 서빙 중

## 파일 위치

- HTML: `docs/index.html` (689줄, Three.js r175 ESM CDN)
- 데이터: `docs/reality_map.json` (276 nodes, 49 edges, _meta/thread_edges/parent_edges/sibling_edges 포함)
- 참조 방식: `fetch("reality_map.json")` 상대 경로 (정상)

## GitHub Pages 상태

- 리포: `need-singularity/n6-architecture`
- 페이지 제목: "n=6 Reality Map 3D v6.0 — WebGL"
- 캔버스 렌더링: 정상 (L0~L5 구체, 인과 엣지, X/Y/Z 축 라벨, HUD 모두 동작)
- 콘솔 에러: 1건 (favicon 등 사소 추정, 렌더 무영향)

## 무결성 검증

| 항목 | 로컬 HEAD | 배포본 | 일치 |
|------|----------|--------|------|
| nodes | 276 | 247 | 불일치 |
| EXACT% | (재계산 필요) | 92.3% | - |
| 마지막 커밋 | 26f79544 (2026-04-07 23:35) | 미반영 | - |

브랜치: `main`, origin과 동기화 상태 (`up to date`).
HEAD 커밋(`2cf9141d feat: WebGL 3D지도 + …`)까지 push 완료되었으나 GitHub Pages 빌드가 아직 갱신되지 않은 것으로 추정.

## 권장 조치

1. GitHub Actions Pages 빌드 상태 확인 (`gh run list --workflow=pages-build-deployment`)
2. 강제 재빌드: 빈 커밋 push 또는 Pages 설정 재저장
3. 캐시 우회 검증: `?v=276` 쿼리스트링으로 재방문
4. 빌드 후 노드 수 276 일치 재검증

## 스크린샷

`n6-3dmap-2026-04-08.png` (Playwright 캡처, 1119×1062, viewport)

## 참조 파일

- `$N6_ARCH/docs/index.html`
- `$N6_ARCH/docs/reality_map.json`
- `$N6_ARCH/n6-3dmap-2026-04-08.png`
