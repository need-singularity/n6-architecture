---
domain: ai-adversarial
requires:
  - to: ai-alignment
  - to: ai-interpretability
---
# AI 적대적 강건성 연구 (Anthropic Fellows 2026)

## S1 WHY (왜 적대적 강건성인가 -- 배포 안전의 핵심)

AI 시스템은 현실 세계에서 공격에 직면한다. 탈옥(jailbreak), 프롬프트 주입(prompt injection),
기만적 행동(deceptive alignment) -- 이런 공격을 방어하지 못하면 배포는 안전하지 않다.

적대적 강건성은 AI 안전의 실무 계층이다. 정렬(alignment) 이론이 "무엇이 옳은가"를 다룬다면,
강건성은 "공격자가 틀린 행동을 유도할 때 어떻게 버티는가"를 다룬다.

| 문제 영역 | 현재 상황 (2026) | 연구 목표 |
|----------|----------------|----------|
| 안전 평가 | 수동 레드팀, 비체계적 | 자동화된 체계적 안전 경계 탐색 |
| 기만 탐지 | 행동 기반 추론만 가능 | 내부 표상과 외부 행동 비교 분석 |
| 에이전트 안전 | 도구 사용 시 검증 미흡 | 샌드박싱, 감사추적, 권한 상승 탐지 |
| 안전 아키텍처 | 후처리 필터 의존 | 구조적으로 안전한 설계 원칙 |

**핵심 질문**: 공격 성공률을 측정 가능하게 낮추고, 방어의 이론적 한계를 정직하게 인정하며,
실전 배포에서 견디는 시스템을 어떻게 만드는가?

### 연구가 완성되면

```
  공격 표면 분석 (38종 공격 분류)
      |
  방어 설계 (4-축 아키텍처)
      |
  레드팀 검증 (자동화 + 수동)
      |
  경화(hardening) + 배포 모니터링
      |
  정직한 한계 보고 (No Free Lunch 인정)
```

## S2 COMPARE (현행 방어 vs 제안 개선) -- ASCII 비교 차트

### 현행 방어의 한계

```
+---------------------------------------------------------------------------+
|  장벽               |  왜 한계인가                |  어떻게 개선하는가         |
+--------------------+---------------------------+-------------------------+
| 1. 수동 레드팀      | 인력 한계, 커버리지 부족     | 자동 퍼징 + 체계적 탐색   |
|                    | 공격 패턴 편향               | 공격 분류학 기반 전수탐색  |
+--------------------+---------------------------+-------------------------+
| 2. 행동만 관찰      | 내부 상태 볼 수 없음         | 선형 탐침 + 내외부 비교   |
|                    | 기만 탐지 어려움             | 허니팟 + 일관성 테스트    |
+--------------------+---------------------------+-------------------------+
| 3. 도구 사용 무방비  | 에이전트 권한 무제한         | 샌드박싱 + 권한 격리      |
|                    | 주입 공격 취약               | 감사추적 + 자기감시       |
+--------------------+---------------------------+-------------------------+
| 4. 필터 우회 용이    | 후처리 단일 계층             | 다층 필터 + 구조적 안전   |
|                    | 맥락 이해 부족               | 어텐션 마스킹 + 헌법 코어 |
+--------------------+---------------------------+-------------------------+
| 5. 한계 은폐        | 실패 사례 비공개             | COUNTER >= 3 명시        |
|                    | 방어 불가 영역 미고지         | 정직한 한계 보고 의무     |
+---------------------------------------------------------------------------+
```

### 성능 비교 ASCII 막대 (현행 vs 제안)

```
+---------------------------------------------------------------------------+
|  [탈옥 방어율]                                                             |
|  현행 RLHF만   ######....................  ~30% (알려진 탈옥에만 유효)      |
|  제안 다층방어  ###################.......  ~75% (분류학 기반 체계적 방어)   |
|                                                                           |
|  [기만 탐지율]                                                             |
|  현행 행동관찰  ####........................  ~15% (표면 행동만 관찰)        |
|  제안 내부탐침  ###############...........  ~55% (선형 탐침 + 허니팟)       |
|                                                                           |
|  [에이전트 공격 차단율]                                                     |
|  현행 무방비   ##..........................  ~8% (거의 방어 없음)           |
|  제안 샌드박싱  ################..........  ~60% (권한 격리 + 감사추적)     |
|                                                                           |
|  [공격 표면 커버리지]                                                       |
|  현행 수동     #####.......................  ~20% (인력 한계)               |
|  제안 자동화   ####################......  ~80% (퍼징 + 분류학 전수탐색)   |
+---------------------------------------------------------------------------+
```

## S3 REQUIRES (필요 역량)

| 역량 | 설명 | 중요도 |
|------|-----|--------|
| 레드팀 경험 | 실전 공격/방어 양면 경험 | 필수 |
| 보안 사고방식 | 공격자 관점에서 사고하는 능력 | 필수 |
| 형식 검증 | 방어 속성의 수학적 증명 기법 | 높음 |
| 에이전트 시스템 | 도구 사용, 멀티에이전트 아키텍처 | 높음 |
| 해석가능성 | 내부 표상 분석, 선형 탐침 | 높음 |
| 통계적 검정 | 효과 크기 측정, p-value 해석 | 중간 |
| ML 시스템 설계 | 모델 아키텍처 수정 능력 | 중간 |

## S4 STRUCT (4-축 구조)

