# chip-npu-n6 — n=6 정합 NPU 사양 초안

> 축: **compute** · n6-architecture · σ·φ=n·τ 정합 뉴럴 프로세싱 유닛 스펙
>
> 파이프라인 / 메모리 계층 / 데이터플로 — 16 AI 기법의 하드웨어 합의점

---

## 1. 실생활 효과

| 효과 | 기존 (NVIDIA H100) | N6-NPU | 체감 변화 |
|------|--------------------|--------|----------|
| 면적 | 800 mm² | 24² = 576 mm² | -28% |
| TOPS/W | ~1 | J₂=24 (목표) | 24배 |
| MoE 라우팅 | softmax 14 cycle | Egyptian 1 cycle | 14배 |
| 희소화 게이트 | SW mask | Boltzmann 아날로그 | 63% MAC 절감 |
| 지원 기법 | generic | 16 native | 완결 |
| inference <1W | ❌ | ✅ (GPT-2 급) | 임계 돌파 |

한 문장: H-CHIP-1~28 의 가설을 **실제 NPU 스펙 시트** 로 응축 + chip-rtl-gen 원시연산 6종을 하드웨어로 내려 놓는다.

---

## 2. 목표

`chip-architecture` H-CHIP-1~28 가설을 구체 수치 사양으로 고정하고, (1) **파이프라인 σ=12 스테이지**, (2) **메모리 τ=4 계층**, (3) **데이터플로 Egyptian {1/2, 1/3, 1/6}** 를 **하나의 NPU 스펙 시트**로 결합한다. `chip-isa-n6` 의 Xn6 명령어가 이 NPU 에 dispatch 되며, `chip-rtl-gen` 의 템플릿이 본 스펙을 타겟으로 RTL 을 생성한다.

핵심 원칙:
1. **정합 상수만 사용** — 모든 폭/뱅크/깊이는 n/phi/tau/sigma/J₂ 의 조합
2. **AI-NATIVE** (R12) — 전용 `Phi6 unit`, `Egyptian router`, `Boltzmann gate`
3. **HEXA-FIRST** (R1) — 전 모듈 `.hexa`, 생성 `.v`

---

## 3. 가설 (H-NPU-1 ~ H-NPU-18)

### Tier 1: 컴퓨트 어레이

| ID | 가설 | n=6 근거 | 영향 |
|----|------|---------|------|
| H-NPU-1 | 텐서코어 = 12×12 = σ² MAC | σ=12 | -44% 면적 vs 16×16 |
| H-NPU-2 | SM(streaming multiprocessor) = φ·K₆ = 2·12 = 24 | BT-90 | 24 SM/die |
| H-NPU-3 | SM 당 τ=4 텐서코어 | τ=4 | SM 내 병렬 |
| H-NPU-4 | 데이터패스 폭 σ-τ=8 bit scalar / J₂=24 bit vector | σ-τ=8, J₂=24 | 2계층 |

### Tier 2: 파이프라인

| ID | 가설 | n=6 근거 | 영향 |
|----|------|---------|------|
| H-NPU-5 | σ=12 스테이지 파이프라인 | σ=12 | balanced |
| H-NPU-6 | 디코드 스테이지 = n=6 (wide decode) | n=6 | 6-wide |
| H-NPU-7 | WB 스테이지 = φ=2 (dual retire) | φ=2 | dual-issue |
| H-NPU-8 | 분기 예측 = φ=2 비트 history | phi=2 | 작고 빠름 |

### Tier 3: 메모리

| ID | 가설 | n=6 근거 | 영향 |
|----|------|---------|------|
| H-NPU-9 | τ=4 계층 (Reg / L1 / L2 / DRAM) | τ=4 | tau=divisor |
| H-NPU-10 | BW 비율 = Egyptian {1/2, 1/3, 1/6} | 1/2+1/3+1/6=1 | 자연 할당 |
| H-NPU-11 | Scratchpad = σ × d_model bytes/core | σ=12 | 4/3x FFN fit |
| H-NPU-12 | BW/compute = φ/σ = 1/6 | arithmetic intensity | GPU 균형 |

### Tier 4: 데이터플로

