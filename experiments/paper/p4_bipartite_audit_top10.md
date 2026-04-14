# PAPER-P4-2 — Bipartite 상위 10 쌍 실측 감사

## 목적

`papers/lint_progress.jsonl`(P1-4 산출, 3132줄, 3023 엣지 + 108 paper 기록) 의
`{tech, paper, fit_score, evidence_lens}` bipartite 엣지 중 fit_score 상위 10건을
실제 논문 본문에 대해 grep 실측 감사한다. 키워드가 본문에 등장하는지 확인하여
bipartite heuristic 의 MATCH / NEAR / MISS 비율을 산출한다.

## 입력

- 경로: `/Users/ghost/Dev/n6-architecture/papers/lint_progress.jsonl`
- 엣지 포맷: `{"tech": ..., "tech_cat": ..., "paper": ..., "fit_score": 0.0~1.0, "evidence_lens": "..."}`
- fit_score 분포:
  - fit == 1.0: 2건 (mamba2/anima-soc, rwkv/anima-soc)
  - fit >= 0.96: 10건 (전부 anima-soc 논문 대상)
  - 이하 연속 분포
- 본 감사는 fit 내림차순 상위 10 쌍을 표본으로 삼았으며, 10건 전부 논문
  `papers/n6-anima-soc-paper.md` 단일 타깃으로 수렴했다. 이는 lint_progress 가
  "현대 ML/DL 직결 기법"(compress/sparse/sota) 을 anima-soc 단일 논문에
  몰아매핑한 heuristic 때문이다.

## 감사 방법

1. 각 tech 키워드에 대해 다음 변형을 lower-case substring 매칭:
   - 원형 (`flash_attention`)
   - 언더스코어 → 공백 (`flash attention`)
   - 언더스코어 → 하이픈 (`flash-attention`)
   - 언더스코어 제거 (`flashattention`)
2. 판정:
   - MATCH: 원형 혹은 공백/하이픈 변형이 본문에 존재
   - NEAR: 부분 키워드(4자 이상)만 존재, 원형 없음
   - MISS: 어떤 변형도 존재하지 않음
3. 논문 파일: `/Users/ghost/Dev/n6-architecture/papers/n6-anima-soc-paper.md` (698 줄)

## 결과 표

| #  | tech                        | tech_cat | paper     | fit   | 상태  | hit_line | 비고 |
|----|-----------------------------|----------|-----------|-------|-------|----------|------|
| 1  | mamba2                      | sota     | anima-soc | 1.000 | MISS  | 0        | Mamba / SSM 어휘 전무 |
| 2  | rwkv                        | sota     | anima-soc | 1.000 | MISS  | 0        | RWKV / recurrent 어휘 전무 |
| 3  | boltzmann_gate              | sparse   | anima-soc | 0.986 | NEAR  | 2        | "gate" 는 존재하나 σ(6)=12 공정통합 gate 의미 — boltzmann 무관 |
| 4  | rfilter_phase               | sparse   | anima-soc | 0.985 | MISS  | 0        | r-filter / phase filter 어휘 전무 |
| 5  | bitnet                      | compress | anima-soc | 0.975 | MISS  | 0        | 1-bit LLM / BitNet 어휘 전무 |
| 6  | vq_vae                      | compress | anima-soc | 0.974 | MISS  | 0        | 벡터 양자화 / VAE 어휘 전무 |
| 7  | top_k_sparsity              | sparse   | anima-soc | 0.968 | MISS  | 0        | top-k / sparsity 어휘 전무 |
| 8  | pruning_lottery_ticket      | compress | anima-soc | 0.965 | MISS  | 0        | 가지치기 / lottery ticket 어휘 전무 |
| 9  | tensor_decomposition        | compress | anima-soc | 0.962 | NEAR  | 1        | "축 분해 (Axis Decomposition)" 는 등장 — 텐서 분해와는 다른 σ=12 축 분해 의미 |
| 10 | mixture_of_tokenizers       | compress | anima-soc | 0.961 | MISS  | 0        | tokenizer / MoE 어휘 전무 |

## 집계

| 판정  | 건수 | 비율  |
|-------|------|-------|
| MATCH | 0    | 0.0%  |
| NEAR  | 2    | 20.0% |
| MISS  | 8    | 80.0% |

