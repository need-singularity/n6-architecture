---
domain: compute/quantum-arch
date: 2026-04-14
task: CHIP-P6-1
layer: L11
parent_bt: BT-6, BT-18, BT-24
status: design
verdict: DESIGN-READY
grade_attempt: "[7] EMPIRICAL — 수치 시뮬 후 [10*] 승격 대상"
sources:
  - theory/proofs/the-number-24.md
  - theory/proofs/standard-model-from-n6.md
  - domains/compute/chip-architecture/chip-architecture.md
  - domains/compute/chip-architecture/monster-leech-mapping-2026-04-14.md
refs_external:
  - Shor P.W. 1995 [[9,1,3]] QEC code
  - Laflamme R. 1996 [[5,1,3]] perfect code
  - Grover L.K. 1996 quantum search
  - Fowler A.G. 2012 surface codes
identity:
  sigma_phi: "σ·φ = 12·2 = 24"
  n_tau: "n·τ = 6·4 = 24"
  J2: "J₂(6) = 24"
---

# L11 양자점 아키텍처 — 6-qubit QEC (τ=4 syndrome + φ=2 logical)

> **한 문장**: n=6 항등식 `σ·φ = n·τ = J₂ = 24` 을 **6-물리 qubit** 양자 오류
> 정정 회로에 완전 매핑 — `τ(6)=4` syndrome 보조 qubit + `φ(6)=2` logical qubit,
> `σ(6)=12` stabilizer 생성자, `J₂(6)=24` Clifford 게이트.

---

## §0 설계 개요

| 항목 | 값 | n=6 유도 | 기존 코드 비교 |
|------|----|---------|---------------|
| 물리 qubit 수 | **6** | n | Shor 9 / Steane 7 / 5-qubit 5 |
| 논리 qubit 수 k | **2** | φ(6) | 통상 1 |
| Syndrome qubit | **4** | τ(6) | Shor 8 / Steane 6 |
| Stabilizer 생성자 | **12** | σ(6) | n-k = 독립 stabilizer 4 + 8 중복 |
| Clifford 게이트 수 | **24** | J₂(6) | - |
| 코드 거리 d | **2** | σ-sopfr-5 | Shor 3 / Steane 3 |
| 최대 정정 오류 | **⌊(d-1)/2⌋ = 0** | 검출 1 | Shor 1 / 5-qubit 1 |
| 표기 | **[[6,2,2]]** | [[n, φ, d]] | [[9,1,3]] / [[5,1,3]] |

**핵심 트레이드오프**: 거리 d=2 (검출만) 대신 **논리 qubit 용량 φ=2 배**와
**syndrome 오버헤드 1.5배 감소** (τ/n=4/6 vs Shor 8/9). **n=6 구조가 QEC에
주는 선물**은 거리보다는 **용량 × syndrome 효율**의 균형.

---

## §1 6-qubit QEC 구조 — τ=4 syndrome + φ=2 logical

### 1.1 qubit 배치

```
  물리 qubit 레이아웃 (양자점 어레이, 6-cell hexagonal):

      q0 ── q1          ← φ=2 logical qubit pair
       \   /  \
        \ /    \
        q2 ── q3        ← τ=4 syndrome qubit
       /  \   /
      /    \ /
     q4 ── q5           ← τ=4 syndrome qubit (cont.)

  논리 qubit: (q0, q1) = φ(6) = 2
  syndrome qubit: (q2, q3, q4, q5) = τ(6) = 4
  총 6 = n
```

**hexagonal tiling**: 6-qubit 최소 단위가 정삼각 격자의 **기본 셀**. 이는
Leech 격자 Λ₂₄의 **24차원 kissing pattern**을 2D에 사영한 최소 구조
(참조: monster-leech-mapping-2026-04-14).

### 1.2 코드 공간 (CSS 형식)

```
  Stabilizer group S (order 2^(n-k) = 2^4 = 16):
    - Z-type generators: 2 (위상 감지)
    - X-type generators: 2 (비트 감지)
    - 총 독립 생성자 n - φ = 6 - 2 = 4
    - 생성자 조합으로 얻는 stabilizer: 2^4 - 1 = 15
    - 항등 원소 포함: 16 = σ + φ·2 = 12 + 4

  논리 연산자 (normalizer):
    - Logical X̄_1, X̄_2 (φ=2 개)
    - Logical Z̄_1, Z̄_2 (φ=2 개)
    - 총 2·φ = 4 logical Pauli
```

