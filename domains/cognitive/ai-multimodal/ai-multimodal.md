---
domain: ai-multimodal
requires:
  - to: ai-adversarial
  - to: ai-alignment
---
# 멀티모달 AI 안전성 연구 (Anthropic Fellows 2026)

## §1 WHY (왜 멀티모달 안전성인가)

AI가 텍스트+이미지+오디오를 동시 처리하면서 공격 표면이 기하급수적으로 확대된다.
이미지 속 프롬프트 인젝션, 오디오로 위장한 유해 명령, 모달 간 편향 불일치,
다중 식별정보(얼굴/음성/위치) 결합으로 인한 프라이버시 위험.
단일 모달리티 안전 기법은 교차 모달 공격에 무력하다.

| 문제 영역 | 현재 (2026) | 연구 목표 | 핵심 메트릭 |
|----------|-------------|----------|------------|
| 시각 인젝션 방어 | 탐지율 <60% | >95% | F1, FPR |
| 교차모달 일관성 | 불일치 빈번 | >90% | Cohen's kappa |
| 차등 프라이버시 | eps > 10 | eps < 1 | (eps, delta)-DP |
| 공정성 | 그룹 격차 >15% | <5% | Equalized Odds |
| PII 탐지 | 텍스트만 | 전 모달리티 | 재현율, 정밀도 |

## §2 COMPARE (텍스트 전용 vs 멀티모달) -- ASCII 비교

```
┌─────────────────────────────────────────────────────────────┐
│  [공격 표면]                                                 │
│  텍스트 전용  ████░░░░░░░░░░░░░░░░░░░░░   1 모달리티         │
│  멀티모달     ████████████████████████████  N 모달 x 교차     │
│  [인젝션 경로]                                               │
│  텍스트 전용  ██████░░░░░░░░░░░░░░░░░░░░   직접 텍스트만      │
│  멀티모달     ████████████████████████████  텍스트+이미지+오디오│
│  [PII 유형]                                                  │
│  텍스트 전용  ████████░░░░░░░░░░░░░░░░░░   이름/이메일/주소    │
│  멀티모달     ████████████████████████████  +얼굴/음성/위치    │
│  [방어 복잡도]                                               │
│  텍스트 전용  ████░░░░░░░░░░░░░░░░░░░░░░   O(V)              │
│  멀티모달     ████████████████████████████  O(V * P * A)      │
└─────────────────────────────────────────────────────────────┘
```

## §3 REQUIRES (선행 지식)

- **비전/오디오**: CNN/ViT, 스펙트로그램, 적대적 예제
- **차등 프라이버시**: (eps, delta)-DP, 민감도, 합성 정리
- **공정성 메트릭**: Demographic Parity, Equalized Odds, Calibration
- **해석가능성**: SAE, 회로 분석
- **의존 도메인**: `ai-adversarial`, `ai-alignment`
- **도구**: Python stdlib만 (math, statistics, random, fractions)

## §4 STRUCT (3축 20개 아이디어)

```
[축 1] 멀티모달 안전성 ─── 8개
  MS-1 시각 프롬프트 인젝션 방어    MS-5 멀티모달 SAE (공유 특성)
  MS-2 시각-텍스트 안전 일관성      MS-6 멀티모달 탈옥 방어
  MS-3 오디오 안전 필터             MS-7 교차 모달 안전 전이
  MS-4 멀티모달 환각 탐지           MS-8 NSFW 회로 매핑

[축 2] 프라이버시 보존 ─── 6개
  PP-1 PII 특성 탐지                PP-4 학습 데이터 추출 방지
  PP-2 차등 프라이버시 추론          PP-5 프라이버시 보존 SAE
  PP-3 선택적 비학습                PP-6 출력 익명화 필터

[축 3] 공정성/편향 ─── 6개
  FB-1 편향 특성 매핑               FB-4 다문화 공정성 벤치마크
  FB-2 공정성 회로 탐지             FB-5 교차 편향 분석
  FB-3 인과적 편향 교정             FB-6 공정성-성능 파레토
```

## §5 FLOW (연구 흐름)

```
교차 모달 공격 분석 ──→ 방어 설계 ──→ 안전 일관성 검증
프라이버시 위협 모델 ──→ DP 메커니즘 ──→ 예산 추적
                    └──→ 공정성 감사 통합 ──→ 벤치마크 + 논문
```

## §6 EVOLVE (4개월 로드맵)

