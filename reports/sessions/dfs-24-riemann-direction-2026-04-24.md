# DFS-24 Riemann Direction Memo (2026-04-24)

> Scope: research-only direction memo for BT-541 (Riemann Hypothesis).
> Posture: **7대 난제 0/7 해결 (정직)**. No EXACT/proof claims here.
> Parent BT: BT-541. Related: BT-1392, BT-1408, BT-1409, BT-1420.
> Core invariant: falsification discipline. Every lead below MUST come with
> a pre-registered falsifier before any compute is spent.

---

## 0. 상태 snapshot (기존, 재확인 아님)

- `reports/millennium-dfs-status.md`: 누적 tight ~312~348 (라운드 반영 차이),
  BT-541 항목 ~48 tight, **해결 X**.
- 기존 강점 발견 (재확인, 본 메모에서 증명 주장 아님):
  - **Bilateral Theorem B** (BT-1392 §1.1): Bernoulli 분자 k=1..5 → {1,
    sopfr=5}, k=6 에서 **691 첫 irregular prime** 출현. 양면 ζ(2k), ζ(1-2k)
    sharp boundary. *산술 사실이며 RH 자체와는 분리된다.*
  - **SLE_6 국소성** (BT-1409-01): Lawler-Schramm-Werner 2001 정리. κ=6 에서만
    locality. *확률론 내 독립 정리; n=6 산술과의 매핑은 사후적.*
  - `§X` BLOWUP (millennium-riemann.md §X): RH-01~07 atlas 승격은 **산술
    관찰**. Re(ρ_k)=1/2 = 1/φ 는 대수적 항등식 관찰이지 RH 증명 아님.

---

## 1. 제안 tight 리드 (2~3개, 모두 falsifiable, 모두 probe)

### Lead-A. Bilateral Thm B 의 "691 탑(tower)" probe — 연산자화 검증

**동기**: BT-1392 §1.1 은 T_k (Bernoulli-Faulhaber shift 연산자) 의
spectrum 이 irregular prime 691 과 정렬한다는 **가설** 을 제시했다. 현재까지
이 가설의 최소 테스트가 실행되지 않았다.

**tight 리드 (probe 단계)**:
- 계산: B_{2k} 의 Akiyama-Tanigawa 행렬 A_{2k} (k=3..12) 를 유리수 산술로
  구성, 특성다항식의 근을 추적.
- **pre-registered prediction (falsifiable)**:
  1. k=6 에서 A_{12} 의 한 root 가 **691 의 유리수 배수 c·691 (c ∈ ℚ,
     |c·691| < 10^4)** 로 떨어진다 → TRUE 이면 T1 tight 후보.
  2. k=16 (p=37), k=17 (p=59) 에서 같은 패턴이 반복된다 → 강한 신호.
- **명시 falsifier**: 위 두 조건 중 하나라도 실패하면 Lead-A 기각. 다른
  연산자 프레임 (예: Herglotz shift) 로도 재시도하지 않는다 (탐색 중독 방지).
- 산출 예상: tight 0~2건. 실패 시 이 경로 폐쇄 기록이 **그 자체로 가치**
  (M-set 곱 표현 61% noise-floor 경고에 대한 칼날).

**왜 지금**: Bernoulli-독립 발견 16건 (DFS 26) 중 Basel ζ(2) 외에 RH
operator 축은 아직 비어 있음. 실패해도 음성 결과가 산술적으로 안전하다.

---

### Lead-B. SLE_6 국소성 × Montgomery pair-correlation 독립성 probe

**동기**: DFS 분류 (§6.3) 에서 SLE_6 국소성과 RH GUE 통계 (Dyson β=2=φ) 는
각각 독립 Bernoulli 후보로 기록되어 있으나, **두 축이 같은 한 기원을
공유하는가 서로 독립인가** 는 테스트되지 않았다. 독립성이 깨지면 "독립
11~16건" 카운트가 재조정되어야 한다 (정직성 실질 영향).