- **감사율(MATCH)**: 0 / 10 = **0.0%**
- **포괄 일치(MATCH + NEAR)**: 2 / 10 = **20.0%**
- **MISS 비율**: 8 / 10 = **80.0%**

NEAR 2건(boltzmann_gate, tensor_decomposition) 도 문맥상 실제 관련성이 없다.
"gate" 는 n=6 공정 통합 σ(6)=12 gate 의미이고, "축 분해" 는 σ=12 축을 따라
가른다는 의미라서 머신러닝 맥락의 boltzmann gate / tensor decomposition 과
완전히 다른 사용이다. 엄격히 해석하면 실질 MATCH 율은 **0 / 10 = 0%** 에 수렴한다.

## 한계 (정직 기록)

1. **bipartite 는 키워드 기반 휴리스틱**. lint_progress 는 논문 카테고리 ×
   기법 카테고리 매칭에 의해 fit_score 를 부여하며, 본문 실언급을 확인하지
   않는다. 이 때문에 "ml/dl-direct" 섹션을 가진 논문에 대해 현대 ML 기법
   전부가 높은 fit 을 받게 된다.

2. **anima-soc 논문의 편향**. anima-soc 는 σ=12 / τ=4 / φ=2 파라미터 기반
   n=6 SoC 아키텍처를 다룬 논문으로, 실제로는 mamba2/rwkv/bitnet 등 구체
   ML 기법을 본문에 기술하지 않는다. bipartite 가 이 논문에 ML 기법
   상위를 몰아준 것은 evidence_lens = "ml/dl-direct|compress" 같은 렌즈
   태깅이 과다 포함된 것이 원인으로 추정된다.

3. **키워드 매칭의 맹점**. 논문이 "벡터 양자화" 같은 한글 번역으로 언급했을
   가능성이 있으나, 본 감사에서는 영문 키워드 변형만 시도했다. 추가 감사
   라운드가 필요하다면 다음과 같은 한글 키워드도 포함해야 한다:
   - mamba2 → "맘바", "상태공간모델", "SSM"
   - rwkv → "순환", "linear attention"
   - vq_vae → "벡터 양자화", "벡터양자화 VAE"
   - pruning_lottery_ticket → "가지치기", "lottery ticket"
   - bitnet → "1비트", "BitNet"
   - top_k_sparsity → "top-k 희소성"

4. **상위 10 전부 같은 논문**. fit 0.96~1.0 구간 전부가 anima-soc 로
   몰려있는 것 자체가 bipartite heuristic 이 실세계 관련성을 반영하지
   못한다는 신호이다. 건전한 bipartite 라면 서로 다른 논문(agi-architecture,
   ai-17-techniques 등)으로 분산되어야 한다.

5. **긍정 추정의 보정**. 본 감사는 상위 10 샘플에서 실 MATCH 0% 였으나,
   이것이 전체 3023 엣지 MATCH 율 0% 를 뜻하지는 않는다. 하위 fit 대역에서는
   실제 관련 쌍이 더 많이 등장할 수 있다 (random sample 필요).

## 권고

1. `lint_progress.jsonl` v2 에서는 본문 실 grep 매칭을 fit_score 의 필수
   성분으로 가중 투입할 것. (예: fit_new = 0.5 * category_match + 0.5 * body_grep_match)
2. anima-soc 같이 n=6 구조 중심 논문은 ML 기법 bipartite 대상에서 제외하거나
   별도 카테고리로 분리할 것.
3. 한글 논문 본문 grep 시 한영 쌍 키워드 사전을 사용할 것.
4. PAPER-P4-3 에서는 mid-fit (0.70~0.85) 구간의 random 10 샘플도 감사하여
   fit_score 구간별 MATCH 곡선을 산출할 것.

## 산출물

- 이 문서: `/Users/ghost/Dev/n6-architecture/experiments/paper/p4_bipartite_audit_top10.md`
- 입력: `/Users/ghost/Dev/n6-architecture/papers/lint_progress.jsonl`
- 감사 대상: `/Users/ghost/Dev/n6-architecture/papers/n6-anima-soc-paper.md`
- 감사 일자: 2026-04-14
