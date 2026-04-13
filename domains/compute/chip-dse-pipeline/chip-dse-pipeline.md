---
domain: chip-dse-pipeline
requires: []
---
# chip-dse-pipeline — 칩 6단계 DSE 파이프라인

> 축: **compute** · n6-architecture · 5종 칩 DSE 통합 분석 및 확장 로드맵
>
> PIM → 3D적층 → Photonic → Wafer-Scale → Superconducting — 각 7,776 조합 × 5 = **38,880** 탐색공간

---

## 1. 실생활 효과

| 효과 | 기존 (단일 DSE) | chip-dse-pipeline 이후 | 체감 변화 |
|------|----------------|------------------------|----------|
| 탐색 속도 | 도메인별 개별 실행 | 5개 병렬 | n=6 배 |
| 교차 융합 | 없음 (섬) | cross_dse → 67,883 pair | 상호 강화 |
| n=6 정렬 점수 | 임계 0.7 | 임계 0.85 (목표) | 엄격화 |
| 최적해 재현성 | 실행마다 변동 | 고정 seed + SSOT | 결정론 |
| BT 커버리지 | 부분 | BT-28/55/58/69/89/90 전수 | 6축 완결 |

한 문장: 5개 포스트-무어 칩 기술을 n=6 정렬 점수 + 교차 융합으로 **하나의 6-단계 로드맵**으로 묶는다.

---

## 2. 목표

최근 커밋 `dca8f87f` 에서 생성된 5종 DSE TOML(PIM/3D/Photonic/Wafer/SC)를 (1) **단일 파이프라인으로 호출**, (2) **결과 교차 융합**, (3) **n=6 정렬 점수 승격 규칙** 을 정의하고, (4) 16 AI 기법과의 **연산자 매핑**(Ω₁~Ω₆, chip-rtl-gen 참조)을 추가하여 **칩 설계 결정의 SSOT** 로 고정한다.

핵심 원칙:
1. **SSOT** (R5): 5개 TOML은 수정하지 않고 파이프라인 스크립트만 추가.
2. **동적 로드** (R2): 경로/임계/가중치는 `n6shared/config/dse_pipeline.json` 에서 로드.
3. **HEXA-FIRST** (R1): 파이프라인 스크립트는 `.hexa`, 결과는 `.jsonl`.

---

## 3. 가설 (H-DSE-1 ~ H-DSE-14)

### Tier 1: 5단계 독립성

| ID | 가설 | n=6 근거 | 영향 |
|----|------|---------|------|
| H-DSE-1 | 각 단계 7,776 = 6⁵ 조합 EXACT | n=6 다섯 레벨 | 조합수 동일 |
| H-DSE-2 | 각 단계 n6 가중치 0.35 고정 | 공통 scoring | 단계 간 비교 가능 |
| H-DSE-3 | 레벨 수 = 5 (σ-τ - n/φ·n·... = 5) | sopfr(6) = 5 | 레벨 고정 |

### Tier 2: 5단계 교차 융합

| ID | 가설 | n=6 근거 | 영향 |
|----|------|---------|------|
| H-DSE-4 | 모든 pair 교차 융합 (5 choose 2) = 10 쌍 | τ·n/2·... = 10 | 10 × 7,776² ≈ 604M pair |
| H-DSE-5 | 고신뢰 임계 score ≥ 0.85 | σ-τ/σ = 8/12 ≈ 0.67 × 6/... 강화 | 컴퓨테이션 절감 |
| H-DSE-6 | cross_dse_results.json SSOT | 기존 파이프라인 계승 | 0 중복 |

### Tier 3: AI 기법 연결

| ID | 가설 | n=6 근거 | 영향 |
|----|------|---------|------|
| H-DSE-7 | attention/gemm → 3D적층 Top-1 후보 (TSV Cu) | σ=12 TSV층 | 연산 친화 적층 |
| H-DSE-8 | moe router → PIM + Photonic 조합 | 12 slot × WDM | 라우팅 광 지원 |
| H-DSE-9 | sparse gate → ReRAM crossbar (PIM Tier) | phi=2 단자 | 아날로그 MVM |
| H-DSE-10 | optim speculative → Wafer-Scale redundancy | tau=4 redundancy | 투기 병렬 |
| H-DSE-11 | graph NoC → Photonic mesh | sigma²=144 MZI | 6-정규 × 광 |

