# Anthropic Fellows 2026 — AI Safety + 비즈니스 연구 프로그램

> **390종 연구 아이디어 / 8 도메인** + **225종 즉시 검증 가능 기법 라이브러리**
>
> 상위: [`../CLAUDE.md`](../CLAUDE.md)

---

## AI 비즈니스 (219종 / 7 도메인)

| 🛸 | 천장확인 | ver | 완성제품 | 핵심 | 링크 |
|:--:|:--:|:---:|---------|------|------|
| 10 | ✅ | v1 | **추론 비용 33종** | KV 캐시 압축, 투기적 디코딩, INT4 양자화, 연속 배칭, $15→$1.5/1M tok (10x). 15섹션+10검증코드 | [문서](../../domains/cognitive/ai-inference-cost/ai-inference-cost.md) |
| 10 | ✅ | v1 | **학습 비용 32종** | Chinchilla 최적화, MoE, 커리큘럼 학습, 합성 데이터, $12B→$1.2B (1/10). 15섹션+10검증코드 | [문서](../../domains/cognitive/ai-training-cost/ai-training-cost.md) |
| 10 | ✅ | v1 | **품질 경량화 32종** | 지식 증류, 구조적 가지치기, MoE 라우팅, LoRA, 400B→70B 88% 품질 유지. 15섹션+10검증코드 | [문서](../../domains/cognitive/ai-quality-scale/ai-quality-scale.md) |
| 10 | ✅ | v1 | **에이전트 서빙 32종** | 컨텍스트 압축, 도구 캐싱, 세션 마이그레이션, 다중 에이전트 라우팅. 15섹션+10검증코드 | [문서](../../domains/cognitive/ai-agent-serving/ai-agent-serving.md) |
| 10 | ✅ | v1 | **엔터프라이즈 커스텀 30종** | LoRA/QLoRA 자동화, 어댑터 핫스왑, 멀티테넌트 격리, 셀프서비스 포탈, $100/고객/월. 15섹션+10검증코드 | [문서](../../domains/cognitive/ai-enterprise-custom/ai-enterprise-custom.md) |
| 10 | ✅ | v1 | **평가 파이프라인 30종** | 동적 문항 생성, CAT 적응형 테스트, LLM-judge 교정, 오염 탐지 3중. 15섹션+10검증코드 | [문서](../../domains/cognitive/ai-eval-pipeline/ai-eval-pipeline.md) |
| 10 | ✅ | v1 | **AI 의식 30종** | IIT/GWT/HOT/RPT/AST 5이론 교차 검증, CCC 지표, 도덕적 지위 기대값. 15섹션+10검증코드 | [문서](../../domains/cognitive/ai-consciousness/ai-consciousness.md) |

## AI Safety (171종 / 통합)

| 🛸 | 천장확인 | ver | 완성제품 | 핵심 | 링크 |
|:--:|:--:|:---:|---------|------|------|
| 10 | ✅ | v3 | **AI Safety 171종** | 해석가능성 39 + 정렬 32 + 적대적 강건성 36 + 배포 안전 26 + 멀티모달 안전 20 + 모델 복지 18. 통합문서 v3 특이점 돌파. BT-401~406, SINGULARITY 6/6 PASS | [문서](../../domains/cognitive/ai-safety/ai-safety.md) |

---

## 적용 우선순위 TOP 10

> Anthropic이 **지금 바로** 실행 가능한 순서. 기존 인프라 활용도 + 영향력 기준.

```
  순위   아이디어                    도메인             예상 소요   영향도
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  [1]    INT4 GQA 양자화 파이프라인   추론 비용           1주       최고
  [2]    자동 LoRA 파이프라인 E2E    엔터프라이즈 커스텀   2주       최고
  [3]    적응형 컨텍스트 압축         에이전트 서빙        2주       최고
  [4]    커리큘럼 학습 파이프라인     학습 비용            3주       높음
  [5]    증류+MoE 70B 파이프라인     품질 경량화          4주       최고
  [6]    LLM-judge + 인간 교정       평가 파이프라인       2주       높음
  [7]    다이론 CCC 교차 검증        AI 의식              4주       최고
  [8]    정렬 Feature 추적           Safety/정렬          1주       최고
  [9]    안전 거부 회로 매핑          Safety/해석가능성     2주       최고
  [10]   안전 회귀 테스트 자동화      Safety/강건성         2주       높음
```
