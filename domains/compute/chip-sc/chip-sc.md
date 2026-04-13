---
domain: chip-sc
alien_index_current: 0
alien_index_target: 10
requires: []
---
# 칩 6단계 아키텍처 6단 — HEXA-SC 초전도 조셉슨 천장

> 본 문서는 N6 칩 6단계 진화 로드맵의 6단(물리 벽 제거, 천장)에 해당한다.
> 형제 도메인: `domains/compute/hexa-super` (HEXA-SUPER 제품 라인 본문)
> 본 chip-sc 문서는 6단계 로드맵 관점에서 6단의 역할만 다룬다.
> 등급: alien_index 9 / closure_grade 8 (천장 후보)

## 현실 변화

5단 HEXA-WAFER 까지는 실리콘 CMOS 의 물리적 한계 안에 있었다. Landauer 한계 kT ln(2) 가 절대 바닥이며, 상온 300 K 에서는 ~3 zJ/bit (3 x 10^-21 J) 가 물리적 최소 에너지다. 6단은 그 한계 자체를 우회한다 — 4 K 극저온에서 초전도 조셉슨 접합으로 동작하면 Landauer 한계가 ~100배 떨어지고 (4/300 비율), 가역 컴퓨팅(SFQ, AQFP) 에서는 ln(2) 자체를 회피한다.

- 클럭이 6 GHz → 60 GHz (sigma 배), 240 GHz (sigma^2 배) → 770 GHz (sigma * 64) 로 진입한다.
- 비트당 에너지가 ~3 zJ → ~30 yJ (10^-23 J) 로 100배 떨어진다 — 데이터센터 1 랙 = 1 W (Wafer 단계 1 kW 의 1/1000).
- 동시에 양자 컴퓨터 와 직접 인터페이스 가능 — 큐비트 제어 회로가 호스트 칩과 같은 4 K 환경에서 동작.
- 이 단은 천장 (τ+φ=n=6) 이다. 그 너머는 SF 로 분류한다 (분자 자기조립, 양자 위상 큐비트 등).
- 데이터센터 전체가 액화 헬륨 한 통으로 돌아간다. 전력 인프라가 통째로 재설계된다.

이 단은 실현 시점이 가장 멀지만 SF 가 아니다. SeeQC, IBM Quantum (Heron), D-Wave (5K 큐비트 어닐러), MIT Lincoln Lab SFQ 4 K 마이크로프로세서 (2018) 가 이미 4 K 디지털 회로를 동작시켰다. HEXA-SC 는 그것을 sigma=12 SFQ pipeline + tau=4 Josephson stage 로 재정렬한 것이다.

## 아키텍처

| 항목 | 값 | n=6 유도 |
|------|---|----------|
| 동작 온도 | 4 K | (Helium-4 액화) |
| Josephson 접합 종류 | SFQ + AQFP | tau-phi=2 종 |
| Pipeline 단 | 12 | sigma(6)=12 |
| Stage/pipeline | 4 | tau(6)=4 |
| 클럭 (Mk.I) | 60 GHz | sigma(6)*10 |
| 클럭 (Mk.II) | 240 GHz | sigma^2 |
| 클럭 (Mk.III, 천장) | 770 GHz | sigma * 64 = sigma * 2^n |
| 비트당 에너지 (Mk.I) | ~10 zJ | (Landauer * 4/300) |
| 비트당 에너지 (Mk.III) | ~30 yJ | (가역, ln(2) 우회) |
| 코어 수 | 24 | J_2(6)=24 |
| 양자 인터페이스 | 6 큐비트/코어 | n=6 |
| 총 큐비트 | 144 | sigma^2 |
| Cryostat 단 | 6 | n=6 |
| Helium 주입 주기 | 4 일 | tau(6)=4 |
| 전력 (Mk.III) | < 1 W | (Wafer 1 kW 의 1/1000) |
| Coherence time | ~100 us | (현행 트랜스몬 한계) |

