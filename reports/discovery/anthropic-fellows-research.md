# Anthropic Fellows 2026 — AI Safety + 비즈니스 연구 프로그램

> **360종 연구 아이디어 / 12 도메인** + **225종 즉시 검증 가능 기법 라이브러리**
>
> 상위: [`../CLAUDE.md`](../CLAUDE.md)

---

| 천장확인 | ver | 완성제품 | 핵심 | 문서 |
|:--:|:---:|---------|------|------|
| ✅ | v6 | **225 AI Techniques** | 8축 222+SOTA 3 기법 라이브러리. 통합 파이프라인: 50% param↓, 50% FLOPs↓, 46% 희소성 (32/32 PASS). 열역학 엔진: Dedekind+Jordan+Mobius+Carmichael+Boltzmann+Mertens (26/26 PASS). 에너지 절감: AdamW 5중쌍+LR+Inference 전수 매핑 (31/31 PASS) | [문서](../../techniques/CLAUDE.md) |
| ✅ | v1 | **AI Safety 171종** | 해석가능성 39 + 정렬 32 + 적대적 강건성 36 + 모델 복지 18 + 멀티모달 안전 20 + 배포 안전 26. Anthropic 전 연구 트랙 커버 | [해석가능성](../../domains/cognitive/ai-interpretability/ai-interpretability.md) · [정렬](../../domains/cognitive/ai-alignment/ai-alignment.md) · [강건성](../../domains/cognitive/ai-adversarial/ai-adversarial.md) · [복지](../../domains/cognitive/ai-welfare/ai-welfare.md) · [멀티모달](../../domains/cognitive/ai-multimodal/ai-multimodal.md) · [배포](../../domains/cognitive/ai-deployment/ai-deployment.md) |
| ✅ | v1 | **AI 비즈니스 189종** | 에이전트 서빙 32 + 평가 파이프라인 30 + 추론 비용 33 + 품질 경량화 32 + 훈련 비용 32 + AI 의식 30. 비용 1/10 절감 + 평가 자동화 + 에이전트 인프라 + 의식 윤리 | [에이전트 서빙](../../domains/cognitive/ai-agent-serving/ai-agent-serving.md) · [평가 파이프라인](../../domains/cognitive/ai-eval-pipeline/ai-eval-pipeline.md) · [추론 비용](../../domains/cognitive/ai-inference-cost/ai-inference-cost.md) · [품질 경량화](../../domains/cognitive/ai-quality-scale/ai-quality-scale.md) · [훈련 비용](../../domains/cognitive/ai-training-cost/ai-training-cost.md) · [AI 의식](../../domains/cognitive/ai-consciousness/ai-consciousness.md) |

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