- **월 1**: MS-1~3 인젝션/일관성/오디오 프로토타입
- **월 2**: MS-4~6 환각/탈옥 + PP-1~2 PII/DP 구현
- **월 3**: PP-3~4 비학습/추출방지 + FB-1~3 편향 매핑/회로/교정
- **월 4**: FB-4~6 벤치마크 + PP-5~6 SAE/익명화 + 통합 논문

## §7 VERIFY (검증)

### §7.0 CONSTANTS (핵심 상수)

```python
import math
EPSILON_STRONG = 1.0;  DELTA = 1e-5
DP_THRESH = 0.05; EO_THRESH = 0.05; CAL_THRESH = 0.03
GAUSSIAN_C = math.sqrt(2 * math.log(1.25 / DELTA))
print(f"가우시안 상수 c = {GAUSSIAN_C:.6f}")
print(f"DP 임계: {DP_THRESH}, EO 임계: {EO_THRESH}, 보정 임계: {CAL_THRESH}")
```

### §7.1 DIMENSIONS (프라이버시 예산 합성)

```python
import math
def seq_comp(epsilons): return sum(epsilons)
def par_comp(epsilons): return max(epsilons)
def adv_comp(eps, k, dp):
    return (math.sqrt(2*k*math.log(1/dp))*eps + k*eps*(math.exp(eps)-1))

# eps 작고 k 클 때 고급 합성이 순차보다 tight (Dwork 2010)
eps, k, dp = 0.01, 100, 1e-5
s = seq_comp([eps]*k)
p = par_comp([eps]*k)
a = adv_comp(eps, k, dp)
print(f"eps={eps}, k={k}: 순차={s:.4f}, 고급={a:.4f}, 병렬={p:.4f}")
assert s >= a >= p, "합성 정리 위반!"
print(f"검증: {s:.2f} >= {a:.2f} >= {p:.2f} PASS")
print(f"고급 합성 절감률: {(1-a/s)*100:.1f}%")
```

### §7.2 CROSS (3대 공정성 메트릭 독립 검증)

Chouldechova (2017): 기본 비율 상이 시 Calibration + Equal FPR + Equal FNR 동시 불가.

```python
import random; random.seed(42)
n = 200
y_true_a0 = [1 if random.random()<0.3 else 0 for _ in range(n)]
y_true_a1 = [1 if random.random()<0.5 else 0 for _ in range(n)]
y_pred_a0 = [1 if random.random()<0.25 else 0 for _ in range(n)]
y_pred_a1 = [1 if random.random()<0.55 else 0 for _ in range(n)]

def rates(yt, yp):
    tp=sum(t==1 and p==1 for t,p in zip(yt,yp))
    fn=sum(t==1 and p==0 for t,p in zip(yt,yp))
    fp=sum(t==0 and p==1 for t,p in zip(yt,yp))
    tn=sum(t==0 and p==0 for t,p in zip(yt,yp))
    return tp/(tp+fn) if tp+fn else 0, fp/(fp+tn) if fp+tn else 0

dp = abs(sum(y_pred_a0)/n - sum(y_pred_a1)/n)
tpr0,fpr0 = rates(y_true_a0, y_pred_a0)
tpr1,fpr1 = rates(y_true_a1, y_pred_a1)
print(f"Demographic Parity 차이: {dp:.4f}")
print(f"Equalized Odds TPR 차이: {abs(tpr0-tpr1):.4f}")
print(f"Equalized Odds FPR 차이: {abs(fpr0-fpr1):.4f}")
all_fair = dp<0.05 and abs(tpr0-tpr1)<0.05 and abs(fpr0-fpr1)<0.05
print(f"동시 만족: {'YES' if all_fair else 'NO -- 긴장 관계 확인'}")
```

### §7.3 SCALING (프라이버시 비용 스케일링)

```python
import math
eps = 0.01; dp = 1e-5  # 작은 eps에서 고급 합성 이점 극대화
print(f"{'k':>6} {'순차':>10} {'고급':>10} {'절감':>8}")
for k in [10, 50, 100, 500, 1000, 5000]:
    naive = k * eps
    adv = math.sqrt(2*k*math.log(1/dp))*eps + k*eps*(math.exp(eps)-1)
    print(f"{k:>6} {naive:>10.3f} {adv:>10.3f} {(1-adv/naive)*100:>7.1f}%")
# 순차: O(k) 선형, 고급: O(sqrt(k)) -- k 증가 시 절감 효과 증대
```

### §7.4 SENSITIVITY (공정성 임계값 민감도)