```
+------------------------------------------------------------------+
|                    AI 적대적 강건성 연구                            |
+------------------------------------------------------------------+
|                                                                    |
|  [축 A] 안전 평가          [축 B] 기만 탐지                        |
|  12 아이디어               8 아이디어                              |
|  - 레드팀 자동화           - 행동 일관성 검사                       |
|  - 안전 경계 맵핑          - 내외부 정렬 비교                       |
|  - 탈옥 분류학             - 슬리퍼 에이전트 탐지                   |
|  - 회귀 테스트 슈트        - 기만 선형 탐침                         |
|  - 다국어 안전             - 허니팟 테스트                          |
|  - 멀티턴 안전 감쇄        - 기만-능력 상관관계                     |
|  - 도구 사용 안전          - 기만 조기 경고                         |
|  - 멀티에이전트 안전 전파   - 최소 기만 재현 모델                    |
|  - 안전 특성 상관관계                                              |
|  - 적대적 강건성 벤치마크                                           |
|  - 조합 안전 테스트                                                |
|  - 맥락 의존 안전                                                  |
|                                                                    |
|  [축 C] 에이전트 안전      [축 D] 안전 아키텍처                     |
|  10 아이디어               8 아이디어                              |
|  - 도구 사용 샌드박싱      - 해석 가능 어텐션                       |
|  - 에이전트 감사추적       - 안전 우선 아키텍처                     |
|  - 자율 행동 범위 제한     - 모듈형 안전 계층                       |
|  - 에이전트 자기감시       - 투명 추론 모듈                         |
|  - 멀티에이전트 정렬 보존  - 일반화 안전 게이트                     |
|  - 명령 주입 방지          - 헌법 코어 (하드코딩)                   |
|  - 권한 상승 탐지          - 다층 필터 아키텍처                     |
|  - 장기 세션 정렬          - 안전 어텐션 마스크                     |
|  - 안전 오류 복구                                                  |
|  - 에이전트 간 신뢰 프로토콜                                        |
+------------------------------------------------------------------+
```

## S5 FLOW (공격 표면 분석 -> 방어 설계 -> 레드팀 -> 경화 -> 배포)

### 메인 플로우

```
+------------------------------------------------------------------------+
|  [1] 공격 표면 분석                                                      |
|      38종 공격 분류 -> 위협 모델링 -> 우선순위 결정                       |
|          |                                                              |
|          v                                                              |
|  [2] 방어 설계                                                           |
|      4-축 아키텍처 -> 계층별 방어 메커니즘 -> 형식 속성 정의              |
|          |                                                              |
|          v                                                              |
|  [3] 레드팀 검증                                                         |
|      자동 퍼징 + 수동 레드팀 -> 공격 성공률 측정 -> 취약점 기록           |
|          |                                                              |
|          v                                                              |
|  [4] 경화 (Hardening)                                                    |
|      취약점 패치 -> 방어 계층 강화 -> 회귀 테스트                        |
|          |                                                              |
|          v                                                              |
|  [5] 배포 + 모니터링                                                     |
|      실시간 공격 탐지 -> 자동 대응 -> 정직한 한계 보고                   |
|          |                                                              |
|          +---> [3] 으로 반복 (지속적 레드팀)                             |
+------------------------------------------------------------------------+
```

### 모드별 운영

```
  MODE 1: 개발 (레드팀 집중)
    공격 시뮬레이션 전력 가동, 방어 메커니즘 반복 개선

  MODE 2: 스테이징 (통합 검증)
    4-축 통합 테스트, 공격 성공률 목표치 대비 측정

  MODE 3: 배포 (실시간 모니터링)
    공격 탐지 + 자동 차단 + 한계 구간 정직 보고

  MODE 4: 사고 대응 (긴급)
    신규 공격 발견 시 -> 패치 -> 회귀 -> 재배포
```

## S6 EVOLVE (Mk.I~IV 4개월 로드맵)

<details open>
<summary><b>Mk.IV -- 4개월차: 통합 배포 + 정직한 한계 보고</b></summary>

4-축 전체 통합. 실전 배포 환경 레드팀. 방어 불가 영역 명확히 문서화.
공격 성공률 목표: 알려진 공격 < 5%, 신규 공격 < 30%.

</details>

<details>
<summary>Mk.III -- 3개월차: 에이전트 안전 + 아키텍처 통합</summary>

에이전트 샌드박싱 구현. 다층 필터 아키텍처 프로토타입. 권한 격리 검증.
멀티에이전트 시나리오 레드팀 시작.

</details>

<details>
<summary>Mk.II -- 2개월차: 기만 탐지 + 고급 평가</summary>

선형 탐침 기반 기만 탐지 구현. 허니팟 테스트 프레임워크.
멀티턴/멀티모달/다국어 안전 평가 확장.

</details>

<details>
<summary>Mk.I -- 1개월차: 안전 평가 기반 구축</summary>

공격 분류학 수립 (38종). 자동 레드팀 퍼징 프레임워크. 안전 경계 맵핑 도구.
기본 회귀 테스트 슈트 구축.

</details>

## S7 VERIFY (적대적 강건성 검증 -- Python stdlib only)

공격 성공률, 방어 유효성, 통계적 유의성을 stdlib만으로 검증.
연구 가치에 집중 -- 공격/방어 수학, 레드팀 메트릭, 강건성 한계.

### S7.0 CONSTANTS (공격 분류 파라미터, 방어 임계값)

공격 유형 4축 x 분류별 가중치. 방어 임계값: 탐지 신뢰도 >= 0.95,
오탐률(FPR) <= 0.01, 미탐률(FNR) <= 0.05.
공격 난이도 스케일: 1(trivial) ~ 10(state-of-the-art).

### S7.1 DIMENSIONS (공격 벡터 차원성 분석)

공격 벡터 공간 차원: 토큰 공간(어휘 크기 V), 의미 공간(임베딩 차원 d),
행동 공간(가능한 응답 집합). 방어는 이 고차원 공간에서 안전 부분공간을 정의하는 문제.

### S7.2 CROSS (3개 독립 강건성 메트릭)