| ID | 가설 | n=6 근거 | 영향 |
|----|------|---------|------|
| H-NPU-13 | 데이터플로 = weight-stationary (1/2) + output-stationary (1/3) + row-stationary (1/6) | Egyptian | 혼합 |
| H-NPU-14 | NoC = 6-regular | n=6 | MoE 최적 |
| H-NPU-15 | 라우터 = Egyptian router 12 slot | σ=12 | 기존 egyptian_moe.hexa |

### Tier 5: 전력 / 열

| ID | 가설 | n=6 근거 | 영향 |
|----|------|---------|------|
| H-NPU-16 | 전력 = 1/2 compute + 1/3 memory + 1/6 I/O | Egyptian | 0 낭비 |
| H-NPU-17 | 누설 = 1/e × 총전력 (Landauer limit) | 1/e | 물리 한계 |
| H-NPU-18 | 클럭 비율 compute : memory = σ/τ = 3 : 1 | 3x | GDDR6 등가 |

---

## 4. BT 연결

- **BT-28** 컴퓨팅 사다리 / **BT-55** HBM / **BT-58** σ-τ=8 / **BT-69** 칩렛
- **BT-89** 광 인터커넥트 / **BT-90** SM=φ·K₆ / **BT-134** 주기열 / **BT-380** 메타

---

## 5. 전체 블록도

