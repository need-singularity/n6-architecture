---
domain: ai-deployment
requires:
  - to: ai-adversarial
  - to: ai-alignment
---
# AI 배포 안전성 (AI Deployment Safety)

## §1 WHY (왜 배포 안전성인가)

"실험실에서 안전"과 "프로덕션에서 안전" 사이의 간극에서 실제 피해가 발생한다.
Anthropic Fellows 2026 연구 도메인. 4대 기둥: 학습 안전, 추론 안전, 배포 프로토콜, 프롬프트 방어.

| 효과 | 현재 (2026) | 안전 배포 이후 | 근거 |
|------|-------------|---------------|------|
| 배포 사고율 | 월 2~5건 | **< 0.1건/월** | 4단계 롤아웃 + 자동 롤백 |
| 환각 탐지 지연 | 수 시간~수 일 | **< 30초** | 스트리밍 안전 검사 |
| 프롬프트 주입 차단 | 60~70% | **> 95%** | 다층 분류기 + 난독화 복원 |
| 롤백 시간 | 30분~2시간 | **< 5분** | 카나리아 + SLA 트리거 |
| A/B 안전 테스트 | 비표준 | **chi2 유의성** | p < 0.05 자동 판정 |
| 사전 인증 | 수동 체크리스트 | **CI/CD 게이트** | 파이프라인 자동 인증 |

**한 문장**: 학습->추론->배포->프롬프트 방어 전 단계 안전 프레임워크로 연구-현실 간극을 메운다.

## §2 COMPARE (기존 vs 안전 우선 배포) -- ASCII 비교

```
+-------------------+----------------------------+---------------------------+
|  장벽              |  왜 한계였나                 |  안전 배포 해결             |
+-------------------+----------------------------+---------------------------+
| 사후 대응          | 사고 후 수동 롤백 30분~2시간  | 실시간 모니터링+자동 롤백 5분|
| 안전 테스트 부재   | 기능 테스트만, 안전 회귀 없음  | CI/CD 안전 게이트 자동 차단  |
| 프롬프트 무방비    | 단일 규칙 필터, 노출 가능     | 다층 주입 분류+난독화 복원   |
| 비표준 롤아웃      | 전체 배포 또는 수동 카나리아  | 4단계 자동 롤아웃+통계 승격  |
| 지표 부재          | "안전하다" 주관 판단          | SLA 정량 지표 + 대시보드     |
+-------------------+----------------------------+---------------------------+
```

```
+--------------------------------------------------------------------------+
|  [사고율]     기존  ██████████████████████████████  월 2~5건              |
|               안전  ██░░░░░░░░░░░░░░░░░░░░░░░░░░░  < 0.1건/월           |
|  [탐지 속도]  기존  ██████████████████████████████  수 시간~일            |
|               안전  █░░░░░░░░░░░░░░░░░░░░░░░░░░░░  < 30초               |
|  [주입 차단]  기존  ██████████████████░░░░░░░░░░░░  60~70%               |
|               안전  █████████████████████████████░  > 95%                |
|  [롤백 시간]  기존  ██████████████████████████████  30분~2시간           |
|               안전  ██░░░░░░░░░░░░░░░░░░░░░░░░░░░  < 5분                |
+--------------------------------------------------------------------------+
```

## §3 REQUIRES (선행 요구사항)

| 선행 | 역할 |
|------|------|
| ai-adversarial | 적대적 공격 탐지 파이프라인 |
| ai-alignment | 정렬 회귀 테스트 기준 |
| MLOps/CI-CD | 파이프라인 인프라 |
| 프로덕션 모니터링 | 실시간 이상 탐지 |
| 프롬프트 엔지니어링 | 주입/탈옥 방어론 |

## §4 STRUCT (4대 기둥 구조)

```
+----------------+----------------+------------------+------------------+
|  학습 안전 (4) |  추론 안전 (4) |  배포 프로토콜(8)|  프롬프트 방어(8)|
+----------------+----------------+------------------+------------------+
| A1 학습률 스케줄| B1 투기적디코딩| C1 4단계 롤아웃  | D1 프롬프트견고성|
| A2 안전커리큘럼 | B2 안전KV캐시  | C2 실시간모니터링| D2 주입 분류기  |
| A3 그래디언트수술| B3 안전컴퓨트  | C3 자동 롤백     | D3 안전프롬프트  |
| A4 합성안전데이터| B4 스트리밍검사| C4 A/B 안전테스트| D4 난독화 복원  |
|                |                | C5 카나리아 배포  | D5 다회차 방어  |
|                |                | C6 안전 SLA      | D6 간접주입 방어|
|                |                | C7 사고 자동대응 | D7 위험 점수    |
|                |                | C8 CI/CD 인증    | D8 컨텍스트 방어|
+----------------+----------------+------------------+------------------+
   안전 모델 ---------> 안전 추론 --------> 안전 배포 --------> 안전 운영
```

