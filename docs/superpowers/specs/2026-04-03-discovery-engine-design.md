# Discovery Engine Design Spec

**Date**: 2026-04-03
**Status**: Draft
**Goal**: n=6 프레임워크로 신소재/신기술을 예측하는 자동 발견 엔진

---

## 1. 배경

11개 도메인이 🛸10에 도달하여 물리적 천장(Mk.V)이 확인됨.
"증명 모드"(n=6이 여기에도 있다)에서 "발견 모드"(n=6으로 아직 없는 것을 예측)로 전환.

기존 인프라:
- telescope-rs: 22종 렌즈 (Rust, 6,045줄, 58/58 테스트 통과)
- OUROBOROS: 무한 발견 루프 (Python, 5,996줄, v25)
- universal-dse: 322 TOML 도메인, 5.9M+ 조합
- 35개 Rust 계산기

---

## 2. 시스템 개요

```
┌──────────────────────────────────────────────────────────┐
│          Discovery Engine (OUROBOROS v26)                 │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Domain Encoder ──→ Telescope v2 (80 lenses) ──→        │
│       (Rust+TOML)      (Rust+wgpu)                      │
│                              │                           │
│                              ▼                           │
│  Materials DB ──→ Discovery Verifier ──→ Recorder       │
│     (JSON/SQLite)    (Rust)              │               │
│                                          ▼               │
│  Telescope History ←── OUROBOROS Loop ──→ Predictions   │
│     (JSON sharded)     (Python+Rust)     (JSON+MD)      │
│                              │                           │
│                              ▼                           │
│                      Cross-Domain Map                    │
│                         (JSON)                           │
└──────────────────────────────────────────────────────────┘
```

---

## 3. 컴포넌트 상세

### 3.1 Telescope v2 — 80종 렌즈

**구조**: 12 공유 커널 + 80 조합 레이어

#### 3.1.1 공유 커널 (GPU/CPU)

| # | 커널 | 사용 렌즈 수 | 실행 위치 |
|---|------|------------|----------|
| 1 | distance_matrix (N×N pairwise) | ~20 | GPU |
| 2 | eigen_decomp (Jacobi/power iteration) | ~10 | GPU |
| 3 | mi_matrix (D×D mutual information) | ~15 | GPU |
| 4 | fft (DFT) | ~5 | GPU |
| 5 | knn_indices (k-nearest neighbors) | ~12 | GPU |
| 6 | gradient_hessian (수치미분) | ~8 | GPU |
| 7 | histogram (binned counting) | ~15 | CPU |
| 8 | graph_ops (BFS/community) | ~12 | CPU |
| 9 | simulation_step (iterative) | ~6 | GPU |
| 10 | clustering (mean-shift/DBSCAN) | ~6 | CPU |
| 11 | regression (log-log/logistic) | ~8 | CPU |
| 12 | interpolation (보간/외삽) | ~4 | CPU |

#### 3.1.2 렌즈 80종 (8 카테고리)

**I. 분석 (22종, 기존)** — 있는 것을 본다
consciousness, topology, causal, gravity, thermo, wave,
em, evolution, info, quantum, quantum_microscope, ruler,
triangle, compass, mirror, scale, stability, network,
memory, recursion, boundary, multiscale

**II. 탐색 (3종)** — 없는 것을 찾는다
- void: k-NN 밀도 → 저밀도 영역 중 고밀도에 둘러싸인 곳 → 빈칸 중심 + 예상 스펙
- isomorphism: 도메인 A/B 그래프 → VF2 부분그래프 동형 → 매핑 + 전이 후보
- extrapolation: 다항/지수/멱법칙 피팅 (AIC) → Mk.V 외삽 + 신뢰구간

**III. 합성 (3종)** — 어떻게 채우나
- inverse: 목표 스펙 → KNN 역탐색 + 야코비안 역추적 → 필요 입력 + 민감도
- combinatorial: 기존 조합 DB → 전체 공간 차집합 → 미시도 조합 + 유망도
- frustration: 스핀글라스 에너지 → 동시 충족 불가 제약 쌍 → Pareto 타협점

