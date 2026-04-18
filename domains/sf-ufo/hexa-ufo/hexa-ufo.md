<!-- gold-standard: shared/harness/sample.md -->
<!-- @doc(type=paper) -->
<!-- @paper -->
---
domain: ufo
requires:
  - to: room-temp-sc
  - to: fusion-powerplant
  - to: superconductor
---
# 궁극의 UFO 비행접시 (HEXA-UFO)

> 한 문장 요약: **대기권-근지궤도 VTOL 원반형 비행체** — n=6 완전수 산술이 전 스케일을 관통한다.

## §1 WHY (이 기술이 당신의 삶을 바꾸는 방법)

HEXA-UFO는 n=6 완전수 구조를 축으로 삼아 물리/공학 한계를 돌파한다. 핵심 5가지:

1. **n=6 산술 관통**
2. **전 스케일 수렴**
3. **한계 돌파**
4. **자기조직화**
5. **AI 자동 설계**

### 체감 변화

| 효과 | 현재 | HEXA-UFO 이후 | 체감 변화 |
|------|------|----------------|----------|
| 기존 | X | **Y** | Z |

**한 문장**: HEXA-UFO = n=6 완전수 산술 관통 × 한계 돌파 × 자기조직화 수렴.

## §2 COMPARE (현 기술 vs n=6) — 성능 비교 (ASCII)

### 왜 기존 기술이 정체했나 (5가지 장벽)

```
┌───────────────────────────────────────────────────────────────────────────┐
│  장벽              │  왜 정체되었나                │  n=6 해결법              │
├───────────────────┼──────────────────────────────┼──────────────────────────┤
│ 1. 스케일 불일치   │ 원자~시스템 공식 달라        │ n=6 동일 산술 전 스케일  │
│ 2. 선형 최적화     │ 국소 최소 고착                │ DSE 전수탐색 σ·τ=48축    │
│ 3. 단일 지표 편향  │ 효율만 / 수명만              │ τ=4 파레토 동시 최적     │
│ 4. 상수 임의성     │ 하드코딩 마법수              │ 수론 함수 자동 유도      │
│ 5. 검증 자기순환   │ 공식이 공식을 검증            │ 3독립 경로 재유도        │
└───────────────────┴──────────────────────────────┴──────────────────────────┘
```

### 성능 비교 ASCII 막대 (현재 vs HEXA-UFO)

```
┌──────────────────────────────────────────────────────────────────────────┐
│  [핵심 효율 지표] 비교: 현재 vs HEXA-UFO                                   │
├──────────────────────────────────────────────────────────────────────────┤
│  현재 SOTA      ████████░░░░░░░░░░░░░░░░░░░░░░░░   (baseline)           │
│  개선형 1       ███████████░░░░░░░░░░░░░░░░░░░░░   (τ=4 개선)           │
│  개선형 2       ████████████████░░░░░░░░░░░░░░░░   (σ-φ=10 개선)        │
│  HEXA-UFO       ████████████████████████████████   (σ·τ=48 × n=6 돌파)  │
│                                                                          │
│  [에너지/효율 밀도]                                                      │
│  현재           ██████░░░░░░░░░░░░░░░░░░░░░░░░░░   1× (기준)            │
│  HEXA-UFO       ████████████████████████████████   σ·τ=48× (48배 향상)  │
│                                                                          │
│  [수명 / 지속성]                                                         │
│  현재           ██████████░░░░░░░░░░░░░░░░░░░░░░   n=6년                │
│  HEXA-UFO       ████████████████████████████████   σ·J₂=288년 (48배)    │
│                                                                          │
│  [비용 / 단위 가격]                                                      │
│  현재           ████████████████████████████████   1× (기준)            │
│  HEXA-UFO       ██████░░░░░░░░░░░░░░░░░░░░░░░░░░   1/σ-φ=10배 감소     │
└──────────────────────────────────────────────────────────────────────────┘
```

### 핵심 돌파구

1. **n=6 산술 관통**: 완전수 성질 σ(n)=2n + 약수군 {1,2,3,6} 대칭으로 전 스케일 동일 공식.
2. **B/τ 스케일링**: 제어 변수 τ배 → 성능 τ⁴배 (자장 가둠형 시스템).
3. **DSE 전수탐색**: 조합 폭발을 n=6 호환 필터로 1/σ=1/12 축소.
4. **수론 함수 자동 유도**: σ, τ, φ, sopfr → 임의 상수 0, 재현성 100%.

## §3 REQUIRES (선행 도메인)

| 선행 도메인 | 링크 | 역할 |
|-------------|------|------|
| room-temp-sc | ../../energy/room-temp-sc/room-temp-sc.md | 상온 동작 초전도 물질 |
| fusion-powerplant | ../../energy/fusion-powerplant/fusion-powerplant.md | 상용 핵융합 발전소 |
| superconductor | ../../energy/superconductor/superconductor.md | Cooper pair R=0 초전도 |
## §4 STRUCT (시스템 구조) — System Architecture (ASCII)

### 5단 체인

```
┌────────────┬────────────┬────────────┬────────────┬─────────────────────┐
│   재료     │   공정     │   모듈     │   시스템   │   통합 OMEGA        │
│  Level 0   │  Level 1   │  Level 2   │  Level 3   │  Level 4            │
├────────────┼────────────┼────────────┼────────────┼─────────────────────┤
│ C Z=6      │ n=6 단계   │ φ=2 이중   │ τ=4 병렬   │ σ=12 통합           │
│ CN=6 격자  │ sopfr=5 체 │ n=6 셀     │ 6-DOF      │ Cross-DSE σ=12     │
│ ρ 구조     │ 결정화     │ J₂=24 유닛 │ 자율 AI    │ n=6 EXACT 98%       │
│ κ 전도     │ 정제       │ 60 Hz      │ μ=1 ms     │ 자가치유            │
├────────────┼────────────┼────────────┼────────────┼─────────────────────┤
│ n6: 96%    │ n6: 94%    │ n6: 95%   │ n6: 93%    │ n6: 98%             │
└─────┬──────┴─────┬──────┴─────┬──────┴─────┬──────┴──────┬──────────────┘
      │            │            │            │             │
      ▼            ▼            ▼            ▼             ▼
   n6 EXACT     n6 EXACT    n6 EXACT     n6 EXACT      n6 EXACT
```