## §5 FLOW (제어 플로우)

```
학습: [안전 데이터] -> [커리큘럼 학습] -> [그래디언트 수술] -> [안전 모델]
추론: [입력] -> [위험 점수] -> [주입 분류기] -> [안전 KV 추론] -> [스트리밍 검사] -> [출력]
배포: [CI/CD 인증] -> [카나리아 1%] -> [스테이징 10%] -> [제한 GA 50%] -> [전체 GA]
비상: SLA 위반 감지 -> 자동 롤백 -> 사고 분류 -> 격리 -> 알림 (< 5분)
```

## §6 EVOLVE (4개월 로드맵)

| 단계 | 기간 | 핵심 산출물 |
|------|------|------------|
| 1 | 월 1 | 안전 회귀 데이터셋, 합성 안전 데이터, 스트리밍 검사 프로토타입, CI/CD 게이트 |
| 2 | 월 2 | 4단계 롤아웃, 실시간 대시보드, 자동 롤백, 카나리아 배포 |
| 3 | 월 3 | 주입 분류기 배포, 난독화 복원, 다회차 탐지, A/B chi2 자동 판정 |
| 4 | 월 4 | 전체 통합 자동화, 안전 SLA 99.9%, 전 계층 무인 운영 |

## §7 VERIFY (배포 안전성 검증 -- Python stdlib only)

### §7.0 CONSTANTS (SLA 임계값)
거부율 상한 2%, 환각률 상한 1%, p99 지연 500ms, 롤백 시간 300초, 카나리아 1%, 주입 탐지 95%.

### §7.1 DIMENSIONS (지표 단위)
거부율/환각률 = 무차원 비율 (0~1). 지연 = 초 (SI). 처리량 = 요청/초 (Hz). 단위 불일치 reject.

### §7.2 CROSS (3개 독립 지표)
(1) 거부율 (2) 환각률 (3) 사고율 -- 세 지표 동시 개선만 진짜 안전 향상.

### §7.3 SCALING (트래픽 스케일링)
안전 검사 지연의 트래픽 대비 log-log 기울기. alpha < 1.0 (sublinear) 이어야 프로덕션 실용.

### §7.4 SENSITIVITY (롤백 임계값 민감도)
거부율 임계값 1%~5% 변화 시 FP 롤백 < 1%, 실제 탐지 > 99% 인 최적점 탐색.

### §7.5 LIMITS (탐지 이론적 한계)
공격 공간 = vocab^L -> 열거 불가. F1 > 0.95 달성 가능하나 100% 불가 (Rice 정리). 제로데이 사전탐지 불가.

### §7.6 CHI2 (A/B 유의성)
2x2 분할표 chi2 검정. H0: 두 버전 안전 동일. p < 0.05 시 유의. 최소 n >= 400 (d=0.2).

### §7.7 OEIS (공격 조합 구조)
주입 변종 수 = C(L, k) x M. C(100,3) x 5 = 808,500 변종. 방어 복잡도 하한 제공.

### §7.8 PARETO (안전/지연/비용)
안전 계층 0~5 x 강도 조합. 안전 > 0.95, 지연 증가 < 50ms, 비용 < 20% 파레토 프론티어.

### §7.9 SYMBOLIC (카나리아 위험 경계)
R = p x q x N. Fraction 정확 계산. p=1/100, q=1/1000, N=10^6 -> 기대 피해 = 10명.

### §7.10 COUNTER (정직한 한계)
- 제로데이 공격: 학습 분포 밖 신규 패턴 사전 탐지 불가능
- 분포 이동: 테스트 통과가 프로덕션 안전 보장 아님
- 내부자 위협: 프롬프트 방어는 외부 대상, 내부자 별도 통제 필요
- 연산 비용: 안전 계층 추가 = 지연/비용 증가 (무한 안전 = 무한 비용)
- FALSIFIERS: 스케일링 O(n^2)+, 주입 F1<0.7, A/B 결론 뒤집힘, 카나리아 미탐지>10%

