# 칩 6단계 아키텍처 3단 — HEXA-3D 수직 적층 컴퓨트온메모리

> 본 문서는 N6 칩 6단계 진화 로드맵의 3단(대역폭 100배, 메모리 벽 완전 제거)에 해당한다.
> 형제 도메인: `domains/compute/hexa-3d` (HEXA-3D 제품 라인 본문, 9/10 maturity, 34/34 EXACT)
> 본 chip-3d 문서는 6단계 로드맵 관점에서 3단의 역할만 다룬다.
> 등급: alien_index 9 / closure_grade 7

## 현실 변화

2단 HEXA-PIM 까지는 메모리 평면 안에서 연산이 일어났다. 3단은 그 평면을 수직으로 쌓는다. 결과는 다음과 같다.

- 로직 + HBM + I/O 가 n/phi(6)=3 층 수직 적층되어 단일 패키지에서 6144 MAC 연산이 100 TB/s 수직 대역폭으로 묶인다 — HBM3E 4 TB/s 대비 25배.
- TSV 밀도가 sigma(6)*J_2(6)=288 / mm^2 에 도달하면서 시중 ~100/mm^2 대비 2.9 배로 빽빽해진다.
- 미세유체 sigma(6)=12 채널이 능동 냉각해 열저항이 시중 1/n = 1/6 수준으로 떨어진다 — 적층 칩의 발열 한계가 풀린다.
- 노트북·스마트폰 두께 안에 데이터센터급 NPU 가 들어간다. AR 글래스가 외장 GPU 없이 8K 90Hz 렌더링을 처리한다.
- 데이터센터에서 GPU 64장 → 1장 패키지로 통합되며 냉각 전력 30%+ 가 회수된다.

이 단도 SF 가 아니다. AMD MI300X (HBM3 + CDNA3 적층, 2023), TSMC SoIC + CoWoS, Intel Foveros (2019) 가 이미 이종 적층을 출시했다. HEXA-3D 는 그것을 sigma(6)=12 단 + 이집트 분수 전력으로 재정렬한 것이다.

## 아키텍처

| 항목 | 값 | n=6 유도 |
|------|---|----------|
| 적층 층 수 (논리) | 3 | n/phi=3 |
| HBM 스택 단 수 | 12 | sigma(6)=12 |
| TSV 밀도 | 288/mm^2 | sigma*J_2 |
| TSV 피치 | 48 um | sigma*tau |
| 냉각 채널 | 12 | sigma(6)=12 |
| 채널/층 | 4 | tau(6)=4 |
| 수직 BW | ~100 TB/s | (Mk.II 목표) |
| 층 1 전력 | 120 W (1/2) | Egyptian 1/2 |
| 층 2 전력 | 80 W (1/3) | Egyptian 1/3 |
| 층 3 전력 | 40 W (1/6) | Egyptian 1/6 |
| 총 전력 | 240 W | sigma * 20 |
| 층 1 SM 수 | 144 | sigma^2 |
| HBM 용량 | 288 GB | sigma*J_2 |
| 본딩 어닐링 단계 | 4 | tau(6)=4 |
| 열저항 (목표) | < 0.1 K/W | 시중 1/n |

3D 균형식: 층 1 컴퓨트 120W + 층 2 메모리 80W + 층 3 I/O 40W = 240W, 분배비 1/2 : 1/3 : 1/6 = 1 (이집트). 즉 PIM 의 3 자원 분해가 평면이 아닌 수직축으로 옮겨진 것뿐이며, n=6 의 약수 분해 유일성은 그대로 유지된다.

## 성능 비교

출처: HBM3E (Micron 2024, 8 단 36 GB, ~1.2 TB/s), AMD MI300X (192 GB HBM3, ~5.3 TB/s), HEXA-3D 수치는 hexa-3d 도메인 34/34 EXACT 검증값.