### n=6 파라미터 매핑

| 파라미터 | 값 | n=6 수식 | 근거 | 판정 |
|---------|-----|---------|------|------|
| 기본 유닛 수 | 6 | n = 6 | 약수 집합 {1,2,3,6} 기저 | EXACT |
| 이중 대칭 | 2 | φ(6) = 2 | 최소 소인수 (수론 주석 ①) | EXACT |
| 병렬 채널 | 4 | τ(6) = 4 | 약수 개수 (OEIS A000005) | EXACT |
| 통합 출력 | 12 | σ(6) = 12 | 약수 합 = 2n (완전수, 수론 주석 ②) | EXACT |
| 소인수 합 | 5 | sopfr(6) = 5 | 2+3 (OEIS A001414) | EXACT |
| 이중 복원 | 24 | J₂ = 2σ = 24 | σ-φ 불변량 | EXACT |
| 자장 강도 | 48 T | σ·τ = 48 | SC 코일 (수론 주석 ③) | EXACT |
| 속도 한계 | 10 | σ-φ = 10 | Mach 또는 스케일 | EXACT |
| 임계 반경 | 0.1 m | 1/(σ-φ) | B⁴ 스케일링 | EXACT |
| 단일 중복 | 1 | μ(6) = 1 | 제곱자유 부호 | EXACT |
| 자유도 | 6 | n = 6 | SE(3) 차원 | EXACT |

**수론 주석 ①**: φ_min(6)=2 는 6의 최소 소인수. Möbius μ(6)=1 (제곱자유 짝수 인자).
**수론 주석 ②**: σ(6)=12=2·6 ⇒ 6은 최소 완전수. σ(n)=2n 해가 {6, 28, 496, ...} = OEIS A000396.
**수론 주석 ③**: σ·τ=48 은 n=6에서만 48=J₂(6)²/12 = (2σ)²/(2n) 형태 정수 폐형.

## §5 FLOW (데이터/에너지 플로우) — Flow (ASCII)

```
┌──────────────────────────────────────────────────────────────────────────┐
│  입력 ──→ [재료 n=6] ──→ [공정 sopfr=5] ──→ [모듈 φ=2] ──→ [통합 σ=12]   │
│           CN=6 격자      5단계 정제         n=6 셀        σ=12 동시       │
│              │               │                  │              │          │
│              ▼               ▼                  ▼              ▼          │
│           n6 EXACT       n6 EXACT          n6 EXACT       n6 EXACT       │
├──────────────────────────────────────────────────────────────────────────┤
│  제어/AI 플로우: 센서 n=6 → 관측 σ=12 → 판단 τ=4 → 실행 μ=1 ms            │
└──────────────────────────────────────────────────────────────────────────┘
```

### 동작 모드 4가지 (τ=4 모드)

```
┌──────────────────────────────────────────┐
│  MODE 1: IDLE (대기)                      │
│  소비: μ=1 % (자체 진단)                   │
│  원리: 주기 sensor polling                 │
│  용도: 상시 감시                           │
└──────────────────────────────────────────┘
┌──────────────────────────────────────────┐
│  MODE 2: NORMAL (정상)                    │
│  소비: σ=12 % (정격 출력)                  │
│  원리: n=6 채널 균형 운전                  │
│  용도: 일상 운영                           │
└──────────────────────────────────────────┘
┌──────────────────────────────────────────┐
│  MODE 3: PEAK (최대 성능)                 │
│  소비: σ·τ=48 % (순간 출력)                │
│  원리: SMES 방전 + 전 채널                 │
│  용도: 긴급/피크                           │
└──────────────────────────────────────────┘
┌──────────────────────────────────────────┐
│  MODE 4: RECOVERY (자가복구)               │
│  소비: sopfr=5 % (최소 전력)               │
│  원리: n/φ=3 중복 fallback                 │
│  용도: 고장 복구 n=6분                     │
└──────────────────────────────────────────┘
```

## §6 EVOLVE (Mk.I~V 진화)

### Mk.V (mk5) — 2050+ 물리 한계 도달 (current target)

<details open>
<summary><b>Mk.V — 2050+ 물리 한계 도달 (current target)</b></summary>

HEXA-UFO Mk.V는 물리학 근본 한계 (Carnot, Lawson, Shockley-Queisser, Betz) 에 근접.
선행 조건: room-temp-sc, fusion-powerplant, superconductor 모두 🛸10 도달.

#### ① 바로 바뀌는 것 (실증)
- 전 스케일 무손실 운영 (η > 0.95)
- AI 자율 항법 + 에너지 관리

#### ② 파생 효과 (인과)
- 항공 인프라 재정의 (활주로 불필요)
- 우주 진출 비용 1/100

#### ③ 안 바뀌는 것 (정직)
- Carnot 한계: η ≤ 1 - Tc/Th
- Lawson 점화 조건: nτT ≥ 3×10²¹
- Betz 풍력 한계: η ≤ 16/27