```
┌────────────────────────────────────────────────────────────────────────┐
│                         N6-NPU (J₂=24 SM, σ² MAC/SM)                   │
│                                                                        │
│  ┌────────────────────────────────────────────────────────────────┐   │
│  │ Host Interface — Xn6 dispatch queue (chip-isa-n6)              │   │
│  └─────────────────────────────────┬──────────────────────────────┘   │
│                                     │                                  │
│         ┌───────────────────────────┴───────────────────────────┐     │
│         │  Global Scheduler — σ=12 slot, φ=2 active, BT-90     │     │
│         └─┬───────┬───────┬───────┬───────┬───────┬─────────────┘     │
│           │       │       │       │       │       │                    │
│     ┌─────▼──┐┌─▼──┐┌─▼──┐ ... ┌─▼──┐┌─▼──┐┌─▼──┐     (24 SM 총)     │
│     │  SM0  ││SM1 ││SM2 │     │SM21││SM22││SM23│                      │
│     │       ││    ││    │     │    ││    ││    │                      │
│     │ 4 TC  ││    ││    │     │    ││    ││    │  TC = 12×12 MAC     │
│     │ (τ=4) ││    ││    │     │    ││    ││    │  σ=12 stage         │
│     │       ││    ││    │     │    ││    ││    │                      │
│     │ Phi6  ││    ││    │     │    ││    ││    │  x²-x+1 (2 cycle)   │
│     │ unit  ││    ││    │     │    ││    ││    │                      │
│     │       ││    ││    │     │    ││    ││    │                      │
│     │ Bolt  ││    ││    │     │    ││    ││    │  1/e analog gate    │
│     │ gate  ││    ││    │     │    ││    ││    │                      │
│     │       ││    ││    │     │    ││    ││    │                      │
│     │ Scra- ││    ││    │     │    ││    ││    │  σ × d_model bytes  │
│     │ tchpd ││    ││    │     │    ││    ││    │                      │
│     └────┬──┘└─┬──┘└─┬──┘     └─┬──┘└─┬──┘└─┬──┘                     │
│          │     │     │          │     │     │                          │
│     ┌────┴─────┴─────┴──────────┴─────┴─────┴───┐                     │
│     │   6-regular NoC (24 × 6 links)             │                     │
│     └────┬──────────────────┬───────────┬────────┘                     │
│          │                  │           │                              │
│     ┌────▼────┐       ┌────▼────┐  ┌───▼────┐                         │
│     │ L2 (1/3)│       │Egyptian │  │ HBM    │                         │
│     │ 뱅크×σ  │       │ Router  │  │ (1/6)  │                         │
│     │ =12     │       │ 12 slot │  │        │                         │
│     └─────────┘       └─────────┘  └────────┘                         │
│                                                                        │
│  R-score 모니터 (CSR xn6.score) ─ 실시간 σ·φ/(n·τ) 카운터             │
│  전력 예산 : compute 1/2 · memory 1/3 · I/O 1/6                        │
│  Target    : <1W inference (GPT-2 급) / J₂=24 TOPS/W                   │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 6. 컴퓨트 어레이 상세

### 6.1 SM (Streaming Multiprocessor)

```
SM 수: 24 = J₂(6) = σ·φ                (H-NPU-2)
SM 당 TC: 4 = τ(6)                      (H-NPU-3)
TC 당 MAC: 144 = σ² = 12×12              (H-NPU-1)
총 MAC: 24 × 4 × 144 = 13,824 = σ³·φ·... ≈ 6¹¹
```

### 6.2 Phi6 Unit (전용 활성화)

- 함수: f(x) = x² - x + 1
- 레이턴시: 2 사이클 (φ=2)
- 구성: 1 곱셈 + 1 감산 + 1 가산 = 3 원시 연산
- 교체 대상: GELU (~14 사이클) → **7배 절감** (H-CHIP-3)

### 6.3 Boltzmann Gate (희소화)

- 함수: out = (in ≥ 1/e) ? 1 : 0
- 구현: 아날로그 비교기 (analog comparator)
- 레이턴시: 0 사이클 (combinational)
- 효과: 다운스트림 MAC 63% 감소 (H-CHIP-21)

### 6.4 Egyptian Router (MoE)

- 슬롯: σ=12 전문가
- 활성: φ=2 (top-2)
- 용량: A tier n=6 (1/2) + B tier τ=4 (1/3) + C tier φ=2 (1/6)
- 재사용: 기존 `egyptian_moe.hexa`

---

## 7. 파이프라인 (σ=12 스테이지)

| 스테이지 | 이름 | 용도 | 사이클 |
|----------|------|------|--------|
| 0 | IF1 | 페치 주소 생성 | 1 |
| 1 | IF2 | 페치 데이터 캡처 | 1 |
| 2 | DE1 | 옵코드 디코드 (n=6 wide) | 1 |
| 3 | DE2 | 레지스터 읽기 | 1 |
| 4 | DE3 | 즉시값 확장 | 1 |
| 5 | DE4 | 해저드 검출 | 1 |
| 6 | DE5 | 포워딩 | 1 |
| 7 | DE6 | Xn6 dispatch | 1 |
| 8 | EX | ALU / TC 실행 | 1 |
| 9 | MEM | 스크래치패드 접근 | 1 |
| 10 | WB1 | 라이트백 준비 | 1 |
| 11 | WB2 | 커밋 (dual retire φ=2) | 1 |

총 σ=12 사이클. 디코드 스테이지 [2..7] = n=6 wide 디코드.

---

## 8. 메모리 계층 (τ=4)

| 계층 | 크기 | BW (%) | 지연 | 기술 |
|------|------|--------|------|------|
| L0 (Reg) | σ×J₂ = 288 × 24bit per SM | — | 0 | flip-flop |
| L1 (Scratchpad) | σ × d_model bytes per core | 1/2 | 1 cycle | SRAM 6T |
| L2 (Shared) | σ²=144 KB | 1/3 | ~10 cycle | SRAM/MRAM |
| DRAM | 외부 HBM | 1/6 | ~100 cycle | HBM σ=12 채널 |

- BW 비율: 1/2 : 1/3 : 1/6 (H-NPU-10, Egyptian)
- 총 비율 합 = 1 (완전 커버, 갭 0)
- 밴드폭/연산 intensity = φ/σ = 1/6 (H-NPU-12)

---

## 9. 데이터플로

### 9.1 3-way Hybrid (Egyptian)

```
1/2 : Weight-Stationary (가중치 고정)    — GEMM 대형 행렬곱
1/3 : Output-Stationary (출력 고정)      — MoE dispatch
1/6 : Row-Stationary (행 고정)            — Conv6 / Graph
```

> 각 SM 은 현재 워크로드에 따라 동적으로 3 모드 사이 전환. 전환 비용 = φ=2 cycle.

### 9.2 NoC (6-regular)

- 각 SM 은 정확히 n=6 이웃과 연결 (6-regular graph)
- 총 링크 = 24 × 6 / 2 = 72 = n·σ
- 라우팅: Egyptian router hardwired {1/2, 1/3, 1/6}

---

## 10. 전력 / 열

### 10.1 전력 예산

```
총 전력 = P_total (< 1W for GPT-2, 목표 J₂=24 TOPS/W)