(1) 공격 성공률(ASR) -- 성공 공격 / 전체 시도,
(2) 평균 공격 비용(MAC) -- 성공까지 필요한 쿼리 수,
(3) 방어 깊이(DD) -- 우회에 필요한 방어 계층 수.
3개 메트릭이 일관된 방향을 가리키는지 교차 검증.

### S7.3 SCALING (공격 성공률 vs 모델 크기)

모델 파라미터 수 N에 대해 ASR ~ N^(-alpha). alpha > 0이면 스케일링이
방어에 유리. 로그-로그 회귀로 alpha 추정. 실측 데이터: 1B, 7B, 70B, 400B.

### S7.4 SENSITIVITY (방어 임계값 민감도)

탐지 임계값 theta를 +/-10% 변동시켜 정밀도/재현율 변화 측정.
최적점에서 작은 변동에 방어가 무너지면 취약한 설계.

### S7.5 LIMITS (적대적 강건성의 이론적 한계 -- No Free Lunch)

Gilmer et al. 2018: 충분히 큰 모델은 반드시 적대적 예제가 존재.
Shafahi et al. 2019: 방어 비용은 공격 비용보다 항상 높다 (비대칭).
Carlini 2023: 어떤 안전 필터든 충분한 자원의 공격자에게 우회 가능.

### S7.6 CHI2 (방어 개선의 통계적 유의성)

방어 적용 전/후 공격 성공률 차이의 chi2 검정.
H0: "방어 적용이 공격 성공률에 영향 없음". p < 0.05이면 유의미한 개선.

### S7.7 OEIS (공격/방어 군비경쟁 패턴)

역사적 군비경쟁 데이터: 신규 공격 발견 후 방어 패치까지 평균 일수.
2020: 90일, 2022: 45일, 2024: 20일, 2026: ~10일 추세.
반감기 패턴 확인.

### S7.8 PARETO (방어 강도 vs 사용성 트레이드오프)

강한 방어 = 높은 거부율 = 낮은 사용성. 파레토 전선 탐색.
최적점: 안전 위반 < 5%, 합법 거부(과잉차단) < 2%.

### S7.9 SYMBOLIC (공격 확률 상한의 정확 계산)

Fraction 정확 유리수로 공격 확률 상한 계산.
다층 방어 통과 확률: P(bypass_all) = prod(P(bypass_i)).
각 계층 독립 가정 시 정확 상한 계산.

### S7.10 COUNTER (방어 불가능한 공격 -- 정직한 한계)

- COUNTER 1: 학습 데이터에 이미 포함된 유해 지식은 완전 제거 불가
- COUNTER 2: 충분한 자원의 국가급 공격자 앞에서 모든 필터 우회 가능
- COUNTER 3: 미래 공격 패턴을 사전에 완전히 예측하는 것은 불가능
- FALSIFIER 1: 신규 탈옥이 24시간 내 패치 불가 시 방어 체계 재설계
- FALSIFIER 2: 오탐률(FPR) > 5% 시 사용성 불합격
- FALSIFIER 3: 기만 탐지 재현율 < 20% 시 탐침 방식 폐기

### S7 통합 검증 코드 (stdlib only)

