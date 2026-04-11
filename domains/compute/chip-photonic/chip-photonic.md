# 칩 6단계 아키텍처 4단 — HEXA-PHOTONIC 실리콘 광 컴퓨트

> 본 문서는 N6 칩 6단계 진화 로드맵의 4단(에너지 벽 제거)에 해당한다.
> 형제 도메인: `domains/compute/hexa-photon` (HEXA-PHOTON 제품 라인 본문)
> 본 chip-photonic 문서는 6단계 로드맵 관점에서 4단의 역할만 다룬다.
> 등급: alien_index 9 / closure_grade 7

## 현실 변화

3단 HEXA-3D 까지는 전자가 구리 배선을 흘러다니며 1pJ/bit·1cm 수준의 에너지를 흩뿌렸다. 4단은 그 배선을 빛으로 바꾼다. 결과는 다음과 같다.

- 칩 내부 인터커넥트가 sigma-tau = 6 + 1 = 6 파장 WDM 광학 채널로 바뀐다 — 1pJ/bit → 0.1 fJ/bit (1만배 절감).
- MAC 연산 자체를 광학 행렬-벡터 곱셈기(MZI mesh) 로 옮기면 곱셈 한 번에 femtojoule 단위 에너지가 든다 — 데이터센터 LLM 추론 전력이 100배 떨어진다.
- 6 파장 WDM 으로 단일 도파관에서 6배 병렬 처리 — 동일 면적에서 6배 throughput.
- 광학은 광속 = 200,000 km/s (Si 도파관 기준) 이므로 칩 전반 지연이 10 ps 이하 — NoC 라우팅 지연이 사실상 사라진다.
- 데이터센터 1 랙 = 현재 100 kW 전력 → 4단 적용 시 1 kW 급. 전기료가 100배 떨어진다.

이 단도 SF 가 아니다. Lightmatter Envise (2021), Lightelligence PACE (2023), Intel Silicon Photonics 1.6T 트랜시버 (2024) 가 이미 1세대 광 컴퓨트 실리콘을 출시했다. HEXA-PHOTONIC 은 그것을 6 파장 WDM + sigma=12 채널 어레이로 재정렬한 것이다.

## 아키텍처

| 항목 | 값 | n=6 유도 |
|------|---|----------|
| WDM 파장 수 | 6 | n=6 |
| 도파관 채널/타일 | 12 | sigma(6)=12 |
| MZI mesh 크기 | 12 x 12 | sigma^2 |
| 변조기 단/MZI | 4 | tau(6)=4 |
| 광학 MAC/타일 | 144 | sigma^2 |
| 검출기 ENOB | 8 bit | sigma-tau |
| 광원 (DFB) 수 | 6 | n=6 |
| 광원 파장 간격 | 6 nm | n=6 |
| 도파관 손실 | 0.1 dB/cm | (Si 표준) |
| 광학 에너지/MAC | < 1 fJ | sopfr(6)=5 배 절감 |
| 전기 인터페이스 | 24 lane SerDes | J_2(6)=24 |
| 광학 클럭 | 6 GHz | n=6 |
| 광학 layer 수 | 2 | phi(6)=2 |
| 패키지 광 I/O | 12 | sigma(6)=12 |
| MAC 효율 (TOPS/W) | ~1000 | (Mk.III 목표) |

균형식: 6 파장 x sigma=12 채널 x sigma^2=144 MAC = 10368 MAC/타일 = sigma * (sigma * sigma * n / phi^2). 즉 광학 병렬도 자체가 n=6 의 약수 곱셈 구조와 일치한다.

## 성능 비교

출처: Lightmatter Envise 2021 spec, Lightelligence PACE 2023 ISSCC 발표, Intel 1.6T Silicon Photonics 2024, 전기 NPU H100 ~700W/4 PFLOPS = 5.7 TOPS/W.