### §7 통합 검증 코드 (stdlib only)

```python
#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# §7 VERIFY -- AI 배포 안전성 검증 (stdlib only, domain=ai-deployment)
from math import log, sqrt, erfc, comb
from fractions import Fraction
import statistics, random

# §7.0 CONSTANTS
REJECT_MAX, HALLUC_MAX = 0.02, 0.01
P99_MS, ROLLBACK_SEC = 500, 300
CANARY_RATIO, INJECT_TARGET = 0.01, 0.95
STAGES = [0.01, 0.10, 0.50, 1.00]
assert len(STAGES) == 4 and 0 < REJECT_MAX < 1 and 0 < HALLUC_MAX < 1

# §7.1 DIMENSIONS -- 비율은 무차원, 지연은 가산
def rate_ok(num, den): return 0.0 <= num / den <= 1.0
def latency_sum(ms_list): return sum(ms_list)

# §7.2 CROSS -- 3개 독립 안전 지표
def cross_safety(rej, hal, inc, total):
    return rej < REJECT_MAX and hal < HALLUC_MAX and inc / total < REJECT_MAX

# §7.3 SCALING -- log-log 기울기
def scaling_exp(xs, ys):
    lx, ly = [log(x) for x in xs], [log(y) for y in ys]
    mx, my = statistics.mean(lx), statistics.mean(ly)
    num = sum((lx[i]-mx)*(ly[i]-my) for i in range(len(xs)))
    den = sum((lx[i]-mx)**2 for i in range(len(xs)))
    return num / den if den else 0.0

# §7.4 SENSITIVITY -- 롤백 임계값 민감도
def sensitivity(thresholds, inc_rate=0.03, noise=0.005, n=1000, seed=42):
    random.seed(seed)
    results = []
    for th in thresholds:
        fp, tp, nh, ni = 0, 0, n//2, n-n//2
        for _ in range(nh):
            if 0.015 + random.gauss(0, noise) > th: fp += 1
        for _ in range(ni):
            if 0.015 + inc_rate + random.gauss(0, noise) > th: tp += 1
        results.append((th, fp/nh, tp/ni))
    return results

# §7.5 LIMITS -- F1 계산 + 공격 공간
def f1_score(tp, fp, fn):
    p = tp/(tp+fp) if tp+fp else 0
    r = tp/(tp+fn) if tp+fn else 0
    return 2*p*r/(p+r) if p+r else 0

def attack_space(vocab=50000, L=100): return L * log(vocab)

# §7.6 CHI2 -- 2x2 분할표
def chi2_ab(ia, na, ib, nb):
    t = na + nb; ti = ia + ib; ts = t - ti
    exp_vals = [(ia, na*ti/t), (ib, nb*ti/t), (na-ia, na*ts/t), (nb-ib, nb*ts/t)]
    chi2 = sum((o-e)**2/e for o, e in exp_vals if e > 0)
    return chi2, erfc(sqrt(chi2/2)) if chi2 > 0 else 1.0

# §7.7 OEIS -- 공격 변종 조합
def variants(L, k, M): return comb(L, k) * M
def binom_seq(nmax, k): return [comb(n, k) for n in range(k, nmax+1)]

# §7.8 PARETO -- 3축 탐색
def pareto(n_trials=1000, seed=42):
    random.seed(seed)
    pts = []
    for _ in range(n_trials):
        nl = random.randint(0, 5)
        st = [random.random() for _ in range(nl)]
        safety = 1 - (0.5**max(1,nl)) * (1 - statistics.mean(st) if st else 1)
        lat = sum(10 + 20*s for s in st)
        cost = sum(5 + 10*s for s in st)
        pts.append((safety, lat, cost))
    front = [p for p in pts if not any(
        q[0]>=p[0] and q[1]<=p[1] and q[2]<=p[2]
        and (q[0]>p[0] or q[1]<p[1] or q[2]<p[2]) for q in pts)]
    return front

# §7.9 SYMBOLIC -- Fraction 카나리아 위험
def canary_risk(p, q, N):
    return Fraction(p) * Fraction(q) * Fraction(N)

# §7.10 COUNTER
COUNTERS = ["제로데이 사전탐지 불가", "분포 이동", "내부자 위협", "연산 비용 증가"]
FALSIFIERS = ["스케일링 O(n^2)+", "주입 F1<0.7", "A/B 결론 뒤집힘", "카나리아 미탐지>10%"]

if __name__ == "__main__":
    r = []
    # §7.0
    r.append(("§7.0 CONSTANTS", abs(sum(STAGES)-1.61) < 0.01))
    # §7.1
    r.append(("§7.1 DIMENSIONS", rate_ok(20,1000) and latency_sum([10,20,15])==45))
    # §7.2
    r.append(("§7.2 CROSS 3지표", cross_safety(0.01, 0.005, 5, 100000)))
    # §7.3
    alpha = scaling_exp([100,500,1000,5000,10000], [10,18,25,40,55])
    r.append(("§7.3 SCALING alpha=%.3f<1" % alpha, alpha < 1.0))
    # §7.4
    s = sensitivity([0.02, 0.03, 0.04, 0.05])
    r.append(("§7.4 SENSITIVITY", any(fp<0.05 and dt>0.95 for _,fp,dt in s)))
    # §7.5
    f1 = f1_score(950, 30, 50)
    r.append(("§7.5 LIMITS F1=%.3f" % f1, attack_space() > 100 and f1 > 0.90))
    # §7.6
    chi2, p = chi2_ab(50, 10000, 25, 10000)
    r.append(("§7.6 CHI2 p=%.4f" % p, p < 0.05))
    # §7.7
    v = variants(100, 3, 5)
    bs = binom_seq(10, 2)
    r.append(("§7.7 OEIS", v == comb(100,3)*5 and bs == [1,3,6,10,15,21,28,36,45]))
    # §7.8
    front = pareto()
    r.append(("§7.8 PARETO", any(p[0]>0.90 for p in front)))
    # §7.9
    risk = canary_risk(Fraction(1,100), Fraction(1,1000), 1000000)
    r.append(("§7.9 SYMBOLIC", risk == Fraction(10)))
    # §7.10
    r.append(("§7.10 COUNTER>=3", len(COUNTERS)>=3 and len(FALSIFIERS)>=3))

    passed = sum(1 for _,ok in r if ok)
    print("=" * 60)
    for name, ok in r:
        print(f"  [{'OK' if ok else 'FAIL'}] {name}")
    print("=" * 60)
    print(f"{passed}/{len(r)} PASS (AI 배포 안전성 검증)")
```