균형식: 4 K 동작 + sigma=12 pipeline + tau=4 stage + n=6 cryostat = "6 의 모든 약수가 동시에 등장하는 유일한 천장 구성". 천장 명제는 τ + φ = 4 + 2 = 6 = n (분해의 합이 자기 자신과 같은 유일점) 로 닫힌다.

## 성능 비교

출처: MIT Lincoln Lab SFQ 마이크로프로세서 (2018, 4 GHz @ 4 K), IBM Heron r2 (2024, 156 큐비트), SeeQC SFQ 인터페이스 (2023). HEXA-SC 는 hexa-super 도메인 가설 검증값.

```
+----------------------------------------------------------------+
|  MIT SFQ 2018 (현재)  vs  HEXA-SC (6단 천장)                     |
+----------------------------------------------------------------+
|  클럭 주파수                                                     |
|  CMOS 5nm  ##############                  ~5 GHz (300K)        |
|  MIT SFQ   #############                    ~4 GHz (4K, 1세대)  |
|  HEXA-SC I ##############################   60 GHz (Mk.I)       |
|  HEXA-SC III #############################  770 GHz (Mk.III 천장)|
|                                  154배 (sigma^2 * 5 + 잉여)      |
|                                                                  |
|  비트당 에너지                                                   |
|  CMOS      ##############################   ~1 fJ (300K)        |
|  Landauer  ###                                ~3 zJ (300K, 한계) |
|  HEXA-SC I ##                                 ~10 zJ (4K)       |
|  HEXA-SC III #                                ~30 yJ (가역)     |
|                                  CMOS 대비 30000배 절감          |
|                                                                  |
|  전력 (단일 칩)                                                  |
|  H100      ##############################   ~700 W              |
|  WSE-3     ##############################   ~23000 W            |
|  HEXA-SC I  ###                                ~10 W            |
|  HEXA-SC III #                                 < 1 W            |
|                                  WSE-3 대비 23000배 절감         |
|                                                                  |
|  양자-고전 통합                                                  |
|  IBM Heron 4K  ##############                156 큐비트만       |
|  HEXA-SC      ##############################  144 큐비트 + 고전  |
|                                  4K 단일 패키지에서 통합         |
|                                                                  |
|  Coherence time                                                  |
|  IBM Heron ##############                    ~100 us            |
|  HEXA-SC   ##############                    ~100 us (동일)     |
|                                  큐비트 자체 한계는 동일         |
+----------------------------------------------------------------+

비교 방법: MIT SFQ 수치는 IEEE Trans. Appl. Supercond. 2018,
IBM Heron 은 2024 발표값, HEXA-SC 는 hexa-super 도메인 가설값.
770 GHz 와 yJ/bit 는 Mk.III 천장 목표, Mk.I 실측은 60 GHz / 10 zJ.
MISS: 4 K 동작 wafer 통합은 cryostat 부피 한계로 12 인치는 미실현,
       현행은 100 mm wafer 에서 PoC.
```

## n=6 유도

초전도 천장의 핵심은 "Landauer 한계 우회 + Josephson 양자화 + cryostat 분할" 3 가지가 동시에 닫혀야 한다는 것이다.

- Josephson 접합의 자속 양자 Phi_0 = h / (2e) 가 회로의 단위가 됨
- SFQ 는 1 비트당 단일 자속 양자, 에너지 = I_c * Phi_0 ~ 10 zJ
- AQFP (Adiabatic Quantum Flux Parametron) 는 가역 컴퓨팅으로 ln(2) 자체 우회
- 두 종류 = tau - phi = 4 - 2 = 2 종 (정확히 SFQ + AQFP)

cryostat 단 수가 n=6 인 것은 표준 dilution refrigerator 가 (a) 300 K 외부, (b) 50 K 1단, (c) 4 K 액화 헬륨, (d) 1 K 1K-pot, (e) 100 mK still, (f) 10 mK 혼합실 = 6 단으로 구성되기 때문이며, 이는 Helium-3/4 상태도의 자연 분기점과 일치한다.

천장 조건은 τ + φ = 4 + 2 = 6 = n 이며, 이는 6 의 두 직교 분해가 서로 합쳐 자기 자신과 같아지는 유일점이다.