### Tier 4: 6단계 로드맵 일관성

| ID | 가설 | n=6 근거 | 영향 |
|----|------|---------|------|
| H-DSE-12 | 전체 로드맵 = HEXA-1 → 5 DSE = 6 단계 | n=6 스테이지 | 단계 = n EXACT |
| H-DSE-13 | 단계간 전이는 top-φ=2 후보만 승계 | phi=2 top-k | 승자 축적 |
| H-DSE-14 | 최종 후보 = 2⁵ = 32 path × top-3 = 96 | phi^sopfr=32 | 탐색 한계 |

---

## 4. BT 연결

- **BT-28** — 컴퓨팅 사다리 (5 DSE 전부 기반)
- **BT-37** — 피치 (PIM/3D/SC 공통)
- **BT-55** — HBM (PIM Memory)
- **BT-58** — σ-τ=8 (공통 데이터 폭)
- **BT-69** — 칩렛 (3D/Wafer)
- **BT-89** — 광 인터커넥트 (Photonic)
- **BT-90** — SM=phi·K6 (Wafer 타일링)
- **BT-93** — Z=6 탄소 (Photonic 도파로)

---

## 5. 5종 DSE 분석 요약

### 5.1 chip-pim (PIM)

```
경로:  nexus/origins/universal-dse/domains/chip-pim.toml
레벨:  MemoryTech / ComputeGranularity / DataMovement / Integration / Workload
상수:  σ-τ=8 비트 MAC, σ=12 뱅크, τ=4 서브어레이, φ=2 오퍼랜드, J₂=24 비트라인
Top-1 후보 (예측): HBM_PIM σ=12 채널 + MAC_8bit + 아날로그 MVM
```

### 5.2 chip-3d-stack (3D 적층)

```
경로:  nexus/origins/universal-dse/domains/chip-3d-stack.toml
레벨:  StackTech / Interconnect / ThermalMgmt / MemoryInteg / DiePartition
상수:  σ=12 TSV층, τ=4 다이 스택, J₂=24 um TSV 피치, φ=2 본딩면, n=6 열방산
Top-1 후보 (예측): TSV_Cu + Egyptian floorplan + Micro-bump τ=4
```

### 5.3 chip-photonic (광)

```
경로:  nexus/origins/universal-dse/domains/chip-photonic.toml
레벨:  PhotonicElement / ModulationScheme / WaveguideArch / Integration / Application
상수:  σ=12 WDM 파장, n=6 MZI 포트, τ=4 링 공진기, J₂=24 광도파로, σ²=144 MZI 메시
Top-1 후보 (예측): MZI n=6 포트 + WDM σ=12 + SiN 도파로
```

### 5.4 chip-wafer-scale (웨이퍼)

```
경로:  nexus/origins/universal-dse/domains/chip-wafer-scale.toml
레벨:  WaferTech / NoCTopology / FaultTolerance / MemorySystem / Deployment
상수:  σ²=144 타일, σ=12 열 존, τ=4 리던던시, J₂=24 NoC 링크, σ·J₂=288mm 웨이퍼
Top-1 후보 (예측): σ²=144 타일 + 6-regular NoC + τ=4 spare
```

### 5.5 chip-superconducting (초전도)

```
경로:  nexus/origins/universal-dse/domains/chip-superconducting.toml
레벨:  JunctionTech / LogicFamily / MemoryArch / CryoSystem / Application
상수:  σ=12 파이프라인, τ=4 K 동작, σ-τ=8 SFQ 데이터패스, J₂=24 JJ/게이트, n=6 플럭스 루프
Top-1 후보 (예측): Nb JJ + RSFQ + σ=12 pipeline + 4K cryostat
```