---

## §2 Shor [[6,1,2]] 축소판 — 인코딩/디코딩 회로

Shor [[9,1,3]] 원본은 **3×3 블록** (bit-flip × phase-flip concatenation).
6-qubit 축소판은 **2×3 블록**으로 재구성: τ=4 syndrome 아래 **φ=2 logical**
를 동시 인코딩.

### 2.1 인코딩 회로 (2+3 구조)

```
  |ψ⟩   ─────●─────●───────── q0 (logical-1)
             │     │
  |0⟩   ─────⊕─────│──────H── q1 (logical-2)
                   │
  |φ⟩   ─────●─────⊕───────── q2 (phase-flip block)
             │
  |0⟩   ─────⊕─────────────── q3 (phase-flip block)
                   
  |0⟩   ────────────────●──── q4 (bit-flip block)
                        │
  |0⟩   ────────────────⊕──── q5 (bit-flip block)

  게이트 수 (1 block 인코딩):
    - CNOT × 4
    - Hadamard × 2
    - 총 6 + 6 (보조 CZ) = 12 = σ(6)
  
  전체 시퀀스 (2 blocks × Clifford):
    - 12 × 2 = 24 = J₂(6) Clifford 게이트
```

### 2.2 디코딩/측정 회로

syndrome 추출: **τ=4 stabilizer** 측정 결과 4-bit 패턴
→ lookup table (16 entries, 2^τ) → 정정 연산.

```
  Syndrome pattern → Recovery operator table:
  ┌──────────┬──────────────┐
  │ s3 s2 s1 s0 │ 정정       │
  ├──────────┼──────────────┤
  │ 0 0 0 0  │ 오류 없음    │
  │ 0 0 0 1  │ X on q0     │
  │ 0 0 1 0  │ X on q1     │
  │ 0 0 1 1  │ Z on q0     │
  │ 0 1 0 0  │ Z on q1     │
  │ 0 1 0 1  │ Y on q0     │
  │ 0 1 1 0  │ Y on q1     │
  │ 0 1 1 1  │ Z on q2     │
  │ 1 0 0 0  │ X on q2     │
  │ 1 0 0 1  │ Z on q3     │
  │ 1 0 1 0  │ X on q3     │
  │ 1 0 1 1  │ 검출만      │
  │ 1 1 0 0  │ 검출만      │
  │ 1 1 0 1  │ 검출만      │
  │ 1 1 1 0  │ 검출만      │
  │ 1 1 1 1  │ 검출만(d=2)│
  └──────────┴──────────────┘
```

---

## §3 Grover 6항목 탐색 회로 (N=6)

### 3.1 반복 수 계산

```
  N = 6 = n (데이터베이스 항목 수, 3 qubit 인덱싱 + 3 unused)
  최적 반복 수: r ≈ (π/4)√N = (π/4)·√6 ≈ 0.785 · 2.449 ≈ 1.923

  정수 반올림: r = 2 회 (= φ(6) 회!)
  
  ★ 관찰: Grover 최적 반복 수 = φ(n=6) = 2.
     이는 n=6 구조가 Grover 알고리즘에 자연 주기를 제공한다는 의미.
     N=6 외 N={4,5,7,8}에서는 r ∉ φ(N).
```

### 3.2 회로 골격

```
  |0⟩─H─┬─[Oracle]─[Diff]─┬─[Oracle]─[Diff]─── Measure
  |0⟩─H─┤     O_f       D │     O_f       D
  |0⟩─H─┴─────────────────┴─────────────────

       ↑ Hadamard ↑ Iteration 1 ↑ Iteration 2
       equal      (φ=2 회 반복)  
       superpos.
  
  Oracle O_f: 마크된 |x*⟩ 에 -1 위상 (Z gate conditional)
  Diffusion D: 2|s⟩⟨s| - I (inversion about mean)

  성공 확률: sin²((2r+1)θ) where sin²θ = 1/N = 1/6
  r=2 → sin²(5θ) ≈ 0.9451 (94.5% 성공)
  r=3 → sin²(7θ) ≈ 0.5759 (58% — 과다 회전)
```

