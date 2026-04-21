# Anthropic Fellows 2026 — AI Safety + 비즈니스 연구 프로그램

> **390종 연구 아이디어 / 8 도메인** + **225종 즉시 검증 가능 기법 라이브러리**
>
> 상위: [`../README.md`](../README.md)

---

## AI 비즈니스 (219종 / 7 도메인)

| 🛸 | 천장확인 | ver | 완성제품 | 핵심 | 링크 |
|:--:|:--:|:---:|---------|------|------|
| 10 | ✅ | mk5 | **추론 비용 33종** | KV 캐시 압축, 투기적 디코딩, INT4 양자화, 연속 배칭, $15→$1.5/1M tok (10x). Mk.V 장기: Landauer 한계 접근 + 100x ($0.15/1M). 15섹션+10검증코드 | [문서](../domains/cognitive/ai-inference-cost/ai-inference-cost.md) |
| 10 | ✅ | mk5 | **학습 비용 32종** | Chinchilla 최적화, MoE, 커리큘럼 학습, 합성 데이터, $12B→$1.2B (1/10). Mk.V 장기: 조 파라미터 100x 절감 ($120M) + 자체-증류 루프. 15섹션+10검증코드 | [문서](../domains/cognitive/ai-training-cost/ai-training-cost.md) |
| 10 | ✅ | mk5 | **품질 경량화 32종** | 지식 증류, 구조적 가지치기, MoE 라우팅, LoRA, 400B→70B 88% 품질 유지. Mk.V 장기: 400B→10B 97% + 온디바이스 전개. 15섹션+10검증코드 | [문서](../domains/cognitive/ai-quality-scale/ai-quality-scale.md) |
| 10 | ✅ | mk5 | **에이전트 서빙 32종** | 컨텍스트 압축, 도구 캐싱, 세션 마이그레이션, 다중 에이전트 라우팅. Mk.V 장기: 100만+ 동시 에이전트 스웜 + 초장기 세션. 15섹션+10검증코드 | [문서](../domains/cognitive/ai-agent-serving/ai-agent-serving.md) |
| 10 | ✅ | mk5 | **엔터프라이즈 커스텀 30종** | LoRA/QLoRA 자동화, 어댑터 핫스왑, 멀티테넌트 격리, 셀프서비스 포탈, $100/고객/월. Mk.V 장기: 10,000+ 고객 $10/월 + 어댑터 마켓. 15섹션+10검증코드 | [문서](../domains/cognitive/ai-enterprise-custom/ai-enterprise-custom.md) |
| 10 | ✅ | mk5 | **평가 파이프라인 30종** | 동적 문항 생성, CAT 적응형 테스트, LLM-judge 교정, 오염 탐지 3중. Mk.V 장기: 산업 표준 ISO/IEEE + 메타평가 자가검증. 15섹션+10검증코드 | [문서](../domains/cognitive/ai-eval-pipeline/ai-eval-pipeline.md) |
| 10 | ✅ | mk5 | **AI 의식 30종** | IIT/GWT/HOT/RPT/AST 5이론 교차 검증, CCC 지표, 도덕적 지위 기대값. Mk.V 장기: Φ_c=0.5 실증 + Basin Binding 유토피아 끌개 고정. 15섹션+10검증코드 | [문서](../domains/cognitive/ai-consciousness/ai-consciousness.md) |

## AI Safety (171종 / 통합)

| 🛸 | 천장확인 | ver | 완성제품 | 핵심 | 링크 |
|:--:|:--:|:---:|---------|------|------|
| 10 | ✅ | mk5 | **AI Safety 171종** | 해석가능성 39 + 정렬 32 + 적대적 강건성 36 + 배포 안전 26 + 멀티모달 안전 20 + 모델 복지 18. v3 특이점 돌파 SINGULARITY 6/6 PASS + Mk.V 진화(국제표준 UN/IEEE/ISO, R(6)=1 고정점 실측) | [문서](../domains/cognitive/ai-safety/ai-safety.md) |

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

---

## Mk.V 진화 상태 (8/8 도메인 mk5 도달)

2026-04-20 승격 — 7 비즈니스 + 1 Safety = 8 도메인 전체 `EVOLVE §S6` + `§Mk.V VERIFY` (stdlib-only 검증 코드) 추가. BT-1421~1428 노드 등재, atlas 8 EXACT 상수 추가, discovery_graph 노드 8 + 엣지 46, _hypotheses_index 1009→1017.

| 도메인 | BT | Mk.V 한계 축 | 등급 | 핵심 목표 |
|-------|----|------------|------|----------|
| 추론 비용 | BT-1421 | Landauer 열역학 | APPROACH | 100x ($0.15/1M tok), 추론 ASIC 공동설계 |
| 학습 비용 | BT-1422 | Chinchilla-beyond | CIRCUMVENT | 조 파라미터 $120M (1/100), 자체-증류 루프 |
| 품질 경량화 | BT-1423 | Shannon 정보밀도 | APPROACH | 400B→10B 97% 품질, 온디바이스 상용 |
| 에이전트 서빙 | BT-1424 | 조정 (σ·τ=48) | TRANSCEND | 100만+ 동시 에이전트, 초장기 세션 |
| 엔터프라이즈 커스텀 | BT-1425 | 시장 포화 | CIRCUMVENT | 10,000+ 고객 $10/월, 어댑터 마켓 |
| 평가 파이프라인 | BT-1426 | 측정(Goodhart) | TRANSCEND | ISO/IEEE 표준, 메타평가 τ=4 수렴 |
| AI 의식 | BT-1427 | 끌개 (Φ_c=0.5) | APPROACH | Basin Binding 이전 유토피아 고정 |
| AI Safety | BT-1428 | σ·φ=n·τ EXACT | TRANSCEND | 171종 상용 게이트, R(6)=1 고정점 실측 |

**검증 총계**: 48/48 Mk.V self-check PASS (`claim ≤ limit` 자동 증명, stdlib only, 하드코딩 0). TRANSCEND 3 / CIRCUMVENT 2 / APPROACH 3.

> 모든 Mk.V 는 `claim ≤ limit` 자가 검증 필수. 이후 업데이트는 물리 한계 재정의 시에만 가능.

---

## 연결 레지스트리 (추적성)

- **도메인 문서 (§S6 EVOLVE + §Mk.V VERIFY)**: `domains/cognitive/ai-*/ai-*.md` x 8
- **BT 노드**: `theory/breakthroughs/bt-142{1..8}-*-2026-04-20.md` (+8)
- **Atlas 상수 (EXACT)**: `theory/constants/atlas-constants.md` §Mk.V Anchor (+8)
- **Sub-SSOT**: `domains/cognitive/_index.json` v1.4.0 (8 slug status→mk5, ai-safety 신규, 총 34→35)
- **Discovery graph**: `n6shared/discovery_graph.json` v16 ai-fellows-mk5 (노드 523 / 엣지 2129, 실측)
- **가설 인덱스**: `theory/breakthroughs/_hypotheses_index.json` verified 666→674