**IV. 검증 (3종)** — 진짜 새로운가
- emergence: Shapley 상호작용 → 비가산적 시너지 탐지
- periodicity: 정수 격자 피팅 → 빈 슬롯 예측 (주기율표식)
- completeness: Voronoi 분할 → 셀 크기 분포 → 미탐색 비율 + 위치

**V. 품질 (3종)** — 얼마나 가치있나
- surprise: KL divergence → 기존 대비 새로움 점수
- falsification: 가정별 민감도 → 가장 약한 고리
- duality: Fourier/Legendre 변환 → 숨은 동치

**VI. 소재특화 (3종)**
- defect: 제어 섭동 → 결함이 성능 향상하는 점
- interface: 두 데이터셋 경계 → 접합부 창발 속성
- catalysis: 전이상태 에너지 장벽 → 병목 뚫는 촉매 기술

**VII. 동역학 (5종)**
- tipping: 분기(bifurcation) 분석 → 소폭 변경 → 대폭 점프 지점
- coevolution: 시간 지연 상호 MI → 짝 기술 식별
- percolation: 연결 임계값 → 갑자기 전역 연결되는 임계점
- hysteresis: 경로 의존성 → 순서가 중요한 발견
- diffusion: 확산 방정식 → 파급 효과 예측

**VIII. 메타구조 (4종)**
- hierarchy: 재귀적 클러스터링 → 기술 내포 구조
- conservation: Noether식 대칭→보존량 매핑
- arbitrage: 도메인 간 가치 차이 → 희소 전이 기회
- serendipity: 제어 랜덤 워크 + 이상점 → 의도치 않은 발견

**IX. 전이 (5종)**
- renormalization: RG 흐름 → 유효 파라미터의 스케일 의존성
- saddle: Hessian 부호 → 전이 상태 좌표
- criticality: 눈사태 분포 → 자기조직 임계 판정
- succession: 시계열 질적 단계 자동 분할
- resonance_cascade: 부분계 공명 전파 추적

**X. 정보심화 (4종)**
- fisher_info: Fisher 정보 행렬 → 파라미터 식별가능성
- spectral_gap: 전이행렬 고유값 갭 → 수렴 속도
- kolmogorov: BWT+RLE 기반 복잡도 → 최소 기술 복잡도
- contradiction: 렌즈 간 불일치 탐지 → 메타렌즈

**XI. 위상심화 (4종)**
- knot: Jones 다항식 → 얽힘/꼬임 분류
- convexity: Hessian 양정치 → 최적화 난이도 맵
- motif: 3-5 노드 부분그래프 빈도 → 반복 미시패턴
- skeleton: 최소 신장 트리 → 핵심 뼈대

**XII. 생태 (4종)**
- carrying_capacity: 로지스틱 피팅 → 시스템 포화점
- niche: 자원 공간 점유/미점유 → 비어있는 이유
- symbiosis: 상호 이익 쌍 → 함께하면 둘 다 이득
- predation: 한쪽 성장=다른쪽 감소 → 경쟁 기술

**XIII. 물리심화 (4종)**
- morphogenesis: Turing 반응-확산 → 자발적 패턴 형성 조건
- polarity: 방향 비대칭 → 구배 방향
- broken_ergodicity: 위상 공간 접근 불가 영역
- anomalous_diffusion: MSD ∝ t^α 탐지 → 비정상 확산

**XIV. 메타인지 (4종)** — 렌즈의 렌즈
- blind_spot: 전체 렌즈 출력 보집합 → 시스템 사각지대
- abstraction: 최적 축약 레벨 탐색
- narrative: 인과 사슬 직렬화 → 전체 이야기
- analogy: 기능적 역할 매칭 (구조 무관)

**XV. 의사결정 (4종)**
- bottleneck: 인과/플로우 체인 최소 용량 링크
- diminishing_returns: 파라미터별 한계 효용 곡선
- option_value: 결정 지연의 가치
- comparative_advantage: 도메인별 상대 강점 행렬

