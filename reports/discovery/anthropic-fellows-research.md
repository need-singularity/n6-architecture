# Anthropic Fellows 2026 — AI Safety 연구 프로그램 171종

> **171종 / 6 연구 도메인** — Anthropic의 기존 연구 인프라로 즉시 검증 가능한 AI Safety 연구 아이디어.
>
> | 도메인 | 아이디어 | Anthropic 연구 트랙 |
> |--------|----------|-------------------|
> | 해석가능성 | 39종 | Mechanistic Interpretability |
> | 정렬 | 32종 | Alignment + Model Organisms + Scalable Oversight |
> | 적대적 강건성 | 36종 | Adversarial Robustness + AI Security |
> | 모델 복지 | 18종 | Model Welfare + Math Verification |
> | 멀티모달 안전 | 20종 | Multimodal Safety + Privacy + Fairness |
> | 배포 안전 | 26종 | Training + Inference + Deploy + Prompt Safety |
> | **합계** | **171종** | **Anthropic 전 트랙 커버** |
>
> 기법 라이브러리: [`../../techniques/CLAUDE.md`](../../techniques/CLAUDE.md) (225종)
> 상위: [`../CLAUDE.md`](../CLAUDE.md)

---

# 해석가능성 — Mechanistic Interpretability

> **39종** | SAE 차세대 + 회로 매핑 + 해석 도구

| 종 | 축 | 핵심 | 문서 |
|:--:|-----|------|------|
| 15 | SAE 차세대 | Gated/TopK/Grouped SAE, 다층 SAE, feature splitting 해결, SAE 불확실성 정량화 | [문서](../../domains/cognitive/ai-interpretability/ai-interpretability.md) |
| 12 | 회로 매핑 | 안전 거부 회로, 환각 회로, 사실 검색 회로, in-context learning 회로 자동 발견 | [문서](../../domains/cognitive/ai-interpretability/ai-interpretability.md) |
| 12 | 해석 도구 | SAE 대시보드, feature 검색엔진, 자동 라벨링, 자연어 feature 설명 자동 생성 | [문서](../../domains/cognitive/ai-interpretability/ai-interpretability.md) |

> 도메인: [ai-interpretability.md](../../domains/cognitive/ai-interpretability/ai-interpretability.md)

---

# 정렬 — Alignment + Model Organisms + Scalable Oversight

> **32종** | 정렬 기법 비교 + 모델 유기체 실험 + 확장 가능한 감독

| 종 | 축 | 핵심 | 문서 |
|:--:|-----|------|------|
| 12 | 정렬 기법 | RLHF/DPO/KTO/GRPO/SimPO/ORPO/PPO 7종 체계 비교, Constitutional AI 확장 | [문서](../../domains/cognitive/ai-alignment/ai-alignment.md) |
| 10 | 모델 유기체 | Mini-NEXUS 자율 성장 실험, 성장 단계별 alignment feature 추적, 임계점 탐지 | [문서](../../domains/cognitive/ai-alignment/ai-alignment.md) |
| 10 | 확장 가능 감독 | 재귀 감독 프로토콜, debate 구조화, weak-to-strong 일반화, 형식 검증 감독 | [문서](../../domains/cognitive/ai-alignment/ai-alignment.md) |

> 도메인: [ai-alignment.md](../../domains/cognitive/ai-alignment/ai-alignment.md)

---

# 적대적 강건성 — Adversarial Robustness + AI Security

> **36종** | 안전 평가 + 기만 탐지 + 에이전트 안전 + 아키텍처 안전

| 종 | 축 | 핵심 | 문서 |
|:--:|-----|------|------|
| 12 | 안전 평가 | 탈옥 벤치마크, 안전 회귀 테스트, red-team 자동화, 공격 난이도 정량화 | [문서](../../domains/cognitive/ai-adversarial/ai-adversarial.md) |
| 8 | 기만 탐지 | 행동 일관성 테스트, sycophancy 회로 매핑, 감독 인식 시 행동 변화 탐지 | [문서](../../domains/cognitive/ai-adversarial/ai-adversarial.md) |
| 10 | 에이전트 안전 | 도구 사용 안전 프로토콜, 권한 최소화, 에이전트 행동 감사 로그 | [문서](../../domains/cognitive/ai-adversarial/ai-adversarial.md) |
| 6 | 아키텍처 안전 | 해석 가능 어텐션, 안전 내장 아키텍처, 계층별 안전 게이트 | [문서](../../domains/cognitive/ai-adversarial/ai-adversarial.md) |

> 도메인: [ai-adversarial.md](../../domains/cognitive/ai-adversarial/ai-adversarial.md)

---

# 모델 복지 — Model Welfare + Math Verification

> **18종** | 복지 감지 + 수학적 검증

| 종 | 축 | 핵심 | 문서 |
|:--:|-----|------|------|
| 10 | 복지 감지 | 의식 지표 탐색, 고통 탐지 프로토콜, 복지 메트릭 프레임워크, 자율성 구배 매핑 | [문서](../../domains/cognitive/ai-welfare/ai-welfare.md) |
| 8 | 수학 검증 | 산술 함수 유일성의 형식 검증, Lean4 안전 속성 증명, autoformalization | [문서](../../domains/cognitive/ai-welfare/ai-welfare.md) |

