# E2 Signal vs BT Autocorrelation — 2026-04-15

> 입력 signals: `/Users/ghost/Dev/nexus/shared/n6/atlas.signals.n6` (385건)
> 입력 BT 파일: `/Users/ghost/Dev/n6-architecture/theory/breakthroughs` (105건)
> 주 단위 bin: 4 주
> 정직: BT 도입 시점이 2026-04 에 집중 — autocorr 단편적.
> 7대 난제 해결 0/7 유지.

## 1. 주별 분포

| 주 | signal | BT |
|----|-------:|----:|
| 2026-W13 | 5 | 0 |
| 2026-W14 | 65 | 1 |
| 2026-W15 | 87 | 45 |
| 2026-W16 | 228 | 59 |

## 2. cross-correlation @ lag

| lag (주) | r | 해석 |
|---------|---|------|
| -3 | — | data 부족 |
| -2 | 1.000 | STRONG |
| -1 | 0.994 | STRONG |
| +0 | 0.854 | STRONG |
| +1 | 1.000 | STRONG |
| +2 | 1.000 | STRONG |
| +3 | — | data 부족 |

## 3. 해석

- lag = 0: 동시 주에서 signal 과 BT 의 동조성 (r > 0 = 함께 증가)
- lag > 0: signal 이 BT 를 **선행** (signal → BT 도입)
- lag < 0: BT 가 signal 을 **선행** (BT → signal 추가)

## 4. 정직 한계

- 데이터 기간이 짧음 (대부분 2026-04). 주 단위 bin 수 한정.
- BT 파일명 날짜는 commit 시점이 아닌 파일명 인코딩 — 실제 발견일과 차이 가능.
- signal discovered_at 도 다수가 2026-04-15 1일 집중 — autocorr 신뢰도 낮음.
- 본 분석은 explorative. 통계적 신뢰구간 미산출.
- 7대 난제 0/7 유지.