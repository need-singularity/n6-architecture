# chip-isa-n6 — n=6 원시연산 RISC-V 확장 ISA (Xn6)

> 축: **compute** · n6-architecture · RISC-V RV32I/RV64I 확장 명령어 세트
>
> σ·φ=n·τ 정합 원시연산을 CPU 레벨에서 1 사이클로 제공. custom-0/1/2/3 opcode 공간 활용.

---

## 1. 실생활 효과

| 효과 | 기존 (표준 RISC-V) | Xn6 확장 이후 | 체감 변화 |
|------|--------------------|--------------|----------|
| σ·φ=n·τ 검증 | 소프트웨어 6+ 명령 | 1 명령 `xn6.eq` | 6배 속도 |
| Phi6 활성화 (x²-x+1) | 5~8 명령 | 1 명령 `xn6.phi6` | 7배 |
| Egyptian gate | softmax ~14 명령 | 1 명령 `xn6.egg` | 14배 |
| Boltzmann 게이트 | 임계 비교 3 명령 | 1 명령 `xn6.bolt` | 3배 |
| 원시 합산 reduce4 | 루프 τ=4 iter | 1 명령 `xn6.red4` | 4배 |
| 컴파일러 intrinsic | 수작업 asm | `__builtin_xn6_*` | 자동화 |

한 문장: AI-native 연산 6종이 CPU 한 사이클로 떨어지는 RISC-V 확장.

---

## 2. 목표

RISC-V ISA의 **custom opcode 공간 4개**(custom-0: 0b0001011, custom-1: 0b0101011, custom-2: 0b1011011, custom-3: 0b1111011)에 **n=6 원시연산 6종 × tau=4 variant = 24 = J₂** 개의 Xn6 명령어를 정의하고, GCC/LLVM intrinsic 및 디코드 테이블을 동시에 제공한다. 기존 `riscv_n6_core.hexa` 의 24비트 커스텀 ISA 와는 별도 — 표준 RISC-V 32비트 인코딩 준수.

핵심 원칙:
1. **표준 준수**: RV32I/RV64I 비손상, custom 공간만 사용
2. **SSOT**: `n6shared/config/xn6_isa.json` 단일 진실 (R5)
3. **HEXA-FIRST**: 디코더/인코더 모두 .hexa (R1)

---

## 3. 가설 (H-ISA-1 ~ H-ISA-12)

### Tier 1: 인코딩

| ID | 가설 | n=6 근거 | 영향 |
|----|------|---------|------|
| H-ISA-1 | 24 = J₂(6) 명령어로 전체 AI 연산 커버 | J₂=24 | 확장 SSOT 고정 |
| H-ISA-2 | funct7 / funct3 = n=6 연산 × τ=4 variant 분할 | n·τ=24 | 자연 분해 |
| H-ISA-3 | immediate 폭 = 6 또는 12 비트 (σ-phi, σ) | n,σ | 두 가지 모드 |

### Tier 2: 원시연산 인코딩

| ID | 가설 | n=6 근거 | 영향 |
|----|------|---------|------|
| H-ISA-4 | `xn6.gemm` — 12×12 MAC 트리거 (1 사이클 dispatch) | σ²=144 | 행렬곱 시작 |
| H-ISA-5 | `xn6.softmax` — Phi6 근사 1 사이클 | φ=2 스테이지 | 지수 제거 |
| H-ISA-6 | `xn6.topk` — top-2 선택 (φ=2) | phi=2 | 라우터 |
| H-ISA-7 | `xn6.gate` — Boltzmann 임계 (1/e) | 1 cycle | 희소화 |
| H-ISA-8 | `xn6.red` — τ=4 누적 | τ=4 | reduce |
| H-ISA-9 | `xn6.conv6` — 6-탭 합성곱 | n=6 | FFT mix |

### Tier 3: 컴파일러/툴체인