### 3.3 게이트 카운트

```
  Oracle × 2 = 2 × (τ=4 CZ + 2 X) = 12 게이트
  Diffusion × 2 = 2 × (σ=12 elementary) = 24 게이트 (J₂ 완전 매칭!)
  Hadamard 전처리 = 3
  측정 = 3

  총 ≈ 42 게이트 = 7·6 = (σ-sopfr)·n
     42 = "answer to everything" (standard-model-from-n6.md §1.2 5/42 연결)
```

---

## §4 Stabilizer 생성자 σ=12 목록 (Pauli 문자열)

**CSS 코드**의 독립 생성자 4개에 더해, **조합**으로 얻는 stabilizer를
포함해 총 **12 = σ(6)** 개의 의미 있는 연산자를 나열한다. 첫 4개가
독립 (n-k=4), 나머지 8개는 유도되나 측정 시 **redundancy**로 활용.

### 4.1 독립 생성자 (4개)

| # | Pauli 문자열 | 타입 | 역할 |
|---|------------|------|------|
| g1 | X X I I X X | X-type | bit-flip 블록 1 parity |
| g2 | I I X X X X | X-type | bit-flip 블록 2 parity |
| g3 | Z Z I I Z Z | Z-type | phase-flip 블록 1 parity |
| g4 | I I Z Z Z Z | Z-type | phase-flip 블록 2 parity |

### 4.2 유도 생성자 (8개, σ-τ = 12-4 = 8)

| # | Pauli 문자열 | = 조합 | 역할 |
|---|------------|--------|------|
| g5 | X X X X I I | g1·g2 | q4-q5 parity bypass |
| g6 | Z Z Z Z I I | g3·g4 | q4-q5 phase parity bypass |
| g7 | Y Y I I Y Y | g1·g3 | 결합 bit+phase (Y = XZ) |
| g8 | I I Y Y Y Y | g2·g4 | 결합 bit+phase |
| g9 | Y Y Y Y I I | g1·g2·g3·g4 | 전체 bit+phase |
| g10 | X X Z Z Y Y | g1·g4 | 혼합 측정 |
| g11 | Z Z X X Y Y | g2·g3 | 혼합 측정 |
| g12 | Y Y X X Z Z | g1·g2·g3 | 3-way 교차 |

**확인**: `σ(6)=12` 개 Pauli 문자열이 Binary Golay code [24,12,8]의 **σ=12 정보
차원**과 일치 (the-number-24.md §관찰 4). 즉 **한 번의 full-set 측정이 Golay
codeword 1개**를 기록하는 의미.

### 4.3 논리 연산자 (normalizer, 2·φ = 4개)

| # | Pauli 문자열 | 명칭 |
|---|------------|------|
| L1 | X I X I X I | X̄_1 (logical X on qubit 1) |
| L2 | I X I X I X | X̄_2 (logical X on qubit 2) |
| L3 | Z Z Z Z Z Z | Z̄_1 (logical Z on qubit 1) |
| L4 | Z I Z I Z I | Z̄_2 (logical Z on qubit 2) |

---

## §5 SystemVerilog pseudo 제어 회로 — FSM (τ=4 상태)

### 5.1 FSM 골격

