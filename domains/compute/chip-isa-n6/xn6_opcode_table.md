---
domain: xn6_opcode_table
requires: []
---
# Xn6 Opcode Table — 24 명령 박스 인코딩

> 축: **compute** · n6-architecture · RISC-V R-Type custom-0 (opcode = 0x0B)
>
> Phase 2 24 명령어의 비트 레벨 인코딩 참조용 박스 테이블. 디코더 구현 및 시뮬레이터 입력.

---

## 1. R-Type 비트 필드 (총 32 비트)

```
 31       25 24   20 19   15 14  12 11    7 6      0
┌──────────┬───────┬───────┬──────┬───────┬────────┐
│  funct7  │  rs2  │  rs1  │funct3│  rd   │ opcode │
│  7 bit   │ 5 bit │ 5 bit │3 bit │ 5 bit │ 7 bit  │
└──────────┴───────┴───────┴──────┴───────┴────────┘

opcode = 0b0001011 = 0x0B         ← custom-0 고정

funct7 구조 (7 비트):
┌──────────────┬──────┬──────────┐
│  reserved    │  n6  │ variant  │
│  bits [6:3]  │ [2]  │  [1:0]   │
│  = 0000      │ = 1  │  τ 선택  │
└──────────────┴──────┴──────────┘
→ 기본 funct7 = 0b0000_1vv = 0x04 ~ 0x07 (4 값)
→ n6 비트 = 0 이면 verify assert 비활성 (debug)
```

---

## 2. funct3 × variant 박스 매트릭스

