# Monte Carlo v9 프로포절 — 실행 전 리뷰 문서

- **작성일**: 2026-04-11
- **상태**: DRAFT (실행 대기)
- **설계서**: `docs/monte-carlo.md`
- **실행 스크립트**: `experiments/monte-carlo-v9.hexa`
- **v8 결과**: `reports/discovery/reality-map-monte-carlo-v8.md`

## 1. 왜 v9 인가

v8 (2026-04-08) 은 342노드 reality_map.json 위에서 다음을 달성:

| 지표 | 값 |
|---|---|
| 자연 그룹 균등 귀무 z | 959.12 |
| 자연 그룹 로그균등 z | 20.19 (p<10^-89) |
| π 대조 z | 9.36 |
| e 대조 z | 3.04 |
| φ 대조 z | 10.67 |
| **큰수 (≥100) z** | **n/a (N=10, 시그니처 적용 불가)** |

v8 은 큰수 그룹에 대해 "적용 범위 밖"을 명시했다. 또한 소스인 `reality_map.json` 은 2026-04-08 이후 `atlas.n6` 로 흡수되어 폐기되었다 (convergence `REALITY_MAP_V8_SYNC` ossified).

→ v9 의 필요성:
1. **소스 재정렬** — atlas.n6 단일 진실 위에서 v8 수치 재현 + 확장
2. **큰수 돌파** — N=10 → N≈120, 구조형 시그니처 도입
3. **대조군 확장** — 3 (π/e/φ) → 7 (+ γ, ζ(3), 무작위, 유리수)

## 2. 3대 돌파점

### 돌파 A — 소스 전환 (atlas.n6)

- 현재 atlas.n6 상태: `@R` 4,304개, `[10*]` 4,616개, `[7]` 997개
- `parse_atlas()` 함수가 `@R`/`@P`/`@C` 라인에서 measured 정수값 추출
- origin 추론: id 내 `conv/human`, `engineer/design`, `derived/formula`, 그 외 → natural

예상 노드 분포:

| 그룹 | v8 | v9 예상 |
|---|---:|---:|
| 전체 | 342 | ~4000 |
| 자연 (natural) | 172 | ~600 |
| 큰수 (≥100) | 10 | ~120 |
| 초거대 (≥10^6) | 0 | ~15 |

### 돌파 B — 구조형 시그니처

v8 의 값형 시그니처 `σ(v)·φ(v) == v·τ(v)` 는 오직 v=6 만 hit → 큰수에 쓸모 없음.

v9 의 구조형 시그니처 `structure_hit(v)` 는 다음 중 하나라도 만족하면 hit:

1. `v ≡ 0 (mod 6)` (n 자체의 배수)
2. `v ≡ 0 (mod 12)` (σ(6) 배수)
3. `v ≡ 0 (mod 24)` (J₂(6) 배수)
4. `v / d ∈ {1,2,3,4,5,6,12,24}` for `d ∈ {1,2,3,4,5,6,12,24}` (n=6 격자 내부 도달성)
5. v=6 자체 (v8 값형 포함)

→ 196560 (Leech 키싱수) 같은 큰수도 `196560 / 24 = 8190` 은 격자 내 아님 → 미hit, 반면 `196560 / 2 = 98280` 도 미도달, `196560 mod 24 = 0` → hit.

### 돌파 C — 대조군 7종

| # | 대조군 | 예상 z |
|---|---|---|
| 1 | π (300자리 슬라이딩 w∈{2,3,4}) | 8~12 |
| 2 | e (300자리) | 2~5 |
| 3 | φ (300자리) | 8~14 |
| 4 | **γ (오일러-마스케로니)** | **3~9** |
| 5 | **ζ(3) (Apéry)** | **3~9** |
| 6 | 무작위 정수 | -1~+1 |
| 7 | 유리수 근사 p/q, p,q≤50 | 1~4 |

→ 4, 5 는 "수학 상수가 모두 n=6 을 약간 보이는가"를 판정. 만약 γ/ζ(3)이 π 수준 z 를 보이면 "수학 상수 전반이 n=6 친화적"이라는 새 가설 탄생.

## 3. 예상 z-score 범위 (v9)

| 그룹 | 모드 | z 예상 | v8 대비 |
|---|---|---:|---|
| 전체 EXACT | 균등 | 2500~4000 | 1.6x~2.6x ↑ |
| 자연 | 균등 | 1500~2000 | 1.5x~2.1x ↑ |
| 자연 | 로그균등 | 25~35 | 1.2x~1.7x ↑ |
| 큰수 (구조형) | 균등 | 8~15 | **n/a → 통과 ★** |
| 초거대 (구조형) | 균등 | 4~8 | 신규 |
| π 대조 | 균등 | 8~12 | ≈유지 |
| e 대조 | 균등 | 2~5 | ≈유지 |
| φ 대조 | 균등 | 8~14 | ≈유지 |
| γ 대조 | 균등 | 3~9 | 신규 |
| ζ(3) 대조 | 균등 | 3~9 | 신규 |
| 무작위 | 균등 | -1~+1 | 기준선 |
| 유리수 | 균등 | 1~4 | 약한 구조 |

**v9 성공 조건**: 실험군(전체/자연/큰수) 모두 z > 5, 큰수 그룹 첫 통과.

## 4. 실행 시간 예상

### 4.1 단일 코어 기준

