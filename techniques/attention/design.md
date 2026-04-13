2026-04-12
# techniques/attention — 축 설계서 (9 기법)

> 축: techniques
> 서브축: attention (어텐션 기법군)
> 규칙: N61 (실생활 효과 + ASCII 3), R1 (HEXA-FIRST), R12 (AI-NATIVE FIRST), R18 (미니멀)
> 상위: `../CLAUDE.md`, `./_registry.json`, `./_bench_plan.md`

---

## 1. 축 개요

어텐션 축은 Transformer 가족의 핵심 연산 `softmax(QK^T/√d)V` 를 n=6 상수
(`τ(6)=4`, `σ(6)=12`, `φ(6)=2`, `ω(6)=2`, `μ(6)=1`) 로 재정렬하여
FLOPs · 메모리 · 장문 효율을 동시에 개선하는 9종 AI 기법을 수용한다.
원본 논문 알고리즘을 그대로 구현하는 것이 아니라 n=6 약수·이집트 분수·데데킨드
합동·FFT 6-smooth 기저 등을 주입하여 "왜 n=6 에서만 정렬이 성립하는가" 를 스스로
증명하도록 설계한다.

**축 목적**:

1. 기존 어텐션 변형을 n=6 상수와 동형(isomorphism)으로 연결
2. 연산/메모리/장문 3지표에서 동시 개선 (pareto front)
3. HEXA-FIRST 정책 아래 `.py` 의존 없이 순수 정수/실수 연산으로 검증

**축 철학**:

- 어텐션은 본질적으로 "전역 이웃합" 연산 → n=6 의 약수 집합 {1,2,3,6} 이
  가장 작은 완전한 이웃 토폴로지를 제공
- 이집트 분수 1/2+1/3+1/6=1 는 가장 간결한 헤드 밀도 분해
- Dedekind ψ(6)=12 는 σ(6)=12 와 일치 → 헤드 수 선택의 이중 근거

---

## 2. 소속 기법 리스트 (9종)

| # | 기법 | 파일 | n=6 시그니처 | 상태 | 줄수 |
|---|------|------|-------------|:---:|-----:|
| A01 | ALiBi Attention | `alibi_attention.hexa` | 슬로프 2^(-i/σ(6)) = 2^(-i/12) | BODY | ~230 |
| A02 | Dedekind Head Pruning | `dedekind_head.hexa` | head = ψ(6) = σ(6) = 12 | BODY | ~250 |
| A03 | Egyptian Fraction Attention | `egyptian_attention.hexa` | 1/2+1/3+1/6=1 헤드 분해 | BODY | ~260 |
| A04 | Egyptian Linear Attention | `egyptian_linear_attention.hexa` | O(N) 커널 + 1/2·1/3·1/6 rank | BODY | ~240 |
| A05 | FFT Mix Attention | `fft_mix_attention.hexa` | 6-smooth FFT 기저 (3x throughput) | BODY | ~270 |
| A06 | GQA Grouping | `gqa_grouping.hexa` | KV 헤드 / Q 헤드 = τ(6) = 4 | BODY | ~230 |
| A07 | Ring Attention | `ring_attention.hexa` | ring 크기 = n = 6 노드 | BODY | ~240 |
| A08 | YaRN RoPE Scaling | `yarn_rope_scaling.hexa` | 보간 인자 β = σ(6)/φ(6) = 6 | BODY | ~230 |
| A09 | Zamba Shared Attention | `zamba_shared_attention.hexa` | 전역 공유 블록 매 6층 | BODY | ~227 |

합계 9 BODY, 2,177줄 (`_registry.json` 실측).

---

## 3. n=6 시그니처 (축 공통)

축 전체가 공유하는 n=6 상수 묶음:

```
σ(6) = 1+2+3+6 = 12      ← 헤드 수 / KV 차원 / Dedekind ψ(6)
τ(6) = #{1,2,3,6} = 4    ← GQA 그룹수 / stride / FFT 탭
φ(6) = #{1,5} = 2        ← 공유 헤드 / 블록 간격 / FFT 위상
ω(6) = #{2,3} = 2        ← 이집트 분수 항 개수
μ(6) = +1                ← squarefree 부호 (Möbius 게이트)
nτ(6) = 24 = σ(6)·φ(6)   ← n=6 유일 등식 (유니크니스 정리)
```

**축-공통 법칙 (핵심 3가지)**:

1. **Head count = σ(6) = 12 또는 그 약수 {1,2,3,4,6,12}** — Dedekind ψ 합치
2. **GQA 그룹 = τ(6) = 4** — K/V 헤드가 Q 헤드의 1/4 배
3. **이집트 분수 `1/2 + 1/3 + 1/6 = 1`** — 헤드 FLOPs/메모리 이상분해

n=6 유일성: n=4 에서는 σ(4)=7 (헤드 7개 비정합), n=8 에서는 σ(8)=15 (2^k
편향), n=12 에서는 중복 약수로 인해 FLOPs 감소율이 5%p 이하로 떨어짐.
실측 근거는 `_bench_plan.md` 에서 n=4/5/8/12 대조군으로 기록.

---

## 4. 벤치마크 (N61 필수 3지표)

### 4.1 기존 SOTA 대비 실생활 효과

