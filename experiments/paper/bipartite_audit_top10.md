# PAPER-P4-2 — bipartite fit>=0.95 상위 10 쌍 grep 감사

**작성일**: 2026-04-14
**근거**: PAPER-P4-2 로드맵 항목
**소스**: papers/lint_progress.jsonl (3023 엣지, record_type=edge)
**감사 방법**: 각 쌍의 tech 키워드를 해당 papers/n6-{paper}-paper.md 본문에서
  정확 일치 + 변형 5종(underscore→space, dash, 한글 번역, 약어) grep

---

## 감사 대상 — fit 내림차순 상위 10

| # | tech | paper | fit | 변형 검색어 | 결과 |
|---|------|-------|-----|-----------|------|
| 1 | mamba2 | anima-soc | 1.0000 | mamba, mamba2, mamba-2, ssm, state space | **MISS** |
| 2 | rwkv | anima-soc | 1.0000 | rwkv, RWKV, receptance weighted | **MISS** |
| 3 | boltzmann_gate | anima-soc | 0.9863 | boltzmann, energy gate, 볼츠만 | **MISS** |
| 4 | rfilter_phase | anima-soc | 0.9851 | rfilter, phase filter, 위상 필터 | **MISS** |
| 5 | hyena | anima-soc | 0.9810 | hyena, long conv, 장거리 합성곱 | **MISS** |
| 6 | bitnet | anima-soc | 0.9751 | bitnet, 1-bit, 이진 가중치 | **MISS** |
| 7 | vq_vae | anima-soc | 0.9739 | vq-vae, vqvae, vector quantiz, 벡터 양자화 | **MISS** |
| 8 | rwkv | unified-soc | 0.9711 | rwkv, RWKV, receptance weighted | **MISS** |
| 9 | hyena | unified-soc | 0.9681 | hyena, long conv, 장거리 합성곱 | **MISS** |
| 10 | top_k_sparsity | anima-soc | 0.9675 | top-k, top_k, sparsity, 희소 | **MISS** |

## 감사 결과

**0 / 10 PASS — 전수 MISS**

## 진단

### 원인 분석

bipartite 매칭 3023 엣지는 **도메인 태그 · 기법 카테고리 · 제목 토큰** 기반
휴리스틱으로 산출되었으며, 논문 본문(body text) 전수 검색이 아니다.

상위 10 쌍이 전부 MISS 라는 결과는 매칭 알고리즘이 **메타데이터 유사성**만
반영하고 **본문 내 기술 기술(description)** 은 전혀 검증하지 않았음을 확인한다.

특히 fit=1.0 2건(mamba2, rwkv → anima-soc)은 "sota" 렌즈 기반으로
"SoC 도메인에 SOTA 기법이면 fit=1.0" 규칙이 적용된 것으로 추정되며,
anima-soc 논문 본문에는 mamba2, rwkv 단어가 단 한 번도 등장하지 않는다.

### 거짓 양성 비율

| 구간 | 대상 | PASS | MISS | 거짓 양성율 |
|------|------|------|------|-----------|
| fit=1.0 | 2건 | 0 | 2 | 100% |
| fit>=0.95 | 10건 | 0 | 10 | 100% |

honest-limitations.md §9에 기록된 우려가 감사로 확인됨.

### 논문 상태 확인

- `papers/n6-anima-soc-paper.md`: 존재, 698줄, 24,230 bytes, [CANONICAL v2]
- `papers/n6-unified-soc-paper.md`: 존재 (미확인 길이)
- 두 논문 모두 n=6 산술 좌표 매핑 형태로 작성되어 있으며,
  개별 AI 기법(mamba2, rwkv 등)을 기술하는 섹션이 부재.

## 후속 조치 권고

1. **bipartite 매칭 알고리즘 재설계 필요** — 현 키워드 휴리스틱은
   본문 언급 여부와 무관한 거짓 양성을 대량 생산. 최소한 TF-IDF 또는
   본문 토큰 교집합 기반 필터 추가 필요.

2. **fit_score 해석 주의** — fit=1.0 이 "논문이 해당 기술을 깊이 다룸" 을
   의미하지 않는다. 현재는 "같은 카테고리 태그를 공유함" 수준.

3. **honest-limitations.md §9 통계 추가** — 본 감사 결과(0/10 PASS, 
   거짓 양성율 100%)를 §9에 append.

4. **3023 엣지 중 본문 언급 비율 추정** — 전수 감사는 비용 초과이므로
   랜덤 50건 샘플링으로 추정치 산출 권고.

---

## 정직 기록

본 감사는 R0 (정직 검증), R3 (측정값 필수) 원칙을 준수한다.
MISS 결과를 축소하거나 부분 매칭으로 완화하지 않는다.

검색 변형 확장(5종)에도 불구하고 0/10 이므로 결과는 강건하다.