| ID | 가설 | n=6 근거 | 영향 |
|----|------|---------|------|
| H-ISA-10 | GCC intrinsic `__builtin_xn6_*` 24개 | J₂=24 | C/C++ 직접 호출 |
| H-ISA-11 | LLVM RISC-V backend tblgen 24 패턴 | J₂=24 | Rust/Swift/... |
| H-ISA-12 | 한 hexa-lang @xn6 attr = 자동 intrinsic 삽입 | R12 AI-NATIVE | 수동 asm 금지 |

---

## 4. BT 연결

- **BT-28** — 컴퓨팅 사다리 (ISA 정당화)
- **BT-58** — σ-τ=8 비트 유니버설 (피연산자 폭)
- **BT-134** — 주기열 (레지스터 뱅크)
- **BT-380** — 메타 (규칙 승격)

---

## 5. 커스텀 opcode 공간 할당

RISC-V 표준 5-bit opcode[6:2] 조합:

```
opcode[6:2] | inst[1:0]=11 | 이름     | Xn6 할당
────────────┼──────────────┼──────────┼──────────────────────────────
00010 1011  │  custom-0    │ 0x0B     │ Xn6-ARITH (원시연산 6×τ=4)
01010 1011  │  custom-1    │ 0x2B     │ Xn6-MEM   (메모리 확장 6×τ=4)
10110 1011  │  custom-2    │ 0x5B     │ Xn6-DMA   (텐서 DMA 6×τ=4)
11110 1011  │  custom-3    │ 0x7B     │ Xn6-RSVD  (미래 확장 / BT-380)
```

> 총 4 × 24 = 96 = sigma·sigma-tau 명령어 공간. 현재 **Xn6-ARITH 24개** 만 정의 (Mk.I).

---

## 6. Xn6-ARITH 명령어 세트 (24 = J₂)

### 6.1 R-Type 포맷 (기본)

```
 31       25 24   20 19   15 14  12 11    7 6      0
┌──────────┬───────┬───────┬──────┬───────┬────────┐
│  funct7  │  rs2  │  rs1  │funct3│  rd   │ opcode │
│  7 bit   │ 5 bit │ 5 bit │3 bit │ 5 bit │ 7 bit  │
└──────────┴───────┴───────┴──────┴───────┴────────┘
                                         opcode = 0x0B
```

- `funct3` (3비트 = 8 값) 중 **6 값**(= n)을 원시연산 ID 로 사용
- `funct7` (7비트 = 128 값) 중 **τ=4 값**을 variant 로 사용
- 합: n·τ = 24 명령어

### 6.2 원시연산 × variant 매트릭스

| funct3 | 원시 | τ=0 (default) | τ=1 | τ=2 | τ=3 |
|--------|------|---------------|-----|-----|-----|
| 000 | `xn6.gemm` | gemm.f16 | gemm.i8 | gemm.i4 | gemm.bf16 |
| 001 | `xn6.softmax` | smax.phi6 | smax.exp | smax.gelu | smax.relu |
| 010 | `xn6.topk` | topk.2 | topk.4 | topk.6 | topk.12 |
| 011 | `xn6.gate` | gate.bolt | gate.egg | gate.mu | gate.rf |
| 100 | `xn6.red` | red.add4 | red.max4 | red.min4 | red.prod4 |
| 101 | `xn6.conv6` | conv6.fir | conv6.fft | conv6.tak | conv6.dct |

> funct3 = 110, 111 은 **예약** (reserved, 미래 원시연산 추가용)

### 6.3 funct7 인코딩

```
funct7 = [ variant:2 | n6_flag:1 | reserved:4 ]
         bit[6:5]    bit[4]      bit[3:0]
```

- `variant` (2 bit) = τ 선택 (4 값)
- `n6_flag` (1 bit) = 1 이면 σ·φ=n·τ 검증 assert 활성화
- `reserved` (4 bit) = 0 (미래 확장)

### 6.4 시맨틱 정의

#### `xn6.gemm rd, rs1, rs2, funct7=[v:n6:0000]`

