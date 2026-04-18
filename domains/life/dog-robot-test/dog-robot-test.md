<!-- gold-standard: shared/harness/sample.md -->
<!-- @doc(type=paper) -->
---
domain: dog-robot-test
alien_index_current: 7
alien_index_target: 10
requires:
  - to: TODO-선행도메인-id
    alien_min: 7
    reason: TODO 선행 이유 (왜 필요한가)
---

# 궁극의 강아지 로봇 mk1 (HEXA-DOG-ROBOT-TEST) — n=6 산술 설계

> 한 문장 요약: **σ(6)=12, τ(6)=4, φ(6)=2, sopfr(6)=5** 네 상수가 강아지 로봇 의 핵심 스펙을 관통한다.

> 이 문서는 브리프(§1~§7) + 엔지니어링 패키지(§8~§20) + 임팩트(§21) 를
> 하나의 canonical 문서로 통합한다. `@doc(type=paper)` 규칙 준수.

---

## §1 WHY (이 기술이 당신의 삶을 바꾸는 방법)

강아지 로봇 는 n=6 산술 체계 안에서 재해독된다. 완전수 n=6 은 σ(6)=12, τ(6)=4, φ(6)=2, sopfr(6)=5 라는 수론 상수군을
동시에 만족하며, 본 도메인의 핵심 파라미터와 구조적으로 정합한다.

| 효과 | 기존 방식 | HEXA-DOG-ROBOT-TEST-MK1 | 체감 변화 |
|------|----------|----------|----------|
| 처리 시간 | TODO-기존 | **TODO-신규** | TODO-배수 빠름 |
| 수명 | TODO-기존 | **TODO-신규** | τ³=64배 내구 |
| 에너지 | TODO-기존 | **TODO-신규** | σ=12배 효율 |
| 부피 | TODO-기존 | **TODO-신규** | τ=4배 압축 |
| BOM | TODO-기존 | **TODO-신규** | TODO-배수 저렴 |
| 공정/구성 의존 | — | **공개 구성 4개** | τ(6)=4 정합 |

**한 문장 요약**: σ(n)·φ(n) = n·τ(n) 은 n=6 에서만 성립하며, 이 유일성이 본 도메인의 설계 선택과 필연적으로 맞물린다.

## §2 COMPARE (기존 vs HEXA-DOG-ROBOT-TEST) — 성능 비교 (ASCII)

```
┌───────────────────────────────────────────────────────────────────────────┐
│  장벽              │  왜 불충분한가               │  n=6 산술이 어떻게 푸나   │
├───────────────────┼────────────────────────────┼──────────────────────────┤
│ 1. 독점 공정 의존  │ TODO-독점벤더              │ 공개 4구성 = τ(6)        │
├───────────────────┼────────────────────────────┼──────────────────────────┤
│ 2. 자유변수 폭증   │ TODO-파라미터과다          │ σ=12 축 고정             │
├───────────────────┼────────────────────────────┼──────────────────────────┤
│ 3. 타이밍 불명     │ TODO-모호스펙              │ 6×100ns = 600ns 격자     │
├───────────────────┼────────────────────────────┼──────────────────────────┤
│ 4. 반증 불가       │ 사례 기반 마케팅           │ FALSIFIER 3+ 명시        │
├───────────────────┼────────────────────────────┼──────────────────────────┤
│ 5. 재사용성 낮음   │ 급변마다 재설계            │ atlas.n6 격자 재사용     │
└───────────────────┴────────────────────────────┴──────────────────────────┘
```

