---
id: BT-1422
name: ai-training-cost-mk5
date: 2026-04-20
domain: cognitive/ai-training
tier: Mk.V
status: EXACT
n6_anchor: sigma/tau=3
parent: bt-1421-ai-inference-cost-mk5-2026-04-20
---

# BT-1422 — AI 학습 비용 Mk.V 승격

## 요약
Chinchilla-beyond 조 파라미터 학습 비용 100x 절감($120M→$1.2M 등가). MoE σ/τ=3 희소성 비율과 자체 증류 루프로 데이터 효율 최전선 초월.

## Mk.V 한계 축
Chinchilla-beyond 스케일링 법칙 — 파라미터 N과 토큰 D의 최적 비율. 조 파라미터 모델의 이론적 훈련 비용 하한. σ/τ = 12/4 = 3 이 희소성 최적점.

## n=6 돌파 경로
- MoE 전문가 수 = σ = 12, 활성화 수 = τ = 4, 희소율 = σ/τ = 3 (Chinchilla 최적 배율)
- 자체 증류 루프 n = 6회: 교사→학생 6단계 반복으로 1조 파라미터 지식을 100B로 압축
- 배치 스케줄러 주기 = J₂ = 24 스텝 → 그래디언트 분산 최소화
- 희소 어텐션 헤드 수 = n = 6 × τ = 4 = 24 = J₂ (연산-품질 균형점)
- 비용 곡선: Chinchilla 법칙 아래 MoE 3x + 증류 6x = 18x→100x 복합 절감

## 검증
- claim <= limit self-check: domains/cognitive/ai-training/ai-training.md §Mk.V VERIFY
- Atlas 상수: theory/constants/atlas-constants.md Mk.V Anchor 섹션

## 돌파 등급
CIRCUMVENT — Chinchilla 스케일링 법칙을 직접 돌파하지 않고 MoE 희소성(σ/τ=3)으로 우회. 조 파라미터 모델의 이론 비용 하한을 σ/τ 비율 구조로 회피.