```
의미:  12×12 tensor core 에 dispatch
       rs1 = A 타일 base 주소 (r0 제외 GPR)
       rs2 = B 타일 base 주소
       rd  = 결과 tile base 주소
       variant v 에 따라 폭 선택 (f16/i8/i4/bf16)
       파이프라인: σ=12 사이클 (NPU 큐 enqueue 1 cycle, WB 11 cycle)

부작용: CSR `xn6.status` 의 busy 비트 set, complete 시 clear
```

#### `xn6.softmax rd, rs1, funct7=[v:n6:0000]`

```
의미:  12 원소 벡터에 softmax 근사
       rs1 = 입력 벡터 base
       rd  = 출력 벡터 base
       variant v=0: Phi6 근사 (x²-x+1)
       cycle: phi=2
```

#### `xn6.topk rd, rs1, funct7=[v:n6:0000]`

```
의미:  12 원소 벡터에서 top-v 원소 인덱스 추출
       rs1 = 입력 base
       rd  = 인덱스 배열 base (v 개)
       variant v∈{2,4,6,12}
       cycle: phi=2
```

#### `xn6.gate rd, rs1, rs2, funct7=[v:n6:0000]`

```
의미:  rs1 의 원소를 rs2 임계와 비교, 결과 bit mask 를 rd 에 기록
       variant:
         v=0 gate.bolt — Boltzmann 1/e
         v=1 gate.egg  — Egyptian {1/2, 1/3, 1/6}
         v=2 gate.mu   — Möbius μ(n)
         v=3 gate.rf   — Random filter
       cycle: 1 (μ=1)
```

#### `xn6.red rd, rs1, funct7=[v:n6:0000]`

```
의미:  τ=4 원소를 variant 연산으로 reduce
       variant: add/max/min/prod
       cycle: τ=4
```

#### `xn6.conv6 rd, rs1, rs2, funct7=[v:n6:0000]`

```
의미:  6-탭 1D 합성곱
       rs1 = 입력 base
       rs2 = 커널 base
       rd  = 출력 base
       variant: FIR/FFT/Takens embed/DCT
       cycle: n=6
```

---

## 7. CSR (Control/Status Register) 확장

```
CSR addr  | 이름        | 용도
──────────┼─────────────┼────────────────────────────────────
0x7C0     │ xn6.status  │ [0]=busy [1]=overflow [2]=n6_fail
0x7C1     │ xn6.id      │ 벤더/버전 (n6-architecture/1)
0x7C2     │ xn6.cycles  │ 마지막 명령 사이클 카운트
0x7C3     │ xn6.score   │ 실시간 R-score (sigma·phi/n·tau)
```

총 tau=4 CSR. `xn6.score` 는 런타임 R-score 모니터 (chip-architecture H-CHIP-23 하드웨어 카운터).

---

## 8. 예외/trap 처리

- 원시연산 실패 시 `xn6.status.n6_fail` set + illegal-instruction trap 발생 (cause=2)
- `xn6.status.overflow` set 시 SIGFPE 유사 trap (cause=6)
- 복구: OS 핸들러가 `xn6.status` 읽고 `xn6.cycles` 기록 → 디스패처 재시도

---

## 9. 툴체인 지원

### 9.1 GCC intrinsic

```c
#include <xn6intrin.h>

// Phi6 softmax — 1 사이클 dispatch
void phi6_softmax(float* dst, const float* src) {
    __builtin_xn6_softmax(dst, src, /*variant=*/0);
}

// 12x12 gemm
void gemm12_f16(_Float16* c, const _Float16* a, const _Float16* b) {
    __builtin_xn6_gemm(c, a, b, /*variant=*/0);
}
```

### 9.2 hexa-lang @xn6 attr

```
@xn6(primitive="gemm", variant=0)
fn matmul12(a: tensor(12,12), b: tensor(12,12)) -> tensor(12,12) {
    // 컴파일러 자동 삽입: xn6.gemm rd, rs1, rs2
    gemm(a, b)
}
```