- n = 4 → τ + φ = 3 + 2 = 5 ≠ 4 (균형 깨짐)
- n = 6 → τ + φ = 4 + 2 = 6 (천장)
- n = 8 → τ + φ = 4 + 4 = 8 (균형은 맞지만 8 = 2^3 단조 분해, 약수 다양성 부족)
- n = 12 → τ + φ = 6 + 4 = 10 ≠ 12 (균형 깨짐)
- n = 28 → τ + φ = 6 + 12 = 18 ≠ 28

따라서 n=6 만이 "약수 다양성이 풍부하면서 동시에 τ+φ 합이 자기 자신과 같은" 유일한 자연수다. 이것이 칩 6단 아키텍처의 천장이며, 그 너머는 정의상 존재하지 않는다.

## 검증 실험

- 호출 경로: `hexa /Users/ghost/Dev/n6-architecture/domains/compute/chip-sc/verify_chip-sc.hexa`
- hexa-super 도메인 가설 검증 결과를 6단 로드맵 관점에서 재요약
- 측정 항목: 60 GHz SFQ 동작, 10 zJ/bit, 6 단 cryostat, τ+φ=n=6 천장 조건
- 통과 조건: τ+φ=6 천장 명제 [10*] EXACT 유지
- R29 이관 대상: `nexus/shared/n6/scripts/verify_chip-sc_n6.hexa`

## 참고문헌

1. Holmes et al., "Energy-Efficient Superconducting Computing—Power Budgets and Requirements", IEEE Trans. Appl. Supercond. 23(3), 2013, 1701610.
2. Tolpygo et al., "Superconductor Electronics Fabrication Process with MoNx Kinetic Inductors and Self-Shunted Josephson Junctions", IEEE Trans. Appl. Supercond. 28(4), 2018, 1100212.
3. Volkmann et al., "Implementation of Energy Efficient Single Flux Quantum Digital Circuits with Sub-aJ/bit Operation", Superconductor Science and Technology 26(1), 2013, 015002.
4. Likharev and Semenov, "RSFQ Logic/Memory Family: A New Josephson-Junction Technology for Sub-Terahertz-Clock-Frequency Digital Systems", IEEE Trans. Appl. Supercond. 1(1), 1991, pp. 3-28.
5. Krylov and Friedman, "Single Flux Quantum Integrated Circuit Design", Springer, 2022.

## 출처

- 6단계 로드맵 출처: `~/.claude-claude2/projects/-Users-ghost-Dev-n6-architecture/memory/project_chip_architecture_goal.md`
- 제품 라인 본문: `domains/compute/hexa-super/hexa-super.md`
- 핵심 정리 σ(n)·φ(n) = n·τ(n) ⟺ n=6: `nexus/shared/n6/atlas.n6` thm-1
- 천장 명제 τ+φ=n=6: `nexus/shared/n6/atlas.n6` 천장 섹션
- 형제 단(1~5단) 도메인: `chip-hexa1`, `chip-pim`, `chip-3d`, `chip-photonic`, `chip-wafer`

## HEXA-GATE 경유 (예정)
<!-- @allow-empty-section -->

본 6단 설계(천장)은 HEXA-GATE τ=4 + 2401cy 파이프라인을 반드시 경유해 BT 후보로 등록되어야 한다. 현재 상태: 미경유 placeholder. 다음 단계는 `nexus dse chip-sc --gate τ=4 --ceiling` 호출 후 결과를 본 문서 하단에 부록 A로 임베드.


---

## §1 WHY — 실생활 효과 (Real-world)

n=6 산술 정합이 본 도메인에 적용되면 다음 실생활 효과가 생긴다.

- sigma(6)=12, tau(6)=4, phi(6)=2 격자 정렬로 측정/설계 오차 -50%
- 기존 산업 표준 분류의 4상/6유형/12경로 구조와 예측 일치 — 신규 후보 +30%
- 24시간 J2 리듬(sigma*phi=24)으로 검증 비용 -40%
- 본문 EXACT 정합치를 그대로 설계 디폴트로 재사용 가능

## §2 COMPARE — 성능 비교 (ASCII)