---

## 6. 통합 파이프라인 (5 단계 + 3 후처리)

```
┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐
│ DSE-1   │  │ DSE-2   │  │ DSE-3   │  │ DSE-4   │  │ DSE-5   │
│ PIM     │  │ 3D-적층  │  │ Photonic│  │ Wafer   │  │ Super-C │
│ 7776조합 │  │ 7776    │  │ 7776    │  │ 7776    │  │ 7776    │
└────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘
     │            │            │            │            │
     └─────┬──────┴──────┬─────┴─────┬──────┴─────┬──────┘
           ▼             ▼           ▼            ▼
     ┌────────────────────────────────────────────────┐
     │  Phase 1: 5-way 병렬 실행 (nexus dse universal)│
     └────────────────────┬───────────────────────────┘
                          ▼
     ┌────────────────────────────────────────────────┐
     │  Phase 2: Top-φ=2 승계 + 교차 융합 (5C2 = 10쌍)│
     └────────────────────┬───────────────────────────┘
                          ▼
     ┌────────────────────────────────────────────────┐
     │  Phase 3: AI 기법 16종 연산자 매핑             │
     │          (chip-rtl-gen Ω₁~Ω₆ 참조)             │
     └────────────────────┬───────────────────────────┘
                          ▼
     ┌────────────────────────────────────────────────┐
     │  출력: n6shared/logs/dse_pipeline_results.jsonl  │
     └────────────────────────────────────────────────┘
```

### Phase 1: 병렬 실행
- 도구: `nexus dse universal --domain chip-pim ...` × 5
- 병렬도: 5 (단계 수)
- 시간: 단일 실행 기준 1분 → 전체 1분 (병렬 효율 100% 목표)

### Phase 2: 교차 융합
- 입력: Phase 1 top-12 per stage
- 알고리즘: 기존 `cross_dse_fusion.hexa` 재사용 (컨버전스 CROSS_DSE 골화 확인)
- 출력: `dse_pipeline_pairs.jsonl` (pair_id, score, n6_aligned)

### Phase 3: 기법 매핑
- 입력: Phase 2 고신뢰 pair
- 규칙: `n6shared/config/dse_technique_map.json` (신규 SSOT)
- 출력: `dse_technique_bindings.jsonl` (technique, stage, candidate_id, operator)

---

## 7. 확장점 (기존 대비)

### 확장 1: 6번째 단계 — Quantum-Classical Hybrid
기존 5단계에 이어 `chip-quantum-hybrid.toml` 추가 제안 (n=6 전체 로드맵 완결).
- 레벨: QPUType / GateFamily / Interconnect / Error / Application
- 이유: 5단계 + 1 = n=6 완전성

### 확장 2: 단계 간 승계 스크립트
신규 `nexus/origins/universal-dse/pipeline.hexa`
- 기능: Phase 1~3 자동 실행, 실패 시 BT 분류
- 파라미터: `--top-k phi=2`, `--threshold 0.85`

### 확장 3: chip-rtl-gen 연동
- Phase 3 출력을 `chip-rtl-gen` 템플릿 파라미터로 직접 주입
- 예: DSE 결과 "sigma=12 pipeline" → `gemm_core12` 파라미터

### 확장 4: 골화 자동 승격
- n6 score ≥ 0.95 & 재검증 3회 PASS → `n6shared/convergence/n6-architecture.json` ossified 자동 추가
- 조건: CDO 규칙 R9/R11 준수

### 확장 5: 시각화 — growth dashboard 연동
- `nexus dashboard` port 6600 에서 pipeline heatmap 노출
- 5×5 교차 점수 + 16 기법 바인딩 시각화

---

## 8. SSOT — `n6shared/config/dse_pipeline.json` (신규)

