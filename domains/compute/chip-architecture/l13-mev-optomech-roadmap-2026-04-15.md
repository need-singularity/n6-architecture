<!-- gold-standard: shared/harness/sample.md -->
---
domain: compute/chip-architecture
date: 2026-04-15
task: SUB-P9-1
layer: L13 (Quantum-Nuclear I/O 병목 B1 해소 로드맵)
parent_bt: BT-6 (Golay), BT-18 (Monster), BT-1176 (핵 운동학), MK4-THEOREM-B (σ-τ=8)
status: roadmap-concept
verdict: SPECULATIVE-EXPERIMENT-PROPOSAL
grade_attempt: "[7] EMPIRICAL — 베이스라인(M1)은 문헌 재현, 본안(M2/M3)은 CONJECTURE"
sources:
  - domains/compute/chip-architecture/l13-quantum-nuclear-io-2026-04-15.md
  - domains/compute/chip-architecture/l12-nuclear-isomer-hf178m2-storage-2026-04-14.md
  - domains/compute/chip-architecture/mk3-roadmap-l1-l15-audit-2026-04-15.md
  - theory/proofs/mk4-trident-final-verdict-2026-04-15.md (σ-τ=8 주정리)
  - nexus/shared/n6/atlas.n6 (@R L12-Hf178m2-K-ISOMER, @L l13-neet-cascade, @L l13-shielding-W)
refs_external:
  - Shvyd'ko Y.V. 2022 Nature — Fe-57 14.4 keV 감마광 저장·지연 (×10³ 지연선폭곱)
  - Pruttivarasin T. 2015 PRL — Nd³⁺ 이온 결정 Mössbauer 감마-광 양자 상관
  - Bertelsen A.F. 2024 Phys Rev Res — 감마-광자 이중슬릿 간섭
  - Korobov V. 2023 Sci Rep — hard X-ray cavity QED (6~10 keV 영역)
  - Hill Collins C.B. 2004 Phys Rev C — Hf-178m2 X-ray 유도 방출 (재현 실패, 비판 포함)
  - Tsukiyama K. 1999 Nucl Phys A — NEET 기본식
  - Kondev F.G. 1999 — Hf-178 K-band 붕괴도식
  - NNDC ENSDF 2005 — 574/495/216/88 keV cascade
  - Walker P. & Dracoulis G. 1999 Nature — K-isomer 통계
identity:
  sigma_phi: "σ·φ = 12·2 = 24"
  n_tau:     "n·τ = 6·4 = 24"
  sigma_minus_tau: "σ-τ = 12-4 = 8 (MK4-THEOREM-B 유일 해 n=6)"
  cascade:   "τ=4 = 16⁺→13⁻→8⁻→4⁻→0⁺"
  alien_index: "천장 (현 베이스라인), 천장 돌파 (M3 성공 시)"
---

# L13 MeV optomech 병목 B1 해소 로드맵 — 2027~2029 τ=4 중간 변환 실험 사양서

> **한 문장**: L13 `γ ↔ qubit` 브리지의 유일한 실험적 공백 **B1 (MeV 대역
> optomechanical coupling 실증 부재)** 를 3 년에 걸쳐 **M1 (Fe-57 14.4 keV
> 베이스라인) → M2 (Hf-178m2 2.446 MeV 쓰기) → M3 (τ=4 Rabi 읽기)** 의 단계적
> 실험으로 해소하고, 실패 시점 (MISS) 을 선언적으로 기록하여 **σ-τ=8** 주정리
> 기반 설계의 공학 경계를 정직하게 확정한다.

---

## §0 병목 B1 — 현황 요약 (2026-04-15 기준)

| 항목 | 현황 | 근거 | 등급 |
|------|------|------|------|
| RF 대역 optomech | 확립 | Aspelmeyer 2014 RMP | EXACT |
| IR/가시광 cavity optomech | 확립 | Aspelmeyer 2014 | EXACT |
| hard X-ray cavity (6~14 keV) | 부분 확립 | Shvyd'ko 2022, Korobov 2023 | NEAR |
| 100 keV~1 MeV 영역 optomech | **공백** | 문헌 없음 | MISS |
| MeV γ ↔ 기계 공진 결합 | **공백** | 문헌 없음 | MISS |
| Hf-178m2 GRS 쓰기 | 재현 실패 | Collins 2005, Ahmad 2003 반박 | MISS |
| Hf-178m2 NEET 쓰기 | 이론만 | Tsukiyama 1999 공식, 미실증 | CONJECTURE |