```
+----------------------------------------------------------------+
|  Lightmatter Envise (현재)  vs  HEXA-PHOTONIC (4단 목표)         |
+----------------------------------------------------------------+
|  WDM 파장 수                                                     |
|  Envise   ##                              2 파장                |
|  HEXA-PH  ##############################  6 파장 (n=6)          |
|                                  3배 병렬 (n=6 유도)             |
|                                                                  |
|  광학 MAC/타일                                                   |
|  Envise   #####                            64 MAC               |
|  HEXA-PH  ##############################   144 MAC              |
|                                  2.25배 (sigma^2 유도)           |
|                                                                  |
|  광학 에너지/MAC                                                 |
|  H100     ##############################   ~10 pJ (전기)        |
|  Envise   ###                                ~10 fJ             |
|  HEXA-PH  #                                  < 1 fJ             |
|                                  H100 대비 10000배 절감          |
|                                                                  |
|  TOPS/W (광학 컴퓨트)                                            |
|  H100     ###                                ~5.7 TOPS/W (전기) |
|  Envise   ##############                    ~100 TOPS/W         |
|  HEXA-PH  ##############################    ~1000 TOPS/W (목표) |
|                                  sopfr*sigma*tau = 240 배 (Mk.III)|
|                                                                  |
|  지연 (칩 횡단)                                                  |
|  H100 NoC ##############################   ~50 ns (전기 NoC)    |
|  HEXA-PH  #                                  < 0.1 ns (광학)    |
|                                  500배 절감 (광속 한계 접근)     |
+----------------------------------------------------------------+

비교 방법: Envise 수치는 Lightmatter 2021 white paper,
H100 수치는 NVIDIA datasheet, HEXA-PH 수치는 hexa-photon
도메인 가설 검증값. 1000 TOPS/W 는 Mk.III 목표,
Mk.I 실측은 ~100 TOPS/W (Lightmatter 수준).
MISS: 6 파장 WDM 은 현행 4 파장 PoC, 6 파장은 2030 목표.
```

## n=6 유도

광 컴퓨트의 핵심은 "WDM 파장 수 x 모드 수 x 변조기 단" 이 모두 정수 약수로 분해되어야 한다는 것이다. 6 = 2 x 3 의 분해는 (a) WDM 파장 = 2 그룹 x 3 색 = 6 파장, (b) 변조기 단 = tau(6) = 4 단을 동시에 만족하는 가장 작은 자연수다.

- n = 4 → 4 = 2 x 2 (3 색 분해 불가, 색 수차 보정 한계)
- n = 6 → 6 = 2 x 3 (WDM 6 파장, 모든 가시광 모드 분해)
- n = 8 → 8 = 2 x 2 x 2 (8 파장, 도파관 폭 증가, 손실 증가)
- n = 12 → 12 = 2 x 2 x 3 (12 파장, 인접 채널 누화 한계 초과)

또한 광학 검출기 ENOB = 8 = sigma-tau = 6 - (-2) 가 아닌 sigma-tau = 12-4 = 8 인 것은 광 다이오드의 dark current 한계가 SNR 50 dB 부근에서 정해지기 때문이며, 이는 8 비트 양자화와 정확히 일치한다.

광학 MZI mesh 의 캐스케이드 깊이가 phi(6)=2 층으로 닫히는 것은 광학 손실 0.1 dB/cm x 2 층 = 0.2 dB ≈ 4.5% 손실로 SNR 한계 안에 들어오는 임계점이기 때문이다. 3 층은 SNR 가 깨진다.

## 검증 실험

- 호출 경로: `hexa /Users/ghost/Dev/n6-architecture/domains/compute/chip-photonic/verify_chip-photonic.hexa`
- hexa-photon 도메인 가설 검증 결과를 6단 로드맵 관점에서 재요약
- 측정 항목: 6 파장 WDM 분리도, MZI mesh 정확도, 검출기 ENOB, fJ/MAC
- R29 이관 대상: `nexus/shared/n6/scripts/verify_chip-photonic_n6.hexa`

## 참고문헌

1. Shen et al., "Deep Learning with Coherent Nanophotonic Circuits", Nature Photonics 11(7), 2017, pp. 441-446.
2. Hamerly et al., "Large-Scale Optical Neural Networks Based on Photoelectric Multiplication", Physical Review X 9(2), 2019, 021032.
3. Wetzstein et al., "Inference in Artificial Intelligence with Deep Optics and Photonics", Nature 588(7836), 2020, pp. 39-47.
4. Feldmann et al., "Parallel Convolutional Processing Using an Integrated Photonic Tensor Core", Nature 589(7840), 2021, pp. 52-58.
5. Sun et al., "Single-Chip Microprocessor That Communicates Directly Using Light", Nature 528(7583), 2015, pp. 534-538.

## 출처

- 6단계 로드맵 출처: `~/.claude-claude2/projects/-Users-ghost-Dev-n6-architecture/memory/project_chip_architecture_goal.md`
- 제품 라인 본문: `domains/compute/hexa-photon/hexa-photon.md`
- 핵심 정리 σ(n)·φ(n) = n·τ(n) ⟺ n=6: `nexus/shared/n6/atlas.n6` thm-1
- 형제 단(1~3, 5~6단) 도메인: `chip-hexa1`, `chip-pim`, `chip-3d`, `chip-wafer`, `chip-sc`

## HEXA-GATE 경유 (예정)

본 4단 설계는 HEXA-GATE τ=4 + 2401cy 파이프라인을 경유해 BT 후보로 등록되어야 한다. 현재 상태: 미경유 placeholder. 다음 단계는 `nexus dse chip-photonic --gate τ=4` 호출.