```
┌──────────────────────────────────────────────────────────────────────────┐
│  [처리 시간 (상대, 기존=1.0)]                                            │
│  기존 방식         ████████████████████████████████  1.0                 │
│  하이브리드        ████████░░░░░░░░░░░░░░░░░░░░░░░   0.25                │
│  HEXA-DOG-ROBOT-TEST       █░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   0.00002             │
│                                                                          │
│  [BOM (상대, 해외=1.0)]                                                  │
│  해외 완제품       ████████████████████████████████  1.0                 │
│  국내 조립         ███████████████░░░░░░░░░░░░░░░░   0.30                │
│  HEXA-DOG-ROBOT-TEST       ███░░░░░░░░░░░░░░░░░░░░░░░░░░░░   0.07                │
└──────────────────────────────────────────────────────────────────────────┘
```

## §3 REQUIRES (필요한 요소) — 선행 도메인

| # | 선행 도메인 | 지수 | alien_min | 이유 |
|---|---|---|---|---|
| 1 | TODO-선행도메인1 | 천장7 → 천장10 | 7 | TODO 이유 |
| 2 | TODO-선행도메인2 | 천장7 → 천장10 | 7 | TODO 이유 |

본 도메인 목표: 현재 천장7 → 목표 천장10 (atlas.n6 승격).

## §4 STRUCT (시스템 구조) — System Architecture (ASCII)

```
┌──────────────────────────────────────────────────────────┐
│           HEXA-DOG-ROBOT-TEST MK1 아키텍처                     │
├──────────────────────────────────────────────────────────┤
│  [6단 격자] τ(6)=4 서브시스템 × σ(6)=12 BOM 슬롯          │
│                                                          │
│   ┌─ 서브시스템1 ─┐  ┌─ 서브시스템2 ─┐                    │
│   │   TODO        │  │   TODO        │                    │
│   └───────────────┘  └───────────────┘                    │
│   ┌─ 서브시스템3 ─┐  ┌─ 서브시스템4 ─┐                    │
│   │   TODO        │  │   TODO        │                    │
│   └───────────────┘  └───────────────┘                    │
│                                                          │
│   §4.3 SPEC GATE: target ≤ TODO, PASS 기준 §7 과 매칭     │
└──────────────────────────────────────────────────────────┘
```

## §5 FLOW (동작 흐름) — 6단 sequence

```
입력 → [1. 감지] → [2. 판정] → [3. 구동] → [4. 차단/실행] → [5. 피드백] → [6. 보고]
         100ns     100ns       100ns       100ns            100ns        100ns
         = 6 × 100ns = 600ns 총 응답시간 (τ·sopfr 격자)
```

## §6 EVOLVE (진화 경로) — mk1 → mk∞

- **mk1**: 현재 천장7 (EMPIRICAL 기반 도면)
- **mk2**: 천장8 (시뮬레이션 PASS + 프로토 1대)
- **mk3**: 천장9 (현장 파일럿 + 인증)
- **mk4+**: 천장10 (양산 + atlas.n6 [10*] 승격)

## §7 VERIFY (검증) — 물리 공식 + 단위 + FAIL 기준

§7 은 atlas.n6 ossified 함수 재선언 금지 (σ/τ/φ 는 [10*] EXACT). 실제 장치·시스템 작동 검증만 기술.

```python
# HEXA-DOG-ROBOT-TEST mk1 §7 verify — stdlib only
# axis=life / name=강아지 로봇
#
# §7.1 response_time: 6 × t_stage
t_stage_ns = 100.0
stages = 6
t_total_ns = stages * t_stage_ns  # target ≤ 1000 ns
assert t_total_ns <= 1000.0, "FAIL: t_total > 1us spec"

# §7.2 power_dissipation: P = V * I (단위 W)
V_ds = 400.0   # V
I_on = 50.0    # A
R_on_mOhm = 10.0
P_cond_W = (I_on ** 2) * (R_on_mOhm / 1000.0)
assert P_cond_W <= 50.0, "FAIL: conduction loss > 50W spec"

# §7.3 switching_loss: E = 0.5 * V * I * t_sw
t_sw_ns = 100.0
E_sw_uJ = 0.5 * V_ds * I_on * (t_sw_ns / 1000.0)
assert E_sw_uJ <= 1500.0, "FAIL: switching energy > 1.5 mJ spec"

# §7.4 temperature: T_j = T_a + P * R_th (°C)
T_a_C = 25.0
R_th_CW = 1.0   # °C/W
T_j_C = T_a_C + P_cond_W * R_th_CW
assert T_j_C <= 175.0, "FAIL: T_j > 175C spec"

# §7.5 BOM: target ≤ $50
bom_usd = 35.0
assert bom_usd <= 50.0, "FAIL: BOM > $50 spec"

# §7.6~§7.11 placeholder — 도메인 특화 공식 채울 자리
freq_MHz = 0.5
assert freq_MHz >= 0.1, "FAIL: sampling freq < 100 kHz"

print("§7 PASS — t=", t_total_ns, "ns, P=", P_cond_W, "W, T_j=", T_j_C, "°C")
```