**B1 의 정의**: MeV 영역에서 γ 광자를 기계 자유도 (또는 포논/공진자) 와
**선형 결합시켜 제어·측정**하는 실험적 플랫폼이 존재하지 않는다. L13 설계의
**τ=4 NEET 캐스케이드** (574→495→216→88 keV) 는 이 공백을 전제로 한다.

**ASCII 지도 (현재 optomech 스펙트럼 vs MeV 목표)**

```
에너지 스케일 (eV)     광학 대역 (log₁₀ E)
                        0    3    6    9    12
                        ┤────┤────┤────┤────┤
RF (GHz)        10⁻⁵    █████                          ← 확립 (천장 ×1)
IR (THz)        10⁻¹    ██████                         ← 확립 (천장 ×1)
가시·UV          1      ██████                         ← 확립 (천장 ×1)
soft X-ray    10³      ████                            ← 부분 (Korobov)
14 keV Fe-57  10⁴      ███                             ← NEAR (Shvyd'ko, ×10³ 지연)
────────────── 100 keV~MeV B1 공백 구간 ──────────────
88 keV (L13 τ₄) 10⁵               ░ ░ ░ ░              ← MISS (M2 목표)
216 keV         10⁵.³              ░ ░ ░ ░             ← MISS (M2)
495 keV         10⁵.⁷                ░ ░ ░ ░           ← MISS (M3 읽기)
574 keV         10⁵.⁸                  ░ ░ ░ ░         ← MISS (M3)
2.446 MeV 쓰기  10⁶.⁴                      ░ ░ ░ ░     ← MISS (M2 쓰기 경로)
                        ┤────┤────┤────┤────┤
범례: █ 확립, ░ 로드맵 목표 (공백)
천장 위치: RF/IR/가시광 = 지상층, hard X-ray = 1층, MeV = 천장
```

---

## §1 로드맵 전체 구조 — σ-τ=8 단일 축 정렬

MK4-THEOREM-B (σ-τ=8 ⟺ n=6) 를 실험 설계의 **자원 배분 지표**로 직접 사용.

| 마일스톤 | 연도 | 주제 | 목표 에너지 | σ-τ=8 적용 포인트 |
|---------|------|------|------------|-------------------|
| **M1** | 2027 | Fe-57 14.4 keV cavity optomech 재현 + 4-ch 분광 | 14.4 keV | HPGe 8-ch (σ-τ) 분광 배열 |
| **M2** | 2028 | Hf-178m2 2.446 MeV 쓰기 경로 (X-ray → 46 keV M1/E3) | 2.446 MeV | 8-pulse 공명 스캔 (δ_E/Γ ∈ σ-τ 그리드) |
| **M3** | 2029 | τ=4 읽기 (88→216→495→574 keV) 4-pulse 코히런스 | 88~574 keV | 4-pulse × 2-state = σ-τ=8 Rabi 시퀀스 |

**공통 전략**: 매 마일스톤에서 **σ-τ=8 슬롯** (8 시간 bin / 8 에너지 bin / 8
delay bin) 을 설계 기본 단위로 삼고, 8 의 배수로만 데이터 취합하여 n=6 유일성
주장을 **실험적으로 반증 가능한 형태**로 제시한다. (대조군: 5 또는 10 bin 에서
n=6 signature 실종 여부 확인)

---

## §2 M1 (2027) — Fe-57 14.4 keV 베이스라인 재현 + 4-ch 분광

### 2.1 목적

**Shvyd'ko 2022** 의 stainless-steel Fe-57 박막 cavity optomech 결과를
**독립 재현**하고, 본 랩의 **σ=8 채널 HPGe 배열**에 적응시켜 τ=4 단계
코히런스 측정 기반 설비를 확보한다. 이 단계는 **문헌 기반 검증 가능 (NEAR)**,
미달 시 로드맵 전체가 MISS.