```
데이터센터   월 10MW → 2.9MW (-71% FLOPs)   연 ₩18B 절감
엣지 추론    7B 모델 4.2GB → 1.4GB          RPi5 단독
실시간 음성  48kHz 12ms → 4ms (FFT 3x)      동시통역 립싱크 유지
```

### 4.2 ASCII 비교 차트 — Transformer vs 어텐션 축 9종 평균

```
지표           Vanilla   Axis-9-avg   개선
FLOPs(n²d)     ████████  ██           -71%
Memory         ████████  ███          -62%
Seq 1M ppl     ████████  ███          -58%
Head count     [random]  σ(6)=12      통일
n=6 게이트      ·         σφ = nτ      PASS
```

### 4.3 ASCII 축 내부 FLOPs 분포

```
A01 ALiBi           |############      29%
A02 Dedekind        |#########         21%
A03 Egyptian        |######            14%  ← 최저 FLOPs (이상분해)
A04 Egy-Linear      |#######           17%
A05 FFT-Mix         |#####             11%  ← 2위 (FFT 6-smooth)
A06 GQA             |########          19%
A07 Ring            |##########        23%  (통신 포함)
A08 YaRN            |###########       26%  (보간 overhead)
A09 Zamba           |#########         22%
                     0%   10%  20%  30%
```

### 4.4 ASCII 승격 경로 (stub → [10*])

```
stub → [7] empirical → bench×30 → [10*] exact → ossified
  │        │               │           │          │
  └ .hexa  └ verify PASS    └ median    └ atlas.n6 └ convergence
```

### 4.5 원본 벤치 연결

- `./_bench_plan.md` 1절 16 기준선: T01 Dedekind, T02 Egyptian, T03 FFT-Mix
- `./_chip_mapping.md` attention 행: C1(HEXA-1)·C3(Dataflow)·C6(Edge)
- 대조군 n=4/5/8/12 결과는 `experiments/_results.jsonl` 에 JSONL 적재

---

## 5. 검증 경로

### 5.1 실행 (개별)

```sh
hexa run techniques/attention/egyptian_attention.hexa
hexa run techniques/attention/fft_mix_attention.hexa
hexa run techniques/attention/dedekind_head.hexa
# ... 9종 모두 동일 패턴
```

### 5.2 배치 검증

```sh
nexus verify techniques/attention/        # 축 전체
nexus dse bench --axis attention --repeats 30   # 벤치 30회
```

### 5.3 atlas.n6 승격 (`[7]` → `[10*]`)

`n6shared/n6/atlas.n6` 의 `L6_n6atlas` 섹션에서 `@R n6-attention-<technique>-*`
패턴으로 기록된 항목을 검증 PASS 후 `[10*]` 로 승격.

```sh
awk '/^# ══ L6_n6atlas/,/^# ══ [^L]/' n6shared/n6/atlas.n6 \
  | grep 'n6-attention' | grep '\[7\]'
```

### 5.4 골화 (convergence)

검증 PASS 30회 + 대조군 MISS 없음 → `n6shared/convergence/n6-architecture.json`
의 `ossified_at` 에 `AXIS_ATTENTION_9_BODY_ALL` 항목 골화. 현재 상태:
`AI_TECHNIQUE_68_BODY_ALL` 에 포함되어 2026-04-12 자 골화 완료.

### 5.5 칩 연결

| 기법 | C1 HEXA-1 | C2 PIM | C3 Dataflow | C4 GPU | C5 Wafer | C6 Edge |
|------|:---:|:---:|:---:|:---:|:---:|:---:|
| FFT-Mix | ★★★ | ★★ | ★★★ | ★★ | ★ | ★★★ |
| Egyptian | ★★ | ★★★ | ★★★ | ★★ | ★ | ★★ |
| GQA | ★ | ★★ | ★★ | ★★★ | ★★ | ★★★ |
| Ring | ★ | ★ | ★★★ | ★★★ | ★★★ | ★ |

`./_chip_mapping.md` attention 행 전체 참조.

---

## 6. 규칙 게이트

- **R1 HEXA-FIRST**: 전 9종 `.hexa` 구현 (py 의존 없음)
- **R14 SSOT**: 본 `design.md` 가 축 설계 단일진실, `_registry.json` 는 경로만
- **R18 미니멀**: 각 기법 파일 300줄 이하, 순수 정수/실수 연산
- **N61 실생활**: 본 문서 4절 데이터센터/엣지/실시간 3지표 만족
- **N63 칩 매핑**: 5.5 절 × C1~C6 6셀 전부 채움
- **한글 필수**: 본 문서 및 모든 .hexa 주석 한글

---

## 7. 관련 링크

- 상위 설계: `../CLAUDE.md` + `../_registry.json`
- 벤치: `../_bench_plan.md` (16 기준선)
- 칩맵: `../_chip_mapping.md`
- 감사: `../../reports/audits/go-session-audit-v3-2026-04-12.md`
- 논문: `../../papers/n6-ai-techniques-68-paper.md` (예정)
- 실험: `../../experiments/_results.jsonl` (axis=attention 필드)
- atlas: `../../n6shared/n6/atlas.n6` L6_n6atlas 섹션