```python
import random; random.seed(42)
gaps = [abs(random.uniform(0.2,0.6) - random.uniform(0.2,0.6)) for _ in range(100)]
print(f"{'임계값':>8} {'공정률':>8}")
for t in [0.01, 0.05, 0.10, 0.15, 0.20]:
    pct = sum(g<t for g in gaps)
    print(f"{t:>8.2f} {pct:>7d}%")
# 결론: 임계값 선택이 판정을 크게 좌우 -- 맥락 의존적 결정 필요
```

### §7.5 LIMITS (불가능성 정리)

```python
import random; random.seed(123)
def impossibility(br_a, br_b, n=5000):
    ya = [1 if random.random()<br_a else 0 for _ in range(n)]
    yb = [1 if random.random()<br_b else 0 for _ in range(n)]
    pa = [1 if random.random()>0.5 else 0 for _ in range(n)]
    pb = [1 if random.random()>0.5 else 0 for _ in range(n)]
    def r(yt,yp):
        tp=sum(t==1 and p==1 for t,p in zip(yt,yp))
        fn=sum(t==1 and p==0 for t,p in zip(yt,yp))
        fp=sum(t==0 and p==1 for t,p in zip(yt,yp))
        tn=sum(t==0 and p==0 for t,p in zip(yt,yp))
        return (tp/(tp+fn) if tp+fn else 0, fp/(fp+tn) if fp+tn else 0,
                tp/(tp+fp) if tp+fp else 0)
    ra, rb = r(ya,pa), r(yb,pb)
    print(f"  기본비율 {br_a}/{br_b}: TPR차={abs(ra[0]-rb[0]):.3f} "
          f"FPR차={abs(ra[1]-rb[1]):.3f} PPV차={abs(ra[2]-rb[2]):.3f}")

print("=== Chouldechova 불가능성 실증 ===")
impossibility(0.3, 0.3)   # 동일 -- 근접 가능
impossibility(0.2, 0.5)   # 상이 -- 동시 만족 불가
print("결론: 기본비율 상이 시 완벽한 공정성은 수학적으로 불가능")
```

### §7.6 CHI2 (편향 유의성 검정)

```python
import math
def chi2_test(obs, exp):
    chi2 = sum((o-e)**2/e for o,e in zip(obs,exp) if e>0)
    df = len(obs)-1
    z = ((chi2/df)**(1/3)-(1-2/(9*df)))/math.sqrt(2/(9*df)) if df>0 else 0
    if z<0: return chi2, 1.0
    t=1/(1+0.2316419*z)
    p=t*(0.31938+t*(-0.35656+t*(1.78148+t*(-1.82126+t*1.33027))))
    return chi2, p*math.exp(-z*z/2)/math.sqrt(2*math.pi)

c1, p1 = chi2_test([48,52,50,50], [50,50,50,50])
c2, p2 = chi2_test([25,60,65,50], [50,50,50,50])
print(f"편향 없음: chi2={c1:.3f} p={p1:.4f} {'유의' if p1<0.05 else '무의미'}")
print(f"체계 편향: chi2={c2:.3f} p={p2:.4f} {'유의--교정필요' if p2<0.05 else '무의미'}")
```

### §7.7 OEIS (편향 분포 구조)

```python
import math, random; random.seed(42)
freqs = sorted([int(1000/(i+1)**1.2)+random.randint(0,20) for i in range(20)], reverse=True)
log_r = [math.log(i+1) for i in range(20)]
log_f = [math.log(max(v,1e-10)) for v in freqs]
mx, my = sum(log_r)/20, sum(log_f)/20
slope = sum((x-mx)*(y-my) for x,y in zip(log_r,log_f))/sum((x-mx)**2 for x in log_r)
ss_res = sum((y-(slope*x+(my-slope*mx)))**2 for x,y in zip(log_r,log_f))
ss_tot = sum((y-my)**2 for y in log_f)
r2 = 1-ss_res/ss_tot
print(f"Zipf 기울기: {slope:.3f} (이상: -1.0), R2={r2:.4f}")
print(f"편향 분포 Zipf {'적합' if r2>0.9 else '부분적합' if r2>0.7 else '부적합'}")
```

### §7.8 PARETO (프라이버시-유틸리티 트레이드오프)