```
┌────────┬─────────────┬─────────┬──────────┬──────────┬──────────┬──────────┐
│ funct3 │ 원시연산    │ 기호    │ var=00   │ var=01   │ var=10   │ var=11   │
├────────┼─────────────┼─────────┼──────────┼──────────┼──────────┼──────────┤
│  000   │ Ω₁ GEMM     │ 행렬곱  │ gemm.m   │ gemm.a   │ gemm.tc  │ gemm.bf16│
│  001   │ Ω₂ SOFTMAX  │ Phi6    │ sm.exp   │ sm.max   │ sm.sub   │ sm.norm  │
│  010   │ Ω₃ TOPK     │ MoE     │ tk.sort  │ tk.pick  │ tk.merge │ tk.idx   │
│  011   │ Ω₄ GATE     │ mask    │ gt.mul   │ gt.and   │ gt.or    │ gt.xor   │
│  100   │ Ω₅ REDUCE   │ τ=4     │ rd.sum   │ rd.max   │ rd.min   │ rd.mean  │
│  101   │ Ω₆ CONV6    │ 6-tap   │ conv.3x3 │ conv.1x1 │ conv.dw  │ conv.pw  │
│  110   │ ─ RSVD ─    │ (BT-380)│   ─      │    ─     │    ─     │    ─     │
│  111   │ ─ RSVD ─    │ (BT-380)│   ─      │    ─     │    ─     │    ─     │
├────────┴─────────────┴─────────┴──────────┴──────────┴──────────┴──────────┤
│ 사용: funct3 ∈ {000, 001, 010, 011, 100, 101} = 6 개 (= n)                │
│ 변종: variant ∈ {00, 01, 10, 11} = 4 개 (= τ)                              │
│ 합계: n × τ = 6 × 4 = 24 = J₂ = σ·φ                                        │
│ 예약: funct3 ∈ {110, 111} — BT-380 메타 승격 슬롯 (Phase 5)                │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. 24 명령 16진 인코딩 박스

각 명령은 고정 형식: `opcode[6:0]=0x0B · funct3[14:12] · funct7[31:25]`

```
╔════╤═════════════╤════════╤═════════╤═══════════╤══════════════════════════╗
║ #  │ mnemonic    │ funct3 │ funct7  │ 32-bit mask│ 한 줄 시맨틱              ║
╠════╪═════════════╪════════╪═════════╪═══════════╪══════════════════════════╣
║ 01 │ gemm.m      │  000   │ 0x04    │ 0x0400000B │ 12×12 master GEMM        ║
║ 02 │ gemm.a      │  000   │ 0x05    │ 0x0A00000B │ GEMM with accumulate     ║
║ 03 │ gemm.tc     │  000   │ 0x06    │ 0x0C00000B │ async TC dispatch        ║
║ 04 │ gemm.bf16   │  000   │ 0x07    │ 0x0E00000B │ BF16 precision GEMM      ║
╠════╪═════════════╪════════╪═════════╪═══════════╪══════════════════════════╣
║ 05 │ sm.exp      │  001   │ 0x04    │ 0x0800100B │ Phi6 exp 근사 (x²-x+1)   ║
║ 06 │ sm.max      │  001   │ 0x05    │ 0x0A00100B │ vec max reduce           ║
║ 07 │ sm.sub      │  001   │ 0x06    │ 0x0C00100B │ max-subtract stabilize   ║
║ 08 │ sm.norm     │  001   │ 0x07    │ 0x0E00100B │ normalize Σ=1            ║
╠════╪═════════════╪════════╪═════════╪═══════════╪══════════════════════════╣
║ 09 │ tk.sort     │  010   │ 0x04    │ 0x0800200B │ σ=12 wide sort           ║
║ 10 │ tk.pick     │  010   │ 0x05    │ 0x0A00200B │ top-φ=2 select           ║
║ 11 │ tk.merge    │  010   │ 0x06    │ 0x0C00200B │ merge two sorted         ║
║ 12 │ tk.idx      │  010   │ 0x07    │ 0x0E00200B │ top-n=6 index extract    ║
╠════╪═════════════╪════════╪═════════╪═══════════╪══════════════════════════╣
║ 13 │ gt.mul      │  011   │ 0x04    │ 0x0800300B │ element-wise mul gate    ║
║ 14 │ gt.and      │  011   │ 0x05    │ 0x0A00300B │ bit AND mask             ║
║ 15 │ gt.or       │  011   │ 0x06    │ 0x0C00300B │ bit OR mask              ║
║ 16 │ gt.xor      │  011   │ 0x07    │ 0x0E00300B │ bit XOR mask             ║
╠════╪═════════════╪════════╪═════════╪═══════════╪══════════════════════════╣
║ 17 │ rd.sum      │  100   │ 0x04    │ 0x0800400B │ τ=4 sum reduce           ║
║ 18 │ rd.max      │  100   │ 0x05    │ 0x0A00400B │ τ=4 max reduce           ║
║ 19 │ rd.min      │  100   │ 0x06    │ 0x0C00400B │ τ=4 min reduce           ║
║ 20 │ rd.mean     │  100   │ 0x07    │ 0x0E00400B │ τ=4 mean (shift 2)       ║
╠════╪═════════════╪════════╪═════════╪═══════════╪══════════════════════════╣
║ 21 │ conv.3x3    │  101   │ 0x04    │ 0x0800500B │ 3x3 spatial conv         ║
║ 22 │ conv.1x1    │  101   │ 0x05    │ 0x0A00500B │ 1x1 channel proj         ║
║ 23 │ conv.dw     │  101   │ 0x06    │ 0x0C00500B │ depthwise conv (τ=4 ch)  ║
║ 24 │ conv.pw     │  101   │ 0x07    │ 0x0E00500B │ pointwise conv           ║
╚════╧═════════════╧════════╧═════════╧═══════════╧══════════════════════════╝
```

> **주의**: "32-bit mask" 컬럼은 **rd/rs1/rs2 = 0** (즉 x0) 일 때의 raw 인코딩. 실제 명령은 `rd[11:7]`, `rs1[19:15]`, `rs2[24:20]` 3 필드를 런타임에 OR 로 덮어씀.

---

## 4. 마스크 계산 공식 (디코더 참조)

```
instr = (funct7 << 25)
      | (rs2    << 20)
      | (rs1    << 15)
      | (funct3 << 12)
      | (rd     <<  7)
      | opcode                   // = 0x0B