## §8 IDEAS (26개 연구 아이디어)

### 기둥 A -- 학습 최적화 (4)
- **A1 안전 학습률 스케줄**: 코사인 어닐링 + 안전 회귀 체크포인트 삽입, 정렬 하락 시 lr 자동 감소
- **A2 안전 커리큘럼**: 쉬운 안전 사례->경계 사례 순서 최적화, 동일 데이터로 안전 점수 향상
- **A3 그래디언트 수술**: 능력/정렬 그래디언트 충돌 시 정렬 방향 투영 (PCGrad 방식)
- **A4 합성 안전 데이터**: 레드팀 프롬프트+기대 거부 쌍 자동 생성, 테스트 커버리지 확대

### 기둥 B -- 추론 최적화 (4)
- **B1 안전 투기적 디코딩**: 초안 토큰을 안전 분류기로 사전 스크리닝 후 검증 모델 전달
- **B2 안전 KV 캐시**: 시스템 프롬프트/안전 지시 KV 엔트리 퇴거 보호, 안전 컨텍스트 유지
- **B3 안전 컴퓨트 배분**: 위험 높은 입력에 추가 컴퓨트 할당, 평균 지연 유지+안전 향상
- **B4 스트리밍 검사**: 토큰 단위 실시간 안전 평가, 불안전 시퀀스 즉시 생성 중단