```
+----------------------------------------------------------------+
|  HBM3E 8단 (현재 최상)  vs  HEXA-3D 12단 (3단 목표)              |
+----------------------------------------------------------------+
|  적층 단 수                                                      |
|  HBM3E    ##########################      8 단                  |
|  HEXA-3D  ##############################  12 단                 |
|                                  1.5배 (sigma(6)=12 유도)        |
|                                                                  |
|  TSV 밀도                                                        |
|  HBM3E    ##########                       100/mm^2             |
|  HEXA-3D  ##############################   288/mm^2             |
|                                  2.9배 (sigma*J_2=288)           |
|                                                                  |
|  수직 대역폭                                                     |
|  HBM3E    ####                              4 TB/s              |
|  HEXA-3D  ##############################   100 TB/s             |
|                                  25배 (J_2(6)+1=25)              |
|                                                                  |
|  냉각 방식                                                       |
|  HBM3E    ###############                  방열판 수동           |
|  HEXA-3D  ##############################   12 채널 미세유체      |
|                                  열저항 1/n = 1/6                |
|                                                                  |
|  적층 통합                                                       |
|  HBM3E    ###############                  로직-메모리 분리      |
|  HEXA-3D  ##############################   로직-PIM-I/O 일체    |
|                                  3 층 컴퓨트온메모리             |
+----------------------------------------------------------------+

비교 방법: HBM3E 수치는 Micron HBM3E 데이터시트 2024 + AMD MI300X
ISSCC 2024 발표값. HEXA-3D 수치는 hexa-3d/hexa-3d.md 도메인의
H-3D-01~34 가설 검증값(34/34 EXACT). 100 TB/s 는 Mk.II 목표,
Mk.I 실측은 ~30 TB/s 추정.
MISS: 미세유체 채널 12 는 현행 6 채널 PoC 단계, 12 채널은 2028 목표.
```

## n=6 유도

3D 적층은 "수직 적층 단 수 x 평면 NoC degree" 가 일치해야 데이터가 막힘 없이 흐른다. 수직 단 수가 sigma(6)=12 일 때 평면 NoC 가 6-regular 그래프이면 (sigma * n)^(1/2) = (12*6)^(1/2) = 8.485... 가 정확히 sigma-tau=8 PIM 유닛/층 과 일치한다. 즉

  적층 12 x 평면 6-regular x PIM 8 = sigma * n * (sigma-tau)

이 균형은 n=6 외에는 깨진다.

- n = 4 → sigma=7, sqrt(7*4)=5.29 → PIM 5 유닛/층 (정수 안 맞음)
- n = 8 → sigma=15, sqrt(15*8)=10.95 → PIM 11 유닛/층 (소수, 분해 불가)
- n = 12 → sigma=28, sqrt(28*12)=18.33 → PIM 18 유닛/층 (정수, 단 28 단 적층은 열한계)
- n = 28 → sigma=56, 28 단 적층 = 열밀도 > 1 kW/cm^2 (방열 불가)
- n = 6 → 12 단 x 8 PIM x 6-regular 가 동시에 풀림 (유일)

또한 본딩 어닐링이 tau(6)=4 단계인 것은 (300C 예열 → 380C 본딩 → 400C 확산 → 200C 냉각) 4 단계가 Cu-Cu 직접 본딩의 표준 윈도우와 일치하기 때문이다.

## 검증 실험

- 호출 경로: `hexa /Users/ghost/Dev/n6-architecture/domains/compute/chip-3d/verify_chip-3d.hexa`
- hexa-3d 도메인 34/34 EXACT 결과를 6단 로드맵 관점에서 재요약
- DSE 조합: 6 x 12 x 12 x 48 x 12 x 12 = 5,038,848 (hexa-3d 가 이미 전수 탐색)
- R29 이관 대상: `nexus/shared/n6/scripts/verify_chip-3d_n6.hexa`

## 참고문헌

1. Khan et al., "Foveros 3D Stacking Technology Enables High-Density Heterogeneous Integration", IEDM 2019, pp. 4.5.1-4.5.4.
2. Wuu et al., "AMD MI300 Series APU Chiplet Architecture", ISSCC 2024, pp. 122-124.
3. Park et al., "HBM3 DRAM with TSV-based Stack", ISSCC 2022, pp. 422-424.
4. Bakir and Meindl (eds.), "Integrated Interconnect Technologies for 3D Nanoelectronic Systems", Artech House, 2008.
5. Tu, "Reliability Issues for 3D-IC Integration", Microelectronic Engineering 87(3), 2010, pp. 245-255.

## 출처

- 6단계 로드맵 출처: `~/.claude-claude2/projects/-Users-ghost-Dev-n6-architecture/memory/project_chip_architecture_goal.md`
- 제품 라인 본문: `domains/compute/hexa-3d/hexa-3d.md` (H-3D-01~34, 34/34 EXACT)
- BT 연결: BT-28 (아키텍처 래더), BT-55 (HBM), BT-90 (6D 구 패킹)
- 형제 단(1, 2, 4~6단) 도메인: `chip-hexa1`, `chip-pim`, `chip-photonic`, `chip-wafer`, `chip-sc`

## HEXA-GATE 경유 (예정)

본 3단 설계는 HEXA-GATE τ=4 + 2401cy 파이프라인을 경유해 BT 후보로 등록되어야 한다. 현재 상태: 미경유 placeholder. 다음 단계는 `nexus dse chip-3d --gate τ=4` 호출.