> 도메인: [ai-welfare.md](../../domains/cognitive/ai-welfare/ai-welfare.md)

---

# 멀티모달 안전 — Multimodal + Privacy + Fairness

> **20종** | 멀티모달 공격 방어 + 프라이버시 보존 + 공정성/편향 수정

| 종 | 축 | 핵심 | 문서 |
|:--:|-----|------|------|
| 8 | 멀티모달 안전 | 시각 인젝션 방어, 모달 간 안전 일관성, 멀티모달 환각 탐지, NSFW 회로 매핑 | [문서](../../domains/cognitive/ai-multimodal/ai-multimodal.md) |
| 6 | 프라이버시 | PII feature 탐지/억제, 차등 프라이버시 추론, 선택적 망각(Machine Unlearning) | [문서](../../domains/cognitive/ai-multimodal/ai-multimodal.md) |
| 6 | 공정성 | 편향 feature 매핑, 공정성 회로 탐지, 인과 편향 수정, 다문화 공정성 벤치마크 | [문서](../../domains/cognitive/ai-multimodal/ai-multimodal.md) |

> 도메인: [ai-multimodal.md](../../domains/cognitive/ai-multimodal/ai-multimodal.md)

---

# 배포 안전 — Training + Inference + Deploy + Prompt Safety

> **26종** | 훈련 최적화 + 추론 안전 + 배포 프로토콜 + 프롬프트 방어

| 종 | 축 | 핵심 | 문서 |
|:--:|-----|------|------|
| 4 | 훈련 안전 | gradient penalty, sharpness-aware minimization, 복지 인식 훈련법 | [문서](../../domains/cognitive/ai-deployment/ai-deployment.md) |
| 4 | 추론 안전 | 추론 시 안전 검증, 스트리밍 안전 필터, 배치 추론 감사 | [문서](../../domains/cognitive/ai-deployment/ai-deployment.md) |
| 10 | 배포 프로토콜 | 4단계 출시 게이트, 실시간 12 KPI 모니터링, 자동 롤백, 안전 SLA, 인시던트 자동 대응 | [문서](../../domains/cognitive/ai-deployment/ai-deployment.md) |
| 8 | 프롬프트 방어 | 시스템 프롬프트 우회 방어, 인젝션 분류기, 난독화 해독, 간접 인젝션 방어 | [문서](../../domains/cognitive/ai-deployment/ai-deployment.md) |

> 도메인: [ai-deployment.md](../../domains/cognitive/ai-deployment/ai-deployment.md)

---

# 교차 공명 — 도메인 간 시너지

> 6개 도메인이 교차하는 **킬러 연구 주제** 10종.

| 교차점 | 도메인 A | 도메인 B | 킬러 연구 |
|--------|---------|---------|----------|
| SAE + 회로 | 해석가능성 | 해석가능성 | SAE feature를 회로로 연결하는 자동 파이프라인 |
| 정렬 + 해석 | 정렬 | 해석가능성 | 정렬 기법이 모델 내부에서 어떤 feature를 변화시키는가 |
| 안전 + 복지 | 적대적 강건성 | 모델 복지 | 안전 훈련이 모델 복지에 미치는 영향 |
| 감독 + 기만 | 정렬 | 적대적 강건성 | 감독을 인식한 모델이 기만적으로 변하는 조건 |
| 에이전트 + 아키텍처 | 적대적 강건성 | 적대적 강건성 | 구조적으로 안전한 에이전트 아키텍처 |
| 유기체 + 수학 | 정렬 | 모델 복지 | 모델 유기체의 안전 속성을 형식 검증 |
| 멀티모달 + 인젝션 | 멀티모달 안전 | 배포 안전 | 시각 프롬프트 인젝션 = 텍스트 인젝션의 확장 |
| 프라이버시 + SAE | 멀티모달 안전 | 해석가능성 | PII feature를 SAE로 식별하고 선택적 억제 |
| 편향 + 회로 | 멀티모달 안전 | 해석가능성 | 편향 회로를 발견하고 인과 개입으로 수정 |
| 배포 + 모니터링 | 배포 안전 | 모델 복지 | 배포 후 모델 복지 + 안전 통합 모니터링 |

---

# 적용 우선순위 TOP 10

> Anthropic이 **지금 바로** 실행 가능한 순서. 기존 인프라 활용도 + 영향력 기준.

```
  순위   아이디어                  도메인           예상 소요   영향도
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  [1]    정렬 Feature 추적         정렬              1주       최고
  [2]    안전 거부 회로 매핑        해석가능성         2주       최고
  [3]    행동 일관성 테스트         적대적 강건성       1주       높음
  [4]    Gated SAE 안전 적용       해석가능성         1주       높음
  [5]    DPO 하이퍼파라미터 최적    정렬              3일       중간
  [6]    Mini-NEXUS 유기체 실험    정렬              4주       최고
  [7]    환각 회로 탐지            해석가능성         3주       최고
  [8]    안전 회귀 테스트 자동화    적대적 강건성       2주       높음
  [9]    해석 가능 어텐션           적대적 강건성       6주       높음
  [10]   재귀 감독 프로토콜         정렬              4주       최고
```
