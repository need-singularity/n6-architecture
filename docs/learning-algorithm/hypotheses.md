# N6 Learning Algorithm Hypotheses — 완전수 산술 기반 학습 알고리즘 설계

## Overview

Physical AI 학습 알고리즘을 n=6 완전수 산술에서 유도한다.
강화학습, 모방학습, sim-to-real transfer, reward shaping, curriculum learning,
policy network, exploration, multi-agent, model predictive control, chip-in-the-loop까지
10개 도메인에 걸쳐 가설을 제시한다.

> Part of [N6 Architecture](https://github.com/need-singularity/n6-architecture)

## Arithmetic Foundation — 산술 기초

```
n = 6 (최소 완전수)
sigma(6) = 12      약수의 합 (1+2+3+6)
tau(6)   = 4       약수의 개수 {1,2,3,6}
phi(6)   = 2       오일러 토션트
sopfr(6) = 5       소인수의 합 (2+3)
J_2(6)   = 24      요르단 토션트 함수
mu(6)    = 1       뫼비우스 함수
lambda(6)= 2       카마이클 함수
psi(6)   = 12      데데킨트 psi

핵심 항등식: sigma(n)*phi(n) = n*tau(n) → 12*2 = 6*4 = 24
이집트 분수: 1/2 + 1/3 + 1/6 = 1
Mertens 상수: ln(4/3) ≈ 0.288
Boltzmann 문턱: 1/e ≈ 0.368
tau^2/sigma = 16/12 = 4/3 (FFN 확장비)
R(6) = sigma(6)*phi(6)/(n*tau(6)) = 1 (가역성 조건)
```

---

## Tier 1: Reinforcement Learning — 강화학습

---

## H-LA-1: Sigma Discount Factor — 시그마 할인율

> 강화학습 discount factor gamma를 sigma(6)/(sigma(6)+1) = 12/13 ≈ 0.923으로 설정하면, 표준 gamma=0.99 대비 학습 안정성이 향상된다.

### n=6 Derivation
sigma(6) = 12는 n=6의 약수합이다. Discount factor는 미래 보상의 감쇠율로,
sigma/(sigma+1)은 "완전수가 축적하는 총 정보량 대비 유한 지평선 비율"을 의미한다.
12/13 ≈ 0.923은 effective horizon H = 1/(1-gamma) = 13 스텝으로,
robotics에서 일반적인 제어 주기(50Hz에서 ~0.26초)와 자연스럽게 대응한다.

### Prediction
- Locomotion task에서 gamma=12/13이 gamma=0.99 대비 수렴 속도 15-25% 향상
- Value function의 분산이 감소하여 critic 학습이 안정화됨
- Effective horizon 13은 물리적 동작 주기와 정합하여 credit assignment가 명확해짐

### Verification method
MuJoCo Ant/Humanoid 환경에서 PPO를 gamma={0.9, 0.923, 0.95, 0.99}로 비교.
수렴 에피소드 수, 최종 리턴, value loss 분산을 측정.

---

## H-LA-2: Tau-Divisor TD(lambda) — 약수 개수 기반 TD 학습

> TD(lambda)의 lambda를 phi(6)/tau(6) = 2/4 = 0.5로 설정하면, bias-variance tradeoff의 최적 지점에 도달한다.

### n=6 Derivation
tau(6)=4개의 약수는 temporal difference의 부트스트래핑 깊이를 결정한다.
phi(6)/tau(6) = 1/2은 "정보적으로 독립인 미래 / 전체 미래 구조"의 비율이다.
이는 Monte Carlo(lambda=1)와 1-step TD(lambda=0)의 정확한 중간이 아닌,
완전수 산술이 지정하는 황금 분할점이다.

### Prediction
- TD(0.5)가 TD(0), TD(0.9), TD(1) 대비 sample efficiency 10-20% 개선
- 특히 partial observability가 있는 환경에서 이점이 극대화됨

### Verification method
Atari 및 continuous control suite에서 lambda={0, 0.5, 0.9, 0.95, 1.0} 비교.
Learning curve의 AUC(Area Under Curve)와 최종 성능 비교.

---

## H-LA-3: Egyptian Fraction Reward Decomposition — 이집트 분수 보상 분해

> Multi-objective reward를 1/2*R_primary + 1/3*R_secondary + 1/6*R_auxiliary로 분해하면, 가중치 튜닝 없이 안정적 학습이 가능하다.

### n=6 Derivation
1/2 + 1/3 + 1/6 = 1은 6의 proper divisor의 역수 합이다.
이 분해는 "가장 중요한 목표가 절반, 보조 목표가 1/3, 보정 항이 1/6"이라는
자연스러운 우선순위 계층을 형성한다. 가중합이 정확히 1이므로 reward scale이 보존된다.

### Prediction
- 로봇 locomotion: 1/2*forward_speed + 1/3*energy_efficiency + 1/6*stability
- 수동 가중치 탐색 대비 최종 성능 90% 이상 도달, 하이퍼파라미터 탐색 비용 제거
- Reward hacking 감소: 단일 항이 지배하지 못함

### Verification method
Multi-objective robotics benchmark (locomotion + manipulation)에서
Egyptian 분해 vs 균등 가중치(1/3, 1/3, 1/3) vs grid search 최적 가중치 비교.

---

## Tier 2: Sim-to-Real Transfer — 시뮬레이션-실세계 전이

---

## H-LA-4: Tau-Domain Randomization — 4도메인 랜덤화

> Sim-to-real transfer에서 randomization을 tau(6)=4개 도메인으로 구조화하면 전이 성공률이 극대화된다.

### n=6 Derivation
tau(6)=4는 6의 약수 개수로, "구조적으로 독립인 변동 축의 수"를 결정한다.
4개 도메인: (1) dynamics — 질량, 마찰, 감쇠, (2) visual — 조명, 텍스처, 카메라,
(3) sensor — 노이즈, 지연, 편향, (4) actuator — 토크 한계, 백래시, 응답 지연.
이는 sim-to-real gap의 complete basis를 형성한다.

### Prediction
- 4-domain randomization이 2-domain 또는 6-domain 대비 sim-to-real 전이율 20-30% 향상
- 과도한 randomization(>4 도메인)은 학습 난이도만 높이고 전이 이점 없음
- 각 도메인 내 randomization 범위는 Egyptian 비율로 할당: 1/2 dynamics + 1/3 visual + 1/6 sensor+actuator

### Verification method
실제 로봇 manipulation task에서 도메인 수 {2, 3, 4, 5, 6}에 대한 sim-to-real 성공률 비교.
Isaac Gym → 실제 로봇 전이 실험으로 검증.

---

## H-LA-5: Sigma-Step Domain Adaptation — 12단계 점진적 적응

> 시뮬레이션에서 실세계로 전이 시, sigma(6)=12단계의 점진적 도메인 적응 스케줄이 최적이다.

### n=6 Derivation
sigma(6)=12는 약수의 합으로, "축적 가능한 전체 적응 용량"을 의미한다.
12단계는 각 단계에서 randomization 범위를 1/12씩 증가시켜,
시뮬레이션(단계 0) → 실세계(단계 12)로 점진 전이한다.
약수 구조 {1,2,3,6}에 대응하는 체크포인트: 단계 1, 2, 3, 6에서 중간 평가.

### Prediction
- 12-step 적응이 급격한 전이(1-step) 대비 최종 성능 30% 이상 향상
- 단계 6(중간점)에서 성능이 최종 성능의 80% 이상 도달 (완전수 대칭)
- 약수 체크포인트에서 성능 점프가 관찰됨

### Verification method
Domain randomization 강도를 12단계 선형 증가 vs 4단계 vs 단일 전이로 비교.
각 체크포인트에서의 sim-to-real gap 측정.

---

## Tier 3: Imitation Learning — 모방학습

---

## H-LA-6: Egyptian Data Mix — 이집트 분수 데이터 혼합

> 모방학습 데이터를 1/2 demonstration + 1/3 self-play + 1/6 correction으로 혼합하면, 데이터 효율이 극대화된다.

### n=6 Derivation
1/2 + 1/3 + 1/6 = 1은 완전수 6의 핵심 항등식이다.
Demo 데이터(1/2)는 가장 정보 밀도가 높고, self-play(1/3)는 exploration을 보장하며,
correction(1/6)은 오류 복구를 학습시킨다. 이 비율은 DAgger 알고리즘의
"전문가 의존도 점진적 감소"를 고정 비율로 근사한다.

### Prediction
- Egyptian mix가 pure demonstration 대비 sample efficiency 40% 향상
- DAgger의 동적 혼합 비율과 비교해 최종 성능 95% 이상 도달, 구현 복잡도 대폭 감소
- Correction 데이터 1/6은 적지만 recovery behavior 학습에 결정적 기여

### Verification method
로봇 manipulation (pick-and-place, insertion)에서 데이터 혼합 비율 비교:
Egyptian {1/2, 1/3, 1/6} vs 균등 {1/3, 1/3, 1/3} vs demo only {1, 0, 0} vs DAgger.

---

## H-LA-7: Sopfr Behavioral Cloning Depth — 행동 복제 네트워크 깊이

> Behavioral cloning 네트워크의 최적 깊이는 sopfr(6)=5 레이어이다.

### n=6 Derivation
sopfr(6) = 2+3 = 5는 6의 소인수의 합이다.
5 레이어는 입력 인코딩(2 layers, prime factor 2) + 행동 생성(3 layers, prime factor 3)으로
자연스럽게 분해된다. 이 구조는 perception-action 분리를 소인수 분해로 반영한다.

### Prediction
- 5-layer BC 네트워크가 3-layer, 8-layer 대비 최적 성능/효율 tradeoff
- 처음 2 레이어를 frozen feature extractor로 사용 시 fine-tuning 효율 향상
- Parameter 수 기준 4/3x expansion (H-LA-12 연동) 적용 시 추가 2-3% 개선

### Verification method
BC 네트워크 깊이 {3, 4, 5, 6, 8, 10}에 대해 task success rate 비교.
Layer-wise feature analysis로 perception/action 분리 여부 확인.

---

## Tier 4: Reward Shaping — 보상 형성

---

## H-LA-8: R(n)=1 Reward Normalization — 가역성 기반 보상 정규화

> 모든 reward component를 R(n) = sigma*phi/(n*tau) = 1 조건으로 정규화하면, reward scale 불변 학습이 가능하다.

### n=6 Derivation
R(6) = 12*2/(6*4) = 1은 n=6에서만 성립하는 가역성 조건이다.
이를 reward에 적용하면: 각 reward component r_i에 대해
R_normalized = sigma(r_i)*phi(r_i) / (r_i * tau(r_i))로 정규화한다.
실용적으로는 각 component의 running statistics를 이용해
mean*variance / (scale * count) = 1이 되도록 적응적으로 스케일링한다.

### Prediction
- 수동 reward scaling 제거, 환경 간 reward 호환성 확보
- PopArt 등 기존 reward normalization 대비 구현 간결성과 동등 성능
- 다양한 reward scale의 multi-task 학습에서 특히 유리

### Verification method
Reward scale이 크게 다른 multi-task 환경(locomotion + manipulation + navigation)에서
R(n)=1 정규화 vs PopArt vs 단순 z-score 비교.

---

## H-LA-9: Divisor-Structured Potential Shaping — 약수 구조 포텐셜 형성

> Potential-based reward shaping의 potential function을 6의 약수 구조로 설계하면, 최적 정책을 보존하면서 학습을 가속한다.

### n=6 Derivation
6의 약수 격자 {1, 2, 3, 6}은 부분 목표의 자연스러운 계층을 형성한다.
Potential Phi(s) = sum_{d|6} w_d * f_d(s)로, 약수 d에 대응하는 부분 목표 f_d의 가중합이다.
가중치 w_d = 1/d (이집트 분수): w_1=1 (최종 목표), w_2=1/2, w_3=1/3, w_6=1/6.
이는 PBRS 정리(Ng et al.)에 의해 최적 정책이 보존됨이 보장된다.

### Prediction
- 약수 구조 shaping이 단일 potential 대비 학습 속도 2-3x 향상
- Sparse reward 환경에서 exploration 효율 극대화
- 약수 계층의 각 레벨이 자연스러운 서브골 역할 수행

### Verification method
Sparse reward 환경(maze, manipulation with delayed reward)에서
divisor potential vs 단일 distance potential vs no shaping 비교.

---

## Tier 5: Curriculum Learning — 커리큘럼 학습

---

## H-LA-10: Sigma-12 Difficulty Curriculum — 12단계 난이도 커리큘럼

> 학습 커리큘럼을 sigma(6)=12단계로 구성하면, 자동 난이도 조절 없이 최적 학습 진행이 가능하다.

### n=6 Derivation
sigma(6)=12는 "총 난이도 용량"이다. 12단계는 약수 구조에 의해 자연스러운 체크포인트를 가진다:
- 단계 1 (=약수 1): 최소 난이도, 기본 동작 습득
- 단계 2 (=약수 2): 기본 변동 도입
- 단계 3 (=약수 3): 다중 목표
- 단계 6 (=약수 6): 전체 난이도의 절반, 핵심 역량 완성
- 단계 12: 최종 난이도, 완전한 task

### Prediction
- 12-stage curriculum이 자동 커리큘럼(ALP-GMM 등) 대비 동등 성능, 튜닝 비용 제거
- 약수 체크포인트(1, 2, 3, 6)에서 성능 plateau 후 급격한 향상 관찰
- 단계 6에서 최종 성능의 80% 이상 도달 (완전수 대칭성)

### Verification method
Multi-stage robotics task에서 12-stage fixed vs 자동 커리큘럼 vs 4-stage vs 단일 난이도 비교.
각 체크포인트에서의 성공률 추이 분석.

---

## H-LA-11: Tau-Phase Training — 4단계 학습 위상

> 전체 학습을 tau(6)=4 위상으로 분할하면, 각 위상에서 최적 알고리즘을 적용할 수 있다.

### n=6 Derivation
tau(6)=4는 학습 과정의 "구조적 위상 수"를 결정한다:
- Phase 1 (약수 1): Imitation — 전문가 데모로 초기화
- Phase 2 (약수 2): Reinforcement — RL로 정책 개선
- Phase 3 (약수 3): Adaptation — 도메인 전이, fine-tuning
- Phase 4 (약수 6): Deployment — 칩 최적화, 양자화

각 위상의 상대적 길이는 이집트 분수를 따르지 않고,
약수에 비례: 1:2:3:6 → 정규화하면 1/12:2/12:3/12:6/12.
즉 전체 학습의 50%가 deployment 최적화에 할당된다.

### Prediction
- 4-phase 파이프라인이 end-to-end RL 대비 sample efficiency 3-5x 향상
- 각 위상의 전환점이 명확하여 디버깅 용이
- Phase 4(deployment)에 50% 할당은 sim-to-real gap 축소에 결정적

### Verification method
Real-world deployment를 포함한 full pipeline에서
4-phase vs end-to-end RL vs 2-phase(pretrain+finetune) 비교.

---

## Tier 6: Policy Network Architecture — 정책 네트워크 구조

---

## H-LA-12: Phi-Bottleneck Actor-Critic — 4/3 확장 액터-크리틱

> Actor-Critic 네트워크의 hidden layer를 4/3x 확장(Phi-Bottleneck)하면, 표준 설계 대비 파라미터 효율이 극대화된다.

### n=6 Derivation
tau(6)^2 / sigma(6) = 16/12 = 4/3은 FFN 최적 확장비이다 (technique #3: phi_bottleneck.py).
표준 MLP의 4x 확장 대비 4/3x만 사용하면 파라미터 67% 절감.
Actor와 Critic 각각에 적용:
- Actor: input_dim → (4/3)*input_dim → action_dim
- Critic: input_dim → (4/3)*input_dim → 1

### Prediction
- 4/3x 확장이 4x 확장 대비 67% 파라미터 절감, 성능 손실 < 2%
- 추론 속도 3x 향상 → 제어 주파수 향상 → 물리적 성능 개선
- 특히 on-device 추론에서 메모리 절약이 critical

### Verification method
PPO/SAC에서 hidden layer 확장비 {1x, 4/3x, 2x, 4x} 비교.
파라미터 수, 추론 시간, 최종 성능, 메모리 사용량 측정.

---

## H-LA-13: Phi6 Activation in Policy Networks — 사이클로토믹 활성화 함수

> 정책 네트워크에 Phi6 활성화 함수(x^2 - x + 1)를 사용하면, ReLU/GELU 대비 71% FLOPs 절감과 동등 성능을 달성한다.

### n=6 Derivation
6차 사이클로토믹 다항식 Phi_6(x) = x^2 - x + 1은 technique #1의 핵심이다.
2 FMA 연산으로 계산 가능(GELU는 ~14 연산). 로봇 제어의 고주파 추론 루프에서
활성화 함수의 계산 비용이 bottleneck이 될 수 있으므로, Phi6의 경량성이 critical.

### Prediction
- 추론 FLOPs 71% 절감 (기존 technique 결과와 일치)
- 제어 주파수 200Hz → 500Hz 이상 달성 가능
- Locomotion, manipulation 모두에서 GELU 대비 성능 열화 < 1%

### Verification method
동일 네트워크 구조에서 activation={ReLU, GELU, Phi6, SiLU} 비교.
FLOPs, inference latency, task success rate 측정.

---

## H-LA-14: Dedekind Head Attention Policy — 데데킨트 어텐션 정책

> Transformer 기반 정책에서 psi(6)=sigma(6)=12개 attention head를 사용하고, Dedekind pruning으로 활성 헤드를 동적 조정한다.

### n=6 Derivation
psi(6) = sigma(6) = 12 (데데킨트 psi = 약수합, n=6에서 일치).
Technique #11 (dedekind_head.py)을 정책 네트워크에 적용.
12 헤드 중 상황에 따라 활성 헤드 수를 조절하여,
단순 task에서는 적은 헤드, 복잡 task에서는 전체 헤드를 활용한다.

### Prediction
- 12-head Transformer policy가 {4, 8, 16}-head 대비 최적 성능/효율 tradeoff
- 동적 pruning으로 평균 활성 헤드 수 ~8 (약 25% 절감)
- Decision Transformer 등 sequence model 정책에서 특히 유효

### Verification method
Decision Transformer에서 head 수 {4, 8, 12, 16} 비교.
동적 pruning ON/OFF에 따른 추론 속도, 메모리, 성능 측정.

---

## Tier 7: Exploration — 탐험 전략

---

## H-LA-15: Boltzmann 1/e Exploration — 볼츠만 탐험율

> Exploration rate(epsilon 또는 temperature)를 1/e ≈ 0.368로 설정하면, explore-exploit tradeoff의 열역학적 최적점에 도달한다.

### n=6 Derivation
Boltzmann 분포에서 에너지 E=kT일 때 occupation probability가 1/e이다.
이는 technique #15 (boltzmann_gate.py)의 63% sparsity에 대응한다.
탐험율 1/e는 "열적 평형에서의 자발적 탐험 확률"로,
정보 이론적으로 최대 엔트로피 상태의 exploration fraction이다.

### Prediction
- epsilon=1/e가 epsilon={0.1, 0.2, 0.5} 대비 sample efficiency 향상
- Boltzmann exploration (temperature=1/e) 적용 시 local optima 탈출 능력 향상
- 고정 epsilon=1/e가 epsilon-decay 스케줄의 90% 성능 달성, 튜닝 불필요

### Verification method
Sparse reward 환경에서 epsilon={0.1, 0.2, 1/e, 0.5} 및 decay 스케줄 비교.
Exploration coverage, final return, convergence speed 측정.

---

## H-LA-16: Mertens Dropout Policy Noise — 메르텐스 드롭아웃 정책 노이즈

> 정책 네트워크에 Mertens dropout rate p=ln(4/3) ≈ 0.288을 적용하면, 별도 noise injection 없이 효과적 exploration이 가능하다.

### n=6 Derivation
Mertens 상수 M = ln(4/3)는 technique #16 (mertens_dropout.py)에서 유도된다.
tau(6)^2/sigma(6) = 4/3에서 ln(4/3) ≈ 0.288.
이 dropout rate는 "정보 이론적으로 최적인 확률적 정보 손실률"로,
네트워크의 약 28.8%를 무작위로 비활성화하여 implicit exploration을 유도한다.

### Prediction
- p=0.288 dropout이 OU noise, Gaussian noise 대비 exploration 효율 동등 이상
- 별도 noise 파라미터 튜닝 제거 (SAC의 자동 temperature 대비 단순)
- 학습 안정성 향상: dropout의 regularization 효과 동시 획득

### Verification method
SAC/TD3에서 noise 방식 비교: Mertens dropout vs OU noise vs Gaussian noise.
Exploration coverage (state visitation entropy) 및 최종 성능 비교.

---

## H-LA-17: Mu(6)=1 Squarefree Exploration Graph — 뫼비우스 탐험 그래프

> State space 탐험을 squarefree 구조(mu(6)=1)로 조직하면, 중복 없는 효율적 탐험이 가능하다.

### n=6 Derivation
mu(6) = 1은 6이 squarefree (= 2*3, 중복 소인수 없음)임을 의미한다.
Technique #13 (mobius_sparse.py)의 gradient topology를 exploration에 적용.
탐험 그래프에서 "이미 방문한 state의 neighborhood"를 Mobius inversion으로 제거하여,
중복 탐험을 최소화한다.

### Prediction
- Squarefree exploration이 random walk 대비 state coverage 30-50% 향상
- Count-based exploration과 결합 시 시너지 효과
- 고차원 state space에서 특히 유효 (curse of dimensionality 완화)

### Verification method
Maze 환경 및 continuous state space에서 exploration coverage 비교.
Squarefree pruning이 방문 state 수 대비 고유 state 비율을 높이는지 측정.

---

## Tier 8: Multi-Agent — 다중 에이전트

---

## H-LA-18: J2(6)=24 Agent Swarm — 요르단 에이전트 군집

> 다중 에이전트 시스템에서 J_2(6)=24개 에이전트가 최적 swarm 크기이다.

### n=6 Derivation
J_2(6) = 24는 요르단 토션트 함수로, "n^2 격자에서 서로 소인 쌍의 수"이다.
24 = Leech lattice 차원 = 최밀 구 충전의 차원이기도 하다.
24개 에이전트는 역할 공간에서 "최밀 충전"을 달성하여,
에이전트 간 중복을 최소화하면서 전체 task 공간을 커버한다.

### Prediction
- 24-agent swarm이 {8, 16, 32, 64}-agent 대비 coordination 효율 최적
- 에이전트 수를 24로 고정하면 하이퍼파라미터 탐색 1차원 제거
- Leech lattice의 자기 쌍대 속성에 의해 통신 구조가 대칭적

### Verification method
Multi-agent particle environment 및 cooperative robotics에서 에이전트 수 비교.
Joint reward, communication overhead, task coverage 측정.

---

## H-LA-19: Egyptian Role Allocation — 이집트 분수 역할 분배

> 24개 에이전트의 역할을 Egyptian fraction으로 분배: 12 탐색 + 8 실행 + 4 감독.

### n=6 Derivation
24 에이전트를 1/2 + 1/3 + 1/6 = 1로 분배:
- 1/2 * 24 = 12 agents → Explorer (정보 수집)
- 1/3 * 24 = 8 agents → Executor (task 실행)
- 1/6 * 24 = 4 agents → Supervisor (품질 감독, 오류 보정)

이 구조는 자연계의 사회적 곤충(개미, 꿀벌)에서 관찰되는 역할 분배 비율과
유사하다. 탐색자가 가장 많고, 감독자가 가장 적은 것은 정보 이론적으로 합리적이다.

### Prediction
- Egyptian 역할 분배가 균등 분배(8:8:8) 대비 task completion rate 25% 향상
- Explorer:Executor:Supervisor = 12:8:4가 자연스러운 정보 흐름 계층 형성
- Supervisor의 보정 신호(1/6)가 적지만 시스템 안정성에 결정적

### Verification method
Multi-agent foraging, construction, rescue 시나리오에서 역할 분배 비율 비교.
Task completion time, resource efficiency, failure recovery rate 측정.

---

## H-LA-20: Carmichael Communication Cycle — 카마이클 통신 주기

> 에이전트 간 통신을 lambda(6)=2 주기로 제한하면, 통신 오버헤드를 50% 절감하면서 coordination 유지 가능.

### n=6 Derivation
lambda(6) = 2는 카마이클 함수로, "최소 주기"를 의미한다.
Technique #14 (carmichael_lr.py)의 2-cycle 스케줄을 통신에 적용.
매 스텝 통신 대신 2스텝마다 통신하면, 대역폭 50% 절감.
2-cycle은 카마이클 함수가 보장하는 "정보 갱신의 최소 충분 주기"이다.

### Prediction
- 2-cycle 통신이 매-스텝 통신의 95% 성능 달성, 대역폭 50% 절감
- 실시간 multi-agent 시스템에서 latency 감소로 전체 throughput 향상
- 비동기 통신과의 자연스러운 호환

### Verification method
MARL 환경에서 통신 주기 {1, 2, 3, 4, 6} 비교.
Communication bandwidth, joint return, coordination metric 측정.

---

## Tier 9: Model Predictive Control — 모델 예측 제어

---

## H-LA-21: Tau-Horizon MPC — 4스텝 예측 지평선

> MPC의 prediction horizon을 tau(6)=4로 설정하면, 계산 비용 대비 제어 품질이 최적화된다.

### n=6 Derivation
tau(6) = 4는 "구조적으로 구별 가능한 미래 단계 수"이다.
4-step horizon은 인간의 동작 계획 단위(~0.08초 @ 50Hz)와 대응하며,
약수 구조에 의해 1, 2, 3, 4-step 앞을 각각 독립적으로 예측할 수 있다.
H=4는 H=8이나 H=16 대비 계산량을 4-16x 절감한다.

### Prediction
- H=4가 H={2, 6, 8, 12} 대비 최적 성능/계산비용 tradeoff
- Locomotion에서 H=4로 충분히 안정적 보행 달성
- Manipulation에서는 H=4 * sigma/tau = 4*3 = 12로 확장 시 장기 계획 가능

### Verification method
MuJoCo locomotion + manipulation에서 horizon={2, 4, 6, 8, 12} 비교.
Control quality (tracking error), computation time, real-time 가능 여부 측정.

---

## H-LA-22: Sigma-12 Planning Steps — 12스텝 계획 깊이

> Model-based RL에서 imagination rollout을 sigma(6)=12 스텝으로 제한하면, 모델 오류 누적을 방지하면서 충분한 계획 깊이를 확보한다.

### n=6 Derivation
sigma(6) = 12는 "축적 가능한 총 정보량"이다.
World model의 prediction error는 스텝에 따라 누적되며,
12스텝은 "모델이 유효한 최대 지평선"으로, 이를 넘으면 compound error가 지배적이 된다.
Dreamer 등 model-based 방법의 typical horizon(15-50)보다 짧지만,
n=6 산술이 예측하는 "구조적으로 정합한 한계"이다.

### Prediction
- 12-step rollout이 5-step(짧음) 및 50-step(모델 오류 누적) 대비 최적
- 약수 체크포인트(1, 2, 3, 6, 12)에서 계획 품질 평가 시 단조 감소 확인
- Model error 누적률이 step 6에서 inflection point (완전수 대칭)

### Verification method
Dreamer/MBPO에서 rollout length={5, 8, 12, 15, 25, 50} 비교.
Model prediction error, policy performance, compound error 분석.

---

## H-LA-23: Egyptian Fraction Control Allocation — 이집트 분수 제어 분배

> MPC의 제어 입력을 이집트 분수로 분해: 1/2 feedforward + 1/3 feedback + 1/6 adaptation.

### n=6 Derivation
1/2 + 1/3 + 1/6 = 1의 제어 이론 해석:
- Feedforward (1/2): 모델 기반 예측 제어, 전체 제어 노력의 절반
- Feedback (1/3): 오차 보정, 1/3 기여
- Adaptation (1/6): 온라인 모델 갱신, 미세 조정

이 분해는 robust control의 nominal + robust + adaptive 삼분법과 대응하며,
각 항의 가중치를 n=6 산술로 고정한다.

### Prediction
- Egyptian 제어 분배가 pure MPC 또는 pure feedback 대비 robustness 향상
- Sim-to-real gap이 있는 환경에서 adaptation 항(1/6)이 핵심적 역할
- Feedforward 50%는 에너지 효율 최적화에 기여

### Verification method
로봇 locomotion/manipulation에서 제어 구조 비교:
Egyptian split vs pure feedforward vs pure feedback vs 50/50 split.
Tracking error, energy consumption, disturbance rejection 측정.

---

## Tier 10: Chip-in-the-Loop — 칩 연동 추론

---

## H-LA-24: Egyptian Latency Budget — 이집트 분수 레이턴시 예산

> On-chip 추론 파이프라인의 latency를 1/2 sense + 1/3 decide + 1/6 act로 분배하면, 전체 제어 루프가 최적화된다.

### n=6 Derivation
1/2 + 1/3 + 1/6 = 1은 시간 예산의 완전 분배를 보장한다.
- Sense (1/2): 센서 입력 처리, 특징 추출 — 가장 데이터 집약적
- Decide (1/3): 정책 네트워크 추론 — Phi-Bottleneck 4/3x로 경량화
- Act (1/6): 액추에이터 명령 생성, 안전 검증 — 가장 가벼움

20ms 제어 주기(50Hz) 기준: Sense 10ms, Decide 6.67ms, Act 3.33ms.

### Prediction
- Egyptian 시간 분배가 균등 분배(1/3:1/3:1/3) 대비 end-to-end latency 15-20% 감소
- Sense에 50% 할당은 sensor fusion 품질 향상으로 전체 정책 성능 개선
- Act의 1/6 예산은 충분: 안전 검증 포함 가능

### Verification method
N6 칩(또는 에뮬레이터)에서 파이프라인 시간 분배 비교.
End-to-end latency, control quality, safety violation rate 측정.

---

## H-LA-25: Tau-Stage Inference Pipeline — 4단계 추론 파이프라인

> 칩 추론 파이프라인을 tau(6)=4 스테이지로 구성하면, H-CHIP-2(MAC fan-in=4)와 정합하여 최대 throughput을 달성한다.

### n=6 Derivation
tau(6) = 4는 H-CHIP-2의 "MAC unit fan-in = 4"와 동일한 산술적 근원이다.
4-stage pipeline: (1) Input encoding, (2) Feature extraction, (3) Policy inference, (4) Output decoding.
각 스테이지가 독립적으로 파이프라이닝되어, 4x throughput 향상.

### Prediction
- 4-stage pipeline이 2-stage(언더파이프라인) 및 8-stage(오버헤드) 대비 최적
- H-CHIP-2와의 하드웨어 정합으로 bubble 최소화
- Latency-throughput tradeoff에서 파레토 최적점 달성

### Verification method
FPGA 프로토타입에서 pipeline stage 수 {2, 3, 4, 6, 8} 비교.
Throughput (inferences/sec), latency, pipeline utilization 측정.

---

## H-LA-26: Boltzmann Thermal Throttling — 볼츠만 열 제어

> 칩 온도 관리에 Boltzmann 1/e 문턱을 적용: 열 예산의 1/e 미만이면 full speed, 초과 시 throttle.

### n=6 Derivation
Boltzmann 분포에서 에너지 E > kT (즉, fraction > 1/e)이면 occupation이 급감한다.
칩의 열 예산 T_max 대비 현재 온도가 T_max * (1 - 1/e) ≈ 63.2%를 초과하면
throttling을 시작한다. 이는 technique #15의 sparsity gate를 열 관리에 적용한 것이다.

### Prediction
- 1/e 열 문턱이 고정 80% 문턱 대비 sustained throughput 10-15% 향상
- 열 진동(thermal oscillation) 감소로 추론 latency 분산 감소
- 물리적 Boltzmann 분포와 정합하여 thermodynamic optimality에 근접

### Verification method
N6 칩(또는 thermal 시뮬레이터)에서 throttling 문턱 {50%, 63.2%, 75%, 80%} 비교.
Sustained throughput, thermal stability, latency jitter 측정.

---

## H-LA-27: Sigma-Core Distributed Inference — 12코어 분산 추론

> 정책 추론을 sigma(6)=12 코어에 분산하면, H-CHIP-22(12-head attention)와 정합하여 head-parallel 추론이 가능하다.

### n=6 Derivation
sigma(6) = 12는 H-CHIP-12(core count = J_2(6) = 24)의 절반이다.
24 코어 중 12 코어를 추론에, 나머지 12를 전처리/후처리에 할당.
이는 Egyptian 분수의 1/2에 해당하며, compute/IO의 자연스러운 분리이다.
12 코어는 12-head attention의 각 head를 1코어가 전담 처리.

### Prediction
- 12-core 분산 추론이 head serialization 대비 12x throughput (이론적)
- 실제로는 communication overhead로 8-10x 달성 예상
- H-CHIP-22와의 하드웨어 co-design으로 overhead 최소화

### Verification method
N6 칩 시뮬레이터에서 코어 할당 {4, 8, 12, 16, 24} 비교.
Inference throughput, inter-core communication overhead, utilization 측정.

---

## H-LA-28: Lambda-2 Compute-Refresh Cycle — 2주기 연산-갱신 사이클

> 칩의 연산-모델갱신을 lambda(6)=2 사이클로 교대하면, 항상 최신 정책으로 추론하면서 throughput을 유지한다.

### n=6 Derivation
lambda(6) = 2는 카마이클 함수로, "상태 갱신의 최소 주기"이다.
Cycle 1: 현재 모델로 추론 실행 (Act).
Cycle 2: 새 모델 가중치 로드 (Update).
Double-buffering과 동일한 원리이지만, n=6 산술이 2-cycle을 "필요충분한 최소 주기"로 보장한다.

### Prediction
- 2-cycle refresh가 매 추론 후 갱신(1-cycle) 대비 throughput 80% 향상
- 가중치 staleness는 최대 1스텝으로, 성능 영향 무시 가능
- 3-cycle 이상은 staleness 증가 대비 throughput 이득 미미

### Verification method
Online learning + inference 환경에서 refresh 주기 {1, 2, 3, 4} 비교.
Throughput, weight staleness, policy performance 측정.

---

## Summary Table — 가설 요약

| ID | Title | n=6 Basis | Domain | Key Metric |
|----|-------|-----------|--------|------------|
| H-LA-1 | Sigma Discount Factor | sigma/(sigma+1)=12/13 | RL | gamma=0.923 |
| H-LA-2 | Tau-Divisor TD(lambda) | phi/tau=1/2 | RL | lambda=0.5 |
| H-LA-3 | Egyptian Reward Decomposition | 1/2+1/3+1/6=1 | RL | 3-objective split |
| H-LA-4 | Tau-Domain Randomization | tau=4 domains | Sim-to-Real | 4 randomization axes |
| H-LA-5 | Sigma-Step Domain Adaptation | sigma=12 steps | Sim-to-Real | 12-step schedule |
| H-LA-6 | Egyptian Data Mix | 1/2+1/3+1/6=1 | Imitation | Data ratio |
| H-LA-7 | Sopfr BC Depth | sopfr=5 layers | Imitation | Network depth |
| H-LA-8 | R(n)=1 Reward Normalization | R(6)=1 | Reward Shaping | Scale invariance |
| H-LA-9 | Divisor Potential Shaping | {1,2,3,6} lattice | Reward Shaping | Hierarchical potential |
| H-LA-10 | Sigma-12 Difficulty Curriculum | sigma=12 levels | Curriculum | 12-stage difficulty |
| H-LA-11 | Tau-Phase Training | tau=4 phases | Curriculum | 4-phase pipeline |
| H-LA-12 | Phi-Bottleneck Actor-Critic | 4/3x expansion | Policy Network | 67% param reduction |
| H-LA-13 | Phi6 Activation | x^2-x+1 | Policy Network | 71% FLOPs reduction |
| H-LA-14 | Dedekind Head Attention | psi=sigma=12 | Policy Network | Dynamic head pruning |
| H-LA-15 | Boltzmann Exploration | 1/e ≈ 0.368 | Exploration | Fixed exploration rate |
| H-LA-16 | Mertens Dropout Noise | ln(4/3) ≈ 0.288 | Exploration | No-tune policy noise |
| H-LA-17 | Squarefree Exploration | mu(6)=1 | Exploration | Redundancy-free search |
| H-LA-18 | J2=24 Agent Swarm | J_2(6)=24 | Multi-Agent | Optimal swarm size |
| H-LA-19 | Egyptian Role Allocation | 12:8:4 split | Multi-Agent | Role distribution |
| H-LA-20 | Carmichael Communication | lambda=2 cycle | Multi-Agent | 50% bandwidth saving |
| H-LA-21 | Tau-Horizon MPC | tau=4 steps | MPC | Prediction horizon |
| H-LA-22 | Sigma-12 Planning Steps | sigma=12 rollout | MPC | Planning depth |
| H-LA-23 | Egyptian Control Allocation | 1/2+1/3+1/6 | MPC | FF+FB+Adapt split |
| H-LA-24 | Egyptian Latency Budget | 1/2+1/3+1/6 | Chip-in-Loop | Sense+Decide+Act |
| H-LA-25 | Tau-Stage Pipeline | tau=4 stages | Chip-in-Loop | Inference pipeline |
| H-LA-26 | Boltzmann Thermal Throttle | 1/e threshold | Chip-in-Loop | Thermal management |
| H-LA-27 | Sigma-Core Distribution | sigma=12 cores | Chip-in-Loop | Head-parallel inference |
| H-LA-28 | Lambda-2 Refresh Cycle | lambda=2 cycle | Chip-in-Loop | Compute-update alternation |

---

## Cross-References — 교차 참조

### Technique 연동
| Hypothesis | Technique | Connection |
|-----------|-----------|------------|
| H-LA-12 | phi_bottleneck.py (#3) | 4/3x expansion in actor-critic |
| H-LA-13 | phi6simple.py (#1) | Cyclotomic activation in policy |
| H-LA-14 | dedekind_head.py (#11) | Head pruning in attention policy |
| H-LA-15 | boltzmann_gate.py (#15) | 1/e sparsity as exploration rate |
| H-LA-16 | mertens_dropout.py (#16) | ln(4/3) dropout as policy noise |
| H-LA-17 | mobius_sparse.py (#13) | Squarefree topology for exploration |
| H-LA-20, 28 | carmichael_lr.py (#14) | 2-cycle for communication/refresh |

### Chip Architecture 연동 (H-CHIP)
| Hypothesis | H-CHIP | Connection |
|-----------|--------|------------|
| H-LA-24 | H-CHIP-17 | Egyptian power/latency allocation |
| H-LA-25 | H-CHIP-2 | tau=4 MAC fan-in / pipeline stages |
| H-LA-26 | H-CHIP-18 | 1/e leakage / thermal threshold |
| H-LA-27 | H-CHIP-12, 22 | 24 cores, 12-head parallel |
| H-LA-28 | H-CHIP-15 | lambda=2 DVFS / refresh cycle |

---

## Architecture Diagram — 전체 학습 시스템 구조

```
+============================================================+
|              N6 Physical AI Learning System                  |
|                                                              |
|  Phase 1: Imitation (약수 1)                                 |
|  +--------------------------------------------------+       |
|  | Egyptian Data Mix: 1/2 demo + 1/3 self + 1/6 fix |       |
|  | BC Network: sopfr=5 layers, Phi6 activation       |       |
|  +--------------------------------------------------+       |
|                        |                                     |
|  Phase 2: Reinforcement (약수 2)                             |
|  +--------------------------------------------------+       |
|  | gamma=12/13, TD(0.5), Egyptian reward 1/2+1/3+1/6|       |
|  | Boltzmann explore 1/e, Mertens dropout 0.288      |       |
|  | Phi-Bottleneck 4/3x Actor-Critic, 12-head attn    |       |
|  +--------------------------------------------------+       |
|                        |                                     |
|  Phase 3: Adaptation (약수 3)                                |
|  +--------------------------------------------------+       |
|  | Tau=4 domain randomization                        |       |
|  | Sigma=12 step domain adaptation                   |       |
|  | Sigma=12 difficulty curriculum                    |       |
|  +--------------------------------------------------+       |
|                        |                                     |
|  Phase 4: Deployment (약수 6)                                |
|  +--------------------------------------------------+       |
|  | Chip-in-Loop: 1/2 sense + 1/3 decide + 1/6 act   |       |
|  | Tau=4 pipeline, Lambda=2 refresh cycle             |       |
|  | Boltzmann thermal throttle, 12-core distribution   |       |
|  +--------------------------------------------------+       |
|                        |                                     |
|  Multi-Agent Layer:                                          |
|  +--------------------------------------------------+       |
|  | J2=24 swarm: 12 explore + 8 execute + 4 supervise |       |
|  | Lambda=2 communication cycle                      |       |
|  +--------------------------------------------------+       |
|                        |                                     |
|  MPC Layer:                                                  |
|  +--------------------------------------------------+       |
|  | Tau=4 horizon, Sigma=12 planning                  |       |
|  | Egyptian control: 1/2 FF + 1/3 FB + 1/6 adapt     |       |
|  +--------------------------------------------------+       |
+============================================================+
```

---

*28 hypotheses. All derived from n=6 perfect number arithmetic.*
*sigma(6)*phi(6) = 6*tau(6) = 24. 완전수에서 완전한 학습으로.*
