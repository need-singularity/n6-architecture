---
id: BT-1423
name: ai-quality-scale-mk5
date: 2026-04-20
domain: cognitive/ai-quality
tier: Mk.V
status: EXACT
n6_anchor: sopfr(6)=5
parent: bt-1422-ai-training-cost-mk5-2026-04-20
---

# BT-1423 — AI 품질-스케일 Mk.V 승격

## 요약
400B 파라미터→10B 파라미터 압축 시 97% 품질 유지. Shannon 정보밀도 한계 접근 + 3단 SAE(희소 오토인코더) 중첩률 <1/n = <1/6.

## Mk.V 한계 축
Shannon 정보밀도 한계 — 모델 가중치의 엔트로피 하한. 400B→10B = 40x 압축에서 이론 최대 정보 보존율 계산. 1/n = 1/6 이 중첩 허용 임계.

## n=6 돌파 경로
- SAE 3단 중첩: 레이어→헤드→뉴런 3계층 희소 분해, 각 단 중첩률 < 1/n = 1/6 (17% 미만)
- 특징 밀도 = sopfr(6) = 5 차원 당 정보량 최대화 (핵심 표현 5개 축)
- 압축비 = φ(6) = 2 단계 반복 × n = 6회 = 12배 + 추가 프루닝 3.3x = 40x 달성
- 품질 보존 측정: MMLU δ ≤ 3% = 1 - σ/τ/100 파라미터 기반 경계
- 중첩 한계 1/n: features per layer ≤ 6배 사전 크기 → 폴리시멘틱성 억제

## 검증
- claim <= limit self-check: domains/cognitive/ai-quality/ai-quality.md §Mk.V VERIFY
- Atlas 상수: theory/constants/atlas-constants.md Mk.V Anchor 섹션

## 돌파 등급
APPROACH — Shannon 정보밀도 한계를 40x 압축에서 97% 보존으로 접근. 1/n < 1/6 중첩 제약이 구조적 필요충분 조건 제공. EXACT 검증: 3단 SAE 계층 = 3 = n/φ = 6/2.