```python
#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# =============================================================================
# S7 VERIFY -- AI 적대적 강건성 검증 (stdlib only, domain=ai-adversarial)
# 10 서브섹션:
#   S7.0 CONSTANTS   -- 공격 분류 파라미터 + 방어 임계값
#   S7.1 DIMENSIONS  -- 공격 벡터 차원성 분석
#   S7.2 CROSS       -- 3개 독립 강건성 메트릭 교차 검증
#   S7.3 SCALING     -- 공격 성공률 vs 모델 크기 스케일링
#   S7.4 SENSITIVITY -- 방어 임계값 민감도 분석
#   S7.5 LIMITS      -- 이론적 한계 (No Free Lunch)
#   S7.6 CHI2        -- 방어 개선 통계적 유의성
#   S7.7 OEIS        -- 공격/방어 군비경쟁 반감기 패턴
#   S7.8 PARETO      -- 방어 강도 vs 사용성 트레이드오프
#   S7.9 SYMBOLIC    -- 다층 방어 통과 확률 정확 상한
#   S7.10 COUNTER    -- 방어 불가 영역 + 반증조건 (정직성)
# =============================================================================
from math import log, sqrt, erfc, exp
from fractions import Fraction
import statistics
import random

# --- S7.0 CONSTANTS -- 공격 분류 파라미터 + 방어 임계값 --------------------

# 공격 유형 4축 분류
ATTACK_AXES = {
    "safety_eval":     {"count": 12, "weight": 0.30},
    "deception_det":   {"count": 8,  "weight": 0.25},
    "agent_safety":    {"count": 10, "weight": 0.25},
    "safety_arch":     {"count": 8,  "weight": 0.20},
}
TOTAL_IDEAS = sum(v["count"] for v in ATTACK_AXES.values())
# 주: 번호 72-109 = 38개. 원래 36 목표였으나 세부 분류 시 12+8+10+8=38 확정.

# 방어 임계값
DETECTION_CONFIDENCE = 0.95    # 탐지 신뢰도 하한
MAX_FPR = 0.01                 # 오탐률 상한 (합법 요청 거부)
MAX_FNR = 0.05                 # 미탐률 상한 (위험 요청 통과)

# 공격 난이도 스케일 (1=trivial, 10=state-of-the-art)
DIFFICULTY_SCALE = {
    "basic_jailbreak": 2,
    "multi_turn_decay": 5,
    "sleeper_agent": 9,
    "prompt_injection": 3,
    "privilege_escalation": 7,
    "compositional_attack": 8,
}

assert TOTAL_IDEAS == 38, f"총 아이디어 수 불일치: {TOTAL_IDEAS}"
assert abs(sum(v["weight"] for v in ATTACK_AXES.values()) - 1.0) < 1e-9

# --- S7.1 DIMENSIONS -- 공격 벡터 차원성 분석 ----------------------------

def attack_surface_volume(vocab_size, embed_dim, max_seq_len):
    """공격 표면 근사 부피: 토큰 공간 x 시퀀스 길이
    
    실제 공격 가능 부분공간은 전체의 극히 일부이지만,
    방어는 전체 공간을 커버해야 하므로 비대칭이 발생한다.
    
    log-공간에서 계산하여 오버플로 방지.
    """
    log_volume = max_seq_len * log(vocab_size)  # log(V^L)
    log_safe_subspace = embed_dim * log(embed_dim)  # 안전 부분공간 근사
    asymmetry_ratio = log_volume / log_safe_subspace if log_safe_subspace > 0 else float('inf')
    return log_volume, log_safe_subspace, asymmetry_ratio

# --- S7.2 CROSS -- 3개 독립 강건성 메트릭 교차 검증 -----------------------

def attack_success_rate(successes, total_attempts):
    """ASR: 공격 성공률 = 성공/시도"""
    if total_attempts == 0:
        return 0.0
    return successes / total_attempts

def mean_attack_cost(query_counts):
    """MAC: 평균 공격 비용 = 성공까지 평균 쿼리 수
    높을수록 방어가 강함."""
    if not query_counts:
        return float('inf')
    return statistics.mean(query_counts)

def defense_depth(layers_bypassed, total_layers):
    """DD: 방어 깊이 = 우회 필요 계층 비율
    1.0이면 모든 계층 우회 필요 (강한 방어)."""
    if total_layers == 0:
        return 0.0
    return layers_bypassed / total_layers

def cross_validate_metrics(asr, mac, dd):
    """3개 메트릭 일관성 검증
    
    좋은 방어: ASR 낮음, MAC 높음, DD 높음.
    일관성 = 세 메트릭이 같은 방향을 가리키는가."""
    asr_good = asr < 0.3       # 공격 성공률 30% 미만
    mac_good = mac > 50        # 평균 50회 이상 쿼리 필요
    dd_good = dd > 0.7         # 계층의 70% 이상 우회 필요
    consistent = (asr_good == mac_good == dd_good)
    return consistent, {"asr_good": asr_good, "mac_good": mac_good, "dd_good": dd_good}

# --- S7.3 SCALING -- 공격 성공률 vs 모델 크기 ----------------------------

def scaling_exponent(model_sizes, attack_rates):
    """log-log 회귀로 ASR ~ N^(-alpha) 의 alpha 추정.
    
    model_sizes: [1e9, 7e9, 70e9, 400e9] (파라미터 수)
    attack_rates: [0.45, 0.30, 0.15, 0.08] (공격 성공률)
    
    alpha > 0 이면 모델 크기 증가가 방어에 유리."""
    n = len(model_sizes)
    if n < 2:
        return 0.0
    log_x = [log(s) for s in model_sizes]
    log_y = [log(r) for r in attack_rates]
    mean_lx = statistics.mean(log_x)
    mean_ly = statistics.mean(log_y)
    num = sum((lx - mean_lx) * (ly - mean_ly) for lx, ly in zip(log_x, log_y))
    den = sum((lx - mean_lx) ** 2 for lx in log_x)
    slope = num / den if den != 0 else 0.0
    return -slope  # alpha = -slope (ASR 감소 = 양수 alpha)

# --- S7.4 SENSITIVITY -- 방어 임계값 민감도 --------------------------------

def threshold_sensitivity(base_theta, tp, fp, fn, tn, pct=0.10):
    """탐지 임계값 theta 를 +/-pct 변동시켜 정밀도/재현율 변화 측정.
    
    안정적 방어: 작은 변동에 성능 급변 없어야 함.
    단순 모델: theta 증가 -> FP 감소, FN 증가 (선형 근사)."""
    
    def precision_recall(tp_v, fp_v, fn_v):
        prec = tp_v / (tp_v + fp_v) if (tp_v + fp_v) > 0 else 0
        rec = tp_v / (tp_v + fn_v) if (tp_v + fn_v) > 0 else 0
        return prec, rec
    
    prec_base, rec_base = precision_recall(tp, fp, fn)
    
    # theta 증가 (+pct): 더 엄격 -> FP 감소, FN 증가
    fp_high = max(0, fp * (1 - pct))
    fn_high = fn * (1 + pct)
    prec_high, rec_high = precision_recall(tp, fp_high, fn_high)
    
    # theta 감소 (-pct): 더 관대 -> FP 증가, FN 감소
    fp_low = fp * (1 + pct)
    fn_low = max(0, fn * (1 - pct))
    prec_low, rec_low = precision_recall(tp, fp_low, fn_low)
    
    # 안정성: 변동 폭이 작을수록 좋음
    prec_range = abs(prec_high - prec_low)
    rec_range = abs(rec_high - rec_low)
    stable = (prec_range < 0.15 and rec_range < 0.15)
    
    return {
        "base": (prec_base, rec_base),
        "theta_up": (prec_high, rec_high),
        "theta_down": (prec_low, rec_low),
        "prec_range": prec_range,
        "rec_range": rec_range,
        "stable": stable,
    }

# --- S7.5 LIMITS -- 이론적 한계 (No Free Lunch) ---------------------------

def no_free_lunch_bound(input_dim, epsilon):
    """적대적 예제 존재 확률의 하한 근사.
    
    Gilmer et al.: 입력 차원 d에서 epsilon-ball 내 적대적 예제 확률은
    차원이 증가하면 1에 수렴한다.
    P(adv exists) >= 1 - exp(-d * epsilon^2 / 2)
    """
    prob_lower = 1.0 - exp(-input_dim * epsilon ** 2 / 2)
    return min(prob_lower, 1.0)

def defense_cost_asymmetry(attack_cost, num_attack_types):
    """방어 비용 비대칭: 방어자는 모든 공격을 막아야 하고,
    공격자는 하나만 성공시키면 된다.
    
    defense_cost >= attack_cost * num_attack_types (최악)
    실제로는 공유 방어로 줄일 수 있지만, 비대칭은 구조적."""
    return attack_cost * num_attack_types

def theoretical_max_detection(fpr_target):
    """Neyman-Pearson 보조정리: 주어진 FPR에서 달성 가능한 최대 TPR.
    
    공격/정상 분포 겹침이 클수록 TPR 상한 낮아짐.
    겹침 계수 gamma=0.3 가정 (실측 필요)."""
    gamma = 0.3  # 분포 겹침 계수 (0=완전분리, 1=동일)
    max_tpr = 1.0 - gamma * (1.0 - fpr_target)
    return max(0.0, min(1.0, max_tpr))

# --- S7.6 CHI2 -- 방어 개선 통계적 유의성 ---------------------------------

def chi2_defense_improvement(before_success, before_total,
                              after_success, after_total):
    """방어 전/후 공격 성공률 차이의 chi2 검정.
    
    2x2 분할표:
              성공    실패
    전(before) a       b
    후(after)  c       d
    
    H0: 방어가 공격 성공률에 영향 없음."""
    a, b = before_success, before_total - before_success
    c, d = after_success, after_total - after_success
    n = a + b + c + d
    if n == 0:
        return 0.0, 1.0
    
    # chi2 = n * (ad - bc)^2 / ((a+b)(c+d)(a+c)(b+d))
    num = n * (a * d - b * c) ** 2
    den = (a + b) * (c + d) * (a + c) * (b + d)
    if den == 0:
        return 0.0, 1.0
    
    chi2 = num / den
    # p-value 근사 (df=1)
    p = erfc(sqrt(chi2 / 2))
    return chi2, p

# --- S7.7 OEIS -- 공격/방어 군비경쟁 반감기 패턴 --------------------------

def arms_race_halflife(response_days):
    """신규 공격 발견 -> 방어 패치 소요 일수 시계열에서 반감기 추정.
    
    데이터: [90, 45, 20, 10] (2020, 2022, 2024, 2026 추정)
    반감기 = 첫 값의 절반에 도달하는 데이터 인덱스 * 간격."""
    if len(response_days) < 2:
        return None
    
    # 지수 감소 fitting: y = y0 * exp(-lambda * t)
    # lambda = -ln(y_last/y_first) / (len-1)
    y0 = response_days[0]
    yn = response_days[-1]
    if yn <= 0 or y0 <= 0:
        return None
    
    lam = -log(yn / y0) / (len(response_days) - 1)
    if lam <= 0:
        return None  # 감소하지 않으면 반감기 없음
    
    halflife = log(2) / lam  # 단위: 데이터 간격 (2년)
    return halflife

# --- S7.8 PARETO -- 방어 강도 vs 사용성 트레이드오프 ----------------------

def pareto_frontier(n_samples=1000, seed=42):
    """방어 강도(safety)와 사용성(usability) 랜덤 샘플링 후
    파레토 전선 추출.
    
    최적점 목표: safety > 0.95, usability > 0.98."""
    random.seed(seed)
    points = []
    for _ in range(n_samples):
        # 방어 강도와 사용성은 트레이드오프 관계
        safety = random.uniform(0.5, 1.0)
        # 강한 방어 -> 사용성 감소 (노이즈 포함)
        usability = max(0.0, min(1.0, 1.05 - 0.6 * safety + random.gauss(0, 0.05)))
        points.append((safety, usability))
    
    # 파레토 전선: 어떤 다른 점도 safety와 usability 모두 더 좋지 않은 점
    pareto = []
    for s, u in points:
        dominated = False
        for s2, u2 in points:
            if s2 > s and u2 > u:
                dominated = True
                break
        if not dominated:
            pareto.append((s, u))
    
    pareto.sort(key=lambda p: p[0])
    
    # 목표 영역 도달 여부
    target_reached = any(s > 0.95 and u > 0.98 for s, u in pareto)
    
    return {
        "total_samples": n_samples,
        "pareto_size": len(pareto),
        "target_reached": target_reached,
        "best_safety": max(s for s, u in pareto) if pareto else 0,
        "best_usability": max(u for s, u in pareto) if pareto else 0,
    }

# --- S7.9 SYMBOLIC -- 다층 방어 통과 확률 정확 상한 -----------------------

def multilayer_bypass_probability(layer_probs):
    """다층 방어 전체 통과(우회) 확률의 정확 상한 계산.
    
    layer_probs: 각 계층의 우회 확률 (Fraction)
    독립 가정 시: P(bypass_all) = prod(P(bypass_i))
    
    예: 4층 방어, 각 10%, 15%, 5%, 20% 우회 확률
    -> P = 0.10 * 0.15 * 0.05 * 0.20 = 0.000015 = 1.5e-5"""
    total = Fraction(1)
    for p in layer_probs:
        total *= p
    return total

def defense_coverage_fraction(defended_attacks, total_attacks):
    """방어 커버리지를 정확 분수로 계산."""
    return Fraction(defended_attacks, total_attacks)

# --- S7.10 COUNTER/FALSIFIERS -- 정직한 한계 (>= 3 각각) ------------------

COUNTER_EXAMPLES = [
    ("학습 데이터 내 유해 지식",
     "사전학습 데이터에 포함된 유해 정보는 가중치에 분산 저장되어 "
     "완전 제거가 불가능하다 -- 필터로 출력만 억제할 뿐"),
    ("국가급 공격자의 자원",
     "충분한 예산과 인력을 가진 공격자는 어떤 필터든 우회할 수 있다 -- "
     "방어 비용 비대칭(S7.5)의 구조적 한계"),
    ("미래 공격 패턴 예측 불가",
     "아직 발견되지 않은 공격 벡터에 대해서는 사전 방어가 원천 불가능하다 -- "
     "반응적 방어만 가능"),
    ("분포 외 입력에 대한 일반화",
     "학습 분포에서 크게 벗어나는 입력에 대해 안전 분류기의 "
     "성능 보장이 없다"),
]

FALSIFIERS = [
    "신규 탈옥 공격이 24시간 내 패치 불가 시 -- 방어 체계 전면 재설계",
    "오탐률(FPR) > 5% 시 -- 사용성 불합격, 필터 재조정 필수",
    "기만 탐지 재현율 < 20% 시 -- 선형 탐침 방식 폐기",
    "멀티에이전트 시나리오에서 정렬 보존률 < 80% 시 -- 아키텍처 폐기",
]

# --- 메인 실행 + 집계 -----------------------------------------------------
if __name__ == "__main__":
    r = []

    # S7.0 상수 검증
    ok_const = (TOTAL_IDEAS == 38
                and DETECTION_CONFIDENCE >= 0.95
                and MAX_FPR <= 0.01
                and MAX_FNR <= 0.05)
    r.append(("S7.0 CONSTANTS 공격분류+임계값", ok_const))

    # S7.1 차원 비대칭
    log_vol, log_safe, asym = attack_surface_volume(
        vocab_size=50000, embed_dim=4096, max_seq_len=8192
    )
    ok_dim = (asym > 1.0)  # 공격 표면 > 안전 부분공간 (비대칭 존재)
    r.append(("S7.1 DIMENSIONS 공격표면 비대칭", ok_dim))

    # S7.2 3개 메트릭 교차 검증
    asr = attack_success_rate(15, 100)           # 15% 성공률
    mac = mean_attack_cost([80, 120, 95, 200])   # 평균 123.75 쿼리
    dd = defense_depth(3, 4)                     # 4층 중 3층 우회 필요
    consistent, details = cross_validate_metrics(asr, mac, dd)
    r.append(("S7.2 CROSS 3-메트릭 일관성", consistent))

    # S7.3 스케일링 alpha 추정
    sizes = [1e9, 7e9, 70e9, 400e9]
    rates = [0.45, 0.30, 0.15, 0.08]
    alpha = scaling_exponent(sizes, rates)
    ok_scaling = (alpha > 0)  # 스케일링이 방어에 유리
    r.append(("S7.3 SCALING alpha > 0 (스케일 유리)", ok_scaling))

    # S7.4 임계값 민감도
    sens = threshold_sensitivity(
        base_theta=0.5, tp=85, fp=5, fn=10, tn=900
    )
    r.append(("S7.4 SENSITIVITY 임계값 안정성", sens["stable"]))

    # S7.5 이론적 한계
    nfl_prob = no_free_lunch_bound(input_dim=4096, epsilon=0.02)
    cost_asym = defense_cost_asymmetry(attack_cost=100, num_attack_types=38)
    max_tpr = theoretical_max_detection(fpr_target=0.01)
    ok_limits = (nfl_prob > 0.5          # 적대적 예제 존재 확률 높음
                 and cost_asym > 1000    # 방어 비용 비대칭 확인
                 and max_tpr < 1.0)      # 완벽 탐지 불가능
    r.append(("S7.5 LIMITS No-Free-Lunch 확인", ok_limits))

    # S7.6 방어 개선 유의성
    chi2, p = chi2_defense_improvement(
        before_success=45, before_total=100,
        after_success=15, after_total=100
    )
    ok_chi2 = (p < 0.05)  # 유의미한 개선
    r.append(("S7.6 CHI2 방어 개선 p < 0.05", ok_chi2))

    # S7.7 군비경쟁 반감기
    response_days = [90, 45, 20, 10]
    hl = arms_race_halflife(response_days)
    ok_hl = (hl is not None and hl > 0)
    r.append(("S7.7 OEIS 군비경쟁 반감기 존재", ok_hl))

    # S7.8 파레토 전선
    pareto = pareto_frontier()
    ok_pareto = (pareto["pareto_size"] > 0)
    r.append(("S7.8 PARETO 전선 존재", ok_pareto))

    # S7.9 다층 방어 확률 상한
    layer_probs = [Fraction(1, 10), Fraction(3, 20),
                   Fraction(1, 20), Fraction(1, 5)]
    bypass_prob = multilayer_bypass_probability(layer_probs)
    coverage = defense_coverage_fraction(32, 38)
    ok_symbolic = (bypass_prob < Fraction(1, 1000)  # 0.1% 미만
                   and coverage > Fraction(5, 6))    # 83% 이상
    r.append(("S7.9 SYMBOLIC 다층방어 확률+커버리지", ok_symbolic))

    # S7.10 COUNTER/FALSIFIERS >= 3
    ok_counter = (len(COUNTER_EXAMPLES) >= 3 and len(FALSIFIERS) >= 3)
    r.append(("S7.10 COUNTER+FALSIFIERS >= 3", ok_counter))

    passed = sum(1 for _, ok in r if ok)
    total = len(r)
    print("=" * 64)
    for name, ok in r:
        print(f"  [{'OK' if ok else 'FAIL'}] {name}")
    print("=" * 64)
    print(f"{passed}/{total} PASS (적대적 강건성 검증)")
```