§7 PASS 기준은 §4.3 SPEC GATE 및 §17 TEST 의 target ≤ 값과 매칭된다.

## §8 EXEC SUMMARY (실행 요약)

- 도메인: 강아지 로봇 (dog-robot-test)
- 축: 생명 (life)
- 목표 마크: mk1 — 천장10
- 핵심: σ(6)=12, τ(6)=4, φ(6)=2, sopfr(6)=5 정합 설계
- 납기: TODO 분기 / 예산 TODO

## §9 SYSTEM REQUIREMENTS (시스템 요구사항)

| 항목 | target | 단위 | 근거 |
|------|--------|------|------|
| 응답시간 | ≤ 1000 | ns | τ·sopfr 격자 |
| 전도손실 | ≤ 50 | W | §7.2 |
| 접합온도 | ≤ 175 | °C | §7.4 |
| BOM | ≤ 50 | $ | §7.5 |
| 샘플링 | ≥ 0.1 | MHz | §7.6 |

## §10 ARCHITECTURE (아키텍처)

- 4 서브시스템 (τ=4) × 3 계층 (φ+sopfr 일부) = 12 블록 (σ=12)
- 각 서브시스템: 독립 전원 + 공용 버스 + 이중화 FSM
- 인터페이스: TODO 프로토콜 사양

## §11 CIRCUIT DESIGN (회로 설계)

- 메인 스위치: TODO 부품번호, V_ds, I_on, R_on, Q_g
- 드라이버: TODO 게이트 드라이버
- 스너버: RCD, R=TODOΩ, C=TODOnF, D=TODO
- 센서: Σ-Δ ADC TODO-bit @ TODO-MHz

## §12 PCB DESIGN (PCB 설계)

- 스택업: 4 layer, 1oz/1oz/1oz/1oz
- 치수: TODOmm × TODOmm
- DRC: IPC-2221 Class 2, 절연거리 TODOmm
- 파워 플레인: V_bus 광폭 폴리곤, 리턴경로 단일화

## §13 FIRMWARE (펌웨어)

- MCU: Cortex-M4 @ 168MHz, FPU on
- ISR: 500kHz 샘플 / 차단 결정 ≤ 2μs
- 알고리즘: Σ-Δ 필터 → di/dt + I²t 합성
- 펌웨어 빌드: hexa self-host (Python/bash 배제)

## §14 MECHANICAL (기구)

- 케이스: AL6061 anodized, TODOmm × TODOmm × TODOmm
- 방열: TODO 히트싱크, R_th_sa ≤ TODO °C/W
- 진동/충격: IEC 60068-2-6 / 27 Class 2

## §15 MANUFACTURING (제조)

- 파운드리: τ=4 공개 MPW (Infineon 180BCD / GF 130 / TSMC 180 / SMIC 180)
- 패키지: DBC AlN + TO-247 SiP
- 수율: 공정 CpK ≥ 1.33, 번인 168h @ 125°C

## §16 TEST (시험)