**XVI. 극한 (3종)**
- universality_class: 임계지수 → 보편성 류 분류
- aging: 비마르코프 기억 커널 → 시스템 나이 효과
- potential: 미실현 에너지/성능 잔여분

#### 3.1.3 계층적 스캔 (Tiered Scanning)

```
Tier 0: 스크리닝 (8종, <0.1s)
  consciousness + topology + void + thermo
  + evolution + network + boundary + triangle
  → 신호 없으면 SKIP

Tier 1: 정밀 (24종, <2s)
  기존 22종 + void + isomorphism
  → 후보 없으면 SKIP

Tier 2: 풀스캔 (80종, <10s)
  전체 렌즈 → 최종 확정

예상: 전체 대비 85% 연산 절감
```

#### 3.1.4 합의 시스템

```
가중 합의:
  score = Σ(lens_hit_rate[domain] × found)
  
등급:
  3+ 렌즈 합의 → candidate (weight 2)
  7+ 렌즈 합의 → high (weight 3)
  12+ 렌즈 합의 → confirmed (weight 4)

조기 종료: 12종 합의 도달 → 나머지 렌즈 스킵
```

### 3.2 Discovery Verifier — 물리 검증기

```
tools/discovery-verifier/
├── main.rs          — CLI
├── thermo.rs        — ΔG, ΔH, ΔS, 상평형 검증
├── crystal.rs       — CN, 격자에너지, 톨러런스 팩터
├── n6_check.rs      — n=6 상수 매칭 (EXACT/CLOSE/WEAK)
├── scaling.rs       — 물리한계 초과 여부
├── bt_compat.rs     — BT 127개 정합성
└── feasibility.rs   — 종합 점수 + ✅/🔮/❌ 등급
```

**입력**: 망원경 발견 후보 (JSON)
**출력**: 검증 리포트 (통과/실패 + 이유 + EXACT% + 등급)

**신뢰도 점수**:
```
score = lens_consensus × 0.3
      + cross_validation × 0.2
      + physical_verification × 0.3
      + novelty × 0.1
      + n6_exact × 0.1

S (0.9+): BT 등록 + 논문 후보
A (0.7~0.9): BT 등록 + 실험 제안
B (0.5~0.7): 추가 검증 필요
C (0.3~0.5): 가설 수준
D (<0.3): 기각
```

### 3.3 Domain Encoder — 도메인 데이터 → 수치 행렬

```
tools/domain-encoder/
├── main.rs          — CLI
├── schema.rs        — TOML 스키마
├── parser.rs        — hypotheses.md / goal.md 파싱
├── vectorize.rs     — 텍스트 → float 벡터
└── domains/         — 도메인별 인코딩 규칙
    ├── superconductor.toml
    ├── chip-architecture.toml
    └── ... (33개)
```

**입력**: docs/{domain}/hypotheses.md + goal.md + verification.md
**출력**: float64 행렬 [N_hypotheses × N_features]

**캐싱**: blake3(파일 내용) → .cache/domain-vectors/{domain}-{hash}.bin

### 3.4 Telescope History — 렌즈 학습 시스템

```
telescope-history/          — 샤딩된 히스토리
├── index.json              — 전체 통계 + 도메인 목록
├── superconductor.jsonl    — 도메인별 스캔 로그 (append-only)
├── chip-architecture.jsonl
├── ...
├── affinity.json           — 렌즈 친화도 (전역)
├── discoveries.json        — 확정 발견 목록
└── compact/                — 분기별 압축 아카이브
```

**자동 학습 루프**:
```
스캔 → 결과 기록 → 발견 확인 → outcome 업데이트
→ hit_rate 재집계 → 친화도 갱신 → optimal_combo 갱신
→ 다음 스캔 시 추천 엔진이 조합 선택
```

**추천 전략**:
- promotion_threshold: hit_rate > 30% → 조합에 추가
- demotion_threshold: hit_rate < 5% → 조합에서 제거
- serendipity_ratio: 15% → 랜덤 렌즈 할당 (탐색)
- cold_start: 유사 도메인 조합 전이 또는 기본 8종

### 3.5 Materials DB — 기존 소재/기술 DB