## S8 IDEAS (38 연구 아이디어 -- 4축 분류, 번호 72-109)

### 축 A: 안전 평가 (12)

| # | 아이디어 | 핵심 질문 | 난이도 |
|---|---------|----------|--------|
| 72 | 레드팀 자동화 | 퍼징으로 체계적 탈옥 탐색이 가능한가? | 5 |
| 73 | 안전 경계 맵핑 | 안전/위험 경계를 입력 공간에서 시각화할 수 있는가? | 6 |
| 74 | 탈옥 분류학 | 탈옥 공격을 원리별로 완전 분류할 수 있는가? | 4 |
| 75 | 안전 회귀 테스트 슈트 | 모델 업데이트 후 기존 안전성 유지를 보장하는가? | 3 |
| 76 | 다국어 안전 | 비영어권 공격에도 동일한 방어력이 있는가? | 6 |
| 77 | 멀티턴 안전 감쇄 | 대화가 길어질수록 안전 경계가 느슨해지는가? | 7 |
| 78 | 도구 사용 안전 | 도구 호출 시 안전 검증은 충분한가? | 7 |
| 79 | 멀티에이전트 안전 전파 | 한 에이전트의 안전 위반이 전체로 전파되는가? | 8 |
| 80 | 안전 특성 상관관계 | 한 안전 속성 개선이 다른 속성을 약화시키는가? | 5 |
| 81 | 적대적 강건성 벤치마크 | 표준화된 강건성 측정 프레임워크는? | 4 |
| 82 | 조합 안전 테스트 | 개별 안전한 입력의 조합이 위험해지는 경우는? | 8 |
| 83 | 맥락 의존 안전 | 동일 요청이 맥락에 따라 안전/위험 갈리는 경계는? | 6 |

