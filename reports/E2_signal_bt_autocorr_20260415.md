# E2 Signal vs BT Autocorrelation -- 2026-04-15

> Input signals: atlas.signals.n6 (385 items)
> Input BT files: theory/breakthroughs (105 items)
> Bin: 4 weeks | BT introduction concentrated in 2026-04, so autocorr fragmentary
> Seven-major-problems resolution held at 0/7

## 1. Weekly Distribution

| Week | signal | BT |
|----|-------:|----:|
| 2026-W13 | 5 | 0 |
| 2026-W14 | 65 | 1 |
| 2026-W15 | 87 | 45 |
| 2026-W16 | 228 | 59 |

## 2. Cross-correlation @ lag

- -2: 1.000 STRONG | -1: 0.994 STRONG | 0: 0.854 STRONG | +1: 1.000 STRONG | +2: 1.000 STRONG

## 3. Interpretation

- lag = 0: simultaneous-week synchrony between signal and BT
- lag > 0: signal leads BT (signal -> BT introduction)
- lag < 0: BT leads signal

## 4. Honest Limits

- Data period short (mostly 2026-04), limited number of weekly bins.
- BT filename dates are filename-encoded, not commit times.
- discovered_at concentrated on 2026-04-15, autocorr reliability low.
- Exploratory analysis; no statistical CI.
- Seven-major-problems 0/7.