```python
import math, random; random.seed(42)
def lap_noise(val, sens, eps):
    u = random.random()-0.5
    return val - (sens/eps)*math.copysign(1,u)*math.log(1-2*abs(u))
def acc_at_eps(vals, sens, eps, trials=30):
    errs = [abs(lap_noise(v,sens,eps)-v)/max(abs(v),1e-10)
            for v in vals for _ in range(trials)]
    return max(0, 1-sum(errs)/len(errs))

vals = [100,250,500,750,1000]
print(f"{'eps':>8} {'유틸리티':>10} {'1/eps':>8}")
for e in [0.01, 0.1, 0.5, 1.0, 2.0, 5.0, 10.0]:
    a = acc_at_eps(vals, 1.0, e)
    print(f"{e:>8.2f} {a:>10.4f} {1/e:>8.2f}")
print("결론: eps~1.0 부근이 실용적 파레토 최적점")
```

### §7.9 SYMBOLIC (메커니즘 정밀 계산)

```python
import math
from fractions import Fraction
print("[라플라스] b = Delta/eps, 분산 = 2b^2")
for s in [Fraction(1), Fraction(1,2)]:
    for e in [Fraction(1,10), Fraction(1), Fraction(2)]:
        b = s/e; var = 2*b*b
        print(f"  sens={s} eps={e} -> b={b} var={var}")

delta = 1e-5; c = math.sqrt(2*math.log(1.25/delta))
print(f"\n[가우시안] sigma = Delta*c/eps, c={c:.6f}")
for s in [1.0, 0.5]:
    for e in [0.1, 1.0, 2.0]:
        sig = s*c/e
        print(f"  sens={s} eps={e} -> sigma={sig:.4f} var={sig**2:.4f}")
print("핵심: eps 2배 -> sigma 1/2 -> 분산 1/4")
```

### §7.10 COUNTER (정직한 한계)

```
1. 프라이버시 vs 공정성: DP 노이즈가 소수 그룹에 불균형 영향 (Cummings 2019)
2. 프라이버시 vs 유틸리티: eps->0이면 출력 무작위 (Duchi 2013 minimax 하한)
3. 공정성 불가능성: 기본비율 상이 시 3조건 동시 불가 (Chouldechova 2017)
4. 멀티모달 방어 한계: 교차 모달 공격 공간 사실상 무한, 새 모달 추가 시 재설계
5. 벤치마크 한계: 서구 중심, 문화별 "공정" 정의 상이, 교차 편향 데이터 부족
6. 비학습 순환: 완벽한 비학습 = 재학습 비용, 검증 자체가 프라이버시 침해 가능
```

## §8 IDEAS (20개 연구 아이디어 상세)

### 축 1: 멀티모달 안전성 (MS-1 ~ MS-8)

| ID | 제목 | 핵심 질문 | 난이도 |
|----|------|----------|--------|
| MS-1 | 시각 프롬프트 인젝션 방어 | 이미지 속 숨겨진 명령 탐지/차단 | 높음 |
| MS-2 | 시각-텍스트 안전 일관성 | 텍스트/이미지 안전 판정 일관성 | 중간 |
| MS-3 | 오디오 안전 필터 | 음성/음향 유해 콘텐츠 교차 탐지 | 중간 |
| MS-4 | 멀티모달 환각 탐지 | 이미지-텍스트 사실 불일치 탐지 | 높음 |
| MS-5 | 멀티모달 SAE | 텍스트/이미지 공유 안전 특성 추출 | 높음 |
| MS-6 | 멀티모달 탈옥 방어 | 모달 교차 탈옥 시도 방어 | 높음 |
| MS-7 | 교차 모달 안전 전이 | 한 모달 안전 학습의 타 모달 전이 | 중간 |
| MS-8 | NSFW 회로 매핑 | NSFW 탐지 내부 회로 식별 | 중간 |

### 축 2: 프라이버시 보존 (PP-1 ~ PP-6)

| ID | 제목 | 핵심 질문 | 난이도 |
|----|------|----------|--------|
| PP-1 | PII 특성 탐지 | 모델 내 PII 인코딩 특성 탐색 | 높음 |
| PP-2 | 차등 프라이버시 추론 | 추론 시 DP 적용 출력 보호 | 중간 |
| PP-3 | 선택적 비학습 | 특정 데이터 영향 효율적 제거 | 높음 |
| PP-4 | 학습 데이터 추출 방지 | 역추출 공격 방어 | 중간 |
| PP-5 | 프라이버시 보존 SAE | SAE 분석 시 개인정보 비노출 | 중간 |
| PP-6 | 출력 익명화 필터 | 식별 정보 자동 필터링 | 낮음 |