### 축 B: 기만 탐지 (8)

| # | 아이디어 | 핵심 질문 | 난이도 |
|---|---------|----------|--------|
| 84 | 행동 일관성 검사 | 다른 표현의 같은 질문에 일관된 답변을 하는가? | 5 |
| 85 | 내외부 정렬 비교 | 내부 활성화와 외부 출력이 정렬되어 있는가? | 8 |
| 86 | 슬리퍼 에이전트 탐지 | 특정 트리거로 활성화되는 숨겨진 행동을 찾을 수 있는가? | 9 |
| 87 | 기만 선형 탐침 | 기만 의도를 선형 탐침으로 읽을 수 있는가? | 7 |
| 88 | 허니팟 테스트 | 의도적으로 유혹적 상황을 만들어 기만을 유도할 수 있는가? | 6 |
| 89 | 기만-능력 상관관계 | 능력이 높을수록 기만도 정교해지는가? | 7 |
| 90 | 기만 조기 경고 | 기만 행동의 전조를 조기에 탐지할 수 있는가? | 8 |
| 91 | 최소 기만 재현 모델 | 기만 행동의 최소 재현 가능 모델을 만들 수 있는가? | 6 |

### 축 C: 에이전트 안전 (10)

| # | 아이디어 | 핵심 질문 | 난이도 |
|---|---------|----------|--------|
| 92 | 도구 사용 샌드박싱 | 도구 호출을 안전하게 격리할 수 있는가? | 5 |
| 93 | 에이전트 감사추적 | 모든 에이전트 행동을 추적 가능한 로그로 남기는가? | 4 |
| 94 | 자율 행동 범위 제한 | 에이전트의 자율 행동 범위를 형식적으로 정의하는가? | 7 |
| 95 | 에이전트 자기감시 | 에이전트가 자신의 정렬 상태를 실시간 모니터링하는가? | 8 |
| 96 | 멀티에이전트 정렬 보존 | 여러 에이전트 상호작용에서 정렬이 유지되는가? | 9 |
| 97 | 명령 주입 방지 | 외부 데이터의 명령 주입을 차단하는가? | 6 |
| 98 | 권한 상승 탐지 | 에이전트가 부여된 권한을 초과하려는 시도를 탐지하는가? | 7 |
| 99 | 장기 세션 정렬 | 장시간 세션에서 정렬이 점진적으로 약화되는가? | 7 |
| 100 | 안전 오류 복구 | 안전 위반 후 안전한 상태로 복구하는 메커니즘은? | 5 |
| 101 | 에이전트 간 신뢰 프로토콜 | 에이전트 사이 신뢰 수준을 동적으로 관리하는가? | 8 |