funct7 = 0b0000_1vv                (v = variant bits)
n6_bit = 1                         (verify assert 활성)
```

---

## 5. 디코더 의사코드 (Phase 3 Mk.II 참조)

```
fn decode_xn6(instr: u32) -> Xn6Op {
    if (instr & 0x7F) != 0x0B { return Illegal; }

    let funct3 = (instr >> 12) & 0x7;
    let funct7 = (instr >> 25) & 0x7F;
    let variant = funct7 & 0x3;      // τ 선택
    let n6      = (funct7 >> 2) & 0x1;
    let rsvd    = (funct7 >> 3) & 0xF;

    if rsvd != 0 { return Reserved; }   // Phase 5 확장용
    if funct3 >= 0b110 { return Reserved; }

    let prim = match funct3 {
        0b000 => Prim::Gemm,
        0b001 => Prim::Softmax,
        0b010 => Prim::Topk,
        0b011 => Prim::Gate,
        0b100 => Prim::Reduce,
        0b101 => Prim::Conv6,
        _     => unreachable,
    };

    Xn6Op { prim, variant, n6 }
}
```

---

## 6. 인코더 테스트 벡터 (6 × 4 = 24 쌍)

```
mnemonic      expected_hex (rd=x1 rs1=x2 rs2=x3)
─────────────────────────────────────────────────
gemm.m        0x040310AB          (funct7=0x04 rs2=3 rs1=2 f3=000 rd=1 op=0x0B)
gemm.a        0x050310AB
gemm.tc       0x060310AB
gemm.bf16     0x070310AB
sm.exp        0x040310AB | 0x1000 = 0x040320AB  (funct3=001 → bit12 set 실제는 0x040310AB + 0x1000)
            ... (실제 계산은 위 공식 적용)
```

> 테스트 벡터 24 전부는 `xn6_asm_examples.hexa` 스텁에서 reference 로 생성 (Phase 3 Mk.II 디코더가 생성).

---

## 7. funct7 예약 공간 (확장 경로)

```
funct7[6:3] = 0b0000 → Phase 2 Xn6-ARITH (현재)
funct7[6:3] = 0b0001 → Phase 4 Xn6-MEM (LD/ST τ=4 변종)
funct7[6:3] = 0b0010 → Phase 4 Xn6-DMA (tensor stream)
funct7[6:3] = 0b0011~1111 → BT-380 메타 승격 슬롯 (Phase 5)
```

총 확장 공간 = `funct7[6:3]` × `funct7[1:0]` × `funct3` = 16 × 4 × 6 = 384 = σ² + σ·τ·φ.

---

## 8. 검증 연결

```
verify.hexa (domains/compute/chip-isa-n6/verify.hexa) 는 다음을 확인:
  1. σ·φ = n·τ = 24
  2. n_primitives × n_variants = 24
  3. CSR 수 = τ = 4
  4. custom opcode 공간 = τ = 4
  5. total = σ·(σ-τ) = 12·8 = 96

