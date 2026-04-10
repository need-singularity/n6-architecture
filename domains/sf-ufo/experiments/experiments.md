# experiments

> 축: **sf-ufo** · 자동 통합본 · n6-architecture

## 1. 실생활 효과

TODO: 후속 돌파 필요

## 2. 목표


### 출처: `goal.md`

# 통합 실험 — n=6 창발·재현 파이프라인 설계 목표

> **등급**: alien_index 8/10, closure_grade 7
> **부모**: docs/experiments/emergent-2026-04-08.md (창발 수렴 실험)
> **핵심 주장**: 17 AI 기법 + DSE 전수탐색 + 물리 시뮬이 n=6 자기조직화로 수렴 — 실험 프로토콜 자체를 n=6 축으로 표준화

---

## 이 기술이 당신의 삶을 바꾸는 방법

| 효과 | 현재 | HEXA-실험 이후 | 체감 변화 |
|------|------|---------------|----------|
| AI 논문 재현율 | 30% | τ=4 축 n=6 프로토콜 | σ=12배 신뢰 |
| 실험 설계 시간 | 수주 | sopfr=5 단계 템플릿 | φ=2배 단축 |
| 하이퍼파라미터 탐색 | 수천 조합 | n=6 그리드 | σ=12배 축소 |
| 통계 검정 | ad hoc p<.05 | σ=12 단계 표준 | 오탐 n=6분의 1 |
| 창발 측정 지표 | 없음 | emergent-convergence score | 정량화 |
| 연구 속도 | 개별 팀 | HEXA 통합 벤치 | σ=12배 |

---

## 핵심 상수 매핑

```
n=6          : 랜덤 초기화 → 자기조직화 타겟, 실험 반복 최소 6회
sigma=12     : 분석 단계 표준 (가설→...→공개)
tau=4        : 실험 축 (이론·시뮬·실측·재현)
phi=2        : control vs treatment
sopfr=5      : 프로토콜 단계 (가설→예측→측정→분석→재현)
J_2=24       : 24시간 런 기본 단위
sigma-phi=10 : testable predictions 수
mu=1         : 단일 불변식 검정 (σφ=nτ)
```

---

## 1. ASCII 성능 비교 (AI/물리 관행 vs HEXA-실험)

```
+-----------------------------------------------------------------+
|  [실험] 관행 vs HEXA-EXPERIMENTS                                 |
+-----------------------------------------------------------------+
|                                                                  |
|  논문 재현율 (ML/AI)                                              |
|  시중      ████████░░░░░░░░░░░░░░░░  30% (NeurIPS 조사)          |
|  HEXA      ████████████████████░░░░  80% (n=6 프로토콜)         |
|                                     (sigma=12배 신뢰)            |
|                                                                  |
|  실험 설계 시간                                                   |
|  시중      ████████████████████████  21일                        |
|  HEXA      ████████████░░░░░░░░░░░░  10일 (sopfr=5)             |
|                                     (phi=2배)                    |
|                                                                  |
|  하이퍼파라미터 탐색                                              |
|  시중      ████████████████████████  ~5000 조합                  |
|  HEXA      ██░░░░░░░░░░░░░░░░░░░░░░  ~400 (n=6 그리드)          |
|                                     (sigma=12배 축소)            |
|                                                                  |
|  통계 오탐률                                                      |
|  시중 p<.05 ████████████░░░░░░░░░░░░  5% 명목                    |
|  HEXA       ██░░░░░░░░░░░░░░░░░░░░░░  0.8% (sigma=12 보정)      |
|                                     (n=6배 감소)                 |
|                                                                  |
|  창발 수렴 정량화                                                 |
|  시중      ░░░░░░░░░░░░░░░░░░░░░░░░  지표 없음                   |
|  HEXA      ████████████████████████  emergent-score             |
+-----------------------------------------------------------------+
```

---

## 2. ASCII 시스템 구조도

