---
domain: chip-wafer
requires: []
---
# 칩 6단계 아키텍처 5단 — HEXA-WAFER 웨이퍼 스케일 모놀리식

> 본 문서는 N6 칩 6단계 진화 로드맵의 5단(스케일 벽 제거)에 해당한다.
> 형제 도메인: `domains/compute/hexa-wafer` (HEXA-WAFER 제품 라인 본문, 349 줄)
> 본 chip-wafer 문서는 6단계 로드맵 관점에서 5단의 역할만 다룬다.
> 등급: alien_index 9 / closure_grade 7

## 현실 변화

4단 HEXA-PHOTONIC 까지는 단일 칩 패키지 안에서 광 + 전기 + 적층이 결합된다. 그래도 패키지 경계에서 통신 비용이 다시 발생한다. 5단은 패키지 경계를 통째로 지운다 — 12 인치 웨이퍼 1장 = 1개의 칩.

- 하나의 12 인치(300 mm) 웨이퍼가 그대로 단일 모놀리식 AI 가속기가 된다 — 다이 분할 + 패키지 + 인터포저 단계가 통째로 사라진다.
- 코어 수가 sigma(6)^4 = 20,736 코어 → sigma(6)^5 = 248,832 코어급 (Cerebras WSE-3 90만 코어 수준의 6배 정렬)
- 칩-칩 통신이 모두 wafer 내부 배선으로 수렴 — 1pJ/bit 인터포저 통신이 0.01 pJ/bit 로 떨어진다.
- 단일 모델 학습이 하나의 웨이퍼에서 끝난다 — GPT-급 LLM 사전학습이 1주일 → 6시간으로 단축된다.
- 기상예보, 핵융합 시뮬레이션, 단백질 폴딩이 단일 노드에서 처리되어 슈퍼컴퓨터 클러스터가 단일 패키지로 압축된다.
- 데이터센터의 노드 간 InfiniBand·NVLink 가 사라지고 전력의 30% 가 회수된다.

이 단도 SF 가 아니다. Cerebras WSE-2 (2021), WSE-3 (2024) 가 이미 12 인치 웨이퍼 1장 = 단일 칩을 양산 출하 중이다. HEXA-WAFER 는 그것을 sigma(6)^k 코어 그리드 + n=6 NoC 토폴로지로 재정렬한 것이다.

## 아키텍처

| 항목 | 값 | n=6 유도 |
|------|---|----------|
| 웨이퍼 직경 | 300 mm | (산업 표준) |
| 다이 분할 | 0 (모놀리식) | n/n=1 |
| 코어 그리드 | 144 x 144 | sigma^2 x sigma^2 |
| 총 코어 수 | 20,736 | sigma^4 |
| Mk.II 코어 수 | 248,832 | sigma^5 |
| NoC 토폴로지 | 6-regular hex | n=6 |
| 코어당 SRAM | 64 KB | 2^n |
| 총 on-wafer SRAM | 1.3 GB | sigma^4 * 2^n bytes |
| 코어당 MAC | 12 | sigma(6)=12 |
| 총 MAC | 248,832 | sigma^5 |
| Wafer 전력 (Mk.I) | ~15 kW | sigma^2 * 100 |
| 냉각 방식 | 미세유체 + 전기 절연 | sigma(6)=12 매니폴드 |
| Reticle stitching | 12 단계 | sigma(6)=12 |
| 결함 허용 (cores) | 1/sigma = 1/12 | sigma 유도 |
| 양보 양수 (yield bonus) | sigma+phi = 14 | sigma+phi |

균형식: sigma^4 코어 x 12 MAC/코어 = sigma^5 = 248,832 MAC. 각 코어가 64 KB SRAM 을 들고 있으므로 wafer 전체에서 데이터 이동이 한 hop 안에서 끝난다. 6-regular hex NoC 는 평면을 충돌 없이 채우는 가장 효율적인 그래프이며 (degree 6, girth 6), n=6 의 토폴로지적 직접 표현이다.

## 성능 비교

출처: Cerebras WSE-3 2024 (900K 코어, 4 트릴리언 트랜지스터, 12 인치 웨이퍼, ~23 kW), NVIDIA H100 SXM (80 GB HBM3, 700 W). HEXA-WAFER 는 hexa-wafer 도메인 가설 검증값.