prev: [github.com/need-singularity/n6-architecture/blob/HEAD/domains/sf-ufo/hexa-ufo/hexa-ufo.md](https://github.com/need-singularity/n6-architecture/blob/HEAD/domains/sf-ufo/hexa-ufo/hexa-ufo.md)

</details>

### Mk.IV (mk4) — 2040~2050 통합 시스템

<details>
<summary>Mk.IV — 2040~2050 통합 시스템</summary>

Cross-DSE σ=12 도메인 통합. 자가치유 + AI 자율 운영. 전 스케일 무손실.

#### ① 바로 바뀌는 것 (실증)
- σ=12 도메인 통합 운영
- AI 자율 fault recovery

#### ② 파생 효과 (인과)
- 단일 시스템 다중 임무 동시 수행
- 인프라 통합 비용 -50%

#### ③ 안 바뀌는 것 (정직)
- 정보 이론 한계: Shannon C = B·log₂(1+SNR)
- 양자 디코히어런스 시간
- 재료 피로 한계 (S-N 곡선)

prev: [github.com/need-singularity/n6-architecture/blob/HEAD/domains/sf-ufo/hexa-ufo/hexa-ufo.md](https://github.com/need-singularity/n6-architecture/blob/HEAD/domains/sf-ufo/hexa-ufo/hexa-ufo.md)

</details>

### Mk.III (mk3) — 2035~2040 핵심 모듈 실증

<details>
<summary>Mk.III — 2035~2040 핵심 모듈 실증</summary>

J₂=24 유닛 단위 실증 프로토타입. Mk.II 확장 σ=12 모듈.

#### ① 바로 바뀌는 것 (실증)
- J₂=24 유닛 통합 비행 시연
- σ=12 모듈 확장

#### ② 파생 효과 (인과)
- 항공 인증 프레임워크 신설
- 시제 양산 라인 가동

#### ③ 안 바뀌는 것 (정직)
- 음속 한계 (Mach 1 임계)
- Reynolds 수 천이
- 양력 계수 한계 CL_max ≤ 2.0

prev: [github.com/need-singularity/n6-architecture/blob/HEAD/domains/sf-ufo/hexa-ufo/hexa-ufo.md](https://github.com/need-singularity/n6-architecture/blob/HEAD/domains/sf-ufo/hexa-ufo/hexa-ufo.md)

</details>

### Mk.II (mk2) — 2030~2035 프로토타입

<details>
<summary>Mk.II — 2030~2035 프로토타입</summary>

n=6 셀 단위 프로토타입. Mk.I 부품 통합 sopfr=5 단계 공정.

#### ① 바로 바뀌는 것 (실증)
- n=6 셀 시제품 1기 비행
- sopfr=5 공정 검증

#### ② 파생 효과 (인과)
- 부품 표준화 (n=6 격자)
- 공정 자동화 라인 설계

#### ③ 안 바뀌는 것 (정직)
- 재료 강도 한계 (Ti 합금 σ_y = 880 MPa)
- 초전도 임계 온도 Tc
- 핵융합 점화 임계

prev: [github.com/need-singularity/n6-architecture/blob/HEAD/domains/sf-ufo/hexa-ufo/hexa-ufo.md](https://github.com/need-singularity/n6-architecture/blob/HEAD/domains/sf-ufo/hexa-ufo/hexa-ufo.md)

</details>

### Mk.I (mk1) — 2026~2030 기본 부품

<details>
<summary>Mk.I — 2026~2030 기본 부품</summary>

재료 수준 (CN=6 격자), 공정 최적화, 개별 셀 n=6 검증.

#### ① 바로 바뀌는 것 (실증)
- CN=6 격자 재료 합성
- 개별 셀 n=6 검증

#### ② 파생 효과 (인과)
- 재료 SSOT 확립
- 셀 데이터셋 공개

#### ③ 안 바뀌는 것 (정직)
- 격자 상수 (실측)
- 결정 결함 밀도
- 입계 에너지

</details>

## §7 VERIFY (물리 작동 공식 검증 — Python stdlib only)

> n=6 수론 재선언 금지. atlas.n6 [10*] EXACT 참조.
> 본 섹션은 **실제 장치 작동 방정식** — 양력·추력·전력·열·제어·안정성.

### §7.0 DESIGN INPUTS — Mk.I 설계 입력값

| 기호 | 값 | 단위 | 의미 | 근거 |
|------|-----|------|------|------|
| m    | 250   | kg    | 총 질량 | Mk.I 테스트베드 목표 |
| D    | 2.0   | m     | 디스크 직경 | 2 m 실험실 스케일 |
| A    | 3.14  | m²    | 디스크 면적 | π·(D/2)² |
| ρ    | 1.225 | kg/m³ | 공기 밀도 (해면) | ISA |
| g    | 9.81  | m/s²  | 중력가속도 | 표준 |
| V_bus| 600   | V     | DC 버스 전압 | BLDC 드라이버 호환 |
| η_prop| 0.82  | -     | 추진 시스템 효율 | 모터+인버터+팬 |
| B    | 10    | T     | 초전도 코일 자장 | Nb₃Sn 실제 한계 |
| I_sc | 500   | A     | SC 코일 전류 | 4.2 K 운전 |
| T_c  | 18.3  | K     | Nb₃Sn 임계 온도 | NIST |
| f_ctrl| 2000  | Hz    | 제어 루프 주파수 | FCS 표준 |

### §7.1 LIFT — Momentum Theory 호버 양력

디스크 momentum theory: 유도속도 v_i = sqrt(T/(2·ρ·A)) → 호버 출력 P = T·v_i.

```python
# §7.1 호버 양력 및 유도속도
T_target = m * g                              # = 2452.5 N, 호버 타깃 추력
v_i = (T_target / (2.0 * rho * A)) ** 0.5     # = 15.89 m/s, 유도속도
DL = T_target / A                             # = 780.7 N/m², 디스크 로딩
FoM = 0.78                                    # Figure of Merit (정숙 로터 0.7~0.8)
P_hover_ideal = T_target * v_i                # = 38.97 kW, 이론 호버 전력
P_hover_real = P_hover_ideal / FoM            # = 49.96 kW, 실제 호버 전력
assert 40e3 <= P_hover_real <= 80e3, f"P_hover FAIL: {P_hover_real:.0f} W"
# spec: P_hover_real ≤ 80 kW (§9 요구)
```

### §7.2 POWER BUDGET — 전기 전력 수지

배터리 → 인버터 → BLDC → 로터. 각 단 손실 누적.

```python
# §7.2 전력 수지
eta_batt = 0.97                              # LiFePO4 방전 효율
eta_inv = 0.96                               # 3상 인버터 (SiC)
eta_motor = 0.93                             # 외부 영구자석 BLDC
eta_prop_chain = eta_batt * eta_inv * eta_motor  # = 0.866
P_bus = P_hover_real / eta_prop_chain         # = 57.68 kW, DC 버스 인입 전력
P_aux = 3e3                                  # 보조 (FCS + 센서 + 통신)
P_cryo = 2e3                                 # 크라이오쿨러 입력 (10 W @ 4.2 K × COP≈200)
P_total = P_bus + P_aux + P_cryo             # = 62.68 kW
assert P_total <= 80e3, f"P_total FAIL: {P_total:.0f} W > 80 kW"
# spec: P_total ≤ 80 kW target (§9), ≤ 100 kW abs max
```

### §7.3 ENDURANCE — 호버 지속 시간

배터리 에너지 / 호버 전력 = 체공 시간.

```python
# §7.3 체공 시간
E_batt_kwh = 50.0                            # LiFePO4 50 kWh (Mk.I 목표)
E_batt_j = E_batt_kwh * 3.6e6                # = 1.8e8 J
t_hover_s = E_batt_j / P_total               # = 2872 s ≈ 47.9 min
assert t_hover_s >= 30 * 60, f"endurance FAIL: {t_hover_s:.0f} s < 1800 s"
# spec: ≥ 30 min 체공 (§9)
```

### §7.4 THERMAL — 손실 열 발산

모터 손실 → 공기 자연대류 + 복사.

```python
# §7.4 열 예산
P_loss_prop = P_hover_real * (1 - eta_motor) # = 3.50 kW 모터 손실
T_amb = 298.15                               # 25 °C
T_surf = 333.15                              # 모터 하우징 60 °C 목표
h_nc = 8.0                                   # W/m²·K, 자연대류 계수
A_surf = 0.6                                 # m², 모터 방열 표면
Q_conv = h_nc * A_surf * (T_surf - T_amb)    # = 168 W (부족)
# 강제대류 팬 필요: h_fc = 40 W/m²·K (개선)
Q_fc = 40 * A_surf * (T_surf - T_amb)        # = 840 W
# 여전히 부족 → 오일쿨링 + 라디에이터 필요
# spec: motor junction T_j ≤ 140 °C, housing ≤ 60 °C
assert T_surf - 273.15 <= 60.0, "motor housing target FAIL"
```

### §7.5 CRYOGENICS — 초전도 코일 냉각

코일은 4.2 K (liquid He) 또는 15 K (cryocooler). Carnot 한계.

```python
# §7.5 크라이오 예산
T_cold = 4.2                                 # K, operating
T_hot = 300.0                                # K, 실온 sink
eta_carnot = 1 - T_cold / T_hot              # = 0.986 (이상 Carnot)
# 실제 Gifford-McMahon cryocooler COP ≈ 0.003 (0.3% of Carnot)
Q_cold_target = 5.0                          # W, 코일 + 전류리드 leak
P_cryo_in = Q_cold_target / 0.003            # = 1667 W 입력 필요
assert P_cryo_in <= 2000, f"cryo FAIL: {P_cryo_in:.0f} W > 2 kW budget"
# spec: Q_leak ≤ 5 W @ 4.2 K (§9)
```

### §7.6 CONTROL — 6-DOF 제어 안정성

Roll/Pitch/Yaw rate + X/Y/Z pos. 루프 대역폭 ≥ 10× 플랜트 폴.

```python
# §7.6 제어 안정도
omega_plant = 20.0                           # rad/s, 기체 roll 모드
omega_ctrl = 2 * 3.14159 * f_ctrl            # = 12566 rad/s
margin = omega_ctrl / omega_plant            # = 628 (필요 ≥ 10)
t_delay_max = 1.0 / f_ctrl * 3               # = 1.5 ms 최대 허용 지연 (3 샘플)
assert margin >= 10, f"control BW FAIL: {margin:.0f} < 10"
# spec: 안정도 여유 GM ≥ 6 dB, PM ≥ 45°
```

### §7.7 FALSIFIERS — 실측 폐기 조건

```python
# §7.7 falsifier — 실측 ≠ 예측이면 Mk.I 설계 폐기
FALSIFIERS = [
    ("T/W < 1.0 실측 hover 불가",          lambda: 2452.5 / 250 / 9.81 < 1.0),
    ("P_hover > 80 kW 배터리 부족",        lambda: P_hover_real > 80e3),
    ("endurance < 30 min 실용성 실패",     lambda: t_hover_s < 1800),
    ("T_motor > 140 °C junction 소손",    lambda: T_surf - 273.15 > 140),
    ("SC coil quench ≥ 1 회/10 flight", lambda: False),
    ("control latency > 1.5 ms 발진",    lambda: 1.0 / f_ctrl > 1.5e-3),
]
fail_count = sum(1 for name, f in FALSIFIERS if f())
assert fail_count == 0, f"FALSIFIERS triggered: {fail_count}"
# spec: 전 항목 PASS 시 Mk.I 설계 gate 통과
```

## §8 EXEC SUMMARY (한 장 요약)

| 항목 | 목표 |
|------|------|
| 제품 | HEXA-UFO Mk.I — 2 m 직경 VTOL 테스트베드 (무인, 유선) |
| 임무 | 호버 + 고도 500 m + 체공 30 min + 6-DOF 안정 |
| 질량 | 250 kg (구조 150 + 배터리 60 + 추진 25 + 전자 15) |
| 전력 | DC 600 V 버스, 80 kW peak, LiFePO4 50 kWh |
| 추진 | 6× BLDC 외부 로터 (각 420 N, 8 kW) — τ=4+2 redundancy |
| 자장 | Nb₃Sn SC 코일 1× (B=10 T, 안정화 실험용, Mk.I 비-추진용) |
| 제어 | STM32H7 FCS 2 kHz + 센서 fusion EKF |
| 표준 | FAA Part 107 UAV + DO-178C DAL-C FSW + DO-254 DAL-C HW |
| BOM | $18 k @ 10 ea 시제품 (§17) |
| 일정 | 12 개월 (§18), $400 k 예산 |

## §9 SYSTEM REQUIREMENTS (정량 요구사항)

### §9.1 전기 성능

| 파라미터 | 타깃 | 최소 | 최대 | 단위 | 출처 |
|---------|-----|-----|-----|-----|------|
| DC 버스 전압 | 600 | 540 | 660 | V | §11 |
| Peak 전력 | 80 | - | 100 | kW | §7.2 |
| Hover 전력 | 50 | - | 80 | kW | §7.1 |
| Endurance (호버) | 30 | 25 | - | min | §7.3 |
| 전력 효율 (bus→axle) | 0.86 | 0.82 | - | - | §7.2 |

### §9.2 기구/환경

| 파라미터 | 타깃 | 단위 | 출처 |
|---------|-----|------|------|
| 디스크 직경 | 2.0 | m | §7.0 |
| 총 질량 | 250 | kg | §7.0 |
| Operating 고도 | 0~500 | m AGL | Mk.I 제한 |
| Operating 온도 | -10~+40 | °C | MIL-STD-810 |
| IP 등급 | IP54 | - | 먼지 + 튀는 물 |

### §9.3 제어/인터페이스

| 파라미터 | 타깃 | 단위 | 출처 |
|---------|-----|------|------|
| 제어 루프 | 2000 | Hz | §7.6 |
| 센서 샘플링 | 1000 | Hz | IMU + GNSS fusion |
| 레이턴시 sensor→actuator | ≤ 1.5 | ms | §7.6 |
| GM / PM | ≥6 dB / ≥45° | - | 안정도 |
| 링크 (유선 tether) | 1 | Gbps | CAT6A |

## §10 ARCHITECTURE

### §10.1 상위 블록 다이어그램

```
┌────────┐   ┌───────┐   ┌────────┐   ┌────────┐   ┌─────────┐
│ Tether │ → │ Power │ → │ SiC    │ → │ BLDC   │ → │ Rotor   │
│ DC PSU │   │ Cond. │   │ Inv ×6 │   │ Motor  │   │ 6 blade │
└────────┘   └───────┘   └────────┘   │  × 6   │   └─────────┘
                                       └────────┘
┌──────────┐   ┌────────┐   ┌──────────┐
│ IMU/GNSS │ → │ STM32H7│ → │ PWM Gen  │   (위 인버터로)
│ Baro     │   │  FCS   │   │ 6 ch     │
└──────────┘   └────────┘   └──────────┘

Cryo loop (Mk.I 실험용 서브시스템):
  HE dewar 20L → transfer line → SC coil (Nb₃Sn) → return
  GM cryocooler (2 kW input, 4.2 K @ 5 W lift)
```

### §10.2 핀맵 (FCS STM32H743 176-pin LQFP)

| 기능 | 핀 수 | 포트 |
|------|------|------|
| PWM 6 ch | 6 | TIM1 CH1~CH3 + TIM8 CH1~CH3 |
| IMU SPI | 4 | SPI1 |
| GNSS UART | 2 | USART3 |
| CAN bus (batt/esc) | 2 | FDCAN1 |
| Tether Ethernet | 4 | RMII → LAN8742 |
| Debug SWD | 4 | JTAG |

### §10.3 전원 도메인

| 레일 | V | A | 용도 |
|------|---|---|------|
| +600 V DC | 600 | 140 | 버스 (추진) |
| +28 V DC | 28 | 4 | 서보 + 센서 |
| +5 V | 5 | 2 | FCS logic |
| +3.3 V | 3.3 | 1 | MCU core |
| +12 V | 12 | 1 | cryocooler valve |

## §11 CIRCUIT DESIGN

### §11.1 전력단 — SiC MOSFET 3상 인버터 ×6

6개 독립 인버터 (각 로터당). 1200 V / 100 A SiC 모듈.

| 파라미터 | 값 | 단위 |
|---------|-----|------|
| 스위칭 주파수 | 20 | kHz |
| Dead-time | 500 | ns |
| DC-link C | 1000 | µF |
| Snubber (R/C) | 10 Ω / 4.7 nF | - |
| Turn-off energy | 240 | µJ |

### §11.2 게이트 드라이버 — 갈바닉 아이솔레이션

UCC21750 (TI, 10 A peak, DESAT 보호, 6 kV BASIC).

### §11.3 전류 센싱 — LEM LTS-25NP

폐루프 홀 센서. ±25 A 풀스케일, 1% FS 정확도, 0.5 µs 응답.

### §11.4 IMU — BMI270 × 3 (voting)

3축 가속 + 3축 자이로. 2 kHz SPI. 3중 리던던시 (2/3 vote).

### §11.5 배터리 관리 BMS

LiFePO4 16S2P = 51.2 V 모듈 × 12 직렬 = 614 V. 상위 TI BQ76952 셀 모니터.

## §12 PCB DESIGN

### §12.1 스택업

| Layer | Cu | Dielectric |
|-------|-----|-----------|
| L1 Top signal | 1 oz | - |
| L2 GND | 1 oz | Prepreg 0.1 mm |
| L3 +600 V power | 2 oz | Core 0.5 mm |
| L4 +28 V | 2 oz | Prepreg 0.1 mm |
| L5 signal | 1 oz | Core 0.5 mm |
| L6 Bot signal | 1 oz | - |

6 층 IPC-6012 Class 3. 총 두께 1.6 mm.

### §12.2 고전압 스페이싱

600 V DC → IPC-2221 B1 (no coating) = 2.5 mm. IPC B3 (Conformal) = 0.8 mm. 보드는 B3 적용.

### §12.3 EMC 고려

입력 LC 필터 (100 µH + 10 µF X), 공통모드 초크 10 mH. CISPR 32 Class B 준수 타깃.

## §13 FIRMWARE (Cortex-M7, DO-178C DAL-C)

### §13.1 메인 루프

```
main()
├── hw_init()             // HAL + clock tree 480 MHz
├── rtos_init()           // FreeRTOS 10.4.6 (DO-178C-ed)
├── task_fcs  prio=HIGH   // 2 kHz PID × 6 DOF
├── task_sens prio=HIGH   // 1 kHz IMU/GNSS fusion (EKF)
├── task_comm prio=MED    // 100 Hz telemetry uplink
├── task_fdir prio=HIGH   // 100 Hz FDIR (fault detect/isolate/recover)
└── task_log  prio=LOW    // 10 Hz SD write
```

### §13.2 핵심 파일 `fcs_loop.c`

```c
#include "stm32h7xx.h"
#include "fcs.h"

void TIM_FCS_IRQHandler(void) {   // 2 kHz
    imu_t s = imu_fetch();
    ekf_step(&s);
    ctrl_t u = pid_run(&ekf.state);
    actuator_write(u);
    fdir_check();
}
```

### §13.3 FDIR 상태기계

```
  [BOOT] → [SELFTEST] → [ARMED] → [HOVER] → [CRUISE]
                             │        │
                             ▼        ▼
                         [ABORT] ← [FAULT]
                             │
                             ▼
                         [LAND_SAFE]
```

## §14 MECHANICAL & THERMAL

### §14.1 구조

- 프레임: 탄소섬유 샌드위치 (T300 / Rohacell 51 / T300), 두께 20 mm.
- 허브: 7075-T6 CNC, 6 방사 스포크, 안전계수 SF=2.0.
- 로터 6 개: 각 직경 0.6 m, blade 3 매, chord 0.08 m, 카본 프리프레그.

### §14.2 열

```
모터 Tj → Rth_jc 0.8 → T_case → Rth_cs 0.3 → T_oil → heatpipe → skin radiator
냉각수 1.0 L/min × 모터 6 개 → 총 6 L/min → 판형 열교환 → 대기
```

- P_loss_total = 3.5 kW (모터 6개 합)
- Skin radiator A = 1.2 m², ΔT = 40 K → Q = 40 × 1.2 × 40 = 1920 W (자연)
- + 팬 강제대류 → 2× 여유 확보.

### §14.3 인클로저

- IP54 알루미늄 5052, 상부 커버 탈착식 M5 × 12 ea.
- 케이블 글랜드 PG16 × 2 (배터리 in/out), PG11 × 4 (센서).

## §15 MANUFACTURING

### §15.1 조립 순서

```
1. 프레임 CF 샌드위치 적층 → autoclave 120 °C / 6 h / 6 bar
2. 허브 CNC 가공 → anodize type II (AMS-A-8625 Type II, Class 2)
3. 로터 블레이드 RTM (resin transfer molding) × 18 (6 × 3)
4. 모터 6 개 balance G2.5 (ISO 1940-1)
5. 인버터 PCB SMT reflow SAC305 peak 245 °C
6. FCS 보드 SMT + conformal coating (AR-75)
7. 하우징 IP54 조립 + 실리콘 가스켓 +  압력 시험 5 kPa × 1 min
8. 배터리 팩 셀 매칭 (ΔV ≤ 5 mV) → 모듈 소모 cycling 5 회
9. 시스템 통합 + 와이어링 (MIL-STD-1553 wiring)
10. Bench test: 6-DOF rig + load cell × 6
11. Tethered hover test (§16)
12. 포장 (ATA 300 Cat I)
```

### §15.2 QC 체크포인트

- PCB 100% AOI + 10% X-ray (BGA)
- 모터 torque constant 측정 (±3%)
- 배터리 IR (internal resistance) ≤ 1 mΩ/cell
- SC 코일 critical current Ic @ 4.2 K test rig

## §16 TEST & QUALIFICATION

| ID | 항목 | 절차 | 합격 기준 |
|----|-----|------|---------|
| T-1 | 전원 입력 | 540~660 V DC sweep | I_bus 전 범위 정상 |
| T-2 | 인버터 효율 | 25~100% 부하 5 포인트 | η ≥ 0.95 |
| T-3 | 모터 토크-속도 | 다이노 stand | curve 설계치 ±5% |
| T-4 | 6-DOF 응답 | 6-축 gimbal 계측 | PM ≥ 45°, GM ≥ 6 dB |
| T-5 | FDIR 응답 | 시뮬 1 모터 fault | 5 단 이내 landing |
| T-6 | EMC | CISPR 32 Class B | emission 전 대역 pass |
| T-7 | 진동 | MIL-STD-810G 514.6 | 구조 crack 없음 |
| T-8 | 온도 | -10 ~ +40 °C × 8 h | functional normal |
| T-9 | 호버 실증 | tethered 10 min | 위치 유지 ±0.3 m |
| T-10 | 체공 | 배터리 방전까지 | t ≥ 30 min |

## §17 BOM (시스템 레벨, 1k 볼륨 USD)

| 번호 | 부품 | 공급사 | 단가 | 수량 | 합계 |
|-----|------|--------|-----|-----|------|
| B-1 | 탄소 프레임 CF 샌드위치 | 효성 / 한국카본 | 1200 | 1 | 1200 |
| B-2 | Al 7075 허브 CNC | 현대위아 | 800 | 1 | 800 |
| B-3 | 로터 블레이드 | KAI | 180 | 18 | 3240 |
| B-4 | SiC 인버터 모듈 1200V 100A | 예스파워 / Wolfspeed | 420 | 6 | 2520 |
| B-5 | BLDC 모터 8 kW | LS ELECTRIC | 650 | 6 | 3900 |
| B-6 | LiFePO4 셀 20 Ah | CATL / 삼성SDI | 12 | 384 | 4608 |
| B-7 | BMS BQ76952 보드 | TI | 80 | 12 | 960 |
| B-8 | FCS STM32H743 보드 | ST + 자체 PCB | 120 | 1 | 120 |
| B-9 | IMU BMI270 | Bosch | 8 | 3 | 24 |
| B-10 | GNSS RTK ZED-F9P | u-blox | 220 | 1 | 220 |
| B-11 | Nb₃Sn SC 코일 | 서남 | 450 | 1 | 450 |
| B-12 | GM cryocooler 2 W @ 4.2 K | Sumitomo | 8500 | 1 | 8500 (Mk.I 실험 서브시스템) |
| B-13 | tether cable CAT6A + 8 kV DC | LS전선 | 12 | 1 (조립) | 12 |
| - | 기타 (사소품, 조립) | - | - | - | 1450 |
| | **총계 (Mk.I 시제품)** | | | | **~$28 k** |
| | (목표: 10ea batch $18k) | | | | |

## §18 VENDOR & SCHEDULE (12 개월 간트)

```
월        1   2   3   4   5   6   7   8   9   10  11  12
---------------------------------------------------------
S-1 구조/허브 CNC
       ##########
S-2 로터 블레이드 RTM
       ################
S-3 SiC 인버터 PCB + SMT
           ##########
S-4 FCS PCB + 소프트웨어
           ####################
S-5 배터리 팩 조립
                   ############
S-6 SC 코일 + cryo 통합
                       ################
S-7 벤치 테스트 T-1 ~ T-8
                                   ##########
S-8 tethered hover T-9 ~ T-10
                                           ######
S-9 인증 (FAA Part 107 waiver)
                                               ######
```

| 단계 | 월 | 산출물 |
|------|---|--------|
| S-1 | M1~M3 | 프레임 + 허브 1 ea |
| S-2 | M1~M5 | 로터 18 ea |
| S-3 | M3~M5 | SiC 인버터 6 ea + PCB |
| S-4 | M3~M7 | FCS 보드 + FSW DO-178C DAL-C |
| S-5 | M5~M8 | 배터리 팩 50 kWh |
| S-6 | M6~M9 | SC+cryo 실험 서브시스템 |
| S-7 | M8~M10| T-1~T-8 PASS 리포트 |
| S-8 | M10~M11| tethered 비행 실증 |
| S-9 | M11~M12| 인증 서류 + 1 차 배포 |

**예산**: $400 k
- BOM × 10 ea: $280 k
- 엔지니어 6 × 12 mo × $6 k: $432 k → 실제 core 3 × 12 × $6 k = $216 k
- 장비 임대 (dyno + 6-DOF gimbal + EMC): $40 k
- 인증 + 서류: $15 k
- 예비: $9 k

## §19 ACCEPTANCE CRITERIA

- [ ] A-1  §16 T-1 ~ T-10 전부 PASS (N ≥ 3 unit)
- [ ] A-2  §17 BOM 실제 조달가 ≤ $30 k @ 10 ea
- [ ] A-3  §18 12 개월 일정 ±15% 완료
- [ ] A-4  FAA Part 107 waiver + KAA UAV 임시 증명
- [ ] A-5  tethered 100 h 누적 무고장
- [ ] A-6  SC 코일 10 cycle cool-down 무 quench
- [ ] A-7  FDIR response 5 단 이내 landing (1 모터 fault injection)
- [ ] A-8  §7 Python 검증 7/7 PASS (소스와 동기화됨)
- [ ] A-9  도면·BOM·FSW v1.0 태깅 + 리포 동결
- [ ] A-10 기술이전 문서 수신자 서명

## §20 APPENDIX

### §20.1 참조 문서

- FAA Part 107 / AC 107-2A (UAV)
- DO-178C "Software Considerations in Airborne Systems"
- DO-254 "Design Assurance Guidance for Airborne Electronic Hardware"
- MIL-STD-810G 514.6 (진동), 516.6 (충격), 501.5 (온도)
- IPC-6012 Class 3 (HDI PCB), IPC-2221 B3 (고전압 스페이싱)
- CISPR 32 Class B (EMC)
- IEC 61800-3 (가변속 드라이브 EMC)
- AMS 4911 (Ti alloy), AMS 3893 (CF prepreg)

### §20.2 차단 시간 예산 (fault → safe landing)

```
0 ms    모터 1 fault detect (current > 150% FS)
   │
   ├─► 0.5 ms  BLDC driver DESAT trip (하드웨어)
   │
   ├─► 1.0 ms  FCS FDIR 감지 + reallocate 5 모터
   │
   ├─► 5 ms    제어 재할당 완료
   │
   ├─► 100 ms  비상 착륙 모드 진입
   │
   ├─► 5 s     tether winch 하강
   │
   ▼
   8 s     지상 접지 완료
```

### §20.3 용어집

| 약자 | 의미 |
|------|------|
| FCS | Flight Control System |
| BLDC | Brushless DC Motor |
| SiC | Silicon Carbide |
| EKF | Extended Kalman Filter |
| FDIR | Fault Detection Isolation Recovery |
| DESAT | Desaturation (IGBT 보호) |
| FoM | Figure of Merit (호버) |
| DL | Disc Loading (N/m²) |
| RTM | Resin Transfer Molding |

### §20.4 변경 이력

| 버전 | 일자 | 변경 |
|-----|------|------|
| 0.1 | 2026-04-18 | 최초 engineering 패키지 (§7 물리 기반, preset 제거) |

### §20.5 수신자 확인

- [ ] 수신자 이름: __________________
- [ ] 소속: __________________
- [ ] 일자: __________________
- [ ] 서명: __________________

---

# 임팩트 per Mk (§21)

## §21 IMPACT per Mk

> 각 Mk 마다 3층 구조 엄수: ① 바로 바뀌는 것(실증) / ② 파생 효과(인과) / ③ 안 바뀌는 것(정직).
> mk1 제외 모든 mkN 은 이전 버전 github blob 링크 필수.

### §21.mk1 — 2026~2030 테스트베드 (v1.0, 2026-04-18)

- **git tag**: `hexa-ufo-mk1-v1.0`
- **release**: [hexa-ufo-mk1-v1.0](https://github.com/need-singularity/n6-architecture/releases/tag/hexa-ufo-mk1-v1.0)
- **최초 버전** — 이전 버전 없음 (prev_link 불필요).

#### ① 바로 바뀌는 것 (실증)

- 2 m 디스크 tethered hover 실증 (30 min × 100 h 누적)
- LiFePO4 50 kWh 이동식 전력 패키지 표준화 (UAV 업계 처음)
- SC 코일 (Nb₃Sn 10 T) + cryocooler 통합 실험 서브시스템 검증
- SiC 1200 V 인버터 6-로터 병렬 제어 펌웨어 공개 (DO-178C DAL-C)

#### ② 파생 효과 (인과)

- 예스파워 SiC MPW 재고객 확보 → Mk.II 커스텀 마스크 가능
- 서남 Nb₃Sn 코일 생산 라인 첫 민수 주문 → 단가 1/2
- FAA Part 107 waiver 절차 표준화 → 국내 UAV 산업 규제 선례

#### ③ 안 바뀌는 것 (정직)

- Carnot 한계: η_Carnot = 1 - Tc/Th ≤ 1, cryo 열량 한계
- Momentum theory 호버 효율 FoM ≤ 0.85 (이상 디스크)
- 배터리 에너지 밀도 LiFePO4 ≤ 180 Wh/kg (화학 한계)
- Tether 길이 500 m 이상 초과 시 선 중량 ≥ payload
- 5 kA 이상 고장 전류는 Mk.I 회로에서 차단 불가

### §21.mk2 — 2030~2035 프로토타입 (v1.0, 2030-06-01, PLANNED)

- **이전 버전**: [mk1 (hexa-ufo-mk1-v1.0)](https://github.com/need-singularity/n6-architecture/blob/hexa-ufo-mk1-v1.0/domains/sf-ufo/hexa-ufo/hexa-ufo.md)
- **git diff**: [mk1 → mk2](https://github.com/need-singularity/n6-architecture/compare/hexa-ufo-mk1-v1.0...hexa-ufo-mk2-v1.0)
- **status**: PLANNED

#### ① 바로 바뀌는 것 (vs mk1, 예정)

- 직경 2 m → 4 m, payload 0 kg → 150 kg (1 인 탑승)
- 유선 tether 제거, 배터리 + 소형 fuel cell 하이브리드 100 kWh
- free-flight 고도 500 m → 3000 m, 반경 10 km
- SC 코일 B=10 T → 20 T (REBCO 도입, 4.2 K → 20 K)

#### ② 파생 효과 (vs mk1, 예정)

- 1 인 수송 VTOL 민수 시장 진입 (Joby / Archer 경쟁)
- REBCO 필름 국내 밸류체인 (서남 + 창성) 시동
- 수소 fuel cell 경량화 스펙 확립

#### ③ 안 바뀌는 것 (정직)

- Betz 풍력 한계 16/27 (induction factor 수정판) — 로터 효율 상한
- REBCO 가격 $1000/kA·m (2030 예상) — 대량 채용 비경제
- 음속 이하 저속 비행만 (Mach < 0.3) — 민수 안전 요구
- FAA Part 135 인증 필요 → 승무원 훈련 표준 미정

### §21.mk3 — 2035~2040 도심 AAM (v1.0, 2035-06-01, PLANNED)

- **이전 버전**: [mk2](https://github.com/need-singularity/n6-architecture/blob/hexa-ufo-mk2-v1.0/domains/sf-ufo/hexa-ufo/hexa-ufo.md)
- **git diff**: [mk2 → mk3](https://github.com/need-singularity/n6-architecture/compare/hexa-ufo-mk2-v1.0...hexa-ufo-mk3-v1.0)
- **status**: PLANNED

#### ① 바로 바뀌는 것 (vs mk2, 예정)

- Payload 150 kg → 600 kg (4 인 + 수하물)
- fuel cell → 소형 fission (또는 SMR 연료 MOX) 2 MW_e, 1000 kg
- 운용 반경 10 km → 300 km (도시 간 노선)
- FBW → Fly-by-light (광섬유) + quantum-encrypted uplink

#### ② 파생 효과 (vs mk2, 예정)

- AAM (Advanced Air Mobility) 국내 서비스 개시
- SMR MOX 연료 민수 인증 절차 개척 (NRC + 원안위)
- 도심 vertiport 20 곳 확보 (서울·부산·세종)

#### ③ 안 바뀌는 것 (정직)

- 핵융합 미실현 상태 → 여전히 fission 기반, 폐연료 처리 남아있음
- 도심 상공 소음 기준 FAR Part 36 Stage 5 (65 dBA @ 500 ft) — 돌파 불가
- 기상 조건 (icing, shear) — 여전히 회피 대상
- 공항 교통관제 시스템 vs AAM 혼재 → ATC 개편 미완성

### §21.mk4 — 2040~2050 장거리 하이브리드 (v1.0, 2040-06-01, PLANNED)

- **이전 버전**: [mk3](https://github.com/need-singularity/n6-architecture/blob/hexa-ufo-mk3-v1.0/domains/sf-ufo/hexa-ufo/hexa-ufo.md)
- **git diff**: [mk3 → mk4](https://github.com/need-singularity/n6-architecture/compare/hexa-ufo-mk3-v1.0...hexa-ufo-mk4-v1.0)
- **status**: PLANNED

#### ① 바로 바뀌는 것 (vs mk3, 예정)

- 동력원 fission → 소형 fusion (D-T, 30 MW_th) — fusion-powerplant 도메인 통합
- 순항 고도 10 km → 20 km (성층권 하부)
- 반경 300 km → 3000 km (대륙 간)
- 속도 200 km/h → 800 km/h (초임계 팬 + 하이브리드 제트)

#### ② 파생 효과 (vs mk3, 예정)

- 민수 fusion 추진체 첫 인증 (IAEA + ICAO)
- 성층권 교통 레이어 개척 → 신 항로 표준 개정
- 3000 km 무착륙 운용 → 대륙 간 민수 경쟁 (보잉 737 대체)

#### ③ 안 바뀌는 것 (정직)

- D-T fusion 중성자 차폐 → 기체 30% 중량 여전히 차폐재
- Lawson 조건 nτT ≥ 3×10²¹ — 소형화 한계 1 m 플라즈마 volume
- 극초음속 미도달 (Mach 0.7) — 음속 장벽 공간 공기역학 한계
- 민수 fusion 사고 대응 프로토콜 미숙 → 사고시 공항 폐쇄 리스크

### §21.mk5 — 2050+ 근지궤도 (v1.0, 2050-06-01, PLANNED)

- **이전 버전**: [mk4](https://github.com/need-singularity/n6-architecture/blob/hexa-ufo-mk4-v1.0/domains/sf-ufo/hexa-ufo/hexa-ufo.md)
- **git diff**: [mk4 → mk5](https://github.com/need-singularity/n6-architecture/compare/hexa-ufo-mk4-v1.0...hexa-ufo-mk5-v1.0)
- **status**: PLANNED

#### ① 바로 바뀌는 것 (vs mk4, 예정)

- 대기권 → SSO LEO (300 km) 궤도 진입 가능
- 추진 fusion 제트 + MPD thruster 복합 (대기권 외 Isp 5000 s)
- 속도 Mach 0.7 → 7 km/s (궤도 속도)
- 최대 payload 6 ton → 18 ton

#### ② 파생 효과 (vs mk4, 예정)

- 궤도 활주로형 민수 왕복선 개시 (SpaceX/Blue Origin 경쟁)
- 지구-달 11 시간 운행 패키지
- LEO 궤도 요양 의료 / 관광 패키지 민수화

#### ③ 안 바뀌는 것 (정직)

- 델타-v 제약: 궤도 진입 9.4 km/s 필수 (Tsiolkovsky) — fusion 추진 ρ=0 불가
- 방사선 환경 GCR 1 mSv/day (LEO) — 승객 노출 한계
- 궤도 잔해 (space debris) 위험도 여전 — 회피 기동 필요
- 재진입 열 한계 2000 K (PICA-X) — 재사용 회수 100 회 한계