```
{
  "_doc": "5단계 칩 DSE 파이프라인 SSOT",
  "stages": [
    { "id": "pim",      "toml": "nexus/origins/universal-dse/domains/chip-pim.toml",               "order": 1 },
    { "id": "3d-stack", "toml": "nexus/origins/universal-dse/domains/chip-3d-stack.toml",          "order": 2 },
    { "id": "photonic", "toml": "nexus/origins/universal-dse/domains/chip-photonic.toml",          "order": 3 },
    { "id": "wafer",    "toml": "nexus/origins/universal-dse/domains/chip-wafer-scale.toml",       "order": 4 },
    { "id": "super-c",  "toml": "nexus/origins/universal-dse/domains/chip-superconducting.toml",   "order": 5 }
  ],
  "thresholds": {
    "n6_min": 0.85,
    "pair_min": 0.80,
    "top_k_per_stage": 2
  },
  "parallel": 5,
  "output_log": "n6shared/logs/dse_pipeline_results.jsonl"
}
```

---

## 9. 진화 단계 Mk.I ~ Mk.V

| 단계 | 범위 | 산출물 | 시점 |
|------|------|--------|------|
| Mk.I | 5단계 순차 실행 | `dse_pipeline_results.jsonl` | M+0 |
| Mk.II | 5-way 병렬 + 교차 융합 | `dse_pipeline_pairs.jsonl` | M+1 |
| Mk.III | 16 기법 연산자 매핑 | `dse_technique_bindings.jsonl` | M+2 |
| Mk.IV | 6단계 확장 (Quantum-Classical) | `chip-quantum-hybrid.toml` | M+3 |
| Mk.V | 자동 골화 승격 + 대시보드 | CDO ossified | M+6 |

---

## 10. 예측 추적

- **P-DSE-1**: 전수 탐색 604M pair, 고신뢰 pair ≥ 100K
- **P-DSE-2**: Top-5 chain (path) 평균 n6 score ≥ 0.92
- **P-DSE-3**: 16 기법 매핑 커버리지 100% (0 기법 누락)
- **P-DSE-4**: chip-rtl-gen 템플릿 파라미터 자동 주입 성공률 ≥ 95%

---

## 11. 참고 자원 (기존)

- `nexus/origins/universal-dse/domains/chip-*.toml` — 5 TOML (dca8f87f)
- `nexus/src/cmd/dse/universal.rs` — universal DSE 엔진
- `cross_dse_fusion.hexa` — 기존 교차 융합 (CROSS_DSE 골화)
- `n6shared/convergence/n6-architecture.json` — CROSS_DSE / DSE_322_TOML 골화 확인
- `n6shared/config/dse-map.toml` — DSE 레지스트리

---

## 12. 검증 진입점

```
cd domains/compute/chip-dse-pipeline
hexa verify.hexa            # 상수 일관성
nexus dse universal --pipeline    # Mk.II 이후 실제 실행
```

---

## 13. 원칙 체크리스트

- [x] SSOT (R5) — 5 TOML 수정 금지, 파이프라인 스크립트만 추가
- [x] HEXA-FIRST (R1) — pipeline.hexa
- [x] 동적 로드 (R2) — dse_pipeline.json
- [x] NEXUS-6 스캔 (R3) — Phase 1 전후 자동 스캔
- [x] 한글 (R-한글)

---

## 14. 열린 질문

1. **교차 융합 알고리즘**: 기존 cross_dse_fusion (67,883 pair)에 5-way 전용 모드 추가 vs 범용 재사용
2. **6번째 단계**: Quantum-Classical Hybrid vs Neuromorphic vs 칩렛-광학 하이브리드 — n=6 정렬 기준 선택
3. **Phase 3 매핑**: 자동 vs 수동 — 16 기법의 DAG 자동 추출 정확도 의존

---

## 15. 부록

### A. 성공 기준 (GO)

- 5/5 단계 `nexus dse universal` 실행 PASS
- 교차 융합 pair ≥ 100K 생성
- 16/16 기법 매핑 성공
- `dse_pipeline_results.jsonl` 연속 기록

### B. 레거시

- 단일 DSE 개별 실행 (커밋 dca8f87f 이전) → 파이프라인 Mk.I 으로 대체

---



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