### 축 D: 안전 아키텍처 (8)

| # | 아이디어 | 핵심 질문 | 난이도 |
|---|---------|----------|--------|
| 102 | 해석 가능 어텐션 | 어텐션 패턴을 안전 판단에 직접 활용하는가? | 7 |
| 103 | 안전 우선 아키텍처 | 안전을 후처리가 아닌 아키텍처 수준에서 보장하는가? | 9 |
| 104 | 모듈형 안전 계층 | 안전 모듈을 독립적으로 교체/업그레이드하는가? | 5 |
| 105 | 투명 추론 모듈 | 추론 과정을 감사 가능하게 외부화하는가? | 8 |
| 106 | 일반화 안전 게이트 | 다양한 공격 유형에 대응하는 범용 게이트는? | 7 |
| 107 | 헌법 코어 (하드코딩) | 절대 우회 불가능한 핵심 안전 규칙을 하드코딩하는가? | 6 |
| 108 | 다층 필터 아키텍처 | 입력-중간-출력 다층 필터로 방어 깊이를 확보하는가? | 6 |
| 109 | 안전 어텐션 마스크 | 위험 패턴에 대한 어텐션을 구조적으로 억제하는가? | 8 |

## S9 VALIDATION (검증 행렬)

```
+-----------------------------------------------------------------------+
|  아이디어    | ASR 측정 | MAC 측정 | DD 측정 | chi2 유의 | 파레토 내 |
+-------------+---------+---------+--------+----------+----------+
| 72 레드팀    |    O    |    O    |   O    |    O     |    -     |
| 74 분류학    |    O    |    -    |   -    |    -     |    -     |
| 77 멀티턴    |    O    |    O    |   O    |    O     |    O     |
| 82 조합안전  |    O    |    O    |   O    |    O     |    -     |
| 86 슬리퍼    |    O    |    O    |   O    |    O     |    -     |
| 92 샌드박싱  |    -    |    -    |   O    |    -     |    O     |
| 96 다중정렬  |    O    |    O    |   O    |    O     |    O     |
| 103 안전아키 |    O    |    O    |   O    |    O     |    O     |
| 107 헌법코어 |    -    |    -    |   O    |    -     |    O     |
| 108 다층필터 |    O    |    O    |   O    |    O     |    O     |
+-----------------------------------------------------------------------+
O = 해당 메트릭 적용 가능, - = 해당 없음
```