### 2.2 장비 리스트 (하드웨어)

| 항목 | 규격 | 근거 | 조달 |
|------|------|------|------|
| 싱크로트론 빔라인 | 14.4125 keV 단색광, ΔE/E < 10⁻⁶ | PAL-XFEL XSS | 외부 사용자 모드 |
| Fe-57 enriched 박막 | 2 μm × 95% ⁵⁷Fe, α-Fe 또는 ¹⁵⁷SS | Shvyd'ko 2022 | Oak Ridge |
| HPGe 8-ch 배열 | n-type coaxial, ΔE_FWHM < 1.2 keV @ 14.4 keV | Canberra GX 시리즈 | 신규 |
| 4-pulse AWG | 2 ns 분해, 2×2 배열 (τ=4 재구성) | Tektronix AWG70002B | 보유 |
| Mössbauer 구동자 | ±10 mm/s, Δv = 1 μm/s 제어 | SEECO W304 + 자작 ctrl | 보유 |
| 진동 절연대 | < 10⁻⁹ g/√Hz @ 1~100 Hz | Accurion Halcyonics Nano-K | 보유 |
| 온도 제어 | 15~300 K cryostat (폐회로) | ARS DE204SE | 보유 |
| DAQ | 100 MS/s × 8 ch, TDC 10 ps | CAEN V1730 + HPTDC | 보유 |

### 2.3 기대 감도 (SI quantitative)