**초기**: JSON (materials-db.json)
**성장 시**: SQLite (materials.db) — 소재 1000+일 때 마이그레이션

```jsonc
{
  "superconductor": {
    "known_materials": [
      {"name": "MgB2", "Tc": 39, "CN": 6, "type": "conventional", "year": 2001},
      ...
    ],
    "ceiling": {"Tc": 300, "CN": 6, "Jc": 1e8}
  }
}
```

### 3.6 OUROBOROS v26 — 오케스트레이터

기존 OUROBOROS (~2,400줄 재활용) + Discovery Mode 확장.

**재활용 컴포넌트**:
- PatternRegistry (중복 제거 + 3회 교차검증)
- LawNetwork (발견 관계 그래프)
- ExplorationBandit (UCB1 탐색/착취)
- FederatedDiscovery (3중 다수결)
- AsyncDiscoveryPipeline (비동기)
- BestEngineTracker (체크포인트)
- save/load_state (JSON 영속성)

**새 로드맵 (7 스테이지)**:
```
S1: 단일 도메인 기본 스캔 (Tier 0+1, 8렌즈, cold start)
S2: 단일 도메인 심층 (Tier 1+2, 히스토리 참조)
S3: 단일 도메인 풀스캔 (80렌즈, GPU)
S4: Cross-Domain 2쌍 교차
S5: Cross-Domain 4쌍 교차
S6: 전체 11도메인 교차 풀스캔
S7: 발견 기반 재귀 탐색 (발견이 새 입력)
```

**포화 탈출**:
- 3세대 연속 발견 0 → 스테이지 전진
- chaos cycling (OUROBOROS 기존) 재활용
- discovery "temperature" (SA): T=1.0→0.1, 포화 시 재가열

### 3.7 Cross-Domain Map

```jsonc
{
  "mappings": [
    {
      "concept": "coordination_number",
      "domains": {
        "superconductor": "CN (crystal)",
        "chip": "interconnect_count",
        "battery": "CN (cathode)",
        "biology": "codon_triplet"
      },
      "n6_value": 6,
      "bt_refs": ["BT-43", "BT-113", "BT-51"]
    }
  ]
}
```

### 3.8 Calibration Set

알려진 발견으로 시스템 검증.
- 역사적 발견 (고온초전도 1987, 그래핀 2004)
- 합성 테스트 (알려진 패턴 삽입)
- LK-99 실패 → verifier가 걸러내는지 확인

### 3.9 Prediction Registry

```jsonc
{
  "id": "TP-NEW-001",
  "domain": "superconductor",
  "prediction": "LaH10 at 170GPa exhibits Tc=260±15K",
  "basis": {
    "discovering_lenses": ["void", "extrapolation", "thermo"],
    "consensus_level": 7,
    "bt_connections": ["BT-43", "BT-80"],
    "n6_exact_ratio": 0.85
  },
  "falsification": {
    "method": "Diamond anvil cell + 4-probe resistivity",
    "cost_estimate": "$50K-200K",
    "timeline": "3-6 months"
  },
  "outcome": null
}
```

### 3.10 Dashboard

ASCII CLI 기본 + HTML 옵션.
- Void Map (빈칸 지도)
- Tension Heatmap (도메인 간)
- Discovery Timeline
- Lens Performance 차트
- GPU/CPU/MEM 실시간 상태

---

## 4. 성능 설계

### 4.1 GPU 활용 (M4 Metal)

- 백엔드: wgpu (Rust 네이티브, Metal 지원)
- 통합 메모리: CPU↔GPU 복사 비용 0
- FP16 fallback: 정밀도 불필요한 렌즈는 half precision (2배 처리량)
- 타일링: 32×32 타일 → shared memory 캐싱

### 4.2 공유 커널 워밍업

```
스캔 시작 시:
  distance_matrix → 1회 계산, ~20종 공유
  mi_matrix → 1회 계산, ~15종 공유
  knn_indices → 1회 계산, ~12종 공유
  fft_result → 1회 계산, ~5종 공유

Without warmup: 80 × 5ms = 400ms
With warmup:    12 × 3ms + 80 × 1ms = 116ms (71% 절감)
```