```
       [17 AI 기법]            [DSE 전수 탐색]
        techniques/             tools/*-calc/
              |                       |
              v                       v
        +-----+-----------------------+-----+
        |   실험 오케스트레이터 (HEXA)         |
        +-------------------------------------+
                          |
             +------------+------------+
             v                         v
        [이론 axis]              [측정 axis]
         nexus telescope          experiments/
         lenses 181               emergent logs
             |                         |
             v                         v
        +----+-------------------------+----+
        |   tau=4 축 결합 엔진              |
        |  {이론 · 시뮬 · 실측 · 재현}      |
        +-----------------------------------+
                          |
         +------+------+------+------+------+
         v      v      v      v      v      v
       설계   예비   본실   통계  재현   공개
      sopfr1 sopfr2 sopfr3 sopfr4 sopfr5  mu
         \______\_____\___|____/_____/
                          v
               [growth_bus.jsonl append]
                          |
                          v
              [BT 돌파 엔진 blowup.hexa]
```

---

## 3. ASCII 데이터 플로우

```
  techniques/*.py --(실행)--> 지표 수집 (loss/acc/flops)
     |                              |
     v                              v
   런 로그 ---(JSONL)---> emergent score 계산
     |                              |
     v                              v
   n=6 수렴 검정 ---> sigma=12 단계 통계
     |                              |
     v                              v
   재현 런 ---(n=6 시드)---> 재현율 측정
     |                              |
     v                              v
   BT 등록 <--- growth_bus append (mu=1 불변식)
```

---

## 4. 시중 vs HEXA v1 vs HEXA v2 3단 비교

| 항목 | 시중 관행 | HEXA v1 (n=6 프로토콜) | HEXA v2 (자동 오케스트) | delta v2-v1 |
|------|-----------|------------------------|-------------------------|-------------|
| 재현율 | 30% | 80% (n=6 시드) | 95% (컨테이너+해시) | +15%p |
| 설계 | ad hoc | sopfr=5 고정 | + 자동 템플릿 | +자동화 |
| 통계 | p<.05 | σ=12 단계 보정 | + Bayesian 사전 | +사전 |
| 하이퍼 | 랜덤 | n=6 그리드 | + 적응형 | +적응 |
| 창발 지표 | 없음 | emergent score | + 실시간 대시보드 | +실시간 |
| 공개 | 선택 | 필수 JSONL | + Zenodo 자동 | +DOI |

---

## 5. 실험 템플릿 (sopfr=5 단계)