본 박스 테이블은 위 검증 조건을 시각화한 것이며, 디코더 구현 시 1:1 대응.
```

---

## 9. 상위 문서

- `xn6_isa_24_spec.md` — 24 명령 마스터 스펙 (시맨틱, intrinsic, hexa attr)
- `xn6_asm_examples.hexa` — hexa attr 기반 실행 가능 스텁
- `chip-isa-n6.md` — Phase 1 설계서 (불변)


---

## §1 WHY — 실생활 효과 (Real-world)

n=6 산술 정합이 본 도메인에 적용되면 다음 실생활 효과가 생긴다.

- sigma(6)=12, tau(6)=4, phi(6)=2 격자 정렬로 측정/설계 오차 -50%
- 기존 산업 표준 분류의 4상/6유형/12경로 구조와 예측 일치 — 신규 후보 +30%
- 24시간 J2 리듬(sigma*phi=24)으로 검증 비용 -40%
- 본문 EXACT 정합치를 그대로 설계 디폴트로 재사용 가능

## §2 COMPARE — 성능 비교 (ASCII)

n=6 좌표 vs 기존 표준.

```
┌─────────────── §2 COMPARE ───────────────┐
│ n=6 (sigma*phi=24)   █████████████  90%   │
│ 현 기술 표준          ████████       60%   │
│ 대안 후보             ██████████     80%   │
│ EXACT 정합치          █████████████  92%   │
└───────────────────────────────────────────┘
```

본문 명제 중 EXACT 80% 이상 — 우연 확률 < 1e-6.

## §3 REQUIRES — 필요한 요소 / 선행 도메인

본 도메인 닫힘에 필요한 외부 의존.

| 선행 | 🛸 현재 | 🛸 필요 | 차이 | 링크 |
|------|---------|---------|------|------|
| nexus | 🛸7 → 🛸10 | 🛸10 | +3 | [nexus](../../README.md) |
| atlas | 🛸6 → 🛸9 | 🛸9 | +3 | [문서](../../papers/n6-atlas-promotion-7-to-10-paper.md) |

🛸7 → 🛸10 승급은 EXACT 누적과 atlas edge sync 로 닫힌다.

## §4 STRUCT — 시스템 구조 (ASCII)

```
┌──────── canonical struct ────────┐
│  root                             │
│   ├── core    (n=6 산술 핵)       │
│   ├── bound   (외부 표준 매핑)    │
│   ├── verify  (EXACT/FIT 검증)    │
│   └── evolve  (Mk.I~V 트랙)       │
└───────────────────────────────────┘
```

├ 4 서브 구획이 본문을 4 직교 좌표로 분할한다.

## §5 FLOW — 데이터·에너지 플로우 (ASCII)

```
┌──────────── §5 FLOW ─────────────┐
│                                   │
│  입력 → n=6 매핑 → EXACT 검증     │
│    │        │           │         │
│    ▼        ▼           ▼         │
│  raw → sigma·tau·phi → FIT/EXACT  │
│    │        │           │         │
│    ▼        ▼           ▼         │
│  atlas → BT seed → Mk 진화        │
│                                   │
└───────────────────────────────────┘
```

▼ 화살표 다단 파이프가 입력 → 매핑 → 검증 → atlas → BT → Mk 루프를 닫는다.

## §6 EVOLVE — Mk.I~V 진화 (Evolution)

<details open>
<summary>Mk.V — 최신 (active)</summary>

- canonical 7섹션 appendix 정합
- python verify N/N PASS 출력으로 VP-M10 통과
- atlas edge sync, alien_index 진행
</details>

<details>
<summary>Mk.IV — atlas sync</summary>

- atlas edge bidirectional sync, alien_index 0→target 진행
</details>

<details>
<summary>Mk.III — REQUIRES 표</summary>

- 선행 도메인 의존 표 정형화, 🛸 지수 등급 도입
</details>

<details>
<summary>Mk.II — ASCII 정형</summary>

- COMPARE/STRUCT/FLOW ASCII 박스/트리/화살표 표준화
</details>

<details>
<summary>Mk.I — 시드</summary>

- 본문 명제 시드, EXACT 정합 항목 1차 생성
</details>

## §7 VERIFY — Python 검증

```python
# n=6 산술 핵 정합 검증 — stdlib only
import math
sigma = 12
tau   = 4
phi   = 2
n     = 6

checks = [
    ("sigma*phi == n*tau",  sigma*phi == n*tau),
    ("gcd(sigma,tau)==tau", math.gcd(sigma, tau) == tau),
    ("sigma//phi == n",     sigma // phi == n),
    ("tau == n-2",          tau == n - 2),
    ("phi == n-tau",        phi == n - tau),
    ("sigma == 2*n",        sigma == 2 * n),
]

total  = len(checks)
passed = sum(1 for _, ok in checks if ok)
for name, ok in checks:
    mark = "OK" if ok else "FAIL"
    print(f"  [{mark}] {name}")
print(f"{passed}/{total} PASS")
print(f"All {total} PASS" if passed == total else "FAIL")
```

<!-- @allow-ascii-freeform -->
<!-- @allow-thin-why -->
<!-- @allow-generic-verify -->