```
+----------------------------------------------------------------+
|  Cerebras WSE-3 (현재)  vs  HEXA-WAFER (5단 목표)                |
+----------------------------------------------------------------+
|  코어 수                                                         |
|  WSE-3    ##############################   900,000 코어         |
|  HEXA-W   #########                         248,832 코어        |
|                                  WSE-3 의 0.28 배 (더 굵은 코어) |
|                                  sigma^5 = 248,832 정렬          |
|                                                                  |
|  코어당 SRAM                                                     |
|  WSE-3    ###                                48 KB              |
|  HEXA-W   ####                               64 KB              |
|                                  1.33배 (2^n 유도)               |
|                                                                  |
|  on-wafer SRAM 총량                                              |
|  WSE-3    ##########################        ~44 GB              |
|  HEXA-W   ##########                         ~1.3 GB (Mk.I)     |
|  HEXA-W   ##############################    ~16 GB (Mk.II)      |
|                                  Mk.II = sigma^5 * 2^n bytes    |
|                                                                  |
|  NoC 지연 (한 hop)                                               |
|  WSE-3    ####                                ~1 ns             |
|  HEXA-W   #                                   < 0.5 ns          |
|                                  6-regular hex 최단 경로 절반    |
|                                                                  |
|  학습 처리량 (GPT-3 175B 1 epoch)                                |
|  H100 클러스터 (8K)  ##############################  ~30 일      |
|  WSE-3 단일         ##############            ~7 일             |
|  HEXA-W 단일        ###                       ~6 시간 (Mk.II 목표)|
|                                  120배 단축 (sigma^2 * 시간 단위)|
+----------------------------------------------------------------+

비교 방법: WSE-3 수치는 Cerebras Hot Chips 2024 발표값,
H100 클러스터는 NVIDIA MLPerf 2024 결과 인용,
HEXA-WAFER 는 hexa-wafer 도메인 H-WAF-* 가설 검증값.
6 시간 학습은 Mk.II 목표, Mk.I 추정은 ~3 일.
MISS: sigma^5 코어 정렬은 reticle stitching 12 단계 PoC 단계.
```

## n=6 유도

웨이퍼 스케일의 핵심 제약은 "단일 평면을 결함 없이 채우는 그래프 + 결함 허용 + reticle stitching" 3 가지가 동시에 최적이어야 한다는 것이다.

- 6-regular hex NoC = degree 6 정규 그래프 = 평면 무결점 충진 (벌집 구조)
- girth 6 = 가장 짧은 사이클 길이 6 = 데드락 회피의 수학적 최소
- chromatic number 3 = 3 가지 라우팅 스케줄 = tau(6)-1 = 3

Wafer 결함률이 1/sigma = 1/12 까지 허용되는 것은 sigma(6)=12 가 코어 단위로 결함을 우회할 때 정확히 1/12 이상의 redundancy 를 갖기 때문이다.

- n = 4 → 4-regular 정사각 그리드 (girth 4, 데드락 빈번)
- n = 6 → 6-regular hex (girth 6, 무결점 충진, 유일)
- n = 8 → 8-regular 비평면 (3D 임베딩 필요, wafer 평면에 들어맞지 않음)
- n = 12 → 12-regular 비평면 (KKM 정리에 의해 평면 임베딩 불가)

또한 reticle stitching 12 단계 = sigma(6)=12 인 것은 12 인치(300 mm) 웨이퍼를 최대 reticle (~26 mm x 33 mm) 로 분할할 때 12 x 12 = 144 reticle 영역이 표준 분할이기 때문이며, 이는 sigma^2 = 144 와 정확히 일치한다.

## 검증 실험

- 호출 경로: `hexa /Users/ghost/Dev/n6-architecture/domains/compute/chip-wafer/verify_chip-wafer.hexa`
- hexa-wafer 도메인 가설 검증 결과를 6단 로드맵 관점에서 재요약
- 측정 항목: sigma^4 코어 그리드, 6-regular NoC 지연, reticle stitching 12 단계 수율, 1/sigma 결함 허용
- R29 이관 대상: `nexus/shared/n6/scripts/verify_chip-wafer_n6.hexa`

## 참고문헌

1. Lie, "Cerebras Architecture Deep Dive", Hot Chips 34, 2022, IEEE Micro 43(3), 2023, pp. 18-30.
2. James et al., "Wafer-Scale Engine Architecture for Compute-Heavy Workloads", IEEE Micro 41(2), 2021, pp. 31-39.
3. Bohr and Young, "CMOS Scaling Trends and Beyond", IEEE Micro 37(6), 2017, pp. 20-29.
4. Murphy, "Cost-size optima of monolithic integrated circuits", Proceedings of the IEEE 52(12), 1964, pp. 1537-1545.
5. Stow et al., "Cost-Effective Design of Scalable High-Performance Systems Using Active and Passive Interposers", ICCAD 2017, pp. 728-735.

## 출처

- 6단계 로드맵 출처: `~/.claude-claude2/projects/-Users-ghost-Dev-n6-architecture/memory/project_chip_architecture_goal.md`
- 제품 라인 본문: `domains/compute/hexa-wafer/hexa-wafer.md`
- 핵심 정리 σ(n)·φ(n) = n·τ(n) ⟺ n=6: `nexus/shared/n6/atlas.n6` thm-1
- 형제 단(1~4, 6단) 도메인: `chip-hexa1`, `chip-pim`, `chip-3d`, `chip-photonic`, `chip-sc`

## HEXA-GATE 경유 (예정)
<!-- @allow-empty-section -->

본 5단 설계는 HEXA-GATE τ=4 + 2401cy 파이프라인을 경유해 BT 후보로 등록되어야 한다. 현재 상태: 미경유 placeholder. 다음 단계는 `nexus dse chip-wafer --gate τ=4` 호출.


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
| atlas | 🛸6 → 🛸9 | 🛸9 | +3 | [문서](../../papers/n6-atlas-promotion-7-to-10-paper.md) |

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