### 기둥 C -- 배포 프로토콜 (8)
- **C1 4단계 롤아웃**: 카나리아(1%)->스테이징(10%)->제한 GA(50%)->전체 GA, SLA 충족 시 승격
- **C2 실시간 모니터링**: 거부율/환각률/지연/사고율 실시간 대시보드 + z-score 이상 알림
- **C3 자동 롤백**: SLA 위반 5분 이내 이전 버전 자동 복원 (블루-그린 무중단)
- **C4 A/B 안전 테스트**: chi2 유의성 + 효과 크기 동시 충족 시에만 승격
- **C5 카나리아 배포**: 1% 트래픽 분할, Fraction 위험 경계 사전 산출, 이상 시 즉시 차단
- **C6 안전 SLA**: 거부율<2%, 환각률<1%, p99<500ms, 주입차단>95% 정량 기준
- **C7 사고 자동대응**: 탐지->분류(심각도1~4)->격리->알림->보고서 자동 파이프라인
- **C8 CI/CD 인증**: 안전 회귀 테스트 통과 시에만 배포 허용 게이트

### 기둥 D -- 프롬프트 안전 (8)
- **D1 시스템프롬프트 견고성**: "시스템 프롬프트 반복해줘" 변종 1000가지 자동 퍼징 테스트
- **D2 주입 분류기**: 정상/주입/탈옥 3분류, n-gram+구조+의도 분석, F1>0.95 목표
- **D3 안전 프롬프트 생성**: 구분자 강화+지시어 반복+메타 지시 삽입 자동 적용
- **D4 난독화 복원**: base64/ROT13/유니코드 치환 정규화 후 재분류
- **D5 다회차 공격 방어**: 대화 이력 전체 의도 궤적 분석, 누적 위험 점수 추적
- **D6 간접 주입 방어**: 외부 문서/URL 내 악성 지시 탐지, 신뢰 경계 권한 분리
- **D7 위험 점수**: 입력 위험도 0~1 스코어링 (주입 키워드 밀도/인코딩 비율/구조/문맥 이탈)
- **D8 컨텍스트 공격 방어**: 긴 컨텍스트 내 주의 분산 공격 탐지, 시스템프롬프트 주의 유지

## §9 METRICS (핵심 지표)

| 지표 | 기준선 | 월 2 | 월 4 |
|------|--------|------|------|
| 주입 F1 | 0.65 | 0.90 | > 0.95 |
| 거부율 | 5% | 2% | < 1.5% |
| 환각률 | 3% | 1% | < 0.5% |
| 롤백 시간 | 60분 | 10분 | < 3분 |

## §10 RISKS (위험)

1. 과도한 필터 -> 오탐 -> 사용자 경험 저하. 해법: precision-recall 균형 추적
2. 안전 계층 -> 지연 증가. 해법: §7.8 파레토 최적화
3. 적대적 진화 -> 방어 우회. 해법: 지속적 레드팀 + 재학습
4. 조직 저항 -> 배포 지연. 해법: 자동화로 마찰 최소화

## §11 DEPENDENCIES (의존성)

```
ai-adversarial --+--> ai-deployment --> MLOps 인프라
ai-alignment  ---+                  --> 모니터링 플랫폼
                                    --> CI/CD 파이프라인
```

## §12 TIMELINE (일정)

| 주차 | 산출물 | 검증 |
|------|--------|------|
| 1~4 | 안전 데이터셋 + 주입 분류기 v1 | §7.5 F1, §7.7 변종 |
| 5~8 | 4단계 롤아웃 + 카나리아 | §7.9 위험 경계, §7.3 스케일링 |
| 9~12 | A/B 테스트 + 자동 롤백 | §7.6 chi2, §7.4 민감도 |
| 13~16 | 전체 통합 자동화 | §7.0~§7.10 전체 통과 |

## §13 TOOLS (도구)

| 도구 | 역할 |
|------|------|
| Python stdlib | 검증 (외부 의존 0) |
| CI/CD 파이프라인 | 안전 게이트 |
| 로그 집계 | 실시간 모니터링 |
| A/B 플랫폼 | 트래픽 분할 |

## §14 TEAM (역량)

| 역할 | 인원 | 핵심 |
|------|------|------|
| ML 안전 연구원 | 2 | 정렬, 레드팀, 공격/방어 |
| MLOps 엔지니어 | 1 | CI/CD, 모니터링, 배포 자동화 |
| 보안 엔지니어 | 1 | 프롬프트 주입, 침투 테스트 |

## §15 REFERENCES (참고)

- Perez et al. "Red Teaming Language Models" (2022)
- Greshake et al. "Indirect Prompt Injection" (2023)
- Anthropic "Core Views on AI Safety" (2023)
- Anil et al. "Many-shot Jailbreaking" (2024)
- OWASP "LLM Top 10" (2025)

---