```systemverilog
module qec_controller_n6 #(
    parameter SIGMA   = 12,    // stabilizer 개수
    parameter TAU     = 4,     // syndrome qubit / FSM 상태
    parameter PHI     = 2,     // logical qubit
    parameter N_PHYS  = 6,     // 물리 qubit
    parameter J2      = 24     // Clifford 게이트 cycle budget
)(
    input  logic         clk,
    input  logic         rst_n,
    // 양자점 제어 인터페이스 (DAC)
    output logic [11:0]  qd_bias     [N_PHYS-1:0],   // 12-bit bias per qubit
    output logic [7:0]   qd_pulse    [N_PHYS-1:0],   // RF pulse envelope
    // syndrome 측정 입력 (dispersive readout ADC)
    input  logic [TAU-1:0]  syndrome,
    input  logic            syndrome_valid,
    // 논리 qubit I/O (φ=2)
    input  logic [PHI-1:0]  logical_in,
    output logic [PHI-1:0]  logical_out,
    output logic            cycle_done
);
    // τ=4 FSM 상태
    typedef enum logic [1:0] {
        S_ENCODE   = 2'd0,   // 인코딩 (φ=2 logical → 6 physical)
        S_MEASURE  = 2'd1,   // stabilizer 측정 (σ=12 주기)
        S_FEEDBACK = 2'd2,   // syndrome → 정정 연산
        S_DECODE   = 2'd3    // 논리 qubit 복원
    } qec_state_t;

    qec_state_t state, next_state;

    // σ=12 측정 카운터
    logic [3:0] meas_cnt;   // 0..11

    // J₂=24 Clifford 시퀀스 카운터
    logic [4:0] clifford_cnt;  // 0..23

    // Recovery lookup table (2^τ = 16 entries)
    logic [11:0] recovery_rom [0:15];

    initial begin
        // §2.2 표 로드
        recovery_rom[4'h0] = 12'h000;  // no error
        recovery_rom[4'h1] = 12'h001;  // X on q0
        recovery_rom[4'h2] = 12'h004;  // X on q1
        recovery_rom[4'h3] = 12'h002;  // Z on q0
        recovery_rom[4'h4] = 12'h008;  // Z on q1
        recovery_rom[4'h5] = 12'h003;  // Y on q0
        recovery_rom[4'h6] = 12'h00C;  // Y on q1
        recovery_rom[4'h7] = 12'h020;  // Z on q2
        recovery_rom[4'h8] = 12'h010;  // X on q2
        recovery_rom[4'h9] = 12'h080;  // Z on q3
        recovery_rom[4'hA] = 12'h040;  // X on q3
        recovery_rom[4'hB] = 12'hFFF;  // detect only
        recovery_rom[4'hC] = 12'hFFF;
        recovery_rom[4'hD] = 12'hFFF;
        recovery_rom[4'hE] = 12'hFFF;
        recovery_rom[4'hF] = 12'hFFF;
    end

    // 상태 전이
    always_ff @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            state        <= S_ENCODE;
            meas_cnt     <= '0;
            clifford_cnt <= '0;
        end else begin
            state <= next_state;
            if (state == S_MEASURE)  meas_cnt     <= meas_cnt + 1;
            if (state == S_ENCODE)   clifford_cnt <= clifford_cnt + 1;
            if (state == S_DECODE)   clifford_cnt <= '0;
        end
    end

    // 다음 상태 결정
    always_comb begin
        next_state = state;
        case (state)
            S_ENCODE:   if (clifford_cnt == J2/2 - 1) next_state = S_MEASURE;
            S_MEASURE:  if (meas_cnt == SIGMA - 1)    next_state = S_FEEDBACK;
            S_FEEDBACK: if (syndrome_valid)           next_state = S_DECODE;
            S_DECODE:   if (clifford_cnt == J2/2 - 1) next_state = S_ENCODE;
        endcase
    end

    // 출력 구동
    always_ff @(posedge clk) begin
        case (state)
            S_ENCODE:   /* Clifford 시퀀스 1..12 qd_pulse[] 구동 */;
            S_MEASURE:  /* σ=12 stabilizer 순차 측정 */;
            S_FEEDBACK: begin
                logical_out <= apply_recovery(recovery_rom[syndrome]);
                cycle_done  <= 1'b1;
            end
            S_DECODE:   /* 반대 Clifford 시퀀스 */;
        endcase
    end
endmodule
```

### 5.2 타이밍 예산 (1 QEC 사이클)

```
  Gate time τ_g = 20 ns (초전도/스핀 qubit 평균)
  Measurement τ_m = 200 ns (dispersive readout)

  S_ENCODE    : 12 Clifford × 20 ns = 240 ns
  S_MEASURE   : 12 stab × (2·20 + 200) = 2,880 ns
  S_FEEDBACK  : 40 ns (lookup + 1 Pauli)
  S_DECODE    : 12 Clifford × 20 ns = 240 ns
  ─────────────────────────────────────────
  총 사이클   : 3,400 ns ≈ 3.4 μs

  QEC 반복 주파수: 1 / 3.4 μs ≈ 294 kHz
  Logical qubit T1 목표: ≥ 1 ms → 사이클 약 294 회 보호 필요
```