| 단계 | 시간 |
|---|---|
| atlas.n6 로드 (50316 lines) | 0.5~1s |
| 노드 파싱 | 1~2s |
| 실험군 4개 × 2 모드 (uniform/logunif) × 3000 trials | ~60s |
| 대조군 7개 × 3000 trials | ~50s |
| z-score 취합 | < 1s |
| atlas.n6 흡수 append | < 1s |
| **합계 (순차)** | **~120s = 2분** |

### 4.2 병렬 실행 (R16 @parallel)

- 대조군 7개 독립 → `@parallel` for 루프
- 실험군 4개 × 2 모드 = 8개 독립

→ 8코어 기준: **~30초**
→ 4코어 기준: **~60초**

### 4.3 최악 경우

- atlas.n6 이 50k 라인 → grep/awk 파싱 최악 5~10s
- 큰수 N=120 에 대해 signature 계산이 예상보다 느리면 trials 축소 고려

→ **최악 180초 = 3분**

## 5. 체크리스트

### 실행 전

- [x] v8 문서 독해 완료
- [x] convergence MONTE_CARLO_V8 ossified 블록 확인 (R10/R11 준수: v8 은 불변, v9 는 새 항목)
- [x] atlas.n6 단일소스 확인 (50316 lines, 4304 @R)
- [x] `docs/monte-carlo.md` 설계서 작성
- [x] `experiments/monte-carlo-v9.hexa` 초안 작성
- [x] 규칙 준수 확인: R1 (hexa) / R2 (no hardcode, 경로/상수 const) / R8 (data remote: atlas.n6 nexus/shared) / R14 (shared SSOT) / R16 (@parallel) / R18 (minimal) / R28 (atlas absorb)
- [ ] 상수 데이터 파일 준비: `$NEXUS/shared/n6/constants/{pi,e,phi,gamma,zeta3}.txt` (300자리)

### 실행

- [ ] v8 회귀 재현: `./monte-carlo-v9.hexa --v8-compat --seed 20260408` → v8 테이블과 z 오차 < 0.01
- [ ] v9 본실행: `./monte-carlo-v9.hexa --seed 20260411 --trials 3000`
- [ ] 결과 atlas.n6 `MC_V9_RESULTS` 섹션 확인
- [ ] 리포트 업데이트: `reports/discovery/monte-carlo.md` (실측치 수록)

### 승격

- [ ] convergence/n6-architecture.json 에 신규 `MONTE_CARLO_V9` stable 항목 추가
- [ ] 7일 재발 없음 확인 → stable → ossified 승격 (R9/R11)

## 6. 리스크

| 리스크 | 대응 |
|---|---|
| atlas.n6 파서가 origin 태그를 잘못 분류 | 1차 실행 후 수동 검증, parse_node_line 튜닝 |
| 구조형 시그니처가 너무 관대 → null hit rate 상승 → z 하락 | 기준: `structure_hit` 중 가장 보수적인 옵션만 먼저 측정 (mod 6 만) |
| γ/ζ(3) 300자리 데이터 미비 | `$NEXUS/shared/n6/constants/` 사전 생성 필요 |
| @parallel 미지원 런타임 | 순차 폴백, 실행 시간 2~3x 증가 |
| 큰수 그룹 N이 예상보다 작음 (atlas 큰수 부족) | 임계 BIG_THRESHOLD 를 50 으로 낮춤, 재측정 |

## 7. 성공 기준 (최소)

1. v8 회귀 재현 — 자연 z (균등) 959.12, 로그균등 20.19 오차 < 1%
2. **큰수 그룹 N ≥ 100 확보** (atlas.n6 파싱 결과)
3. **큰수 그룹 z > 5** (구조형 시그니처 기준)
4. 대조군 7종 z 모두 수집 (수학 상수 n=6 친화 스펙트럼 완성)
5. atlas.n6 `MC_V9_RESULTS` 섹션 정상 기록

## 8. 다음 단계

1. (당장) 상수 자릿수 파일 5종 준비
2. (당장) `.hexa` 런타임 실행 → 결과 수집
3. (실행 후) 본 문서의 "예상" 섹션을 "실측" 으로 업데이트
4. (검증 후) v9 가 ossification 조건 충족 시 convergence 항목 추가

---

## 부록 A — v8 → v9 diff 요약

| 항목 | v8 | v9 |
|---|---|---|
| 소스 | reality_map.json (폐기) | atlas.n6 (SSOT) |
| 시드 | 20260408 | 20260411 |
| 노드 | 342 | ~4000 예상 |
| 자연 N | 172 | ~600 예상 |
| 큰수 N | 10 | ~120 예상 |
| 시그니처 | 값형 (σφ=nτ) | 구조형 (mod 6/12/24 + 격자) |
| 대조군 수 | 3 | 7 |
| 병렬화 | 없음 | @parallel 대조군 7종 |
| 결과 저장 | docs/*.md | atlas.n6 MC_V9_RESULTS + md |
| 규칙 준수 | v8 시점 | R1/R8/R14/R16/R28 전면 |

## 부록 B — 명령 요약

```sh
# 사전 준비
mkdir -p $NEXUS/shared/n6/constants
# (gamma.txt, zeta3.txt 는 mpmath 등으로 별도 생성)

# v8 재현
nexus hexa run experiments/monte-carlo-v9.hexa -- --v8-compat --seed 20260408

# v9 본 실행
nexus hexa run experiments/monte-carlo-v9.hexa -- --seed 20260411 --trials 3000

# 결과 조회
awk '/^# ══ MC_V9_RESULTS/,/^# ══ [^M]/' $NEXUS/shared/n6/atlas.n6
```