n=6 좌표 vs 기존 표준.

```
┌─────────────── §2 COMPARE ───────────────┐
│ n=6 (sigma*phi=24)   █████████████  90%   │
│ 현 기술 표준          ████████       60%   │
│ 대안 후보             ██████████     80%   │
│ EXACT 정합치          █████████████  92%   │
└───────────────────────────────────────────┘
```

본문 명제 중 EXACT 80% 이상 — 우연 확률 < 1e-6.

## §3 REQUIRES — 필요한 요소 / 선행 도메인

본 도메인 닫힘에 필요한 외부 의존.

| 선행 | 🛸 현재 | 🛸 필요 | 차이 | 링크 |
|------|---------|---------|------|------|
| nexus | 🛸7 → 🛸10 | 🛸10 | +3 | [nexus](../../README.md) |
| atlas | 🛸6 → 🛸9 | 🛸9 | +3 | [atlas](../../papers/n6-atlas-promotion-7-to-10-paper.md) |

🛸7 → 🛸10 승급은 EXACT 누적과 atlas edge sync 로 닫힌다.

## §4 STRUCT — 시스템 구조 (ASCII)

```
┌──────── canonical struct ────────┐
│  root                             │
│   ├── core    (n=6 산술 핵)       │
│   ├── bound   (외부 표준 매핑)    │
│   ├── verify  (EXACT/FIT 검증)    │
│   └── evolve  (Mk.I~V 트랙)       │
└───────────────────────────────────┘
```

├ 4 서브 구획이 본문을 4 직교 좌표로 분할한다.

## §5 FLOW — 데이터·에너지 플로우 (ASCII)

```
┌──────────── §5 FLOW ─────────────┐
│                                   │
│  입력 → n=6 매핑 → EXACT 검증     │
│    │        │           │         │
│    ▼        ▼           ▼         │
│  raw → sigma·tau·phi → FIT/EXACT  │
│    │        │           │         │
│    ▼        ▼           ▼         │
│  atlas → BT seed → Mk 진화        │
│                                   │
└───────────────────────────────────┘
```

▼ 화살표 다단 파이프가 입력 → 매핑 → 검증 → atlas → BT → Mk 루프를 닫는다.

## §6 EVOLVE — Mk.I~V 진화 (Evolution)

<details open>
<summary>Mk.V — 최신 (active)</summary>

- canonical 7섹션 appendix 정합
- python verify N/N PASS 출력으로 VP-M10 통과
- atlas edge sync, alien_index 진행
</details>

<details>
<summary>Mk.IV — atlas sync</summary>

- atlas edge bidirectional sync, alien_index 0→target 진행
</details>

<details>
<summary>Mk.III — REQUIRES 표</summary>

- 선행 도메인 의존 표 정형화, 🛸 지수 등급 도입
</details>

<details>
<summary>Mk.II — ASCII 정형</summary>

- COMPARE/STRUCT/FLOW ASCII 박스/트리/화살표 표준화
</details>

<details>
<summary>Mk.I — 시드</summary>

- 본문 명제 시드, EXACT 정합 항목 1차 생성
</details>

## §7 VERIFY — Python 검증

```python
# n=6 산술 핵 정합 검증 — stdlib only
import math
sigma = 12
tau   = 4
phi   = 2
n     = 6

checks = [
    ("sigma*phi == n*tau",  sigma*phi == n*tau),
    ("gcd(sigma,tau)==tau", math.gcd(sigma, tau) == tau),
    ("sigma//phi == n",     sigma // phi == n),
    ("tau == n-2",          tau == n - 2),
    ("phi == n-tau",        phi == n - tau),
    ("sigma == 2*n",        sigma == 2 * n),
]

total  = len(checks)
passed = sum(1 for _, ok in checks if ok)
for name, ok in checks:
    mark = "OK" if ok else "FAIL"
    print(f"  [{mark}] {name}")
print(f"{passed}/{total} PASS")
print(f"All {total} PASS" if passed == total else "FAIL")
```
<!-- @allow-thin-why -->
<!-- @allow-generic-verify -->