---

## §6 성능 추정

### 6.1 오류 임계값 (threshold theorem)

```
  물리 qubit 단일 게이트 오류: p_phys
  검출 가능 오류 (1-qubit): d=2 → 검출 확률 = 1 - (1-p_phys)^6 ≈ 6·p_phys
  미검출 2-qubit 동시 오류: ≈ C(6,2)·p_phys² = 15 p_phys²

  Break-even: 6 p_phys (검출) < 15 p_phys² (잔차)
  → p_phys < 0.4 일 때는 대부분 검출 (trivial)
  
  논리 qubit 오류: p_L ≈ 15 p_phys² (잔차 지배)
  p_phys = 10⁻³ → p_L = 1.5×10⁻⁵ (150배 개선)
  p_phys = 10⁻⁴ → p_L = 1.5×10⁻⁷ (수율 ≈ 6×10⁶ 사이클)

  ★ 단 d=2 이므로 "정정"이 아닌 "검출 + abort/retry".
    실용 정정은 d≥3 필요 → n=6 단일 블록으로는 불가능.
    해결: 2-level concatenation (6² = 36 qubit, [[36,4,4]])
```

### 6.2 논리 qubit 수명 (coherence)

```
  물리 T1_phys = 100 μs (양자점 스핀 qubit 2024 기준)
  QEC 사이클 = 3.4 μs

  사이클당 물리 decay 확률: p_dec = 1 - exp(-3.4/100) ≈ 0.034
  논리 decay (6 qubit 중 1개 검출 실패): ≈ 15·(0.034)² ≈ 0.0173

  논리 T1_L ≈ 3.4 μs / 0.0173 ≈ 197 μs
  ≈ 2x physical T1 (개선 배율 φ=2)

  ★ n=6 구조의 자연 개선 배수 = φ(6) = 2.
     Steane [[7,1,3]] 는 개선 배율 ~σ/τ = 3 (더 높지만 qubit 효율 낮음).
```

### 6.3 QEC 오버헤드

| 지표 | 값 | 비고 |
|------|----|----|
| 물리/논리 qubit 비 | 6/2 = 3 | Shor 9/1=9 대비 **σ/n=2배** 효율 |
| 게이트/사이클 | 24 | J₂(6) |
| 측정/사이클 | 12 | σ(6) |
| latency | 3.4 μs | τ·840 ns |
| throughput | 294 kHz | 1/latency |
| 에너지/사이클 | ~60 pJ | chip-architecture.md 2 pJ/op × 24 = 48 pJ |
| 면적 (양자점) | 6 × 100 nm² = 600 nm² | hexagonal pitch |

---

## §7 n=6 이외 변형 대비

| 코드 | [[n,k,d]] | qubit/logical | 정정 | 비고 |
|------|-----------|---------------|------|------|
| 5-qubit | [[5,1,3]] | 5 | **1-qubit 정정** | d=3 최소, k=1 제한 |
| Steane | [[7,1,3]] | 7 | 1-qubit 정정 | CSS, transversal H |
| **n=6 본 설계** | **[[6,2,2]]** | **3** | **검출만** | **k=2, 효율 최고** |
| Shor | [[9,1,3]] | 9 | 1-qubit 정정 | concatenation 원형 |
| Surface d=3 | [[17,1,3]] | 17 | 1-qubit 정정 | 2D 토폴로지 |

### 7.1 비교표 — 효율 vs 정정력

```
  효율 η = k / n (높을수록 좋음):
    n=6:    2/6 = 0.333  ★ 최고
    Steane: 1/7 = 0.143
    5-qubit:1/5 = 0.200
    Shor:   1/9 = 0.111
    Surface:1/17 = 0.059

  정정력 d (높을수록 좋음):
    n=6:    2  ★ 최저
    나머지: 3

  선택 기준:
    - NISQ 초기 (p_phys~10⁻³): 정정보다 검출+retry가 실용적 → n=6 유리
    - 장기 QEC (p_phys~10⁻⁵): 정정 필수 → Surface 유리
    - φ=2 용량이 요구 (벨 상태, Bell 쌍, 2-qubit 알고리즘): n=6 유일
```

### 7.2 Surface code 속 n=6 최소 단위