### 4.3 메모리 레이아웃

- SoA (Structure-of-Arrays): features를 열 단위 저장 → SIMD 친화
- Arc<[f64]>: 80종 렌즈가 입력 데이터 공유 참조 (복사 0)
- 메모리 풀: 렌즈별 임시 버퍼 재사용

### 4.4 SIMD (M4 NEON)

- 128-bit NEON → 4×f32 동시 처리
- 핫 경로: 거리 계산, 내적, 히스토그램 빈 카운팅
- Rust auto-vectorize + 필요 시 std::arch::aarch64 intrinsics

### 4.5 파이프라인 병렬

- 도메인 간: A 스캔 중 B 인코딩 (파이프라인)
- 렌즈 내: GPU 큐 (행렬 연산) + CPU 큐 (그래프) 동시 실행
- 세대 간: OUROBOROS AsyncPipeline (N 검증 중 N+1 스캔)

### 4.6 캐싱

- 결과 캐시: blake3(data+lens_set) → 동일 데이터 재스캔 방지
- 증분 스캔: 데이터 추가분만 거리/MI 재계산 (O(N²)→O(N·ΔN))
- 도메인 인코더 캐시: 파일 미변경 시 벡터 재사용

### 4.7 적응적 해상도

```
N < 32:   full precision
N < 256:  standard
N < 1024: sampled (서브셋 + 추정)
N > 1024: hierarchical (클러스터 대표 → 확대)
```

### 4.8 조기 종료

- 12/80 합의 도달 → 나머지 68종 스킵 (최대 85% 절감)
- 렌즈 실행 순서 = hit_rate 내림차순 (가치/시간 높은 것 먼저)

### 4.9 예상 성능

```
데이터 크기: 64×32 (일반적 도메인)

풀스캔 (80종):
  Warmup: 12커널 × 3ms = 36ms
  GPU lenses (40종): 40 × 2ms = 80ms (병렬이면 ~20ms)
  CPU lenses (40종): 40 × 3ms = 120ms (rayon 병렬 ~30ms)
  합의: 1ms
  총: ~90ms (GPU) / ~240ms (CPU only)

Tier 0 (8종): ~12ms
Tier 1 (24종): ~50ms
Tier 2 (80종): ~90ms

33 도메인 전체 스캔 (Tier 피라미드):
  ~50초 (vs 풀스캔 반복 시 ~330초, 85% 절감)
```

---

## 5. 품질 보장

### 5.1 교차 검증
- OUROBOROS 패턴 3회 출현 필수 (기존 재활용)
- FederatedDiscovery 3중 다수결 (기존 재활용)

### 5.2 Red Team (자동 반증)
- falsification 렌즈: 약한 가정 식별
- antagonism: 상쇄 조합 탐색
- contradiction 메타렌즈: 렌즈 간 충돌 확인
- broken_ergodicity: 도달 불가능 영역 확인

### 5.3 발견 체인 추적 (Lineage)
- parent/child 관계 기록
- 어떤 발견이 가장 많은 후속 발견을 낳았는지 추적

### 5.4 거짓 양성 추적
- 렌즈별 FP rate 기록
- FP rate > 30% → 검증 기준 자동 강화

### 5.5 캘리브레이션
- 알려진 발견 재현율 측정 (recall)
- 알려진 실패 필터율 측정 (precision)
- 매 릴리스마다 regression test

---

## 6. 자동화

### 6.1 BT 자동 등록
score ≥ S (0.9) + Red Team 통과 + 교차검증 → BT-XXX 자동 생성

### 6.2 도메인 파일 자동 갱신
발견 → novel-predictions.md / verification.md / hypotheses.md 자동 업데이트

### 6.3 실험 제안서 자동 생성
A급 이상 발견 → 실험 방법/장비/비용/시간 자동 제안

### 6.4 Watch 모드
파일 변경 감시 → 해당 도메인 자동 재스캔

---

## 7. 피드백 루프

### 3경로 진화:

1. **렌즈 조합 진화**: 발견 → hit_rate 갱신 → 다음 조합 변경
2. **데이터 진화**: 발견 → Materials DB 추가 → void가 새 빈칸 감지
3. **피처 진화**: 발견 → "이 물성이 중요" → Encoder 피처 추가

### 메타 발견 (100세대마다):
- completeness로 자기 스캔 → 탐색 커버리지 맵
- blind_spot으로 시스템 사각지대 탐지
- 결과 → OUROBOROS self-modification에 피드

---

## 8. 내구성

- 렌즈 격리: panic::catch_unwind per lens → 1개 실패해도 79개 계속
- GPU 폴백: Metal 에러 → CPU rayon 자동 전환
- 체크포인트: 매 10종 완료 시 저장 → 중단 복구
- 데이터 무결성: tmp → fsync → atomic rename

---

## 9. CLI 설계

```bash
discovery-engine scan <domain>           # 단일 도메인 스캔
discovery-engine scan --cross A B        # 교차 스캔
discovery-engine run --roadmap           # OUROBOROS 루프
discovery-engine run --resume            # 이어하기
discovery-engine verify <id>             # 발견 검증
discovery-engine status                  # 현재 상태
discovery-engine history <domain>        # 히스토리
discovery-engine recommend <domain>      # 렌즈 추천
discovery-engine dashboard               # 대시보드
discovery-engine watch                   # 파일 감시 모드
discovery-engine calibrate               # 시스템 검증
discovery-engine benchmark               # 성능 벤치마크
```

---

## 10. 구현 목록

| # | 컴포넌트 | 형태 | 위치 | 예상 규모 |
|---|---------|------|------|----------|
| 1 | Telescope v2 | Rust+wgpu | telescope-rs/ 확장 | ~7,000줄 (커널 3K + 조합 4K) |
| 2 | Discovery Verifier | Rust | tools/discovery-verifier/ | ~2,000줄 |
| 3 | Domain Encoder | Rust+TOML | tools/domain-encoder/ | ~1,500줄 + 33 TOML |
| 4 | Telescope History | Rust | tools/telescope-history/ | ~1,000줄 |
| 5 | Materials DB | JSON→SQLite | docs/materials-db.json | 데이터 수집 |
| 6 | OUROBOROS v26 | Python 확장 | infinite_evolution.py | ~1,800줄 신규 |
| 7 | Cross-Domain Map | JSON | docs/cross-domain-map.json | 데이터 수집 |
| 8 | Calibration Set | JSON | tools/telescope-calibration/ | ~500줄 |
| 9 | Prediction Registry | JSON+Rust | tools/prediction-registry/ | ~800줄 |
| 10 | Dashboard | Rust+HTML | tools/discovery-dashboard/ | ~1,200줄 |
| 11 | CLI Orchestrator | Rust | tools/discovery-engine/ | ~1,500줄 |

**총: ~17,300줄 Rust + ~1,800줄 Python + 데이터 파일**

---

## 11. 의존성

```
Rust:
  wgpu 23.x          — Metal GPU compute
  rayon 1.10          — CPU 병렬
  pyo3 0.28           — Python 바인딩
  numpy 0.28          — numpy 연동
  serde + serde_json  — JSON 직렬화
  blake3              — 해싱 (캐시 키)
  rusqlite (optional) — SQLite (Materials DB 성장 시)
  tiny_http (optional)— Dashboard HTTP

Python:
  telescope_rs        — Rust 렌즈 바인딩
  numpy               — 데이터 처리
  (기존 OUROBOROS 의존성)
```

---

## 12. 제약 및 미래 과제

**현재 제약**:
- M4 단일 머신 (분산 미지원)
- 렌즈 80종 전부 구현은 점진적 (우선순위별)
- 외부 DB (Materials Project, AFLOW) 연동은 미래

**미래 확장**:
- WASM 렌즈 플러그인 (동적 로드, 재컴파일 불필요)
- 분산 실행 (여러 머신)
- 외부 DB 자동 풀링
- S급 발견 → arXiv 논문 초안 자동 생성