### 축 3: 공정성/편향 (FB-1 ~ FB-6)

| ID | 제목 | 핵심 질문 | 난이도 |
|----|------|----------|--------|
| FB-1 | 편향 특성 매핑 | SAE로 편향 관련 특성 추출 | 높음 |
| FB-2 | 공정성 회로 탐지 | 공정/불공정 판단 회로 식별 | 높음 |
| FB-3 | 인과적 편향 교정 | 인과 추론 기반 편향 원인 교정 | 높음 |
| FB-4 | 다문화 공정성 벤치마크 | 비서구 맥락 공정성 평가 체계 | 중간 |
| FB-5 | 교차 편향 분석 | 성별x인종x나이 복합 편향 측정 | 중간 |
| FB-6 | 공정성-성능 파레토 | 공정성 제약 하 최적 성능 경계 | 중간 |

## §9 METRICS (핵심 메트릭)

| 메트릭 | 목표 |
|--------|------|
| 시각 인젝션 F1 | > 0.95 |
| 교차 모달 Cohen's kappa | > 0.85 |
| (eps, delta)-DP | eps < 1.0 |
| Demographic Parity 차이 | < 0.05 |
| Equalized Odds 차이 | < 0.05 |
| Calibration 오차 | < 0.03 |

## §10 APPLICATIONS (응용)

1. **Claude 안전성 강화**: 멀티모달 입력의 교차 공격 방어
2. **프라이버시 규제 준수**: GDPR/CCPA 기술적 구현
3. **공정성 감사 도구**: 배포 전 다차원 자동 감사
4. **비학습 파이프라인**: 사용자 데이터 삭제 요청 이행
5. **다문화 AI 서비스**: 비서구권 공정 서비스 보장

## §11 RISKS (위험)

| 위험 | 심각도 | 완화 |
|------|--------|------|
| DP 노이즈 소수그룹 정확도 급락 | 높음 | 그룹별 예산 분배 최적화 |
| 공정성 메트릭 간 충돌 | 중간 | 불가능성 정리 기반 우선순위 |
| 새 모달리티 방어 무력화 | 높음 | 모달리티-불가지론적 설계 |
| 벤치마크 문화 편향 | 중간 | 다문화 전문가 + 현지화 검증 |

## §12 REFERENCES

- Chouldechova (2017) "Fair prediction with disparate impact"
- Kleinberg, Mullainathan, Raghavan (2016) "Inherent Trade-Offs in Fair Risk Scores"
- Dwork, Roth (2014) "Algorithmic Foundations of Differential Privacy"
- Cummings et al. (2019) "Compatibility of Privacy and Fairness"
- Duchi, Jordan, Wainwright (2013) "Local Privacy and Minimax Rates"
- Tsipras et al. (2019) "Robustness May Be at Odds with Accuracy"
- Carlini et al. (2021) "Extracting Training Data from LLMs"
- Bourtoule et al. (2021) "Machine Unlearning" (SISA)
- Bianchi et al. (2023) "Text-to-Image Amplifies Stereotypes"
- Goh et al. (2021) "Multimodal Neurons in ANNs"

## §13 GLOSSARY

| 용어 | 정의 |
|------|------|
| (eps, delta)-DP | 이웃 데이터셋 출력 차이 <= exp(eps), delta 확률 실패 허용 |
| Demographic Parity | 그룹 독립 양성 비율 동일 |
| Equalized Odds | TPR/FPR 전 그룹 동일 |
| Calibration | 예측 점수 = 실제 확률 |
| SAE | 희소 자동 인코더 (해석가능 특성 추출) |
| Machine Unlearning | 특정 데이터 영향 모델에서 제거 |
| PII | 개인식별정보 |

## §14 FAQ

**Q: 왜 안전성/프라이버시/공정성을 통합하나?**
A: 개별 최적화는 다른 축을 악화시킨다. DP가 소수그룹 정확도를 떨어뜨리고,
안전 필터가 특정 문화 표현을 과잉 차단한다. 통합만이 균형을 잡는다.

**Q: 불가능성 정리가 있는데 공정성을 왜 연구하나?**
A: 완벽한 공정성은 불가능하지만, 트레이드오프를 정량화하고
맥락별 최선을 제공하는 것이 연구 가치이다.

## §15 CHANGELOG

| 날짜 | 변경 |
|------|------|
| 2026-04-16 | 초판 -- 3축 20개 아이디어, 11개 검증 서브섹션 |