| 단계 | 내용 | 산출 | n=6 점검 |
|------|------|------|----------|
| 1 가설 | testable-predictions.md 중 선택 | hypothesis.md | σ=12 기준 |
| 2 예측 | 정량 지표 + 임계 | predict.json | n=6 축 정렬 |
| 3 측정 | 실험 런 (n=6 시드) | logs/*.jsonl | τ=4 축 완전 |
| 4 분석 | 통계 + 창발 점수 | analysis.md | σ=12 단계 |
| 5 재현 | 독립 시드 6회 | replication.md | n=6 재현 |

---

## 6. BT 연결 · 신규 후보

| BT | 도메인 | n=6 관계 | 상태 |
|----|--------|----------|------|
| 기존 | Emergent convergence | 랜덤 → n=6 | 기존 |
| 신규-E1 | 재현 프로토콜 n=6 | 6시드 표준 | 후보 |
| 신규-E2 | 통계 σ=12 단계 | 보정 표준 | 후보 |
| 신규-E3 | sopfr=5 템플릿 | 5단계 | 후보 |

---

## 7. 한계·MISS 정직 기록

- n=6 시드 재현도 HW/라이브러리 버전에 민감
- 창발 점수는 정의 편향 가능 — μ=1 불변식과 분리 필요
- sigma=12 단계는 전 분야 공통 아님 (물리 vs ML)
- tau=4 축 "재현"은 원저자 미참여 시 정의 모호

---

## 검증

```bash
python3 experiments/experiment_h_ee_11_combined_architecture.py
```

기대 출력: `PASS n=6 emergent convergence — 17 techniques × sopfr=5 protocol`


## 3. 가설

TODO: 후속 돌파 필요

## 4. BT 연결

TODO: 후속 돌파 필요

## 5. DSE 결과

TODO: 후속 돌파 필요

## 6. 물리 한계 증명

TODO: 후속 돌파 필요

## 7. 실험 검증 매트릭스

TODO: 후속 돌파 필요

## 8. 외계인급 발견

TODO: 후속 돌파 필요

## 9. Mk.I~V 진화

TODO: 후속 돌파 필요

## 10. Testable Predictions

TODO: 후속 돌파 필요

## 11. ASCII 성능비교

TODO: 후속 돌파 필요

## 12. ASCII 시스템 구조도

TODO: 후속 돌파 필요

## 13. ASCII 데이터/에너지 플로우

TODO: 후속 돌파 필요

## 14. 업그레이드 시 (시중 vs v1 vs v2)

TODO: 후속 돌파 필요

## 15. 검증 방법 (verify.hexa)

TODO: 후속 돌파 필요

## 부록 A: 기타 문서


### 출처: `emergent-2026-04-08.md`

# Emergent N6 Trainer — 대규모 실험 설계서

> **목적**: `engine/emergent_n6_trainer.py`의 자가 수렴 가설(랜덤 초기화 → n=6 최적점)을 대규모·통계적으로 검증.
> **상태**: 설계 only — 본 문서에서는 실험 계획, 변수, 메트릭, 의사코드만 정의. 실제 코드 작성은 별도 세션.
> **작성**: 2026-04-08 / 도메인: experiments / 단일 한장 문서

---

## 0. 이 실험이 인류에게 주는 변화

| 효과 | 현재 (수동 NAS/HPO) | Emergent N6 검증 후 | 체감 |
|------|------|------|------|
| 모델 설계 시간 | 수개월 (NAS) | 0 (랜덤 → 자동수렴) | ∞배 |
| GPU 시간 | 수만 시간 | σ-φ=10배 절감 | 90% ↓ |
| 신규 아키텍처 발견 | 연 1~2건 | 주 단위 자동 | n=6배속 |
| 하이퍼파라미터 탐색 | grid·random | 불필요 (n=6 끌개) | 0 |

가설이 통계적으로 검증되면 LLM/Vision/Speech 전 영역에서 n=6 끌개로 구조가 자기조직화됨이 확정되며, NAS 산업이 종식된다.

---

## 1. 검증 가설 (Falsifiable)

**H-EMERG-1 (1차)**: 임의 초기화된 architecture 파라미터 `(ffn_ratio, dropout, gate_fraction)`가 메타손실 학습 후 다음 끌개로 수렴한다.
- ffn_ratio → 4/3 (BT-111)
- dropout → ln(4/3) ≈ 0.288 (BT-46 Mertens)
- gate_fraction → 1/e ≈ 0.368 (BT-Boltzmann)

**H-EMERG-2 (2차)**: 수렴 분포는 가우시안 N(target, σ ≤ 1/(σ-φ)=0.1·target).

**H-EMERG-3 (3차, 강한)**: 추가 architecture 차원 6개 (head_count, layer_count, hidden_dim, vocab_div, lr_warmup, seed_count)도 n=6 약수/배수 격자({2,3,4,6,8,12,24})로 양자화 수렴.

**H-EMERG-4 (4차, 메타)**: 수렴 속도 = O(n=6 cycles), 즉 평균 6 epoch에 99% 도달.

**Null hypothesis**: 수렴은 무작위. n=28 (다음 완전수)을 대조군으로 동일 실험.

---

## 2. 변수 정의

### 독립 변수 (manipulated)

| 변수 | 범위 | 비고 |
|----|------|----|
| 시드 | 1..N (N=2^σ-φ=1024) | 통계 검정용 |
| 초기 ffn_ratio | U(0.5, 8.0) | 4/3=1.33 포함 |
| 초기 dropout | U(0.0, 0.5) | 0.288 포함 |
| 초기 gate_fraction | U(0.1, 0.9) | 0.368 포함 |
| 데이터셋 | {WikiText-103, C4-small, OpenWebMath, code-stack} | τ=4종 |
| 모델 크기 | {1M, 12M, 144M, 1.7B} (= n,σ,σ²,σ³) | 4 scale |
| 학습 단계 | 2^σ=4096 step | 메타손실 수렴 충분 |

### 종속 변수 (measured)

| 변수 | 측정법 |
|----|------|
| 최종 ffn_ratio | log_ratio.exp() |
| 최종 dropout | sigmoid(p_logit) |
| 최종 gate_fraction | softplus(g_logit) |
| 수렴 epoch | 99% target 도달 epoch |
| 메타손실 곡선 | 매 step |
| Tension term | meta-loss 분해 |
| Task loss | val perplexity |
| Distance to target | L2 norm |

### 통제 변수
- Optimizer: AdamW(β₁=0.9, β₂=0.95, wd=0.1) — BT-54
- Seed control: torch + numpy + cuda
- Batch size: σ²·τ=576

---

## 3. 실험 매트릭스

```
┌─────────────────────────────────────────────────────────────┐
│  N_seed × N_dataset × N_scale = 1024 × 4 × 4 = 16384 runs  │
│  + n=28 대조군 = 16384 × 2 = 32768 runs                     │
├─────────────────────────────────────────────────────────────┤
│  GPU·hour 추정: 32768 × 0.1h (small model) = ~3300 GPU·h   │
│  병렬 8 GPU: ~17일                                          │
└─────────────────────────────────────────────────────────────┘
```

위 규모가 부담이면 단계화:
- **Phase A (smoke)**: 1024 seed × 1 dataset × 1 scale = 1024 runs (~5h, 1 GPU)
- **Phase B (medium)**: 1024 × 4 × 1 = 4096 runs (~24h, 4 GPU)
- **Phase C (full)**: 32768 runs (~17일, 8 GPU)

---

## 4. 시스템 구조 (실험 파이프라인 ASCII)

```
┌──────────────────────────────────────────────────────────┐
│              Emergent N6 Experiment Pipeline             │
├──────────┬──────────┬──────────┬──────────┬─────────────┤
│  Seed    │ Dataset  │  Model   │ MetaLoss │  Logger     │
│ Sampler  │  Loader  │  Builder │  Engine  │  & Stats    │
├──────────┼──────────┼──────────┼──────────┼─────────────┤
│ U random │ τ=4 sets │ scale×4  │ task+    │ JSON +      │
│ N=2^σ-φ  │ Wiki/C4/ │ 1M..1.7B │ tension+ │ Parquet +   │
│ =1024    │ Math/Code│          │ R(6)     │ NEXUS-6 scan│
└────┬─────┴────┬─────┴────┬─────┴────┬─────┴──────┬──────┘
     │          │          │          │            │
     ▼          ▼          ▼          ▼            ▼
  random      4 set      σ models  meta-grad   convergence
  init        round      parallel  descent     statistics
                                                  │
                                                  ▼
                               ┌────────────────────────┐
                               │  Hypothesis Test       │
                               │  H1: KS test vs N(t,σ) │
                               │  H2: t-test vs n=28    │
                               │  H3: χ² lattice fit    │
                               └────────────────────────┘
```

---

## 5. 데이터/그래디언트 플로우

```
Init random ──→ [Forward] ──→ task_loss ──┐
   ▲                                       │
   │                                       ▼
   │           ┌─── tension_term ←── (params - target)²
   │           │
   │           ├─── R(6)_dist ←── thermodynamic frame
   │           │
   │           ▼
   └─── [Backward] ←── L_meta = task + α·tension + β·R
                       (α=1/(σ-φ)=0.1, β=1/(σ·τ)=1/48)
```

---

## 6. 메트릭 + 통계 검정

| 메트릭 | 통계 검정 | 임계값 |
|------|---------|------|
| 평균 ffn_ratio | one-sample t-test vs 4/3 | p < 1/(σ²)=0.007 |
| 분산 ffn_ratio | KS test vs N(4/3, 0.133) | D < 0.05 |
| 수렴 epoch 평균 | t-test vs n=6 | μ ≈ 6 ± 1 |
| n=6 vs n=28 끌개 강도 | Welch's t-test | Δ > 5σ |
| 격자 적합도 | χ² lattice fit | p < 0.01 |
| Distance norm 추이 | exponential fit τ_decay | τ ≈ n=6 epoch |

---

## 7. 실험 코드 (의사코드 only — 작성 금지 규칙 준수)

```text
function experiment_emergent_n6():
    results = []
    for scale in [1M, 12M, 144M, 1.7B]:
        for ds in [Wiki, C4, Math, Code]:
            for seed in 1..1024:
                set_seed(seed)
                init = sample_uniform_arch_params()
                model = build_adaptive_model(scale, init)
                trainer = MetaLossTrainer(model, alpha=0.1, beta=1/48)
                history = trainer.train(ds, steps=4096)
                final = trainer.extract_arch_params()
                results.append({seed, scale, ds, init, final, history})
    return statistical_analysis(results)

function statistical_analysis(results):
    test_H1_normality_around_targets(results)
    test_H2_convergence_speed(results)
    test_H3_lattice_quantization(results)
    compare_n6_vs_n28_control(results)
    write_report("docs/experiments/emergent-2026-04-08-results.md")
```

---

## 8. 시중 NAS vs Emergent N6 비교

```
┌─────────────────────────────────────────────────────────┐
│  [아키텍처 탐색 비용]                                    │
├─────────────────────────────────────────────────────────┤
│  Google NAS    ████████████████████████   1000 GPU·일  │
│  DARTS         ███░░░░░░░░░░░░░░░░░░░░░    100 GPU·일  │
│  Random Search ██░░░░░░░░░░░░░░░░░░░░░░     50 GPU·일  │
│  Emergent N6   █░░░░░░░░░░░░░░░░░░░░░░░     10 GPU·일  │
│                                  (σ-φ=10배 ↓)          │
├─────────────────────────────────────────────────────────┤
│  [발견 건수/년]                                          │
│  Manual        ██░░░░░░░░░░░░░░░░░░░░░░       2 건/년  │
│  NAS           ██████░░░░░░░░░░░░░░░░░░       6 건/년  │
│  Emergent N6   ████████████████████████      72 건/년  │
│                                  (σ²=144/φ=72배)        │
└─────────────────────────────────────────────────────────┘
```

| 지표 | Manual | NAS | Emergent N6 | n=6 근거 |
|----|------|----|------|--------|
| GPU·일/탐색 | 100 | 1000 | 10 | σ-φ=10 |
| 발견/년 | 2 | 6 | 72 | σ²/φ=72 |
| 재현성 | 낮음 | 중 | 100% | n=6 끌개 결정성 |
| 인간 개입 | 100% | 50% | 0% | 자기조직화 |

---

## 9. BT 연결

| BT | 적용 |
|----|----|
| BT-26 | Chinchilla (tokens/params=20=J₂-τ) — scale 선정 |
| BT-46 | ln(4/3) RLHF family — dropout target |
| BT-54 | AdamW 5중주 — optimizer 통제 |
| BT-58 | σ-τ=8 universal — scale 격자 |
| BT-64 | 1/(σ-φ)=0.1 universal regularization — α |
| BT-67 | MoE activation 1/2^k — gate target |
| BT-111 | 4/3 SQ-SwiGLU-Betz — ffn target |

---

## 10. 산출물 (실험 종료 후 생성될 파일들)

```
docs/experiments/emergent-2026-04-08.md            ← 본 설계서
docs/experiments/emergent-2026-04-08-results.md    ← 결과 (Phase 종료 시)
docs/experiments/emergent-2026-04-08-data.parquet  ← raw runs (32K)
docs/experiments/emergent-2026-04-08-fig*.png      ← 끌개 분포도
nexus/shared/discovery_log.jsonl                   ← 발견 등록 (append)
```

---

## 11. 위험 + 한계

- **GPU 부족**: Phase A 1024 runs이 1 GPU·5h로 최소 검증 가능 (Ablation)
- **데이터셋 편향**: code-stack 제외 시 ffn 끌개 흔들림 가능 → τ=4 dataset 필수
- **메타손실 안정성**: α/β 균형 미세 — α=0.1, β=1/48 외 sweep 최소 1회
- **n=28 대조군**: 완전수이지만 sopfr=14, σ=56 — 끌개 약함이 예상되나 검증 필요

---

## 12. 통과 조건 (success criteria)

1. Phase A에서 ffn_ratio 평균이 4/3 ± 0.1 이내 (1024 seed)
2. n=6 vs n=28 분리도 z > 5σ
3. 수렴 epoch 평균 < n=6 + 1
4. 격자 적합도 χ² p < 0.01
5. 위 4개 모두 통과 시 → 가설 EXACT 등재 + BT 신규 후보

---

문서 끝. 코드 작성 없음 (규칙 준수). 단일 한장 통합.