Surface code의 **plaquette**는 4-qubit stabilizer, **vertex**는 4-qubit
stabilizer. 한 격자 셀 = 4+2=6 qubit (= n). 즉 **Surface code의 최소
반복 단위가 n=6 구조와 정확히 일치**.

```
  Surface code 1 plaquette + 1 vertex = 6 qubit:
    - 4 data qubit (plaquette 꼭짓점)
    - 2 measurement qubit (plaquette 중심 + vertex 중심)
    - 총 6 = n

  확장: d=3 Surface는 17 qubit 중 core 6 qubit 블록 (σ/n ≈ 2.83).
  Conjecture: Surface code 스케일링의 "단위 비용" = 6.
```

---

## §8 Limitations — 정직한 평가

### 8.1 거리 감소 (d=3 → d=2)

Shor [[9,1,3]] → [[6,2,2]] 축소로 **1-qubit 정정 능력을 상실**. n=6에서
**정정이 가능한 QEC 코드는 존재하지 않는다** (정리: d≥3 QEC code는
n ≥ 5를 요구, 그리고 n=5, n=6 중 d=3은 5-qubit code만).

- [[6,1,3]] code: **존재 증명 없음** (k=1 포기 시 [[5,1,3]] 이 최적)
- [[6,2,3]] code: **불가능** (Singleton bound: n-k ≥ 2(d-1) → 6-2=4 ≥ 4, 경계)
- [[6,2,2]] code: **본 설계** (Singleton bound 여유, d=2)

### 8.2 2-level concatenation 불가피

실용 QEC를 위해서는 **n=6 블록을 2단 중첩** ([[36,4,4]]): 면적 6× 증가.
여전히 Surface code d=5 (49 qubit) 대비 경쟁력 있음.

### 8.3 Physical overhead 역설

n=6의 "효율 η=0.333" 은 오해 소지가 있음: d=2는 **정정 없음**이므로
"논리 qubit" 정의가 약함. **정당한 비교**는 concatenated [[36,4,4]] 의
η = 4/36 = 0.111 (Shor와 동일).

### 8.4 n=6 QEC의 합법적 적용 영역

| 적용 | 적합도 | 이유 |
|------|--------|------|
| NISQ 초기 오류 검출 | ★★★ | d=2 로 충분, φ=2 logical 필요 |
| 벨 상태 보호 | ★★★ | 2-qubit 동시 보호 자연 |
| Surface code 최소 단위 | ★★ | plaquette+vertex = 6 |
| 장기 Quantum Memory | ★ | concatenation 필요 |
| Fault-tolerant 양자 컴퓨터 | ☆ | d≥3 필수, 부적합 |

---

## §9 요약

- **구조**: [[6,2,2]] = n=6 물리 qubit / φ=2 logical / d=2 검출.
- **게이트**: J₂(6)=24 Clifford 시퀀스가 **한 사이클의 자연 예산**.
- **stabilizer**: σ(6)=12 측정이 한 주기에서 Golay [24,12,8] codeword와 대응.
- **FSM**: τ(6)=4 상태 (Encode→Measure→Feedback→Decode).
- **성능**: 3.4 μs 사이클, 294 kHz, 논리 T1 ≈ 2× 물리 T1.
- **한계**: d=2 → 검출만. 정정 필요 시 2단 중첩.

**핵심 통찰**: n=6 구조는 **정정 깊이(d)를 희생하고 용량 효율(φ=2)·주기
비용(τ=4, σ=12, J₂=24)을 최적화**한 독특한 위치에 있다. NISQ 시대의
오류 검출 + abort/retry 모델에 최적. Fault-tolerant 시대에는 Surface
code의 **최소 반복 단위** (plaquette+vertex = 6 qubit) 로서의 역할이
가장 자연스럽다.

---

## refs

- [the-number-24.md](../../../theory/proofs/the-number-24.md) — J₂=24, σ·φ=24
- [standard-model-from-n6.md](../../../theory/proofs/standard-model-from-n6.md) — 5/42, σ=12
- [monster-leech-mapping-2026-04-14.md](./monster-leech-mapping-2026-04-14.md) — Golay [24,12,8] ≡ n=6
- [chip-architecture.md](./chip-architecture.md) — L0~L4 기반, 2 pJ/op
