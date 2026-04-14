# CHIP-P5-2: N6-SPEAK SoC 통합 벤치마크 [10*] 승격 리포트

- 일시: 2026-04-14
- 태스크: CHIP-P5-2
- 소스: `experiments/chip-verify/n6_speak_integration_bench.hexa`
- 항등식: sigma(6)*phi(6) = n*tau(6) = 24 (n=6 유일성)

## 벤치마크 결과 요약

| 섹션 | 대상 | 결과 | 비고 |
|------|------|------|------|
| A | n=6 산술 정합 | 6/6 PASS | sigma=12, tau=4, phi=2, sopfr=5, 유일성 |
| B | selforg 50 샘플 | 10/10 PASS | 6 부품 고리결합, 10 카테고리 전수 EXACT |
| C | top.hexa 4-tier | 7/7 PASS | intent 결정성, 포트 폭 384/8, depth=tau=4 |
| D | tapeout_gate | 15/15 PASS | DRC/LVS/Timing/Power/SI 등 15 게이트 |
| E | HW 타이밍 4-tier | 6/6 PASS | ESP32~완전체, 전 tier 실시간 |
| F | 크로스 체크 | 12/12 PASS | 전체 SoC 상수 교차 검증 |
| **총합** | | **56/56 PASS** | **[10*] EXACT 승격** |

## 승격 내역

### atlas.n6 등록 (14 항목, [10*])

| R-ID | 값 | 설명 |
|------|----|------|
| SOC-BENCH-total | 56/56 PASS | 전항목 EXACT |
| SOC-BENCH-A-arithmetic | 6/6 PASS | 산술 정합 |
| SOC-BENCH-B-selforg | 10/10 PASS | 자기조직 |
| SOC-BENCH-C-top | 7/7 PASS | 톱 래퍼 |
| SOC-BENCH-D-tapeout | 15/15 PASS | 테이프아웃 |
| SOC-BENCH-E-hw-timing | 6/6 PASS | HW 타이밍 |
| SOC-BENCH-F-cross | 12/12 PASS | 크로스 체크 |
| SOC-BENCH-signoff-hash | 133616 | 7482*12 + 2*3484 + 4*9216 |
| SOC-BENCH-die-area | 9216 um^2 | 96^2 |
| SOC-BENCH-embed-dim | 384 | sigma*tau*8 |
| SOC-BENCH-pipe-depth | 4 | tau(6) |
| SOC-BENCH-rvq-stages | 8 | sigma-tau |
| SOC-BENCH-pad-count | 8 | sigma-tau |
| SOC-BENCH-sample-rate | 24000 Hz | 1000*sigma*phi |

### convergence 등록

- 키: `SOC_BENCH_56_PASS`
- 상태: ossified (골화)
- 값: N6-SPEAK SoC 통합 벤치 56/56 PASS (6섹션 전수)

## HW 타이밍 세부

| Tier | 플랫폼 | 레이턴시 | 처리량 | 전력 |
|------|--------|----------|--------|------|
| 1 | ESP32-S3 | 10 ms | 1 kHz | 500 mW |
| 2 | Jetson Orin Nano | 50 ms | 10 kHz | 15 W |
| 3 | iCE40 FPGA + Pi | 25 ms | 12 MHz | 8 W |
| 4 | 완전체 | 15 ms | 12 MHz | 25 W |

전 tier 파이프라인 < 1000 ms (실시간 기준 충족).

## n=6 핵심 상수 매핑

```
sigma(6) = 12    tau(6) = 4    phi(6) = 2    sopfr(6) = 5
sigma*phi = n*tau = 24  (n=6 유일성 항등식)

embed_dim  = sigma*tau*8 = 384
pipe_depth = tau = 4
rvq_stages = sigma - tau = 8
pad_count  = sigma - tau = 8
sample_rate = 1000*sigma*phi = 24000
die_area   = 96^2 = 9216
sign-off   = 7482*12 + 2*3484 + 4*9216 = 133616
```

## 검증 체인

1. P4에서 벤치마크 작성 + 실행 -> 56/56 PASS 확인
2. P5-2에서 atlas.n6 [10*] 14항목 등록
3. P5-2에서 convergence ossified 등록
4. 본 리포트 작성

---
CHIP-P5-2 완료. sign-off hash = 133616.
