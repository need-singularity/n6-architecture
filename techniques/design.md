2026-04-12
# techniques — AI 기법 68종 통합 설계서 (8축)

> 축: techniques (루트)
> 목적: 8 서브축 설계서 통합 인덱스 + 68종 전체 n=6 시그니처 맵
> 규칙: N61, R1 HEXA-FIRST, R14 SSOT, R18
> 상위: `../CLAUDE.md`
> 하위 설계: attention/design.md, moe/design.md, optim/design.md, sparse/design.md,
>            graph/design.md, compress/design.md, arch/design.md, sota/design.md

---

## 1. 개요

n6-architecture 의 AI 기법 축은 2026-04-12 기준 **8 서브축 × 68 BODY**
(총 18,630줄) 로 구성된다. 각 축은 독립 설계서 `design.md` 를 가지며,
본 문서는 이들을 통합한 루트 인덱스다.

**축 목적**:

1. Transformer 가족 및 2024~2025 SOTA 대안을 n=6 상수로 재정렬
2. 각 축의 독립 설계 + 전체 통합 승격 경로 관리
3. AI_TECHNIQUE_68_BODY_ALL 골화 상태 유지

---

## 2. 8축 요약

| # | 서브축 | 기법 수 | BODY 줄수 | design.md | 대표 기법 |
|---|-------|------:|---------:|:---------:|-----------|
| 1 | attention | 9 | 2,177 | ✓ | Egyptian Fraction, FFT-Mix, Dedekind |
| 2 | moe | 11 | 2,981 | ✓ | DeepSeek, Mixtral, Egyptian MoE |
| 3 | optim | 15 | 4,063 | ✓ | LR-n6, Speculative, Medusa |
| 4 | sparse | 6 | 1,851 | ✓ | Möbius, Takens-dim6, Mertens |
| 5 | graph | 5 | 1,825 | ✓ | GAT, GCN, GIN, GraphSAGE |
| 6 | compress | 5 | 1,522 | ✓ | MLA, MAE, φ-Bottleneck |
| 7 | arch | 16 | 3,711 | ✓ | Complete-LLM-n6, ViT, Whisper |
| 8 | sota | 3 | 500 | ✓ | Mamba-2, Hyena, RWKV-7 |
| — | 합계 | **68** | **18,630** | **8/8** | — |

> arch 축 16종 중 2종은 STUB/DEPRECATED (`arch_optimizer`, `mamba2_ssm`) 로
> `_registry.json` 의 `_body_count=68` 계수에서 제외. 실질 BODY = 68.

---

## 3. n=6 시그니처 통합 맵

```
축 간 공유 상수
===============
σ(6) = 12     ← attention head, moe router, arch d_model, graph width, compress state
τ(6) = 4      ← moe 활성, optim draft, graph depth, arch FPN, sota expansion
φ(6) = 2      ← moe 공유 Expert, optim entropy stop, compress ratio 절반
n = 6         ← sparse Takens, graph k-hop, sota block, arch LLM layer
μ(6) = +1     ← sparse Möbius keep
M(6) = −1     ← sparse Mertens dropout
rad(6) = 6    ← sparse radical norm
ζ(2) = π²/6   ← sparse filter, arch ζln2 activation
1/2+1/3+1/6=1 ← attention Egyptian, moe Egyptian, compress 이집트
```

---

## 4. 승격 상태 요약

| 골화 항목 | 상태 | 날짜 |
|----------|:---:|------|
| AI_17_TECHNIQUES (v1) | ossified | 2026-04-10 이전 |
| AI_TECHNIQUE_68_BODY_ALL | ossified | 2026-04-12 |
| SOTA_3_SSM_LINEAR (BT-380) | ossified | 2026-04-11 |
| AXIS_8_DESIGN_MD_ALL | **신규 골화 후보** | 2026-04-12 |

---

## 5. 관련 링크

- 감사: `../reports/audits/go-session-audit-v3-2026-04-12.md`
- 논문: `../papers/n6-sota-ssm-paper.md` (N6-059)
- 벤치: `./_bench_plan.md` (16 기준선)
- 칩맵: `./_chip_mapping.md`
- 레지스트리: `./_registry.json` v1.3.0
- 수렴: `../n6shared/convergence/n6-architecture.json`