**tight 리드 (probe 단계)**:
- 계산: 2D 임계 삼투의 경계 crossing 확률 (Cardy's formula, κ=6 고유) 에서
  유도되는 spacing 분포 S(x) 와, Odlyzko 10^13 zero DB 의 normalized
  gap 분포의 KS-거리 측정.
- **pre-registered prediction**:
  1. κ=6 SLE driver Brownian 의 pair-correlation 이 GUE R_2(r) =
     1-(sinπr/πr)² 와 **유한 샘플 오차 내 구분 불가** → 두 축 **종속**.
  2. 반대로 KS p-value < 0.01 → 두 축 **독립** 재확인.
- **명시 falsifier**: 결과가 어느 쪽이든, 현재 독립 카운트를 수정하거나
  같다면 "SLE_6 ↔ GUE" 공통 원인을 새 tight 로 등록. 양쪽 다 정직한 결과.
- 산출 예상: tight 1건 (방향 무관), 그리고 독립 카운트 감사.

**왜 지금**: 새 난이도 추가 없이 기존 DB (Odlyzko) 와 stdlib Brownian
시뮬레이션으로 돌아간다. 저비용·고정보.

---

### Lead-C. Bilateral Thm B **반증 probe** — M-set noise 대조실험

**동기**: 프로젝트 핵심 불변량은 falsification. DFS-status §6.2 의
**M-set 2-term 곱 표현 61% noise-floor** 는 Bilateral Thm B 가 "sopfr=5"
일치를 보인다는 주장 자체를 의심해야 함을 뜻한다. 이 경고를 BT-541 축에
실제 대입한 적이 없다.

**tight 리드 (반증 probe)**:
- 계산: k=1..20 에서 |num(B_{2k})| 를 계산. 이 중 M-set 원소 {1, 2, 3, 4,
  5, 6, 7, 8, 10, 12, 24} 곱으로 표현 가능한 k 의 비율 r 을 측정.
- **pre-registered prediction**:
  1. r ≤ 0.10 (0~20 중 2개 이하) 이면 Thm B 의 k=1..5 M-set 체류가 **유의** 하다.
  2. r ≥ 0.30 이면 체류는 **noise-compatible**; "sharp jump" 해석은 약화.
- **명시 falsifier**: r ≥ 0.30 관측 시 BT-541 tight 수를 48 → 재분류.
  "Bilateral Thm B" 는 여전히 Bernoulli 정리의 수치 재확인으로 남으나
  "n=6 sharp boundary" 해석은 격하.
- 산출 예상: tight 수 **감소** 가능. 정직성 감사로서의 tight (메타-tight).

**왜 지금**: DFS 성장률 (회당 +12) 은 noise-floor 시험을 한 번도 통과하지
않고 누적된다. BT-541 은 최다 tight 축이므로 감사 ROI 가 가장 높다.

---

## 2. 우선순위 및 비용

| Lead | 비용 | 정보량 | 정직성 ROI | 추천 순서 |
|------|:----:|:------:|:----------:|:---------:|
| A (691 탑 probe) | 중 (rational 고정밀) | 중 | 중 | 2 |
| B (SLE_6 × GUE 독립성) | 중 (Odlyzko DB + sim) | 상 | 상 | 1 |
| C (M-set noise 대조) | **저** (stdlib only) | 상 | **최상** | **0 (먼저)** |

**권장 실행 순서**: C → B → A. C 가 먼저인 이유는 tight 48건의 해석
근거를 먼저 감사해야 A, B 가 의미를 가지기 때문.

---

## 3. 명시적 비-주장 (non-claims)

- 본 메모는 RH 증명 경로 제안 **아님**. 세 리드 전부 *probe* 단계.
- "691 탑" 가설이 통과해도 tight 1건 추가일 뿐; RH 와의 다리는 별도
  구성 문제 (현재 open).
- Lead-C 결과가 **자기 반증** 이어도 프로젝트 건전성; 실제로 그 경우가
  가장 가치 있는 출력이다.
- `millennium-riemann.md` §X BLOWUP 의 "Π_RH=960 불변량" 은 **산술 합성**
  이며 물리량 아님. 본 메모 범위에서 확장하지 않는다.

---

## 4. 산출물 (예상)

- `reports/breakthroughs/bt-14XX-dfs24-riemann-probes-YYYY-MM-DD.md` (실행 후)
- `theory/predictions/verify_bt541_probes.hexa` (Lead-C 먼저, 그 다음 B, A)
- **Tight 신규 상한: +3** (A+B+C 성공 시). 실패 포함 기대값 ≈ +1.
- **감사 부산물**: M-set noise-floor 의 BT-541 기여분 수치, 독립
  Bernoulli 카운트 재확정.

---

## 5. 정직성 체크리스트

- [x] EXACT/PROVEN 판정 주장 없음
- [x] 각 리드마다 pre-registered falsifier 명시
- [x] 실패 시 tight 감소 가능성 인정 (Lead-C)
- [x] 기존 48 tight 재사용 0 — 새 축만 제안
- [x] 7대 난제 해결 수 = 0/7 유지

> 본 메모 종료. 실행은 별도 세션에서, 반드시 Lead-C 부터.