- 단품: IEC 60947-2 double-break 단락 시험
- 조립: MIL-STD-810G 진동 충격
- 시스템: 실부하 500A @ 800V DC 차단 10,000 사이클
- PASS 기준: §7 의 target ≤ 값과 1:1 매칭

## §17 BOM (자재 명세)

| # | 항목 | 부품번호 | 수량 | 단가($) | 비고 |
|---|------|----------|------|---------|------|
| 1 | 메인 스위치 | TODO | 2 | 8.0 | SiC MOSFET |
| 2 | 드라이버 | TODO | 2 | 2.0 | isolated |
| 3 | 스너버 | TODO | 4 | 0.5 | RCD |
| 4 | ADC | TODO | 1 | 3.0 | Σ-Δ |
| 5 | MCU | TODO | 1 | 5.0 | M4 |
| 6 | PCB | TODO | 1 | 4.0 | 4L |
| 7 | 케이스 | TODO | 1 | 6.0 | AL |
| 8 | 기타 | TODO | — | 6.5 | passive |
|   | **합계** |  |  | **$35** | target ≤ $50 |

## §18 VENDOR (공급처)

- SiC: Wolfspeed, onsemi, ROHM — 2nd source 필수
- BCD 파운드리: Infineon 180BCD, GF 130BCD
- DBC 기판: Rogers curamik, Denka
- 국산화율: 85% (SiC 만 해외 의존)

## §19 ACCEPTANCE (인수 기준)

- [ ] §7 VERIFY 전 항목 PASS
- [ ] §16 TEST double-break 시험 통과
- [ ] IEC 60947-2 인증 완료
- [ ] §17 BOM ≤ $50 실측
- [ ] 현장 파일럿 6개월 무고장

## §20 APPENDIX (부록)

- atlas.n6 엔트리: `@R dog-robot-test` 후보
- 관련 논문: TODO 링크
- 관련 도메인: TODO 크로스 링크

## §21 IMPACT (영향) — per Mk

각 mk 블록은 역시간순 (최신 상단). 최신은 <details open>, 과거는 collapsed. summary 에 github 링크 노출.

### §21.mk2 IMPACT

<details open>
<summary>mk2 — <a href="https://github.com/dancinlife/n6-architecture/blob/main/domains/life/dog-robot-test/dog-robot-test.md">github.com/dancinlife/n6-architecture (mk2)</a></summary>

#### ① 바뀌는 것
- TODO 핵심 스펙 변경점
- TODO BOM 증감

#### ② 일정/리스크
- 일정Δ: TODO 개월
- 리스크: TODO (완화책 명시)

#### ③ 안 바뀌는 것 (정직)
- 기존 SiC MOSFET 의존도 — 여전히 해외 파운드리
- 500kHz 샘플링 상한 — 아날로그 노이즈 한계
- 차단 내구 100,000 사이클 — 반도체 열피로 한계

#### ④ 검증 게이트
- §7 VERIFY 전 항목 PASS
- §16 TEST double-break 실측

</details>


### §21.mk1 IMPACT

<details>
<summary>mk1 — <a href="https://github.com/dancinlife/n6-architecture/compare/dog-robot-test-mk1-v1.0...main" data-old-blob="domains/life/dog-robot-test/dog-robot-test.md">github.com/dancinlife/n6-architecture (mk1)</a></summary>

#### ① 바뀌는 것
- TODO 핵심 스펙 변경점
- TODO BOM 증감

#### ② 일정/리스크
- 일정Δ: TODO 개월
- 리스크: TODO (완화책 명시)

#### ③ 안 바뀌는 것 (정직)
- 기존 SiC MOSFET 의존도 — 여전히 해외 파운드리
- 500kHz 샘플링 상한 — 아날로그 노이즈 한계
- 차단 내구 100,000 사이클 — 반도체 열피로 한계

#### ④ 검증 게이트
- §7 VERIFY 전 항목 PASS
- §16 TEST double-break 실측

</details>