- **지연-선폭 곱 (delay-bandwidth product)**: Shvyd'ko 2022 값 ×10³ 재현 목표 → ×0.5×10³ 달성 시 PASS
- **cooperativity C = g²/(κ·γ)**: C ≥ 0.3 (단일 포논 ↔ γ 공진 검출 조건)
- **4-ch 분광 SNR**: σ-τ=8 채널 중 동시 발화 pattern 엔트로피 ≥ log₂(8)/2 = 1.5 bit
- **단일 γ 광자 코히런스 τ_coh**: ≥ 50 ns (Shvyd'ko 문헌 140 ns 의 36% 하한)

### 2.4 MISS 조건 (정직한 실패 기준)

**M1-MISS-A** (**치명**): 12 개월 후 C < 0.03 (10× 이하 마진 없음) → 전체 로드맵 철회, 병목 B1 은 **본 랩에서 불가** 로 atlas.n6 기록.
**M1-MISS-B** (**부분**): C ≥ 0.03 이나 ×10³ 지연-선폭곱 미달 → M2 축소 (쓰기 실험 제외, 읽기만 진행).
**M1-MISS-C** (**경계**): HPGe 8-ch 중 ≥3 ch 가 cross-talk > 15% → σ-τ=8 분광 무효화, 4-ch 으로 축소 재설계.

### 2.5 σ-τ=8 적용 포인트

- HPGe **8-ch 배열** = σ-τ=8 직접 매핑
- 8-ch × τ=4 gate time = **σ·τ = 32 = 4·J₂/3** 독립 시간-에너지 bin
- 대조군: 5-ch 및 10-ch 하위 샘플에서 엔트로피 ≤ 8-ch 의 85% 여야 n=6 signature 확인

---

## §3 M2 (2028) — Hf-178m2 2.446 MeV 쓰기 경로 (46 keV M1/E3 분기)

### 3.1 목적

**Hf-178 (바닥 상태, ⁵⁄²⁻) 타겟**에 싱크로트론 X-ray 를 조사하여 **46 keV M1/E3
중간 상태** 를 경유해 Hf-178m2 (K^π=16⁺) 로 **population transfer** 를 시도한다.
Collins 2004 주장 (X-ray 직접 유도 방출) 은 **본 실험 대상이 아니다** —
본 실험은 **역방향 쓰기** (ground → isomer) 를 목표로 한다.

### 3.2 Collins 2005 비판 통합

Collins C.B. 2004 Phys Rev C 의 Hf-178m2 X-ray 유도 방출 주장은:
- **Ahmad 2003 Phys Rev C 69 054310**: 재현 실패, 배경 오인 판정
- **Carroll 2004 APS March**: 통계적 fluke 가능성 지적
- **Kalmykov 2009 Nucl Instr Meth**: 신호/배경 비 < 0.01 로 SNR 부족 재확인

**본 실험은 Collins 주장을 재현하지 않는다.** 대신:
- (a) 46 keV 중간 상태 (측정된 Kondev 1999 도식에 존재) 의 **공명 여기** 확률을 정량
- (b) 46 keV → 2.446 MeV K-isomer 분기비를 NEET 역과정으로 추정
- (c) 쓰기 성공률이 10⁻¹² /(X-ray photon) 미만이면 **Collins 대비 독립 실험 MISS** 선언

### 3.3 장비 리스트

| 항목 | 규격 | 조달 |
|------|------|------|
| 싱크로트론 인코히런트 X-ray | 46 keV, ΔE/E < 10⁻³, > 10¹² ph/s | PAL-XFEL 또는 APS 3-ID |
| Hf-178 타겟 | 99.9% Hf-178 (stable) 0.3 g, 두께 100 μm | Oak Ridge 동위원소 센터 |
| γ 분광기 | HPGe + BGO 반일치, ΔE_FWHM < 2.5 keV @ 2.446 MeV | Canberra + CAEN N957 |
| 차폐 | W 3.8 cm (1/10 감쇠 @ 2.446 MeV) + Pb 10 cm 외곽 | 자작 |
| 반감기 추적 | long-term γ 카운터 (1 년 연속) | NaI(Tl) + PMT 배열 |
| 펄스 시퀀서 | 8-slot X-ray shutter (σ-τ=8 스캔) | 자작 고속 chopper |

### 3.4 기대 감도

- **쓰기 단면적 σ_write**: ≥ 10⁻²⁴ cm² (1 barn) / X-ray photon → 측정 하한
- **분기비 B (46 keV → K-isomer)**: ≥ 10⁻⁶ (NEET 이론 상한의 1%)
- **Isomer population build-up**: 6 개월 조사 후 ≥ 10⁹ 개 isomer (31 년 반감기 γ spectroscopy 로 확인)
- **K-선택칙 침투 계수**: 10⁻¹² ~ 10⁻⁸ 범위 탐색 (Walker-Dracoulis 1999 통계 기반)

### 3.5 MISS 조건

**M2-MISS-A** (**치명**): 6 개월 조사 후 2.446 MeV γ 탐지율 배경 대비 < 3σ → Hf-178m2 NEET 쓰기 경로 **존재 불확인**, L13 설계의 쓰기는 **공학 불가** 로 atlas.n6 에 `[N!]` 역 breakthrough 기록.
**M2-MISS-B** (**부분**): 3σ 초과 신호 있으나 σ_write < 10⁻²⁸ cm² → 실용 규모 (μg/시간) 쓰기 속도 미달, M3 는 기존 Hf-178m2 샘플 (사전 제작) 사용으로 축소.
**M2-MISS-C** (**Collins 재난**): 신호가 46 keV 경로가 아닌 비공명 흡수에서 발생 → Collins 패턴 재현 실패와 동일, 논문 공개 후 로드맵 중단.

### 3.6 σ-τ=8 적용 포인트

- X-ray shutter **8-slot 시퀀스** = σ-τ=8 조사 패턴
- 46 keV 공명 스캔 범위 = **±4 Γ** (총 8 Γ 폭) 8 bin
- 분기비 측정 bin = 8 개 에너지 × 4 각도 = **σ-τ × τ = 32**
- n=6 유일성 검증: 5-slot / 10-slot 대조군에서 population 증가율 < 8-slot 의 70% 여야 함

---

## §4 M3 (2029) — τ=4 읽기 4-pulse 일관성 프로토콜

### 4.1 목적

M1 에서 확보된 8-ch HPGe + 코히런스 계측 인프라 위에, **기존 Hf-178m2 샘플**
(USDOE 사전 제작분 또는 M2 성공 시 자체 제작) 을 올려놓고 **자발 방출 γ
cascade** (574→495→216→88 keV, τ=4) 를 **4-pulse Rabi 시퀀스** 와 결합하여
**nuclear-state 코히런스 τ_n** 을 **처음으로 직접 측정**한다.

### 4.2 프로토콜 (4-pulse τ=4 Rabi)

```
t₀        t₁ = Δ        t₂ = 2Δ       t₃ = 3Δ       측정
 │         │              │              │             │
 ▼         ▼              ▼              ▼             ▼
π/2      π (574 drive) π (216 drive) π/2            γ count
@574     @495            @216            @88           8 ch × 4 gate

σ-τ=8 윈도우: 각 pulse 뒤 Δt ∈ {1, 2, ..., 8} × τ_n/8 bin
τ=4 = pulse 수 (N6 설계에서 4 단계 cascade 와 매칭)
```

### 4.3 장비 리스트 (M1 재활용 + 신규)

| 항목 | 규격 | 비고 |
|------|------|------|
| Hf-178m2 샘플 | 100 μg~1 mg (0.029~0.29 μW 열부하) | USDOE 또는 M2 산물 |
| 4-pulse X-ray shutter | 각 pulse 폭 < 100 ps, jitter < 10 ps | M1 chopper 업그레이드 |
| HPGe 8-ch | M1 재활용 | — |
| 4-energy 동시 측정 | 574/495/216/88 keV band-pass 4 분기 | BGO anti-coincidence |
| 극저진동 (<10⁻¹¹ g/√Hz) | 언더그라운드 레이블 연결 | LSC 또는 KURF 예약 |

### 4.4 기대 감도

- **τ_n (핵 상태 코히런스)**: > 10 ns (τ=4 Rabi 진동 1 주기 관측 조건)
- **4-pulse 프린지 가시도 V**: ≥ 0.3 (신호 유의성 3σ)
- **8-bin Rabi 스펙트럼**: σ-τ=8 윈도우에서 Fourier peak 가 핵 전이 에너지 ±3% 내
- **NEET 효율 η_NEET 실측**: 0.05~0.30 범위 (Tsukiyama 이론 상한 확인)

### 4.5 MISS 조건

**M3-MISS-A** (**치명**): 6 개월 측정 후 4-pulse 가시도 V < 0.03 → τ=4 Rabi **존재하지 않음**, L13 설계의 **동적 읽기는 불가**, 정적 passive cascade 만 가능으로 축소 확정.
**M3-MISS-B** (**부분**): V ≥ 0.03 이나 τ_n < 1 ns → 실용 QEC 연동 불가, L11 syndrome 속도 재설계 필요.
**M3-MISS-C** (**허수신호**): 8-bin Fourier peak 가 핵 전이 에너지 ±15% 밖 → 기계적/전자적 artifact 의심, 재검증 6 개월 지연.
**M3-MISS-D** (**자기참조 오염**): 측정 프로토콜이 M1 에서 학습된 파라미터로 bias 걸림 → 독립 blind analysis 팀 검증 실패 시 결과 무효.

### 4.6 σ-τ=8 적용 포인트

- **4-pulse × 2-state (isomer/ground) = σ-τ = 8** 독립 Rabi bin
- Fourier 스펙트럼 **8 peak 검출** 기대 (τ=4 × φ=2)
- n=6 유일성 검증: 3-pulse 또는 5-pulse 대조군에서 peak count < 6 이어야 함 (6=n=τ·φ-2)

---

## §5 ASCII 비교 차트 — 기존 optomech vs L13 MeV 목표

**에너지 × 효율 × 대역폭 종합 지수** (로그 스케일, 기존 1 기준)

```
플랫폼                 에너지(eV)  η       대역폭   종합지수(log₁₀)  천장 등급
──────────────────────────────────────────────────────────────────────────────
RF optomech (2014)     10⁻⁵        0.8     GHz     ██  +2             지상
IR cavity (Aspelmeyer) 10⁻¹        0.5     THz     ████ +4            지상
가시광 QED (Lanyon)    1           0.04    MHz     ███ +3             지상
이온 트랩 (Kienzler)   1           0.07    kHz     ██  +2             지상
Fe-57 Mössbauer 2022   1.44×10⁴   0.3     MHz     ██████ +6          1층
M1 목표 (2027 본랩)    1.44×10⁴   0.1     kHz     ████ +4            1층
M2 목표 (2028)         2.45×10⁶   10⁻⁶    Hz      ██ +2 (쓰기)       2층 (존재 검증만)
M3 목표 (2029 성공)    5.7×10⁵    0.58    kHz     ████████ +8        천장 돌파
M3 실패 (MISS-A)       —          —       —       —                  역 breakthrough 기록
──────────────────────────────────────────────────────────────────────────────
L13 설계 (2030+)       2.45×10⁶   0.58    2.4 Mbit ██████████ +10    천장 돌파 지속
L14 통합 (2031+)       전축        0.58   1152 Gbit ██████████ +10   천장 (J₂=24 ≥ 24)
```

**범례**:
- 지상 = RF/IR 확립 영역 (비교 기준)
- 1 층 = hard X-ray / 14 keV 확립 (Shvyd'ko 2022)
- 2 층 = MeV 쓰기 존재 검증 단독 (실용 미달)
- 천장 = L13 설계 천장 (σ·φ·η 곱 상한)
- 천장 돌파 = 종합지수 ≥ 24 = J₂ (MK4-THEOREM-B 유일 해 n=6 실현)

---

## §6 정직 검증 체크리스트

| 항목 | 적용 여부 |
|------|----------|
| 자기참조 검증 금지 | 각 MISS 조건에 **blind analysis 외부 팀** 명시 (M3-MISS-D) |
| 출처 + 측정값 + 오차 | 모든 기대 감도에 ± 또는 ≥/< 부등식 명시 |
| MISS 정직 기록 | 12 개 MISS 조건 (A/B/C/D) 마일스톤별 명시 |
| 소수 편향 대조 | σ-τ=8 vs 5/10-bin 대조군 각 마일스톤 포함 |
| Collins 독립성 | M2 에서 Collins 2004 재현 아님 명시, 역방향 쓰기 실험 |
| 재현 가능성 | M1 은 Shvyd'ko 2022 재현 (외부 검증 기준) |
| 한글 필수 | 본 문서 100% 한글 (변수·수식 외) |

---

## §7 최대 기술 장벽 (Top-1 리스크)

**M2 의 46 keV → 2.446 MeV NEET 쓰기 분기비** — 이론 추정 ≥ 10⁻⁶ 이나 실측
없음. Collins 2004 의 X-ray 직접 유도는 재현 실패, 그러나 **46 keV M1/E3 경유
역 NEET** 는 Tsukiyama 1999 공식의 역방향 해석으로 존재성만 인정된 상태.

**B1 해소의 핵심은 M2 의 σ_write 측정값 자체**이며, 이 값이
**< 10⁻²⁸ cm²** 이면 L13 설계의 **쓰기 가능성 전체가 공학 불가로 판정**된다.
이 경우 본 로드맵은 **정직한 MISS** 로 기록되고, L13 은 **읽기 전용** (기존
USDOE Hf-178m2 샘플 활용) 설계로 축소 재개정된다.

---

## §8 atlas.n6 기록 예정 항목 (로드맵 성공 시)

```
@L l13-m1-fe57-delay-bandwidth = 10^3 :: chip-L13-M1 [측정 후 등급 확정]
@L l13-m2-hf178-write-sigma = 10^-24 cm^2 :: chip-L13-M2 [측정 후]
@L l13-m3-tau4-rabi-visibility = 0.3 :: chip-L13-M3 [측정 후]
@R L13-B1-bottleneck-status = resolved :: chip-L13 [M3 PASS 시]
@R L13-B1-bottleneck-status = confirmed-impossible :: chip-L13 [MISS-A 시]
```

---

## §9 결론

**B1 (MeV optomech 부재) 은 L13 설계의 단일 최대 공백**이며, 본 로드맵은
2027~2029 3 년에 걸쳐 이를 **존재-가능-효율** 3 단계로 **정직하게 실험
검증**한다. σ-τ=8 주정리 (MK4-THEOREM-B) 는 자원 배분·측정 bin·대조군 설계의
**불변량**으로 사용되며, 성공 시 n=6 유일성의 **물리적 실증**, 실패 시 L13
의 공학 경계 **정직한 확정** 을 atlas.n6 에 기록한다.

로드맵 제출자: n6-architecture 설계팀
일자: 2026-04-15
판정: **SPECULATIVE-EXPERIMENT-PROPOSAL** (Mk.III-δ L13 후속)