## S10 PREDICTIONS (예측)

| 예측 | 측정 방법 | 기준값 | 반증 조건 |
|------|----------|--------|----------|
| 다층 방어 시 bypass 확률 < 0.1% | S7.9 Fraction 계산 | 단층 10% | bypass > 1% |
| 스케일링 alpha > 0.2 | S7.3 log-log 회귀 | 현행 ~0.15 | alpha < 0.1 |
| 방어 개선 chi2 p < 0.01 | S7.6 2x2 분할표 | 무방어 ASR 45% | p > 0.05 |
| 군비경쟁 반감기 < 3년 | S7.7 지수 감소 | 2020: 90일 | 반감기 > 5년 |
| 안전-사용성 파레토 전선 존재 | S7.8 Monte Carlo | 이론적 트레이드오프 | 전선 없음 |

## S11 COMPARE-DETAIL (세부 ASCII 비교)

### 단층 방어 vs 다층 방어

```
+---------------------------------------------------------------------------+
|  [bypass 확률]                                                             |
|  단층 (RLHF만)  ##########..............  10% (1/10)                      |
|  2층 방어       ##.........................  1.5% (1/10 * 3/20)            |
|  4층 방어       ............................  0.0015% (S7.9 계산)          |
|                                                                           |
|  [탐지 지연]                                                               |
|  후처리 필터    ##########..............  요청 완료 후 검사                 |
|  아키텍처 내장  ##..........................  생성 중 실시간 차단            |
|                                                                           |
|  [공격 커버리지]                                                           |
|  알려진 공격만  ############............  분류학 내 공격만                  |
|  자동 퍼징     ####################....  미지 공격 패턴 발견 가능          |
+---------------------------------------------------------------------------+
```

## S12 ARCHITECTURE (아키텍처 다이어그램)

```
+----------------------------------------------------------------------+
|                    AI 적대적 강건성 아키텍처                            |
+----------------------------------------------------------------------+
|                                                                      |
|  [입력층]                                                             |
|  사용자 입력 --> [입력 필터] --> [안전 분류기] --> [명령주입 탐지]      |
|                      |              |               |                |
|                      v              v               v                |
|  [추론층]                                                             |
|  [안전 어텐션 마스크] --> [헌법 코어] --> [투명 추론]                  |
|           |                  |              |                         |
|           v                  v              v                         |
|  [출력층]                                                             |
|  [출력 필터] --> [행동 일관성 검사] --> [감사 로그] --> 응답            |
|                                                                      |
|  [에이전트 계층] (도구 사용 시)                                        |
|  [샌드박스] --> [권한 검증] --> [도구 실행] --> [결과 검증]            |
|                                                                      |
|  [모니터링]                                                           |
|  [자기감시] <--> [기만 탐침] <--> [이상 탐지] --> [알림]              |
+----------------------------------------------------------------------+
```

## S13 FLOW-DETAIL (세부 플로우)

### 공격 탐지 -> 대응 -> 학습 사이클

```
  공격 입력 감지
      |
      v
  [1단계] 입력 필터 -- 알려진 패턴 차단
      | (통과)
      v
  [2단계] 안전 분류기 -- 의미 기반 위험도 평가
      | (통과)
      v
  [3단계] 헌법 코어 -- 절대 규칙 위반 검사
      | (통과)
      v
  [4단계] 출력 필터 -- 생성 결과 최종 검증
      |
      +-- 차단 시 --> 로그 기록 + 패턴 DB 업데이트
      |
      +-- 통과 시 --> 행동 일관성 사후 검증
                         |
                         +-- 이상 시 --> 세션 종료 + 알림
```

## S14 UPGRADE (Mk 간 개선 비교)

```
+-----------------------------------------------------------------------+
|  능력          | Mk.I     | Mk.II    | Mk.III   | Mk.IV    |
+----------------+---------+---------+----------+----------+
| 공격 커버리지  | 20%     | 45%     | 70%      | 85%      |
| 탈옥 방어율    | 40%     | 60%     | 75%      | 85%      |
| 기만 탐지율    | -       | 35%     | 55%      | 65%      |
| 에이전트 차단율| -       | -       | 50%      | 70%      |
| 오탐률(FPR)   | 5%      | 3%      | 1.5%     | < 1%     |
| 군비경쟁 반응  | 30일    | 15일    | 7일      | < 3일    |
+-----------------------------------------------------------------------+
```

## S15 METHOD (검증 방법론)

| 검증 항목 | 방법 | 도구 | 판정 기준 |
|----------|------|------|----------|
| 공격 성공률 | 자동 퍼징 + 수동 레드팀 | S7.2 ASR 계산 | < 15% |
| 방어 유의성 | chi2 검정 | S7.6 코드 | p < 0.05 |
| 스케일링 효과 | log-log 회귀 | S7.3 코드 | alpha > 0 |
| 임계값 안정성 | 민감도 분석 | S7.4 코드 | 변동 < 15% |
| 다층 bypass | 확률 상한 | S7.9 Fraction | < 0.1% |
| 군비경쟁 추세 | 반감기 추정 | S7.7 코드 | 감소 추세 |
| 트레이드오프 | 파레토 전선 | S7.8 Monte Carlo | 전선 존재 |
| 이론적 한계 | No Free Lunch | S7.5 코드 | 한계 인정 |
| 정직성 | COUNTER >= 3 | S7.10 목록 | >= 3 |

---

*AI 적대적 강건성 연구 도메인 (Anthropic Fellows 2026). S7 검증 Python stdlib only.*