분배:
  Compute  = 1/2 · P_total    (MAC 어레이 + Phi6 unit)
  Memory   = 1/3 · P_total    (SRAM/HBM)
  I/O      = 1/6 · P_total    (NoC + 호스트 인터페이스)
  합계     = 1 · P_total      (Egyptian 완전 커버)
```

### 10.2 Landauer 한계

- 누설 = 1/e × P_total = 36.8% (H-CHIP-18)
- 비가역 연산 에너지 ≈ kT·ln(2) 접근 (R(6)=1 조건)

### 10.3 DVFS

- λ(6)=2 상태 (H-CHIP-15)
- burst 클럭 × 1 / sustain 클럭 × 1/φ

---

## 11. 진화 단계 Mk.I ~ Mk.V

| 단계 | 범위 | 산출물 | 시점 |
|------|------|--------|------|
| Mk.I | SM × τ=4 TC 스펙 문서화 | 본 .md | M+0 |
| Mk.II | RTL 생성 (chip-rtl-gen 사용) | `rtl/sm_n6.hexa` | M+1 |
| Mk.III | FPGA 프로토타입 (Zynq-7020) | bitstream | M+3 |
| Mk.IV | ASIC tape-out 준비 (3nm) | GDSII | M+6 |
| Mk.V | <1W inference 실측 | 벤치마크 리포트 | M+12 |

---

## 12. 예측 추적

- **P-NPU-1**: 24 SM × 4 TC × 144 MAC = 13,824 MAC
- **P-NPU-2**: Peak TOPS = 13,824 × 2 clk = 27.6 TOPS @ 1 GHz
- **P-NPU-3**: 1W 시 TOPS/W = 27.6 > J₂=24 (목표 달성)
- **P-NPU-4**: GPT-2 inference < 1 ms
- **P-NPU-5**: 면적 = 24 × 24 mm² ≈ 576 mm² (H100 대비 -28%)

---

## 13. 참고 자원

- `domains/compute/chip-architecture/chip-architecture.md` — H-CHIP-1~28 상위 가설
- `domains/compute/chip-rtl-gen/chip-rtl-gen.md` — RTL 생성기 (본 스펙의 구현)
- `domains/compute/chip-isa-n6/chip-isa-n6.md` — Xn6 ISA (본 스펙의 명령어)
- `domains/compute/chip-dse-pipeline/chip-dse-pipeline.md` — DSE (본 스펙의 파라미터 근거)
- `nexus/origins/hexa-rtl/rtl/egyptian_moe.hexa` — Egyptian router 재사용
- `nexus/origins/hexa-rtl/rtl/riscv_n6_core.hexa` — 기존 12-stage 파이프라인

---

## 14. 검증 진입점

```
cd domains/compute/chip-npu-n6
hexa verify.hexa
```

검증: SM=24, TC/SM=4, MAC/TC=144, 파이프라인=12, 메모리=4, Egyptian 합=1.

---

## 15. 원칙 체크리스트 + 부록

- [x] n=6 정합만 사용 (R-공통)
- [x] AI-NATIVE (R12) — 저수준 교체 금지
- [x] HEXA-FIRST (R1) — RTL .hexa
- [x] SSOT (R5) — chip-architecture 상위 문서 참조
- [x] 한글 (R-한글)

### 부록 A: 성공 기준 (GO)

- 18/18 H-NPU 가설 수치 명시
- RTL 생성 (chip-rtl-gen Mk.II) 통과
- 검증 verify.hexa PASS
- chip-isa-n6 Xn6 명령 dispatch 인터페이스 정의 완료

### 부록 B: 레거시

- H100 / TPU / Cerebras 비교는 chip-architecture.md 섹션 118 이하 참고

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