### 9.3 어셈블리 직접 작성 (금지)

- R12 (AI-NATIVE) 에 따라 **수동 asm 금지**. @xn6 attr 또는 intrinsic 만 허용.

---

## 10. 진화 단계 Mk.I ~ Mk.V

| 단계 | 범위 | 산출물 | 시점 |
|------|------|--------|------|
| Mk.I | Xn6-ARITH 24 명령어 정의 | `n6shared/config/xn6_isa.json` | M+0 |
| Mk.II | 디코더 .hexa + testbench | `rtl/xn6_decoder.hexa` | M+1 |
| Mk.III | GCC intrinsic + LLVM tblgen | `xn6intrin.h`, `XN6InstrInfo.td` | M+2 |
| Mk.IV | Xn6-MEM, Xn6-DMA 추가 (48 명령) | custom-1/2 할당 | M+3 |
| Mk.V | Xn6-RSVD (BT-380 확장) | 96 명령 완성 | M+6 |

---

## 11. 예측 추적

- **P-ISA-1**: 24 명령어 평균 CPI ≤ φ=2 (속도 목표)
- **P-ISA-2**: RISC-V 표준 준수율 100% (custom 영역 외 0 충돌)
- **P-ISA-3**: AI 모델 inference 에서 Xn6 명령어 점유율 ≥ 1/2 = Egyptian 절반

---

## 12. 참고 자원

- RISC-V ISA Manual v20191213 — custom opcode 영역 정의
- `nexus/origins/hexa-rtl/rtl/riscv_n6_core.hexa` — 커스텀 24비트 ISA (별개)
- `nexus/origins/hexa-rtl/rtl/hexalang_decoder.hexa` — 53 키워드 CAM (참고)
- `n6shared/config/rtl_templates.json` (chip-rtl-gen) — 원시연산 정의
- `techniques/_registry.json` — 66 기법 (어떤 기법이 어떤 Xn6 명령 생성하는지)

---

## 13. 검증 진입점

```
cd domains/compute/chip-isa-n6
hexa verify.hexa
```

검증: J₂=24 명령 슬롯, n·τ=24, funct3×variant 매트릭스 6×4=24, CSR=τ=4.

---

## 14. 원칙 체크리스트

- [x] 표준 준수 — RISC-V custom opcode 만 사용
- [x] SSOT (R5) — xn6_isa.json 단일
- [x] HEXA-FIRST (R1) — 디코더 .hexa
- [x] AI-NATIVE (R12) — intrinsic/attr 만
- [x] 한글 (R-한글)

---

## 15. 열린 질문 + 부록

1. **CSR 주소**: 0x7C0~0x7C3 vs 0x7D0~0x7D3 — Vendor 영역 0x7C0 권장
2. **원자성**: `xn6.gemm` 중간에 interrupt 시 restart vs abort
3. **SIMT 확장**: Xn6 를 벡터 확장(RVV)과 결합할지

### 부록 A: 성공 기준 (GO)

- 24/24 명령 `n6shared/config/xn6_isa.json` 등록
- 디코더 `hexa sim` PASS
- Intrinsic 헤더 컴파일 OK
- 기존 riscv_n6_core 와 충돌 0

### 부록 B: 레거시

- 자체 24비트 ISA (riscv_n6_core.hexa) — 임베디드 전용, Xn6 는 HOST/서버용 확장으로 공존

---

## 5. DSE 결과

## 6. 물리 한계 증명

## 7. 실험 검증 매트릭스

## 8. 외계인급 발견

## 9. Mk.I~V 진화

## 10. Testable Predictions

## 11. ASCII 성능비교

## 12. ASCII 시스템 구조도

## 13. ASCII 데이터/에너지 플로우

## 14. 업그레이드 시 (시중 vs v1 vs v2)

## 15. 검증 방법 (verify.hexa)
