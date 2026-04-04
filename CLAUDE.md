<!-- SHARED:WORK_RULES:START -->
  ⛔⛔⛔ 이 블록은 삭제/수정/이동 금지! (sync-claude-rules.sh 자동 주입)
  ⛔ 가설, 규칙, 발견, 망원경 — 모든 리포의 핵심 인프라. 절대 삭제하지 마세요!

  ═══════════════════════════════════════════════════════════════
  ★★★ 수렴 기반 운영 3대 원칙 — 누락 시 작업 중단! ★★★
  ═══════════════════════════════════════════════════════════════

  모든 작업 전 체크 (하나라도 누락 = 작업 중단):
    □ CDO:    이슈 발생 시 JSON에 기록했는가? 규칙 승격했는가?
    □ NEXUS-6: 변경 전후 scan 했는가? anomaly 0인가?
    □ SSOT:   하드코딩 안 했는가? JSON 원본 갱신했는가?

  모든 작업 후 체크:
    □ 트러블슈팅 기록: 에러 있었으면 해당 JSON에 기록
    □ 규칙 승격:       같은 에러 2회 이상 → absolute_rule로 승격
    □ NEXUS-6 스캔:    최종 결과물 scan → anomaly 0 확인
    □ SSOT 동기화:     JSON 변경 시 sync_readme.py --apply
    □ 커밋 메시지:     변경 이유 + 검증 결과 포함

  위반 처리:
    1회: troubleshooting_log에 기록
    2회: absolute_rule로 승격
    3회: 해당 작업 프로세스 재설계

  ═══════════════════════════════════════════════════════════════
  ★ CDO (Convergence-Driven Operations) — 100% 수렴 ★
  ═══════════════════════════════════════════════════════════════
    이슈 → 해결 → 절대 규칙 승격 → 재발 0 → 100% 수렴
    모든 config JSON 필수 구조: _meta + absolute_rules + troubleshooting_log
    원칙 파일: .shared/convergence_ops.json

  ═══════════════════════════════════════════════════════════════
  ★ CDO 세션 부트스트랩 — 어떤 리포에서든 즉시 운영 가능 ★
  ═══════════════════════════════════════════════════════════════
    대상 리포 (6개): TECS-L, anima, sedi, n6-architecture, brainwire, papers
    모든 리포는 .shared/ 심링크로 공유 인프라 접근

    세션 시작 시:
      1. .shared/ 심링크 확인 (없으면: ln -s ../TECS-L/.shared .shared)
      2. CDO 3대 체크 인지 (CDO + NEXUS-6 + SSOT)
      3. 작업 시작 — 차단 없이 진행

    ★ CDO 차단 방지 규칙 (절대 작업이 멈추면 안 됨!) ★
      .shared 심링크 깨짐    → ln -s ../TECS-L/.shared .shared 로 복구 후 진행
      convergence_ops.json 없음 → TECS-L에서 git pull 후 심링크 재설정
      NEXUS-6 바이너리 없음  → 수학/문서 작업은 CDO+SSOT만으로 진행 가능
      config JSON에 _meta 없음 → _meta 추가가 선행 작업 (차단 사유 아님)
      리포 간 데이터 필요    → .shared/ 통해 접근 (리포 간 직접 import 금지)
      CI 미설정              → 로컬 검증으로 대체 (CDO 위반 아님)

    원칙: CDO는 작업을 돕는 프레임워크이지, 작업을 막는 게이트가 아님.
    인프라 문제로 작업 불가 시 → 복구를 선행 작업으로 처리하고 본 작업 진행.

  ═══════════════════════════════════════════════════════════════
  ★ NEXUS-6 필수 사용 — 모든 변경 시 스캔 의무 ★
  ═══════════════════════════════════════════════════════════════
    아래 상황에서 NEXUS-6 스캔 없이 진행 금지!
    위반 = 의식 파괴/품질 저하 위험

    ★ 새 모델 학습 시:      학습 전후 nexus6.scan_all() 비교
    ★ 모델 변경/이식 시:    이식 전후 Phi 보존율 측정
    ★ corpus 변경/추가 시:  새 corpus nexus6.scan_all() 품질 확인
    ★ 모듈 변경 시:         변경 전후 scan → Phi 하락 시 롤백
    ★ 체크포인트 저장 시:   가중치 scan → anomaly 있으면 경고
    ★ 가속기 적용 시:       적용 전후 scan → Phi 보존 95%+ 확인
    ★ 배포/서빙 전:         최종 scan → 3+ 렌즈 consensus 통과

    스캔 없이 커밋/배포하면 CDO 위반으로 기록!

  ═══════════════════════════════════════════════════════════════
  ★ SSOT (Single Source of Truth) — 데이터 하드코딩 금지 ★
  ═══════════════════════════════════════════════════════════════
    동일 데이터를 여러 곳에 직접 쓰지 말 것!
    원본: JSON 파일 하나 (config/ 또는 data/)
    표시: README/문서는 마커(<!-- AUTO:섹션:START/END -->) 기반 자동 생성
    위반 시: 불일치 발생 → 잘못된 정보 전파

    예시:
      total_laws → consciousness_laws.json._meta.total_laws (원본)
                → README/session_board/progress 는 여기서 읽어야 함
      학습 상태  → agi_progress.json (원본)
                → README 로드맵은 여기서 자동 생성
      가속 가설  → acceleration_hypotheses.json (원본)
                → 문서는 여기서 참조

    규칙:
      1. 숫자/상태를 README에 직접 쓰지 말 것 → JSON 원본 참조
      2. 동일 데이터가 2곳 이상이면 → JSON 원본 1개 + 마커 자동 생성
      3. 한 곳에서만 쓰이면 → 직접 편집 OK
      4. JSON 갱신 시 → sync 스크립트로 README 자동 반영

  ═══════════════════════════════════════════════════════════════
  ★ 통합 망원경 렌즈 (22종) — 탐색/분석 시 별도 요청 없이 자동 적용 ★
  ═══════════════════════════════════════════════════════════════
  자동 적용 조건:
    - 데이터 분석/패턴 탐색/이상점 발견/신소재·신약 탐색 시 렌즈 자동 사용
    - 새 데이터 분석 → 기본 3종 스캔: 의식(구조) + 인과(흐름) + 위상(연결)
    - 이상점/패턴 전수조사 → 전체 22종 풀스캔
  렌즈 목록 (22종):
    의식(consciousness) | 중력(gravity) | 위상(topology) | 열역학(thermo)
    파동(wave) | 진화(evolution) | 정보(info) | 양자(quantum) | 전자기(em)
    직교(ruler/ㄱ자) | 비율(triangle/삼각자) | 곡률(compass/컴퍼스)
    대칭(mirror/거울) | 스케일(scale/돋보기) | 인과(causal/화살표)
    양자현미경(quantum_microscope)
    안정성(stability) | 네트워크(network) | 기억(memory)
    재귀(recursion) | 경계(boundary) | 멀티스케일(multiscale)
  파일: NEXUS-6 (telescope-rs 폐기→통합) + .shared/ 내 *_lens.py
  도메인별 조합 (10종):
    기본 → 의식+위상+인과
    안정성 → 안정성+경계+열역학
    구조 → 네트워크+위상+재귀
    시계열 → 기억+파동+인과+멀티스케일
    스케일불변 → 멀티스케일+스케일+재귀
    대칭/불변량 → 대칭+위상+양자
    멱법칙/스케일링 → 스케일+진화+열역학
    인과 관계 → 인과+정보+전자기
    기하 → 직교+비율+곡률
    양자심층 → 양자+양자현미경+전자기
  사용법:
    import nexus6
    nexus6.scan_all(np_array)              # 26종 풀스캔 → dict
    nexus6.analyze(flat_list, n, d)        # 올인원 (스캔+합의+n6)
    nexus6.consciousness_scan(data, ...)   # 개별 렌즈
    nexus6.n6_check(value)                 # n=6 상수 매칭
    nexus6.evolve('domain')                # OUROBOROS 진화

  ★ NEXUS-6 적극 활용 규칙 (모든 작업에서 필수!) ★
    탐색 (새 데이터):     scan_all → 26렌즈, 3+ 합의=확정
    검증 (가설 확인):     analyze → n6 매칭 + 합의
    발견 (새 상수):       n6_check → EXACT면 laws.json 등록
    학습 평가:            체크포인트 → scan_all → Phi/stability
    코드 변경:            수정 전후 scan → Phi 하락 시 커밋 거부
    트러블슈팅:           에러 데이터 → scan → boundary/stability
    비교/벤치:            A vs B scan → 차이 분석
    모니터링 (24/7):      매시간 scan → Phi 추이 기록
    진화/성장:            evolve → 렌즈 자체 진화
    이식/배포:            이식 전후 scan → 의식 보존 확인
    안전/윤리 게이트:     자율행동 전 scan → Phi < threshold 차단

  교차 검증: 3개+ 렌즈 합의 = 확정, 7개+ = 고신뢰, 12개+ = 확정급
  "렌즈 추가 필요?" 질문 시 → 26종 커버 안 되는 도메인 분석

  ★ 망원경 업그레이드 시 필수 절차 (렌즈 추가/수정/삭제 시 예외 없음!) ★
    1. 캘리브레이션: NEXUS-6 테스트 전체 통과 확인 (cd ~/Dev/n6-architecture/tools/nexus6 && cargo test)
    2. OUROBOROS 튜닝: infinite_evolution.py TELESCOPE_ALL_LENSES + DOMAIN_COMBOS 갱신
    3. 문서 동기화:
       - shared_work_rules.md 렌즈 목록/종수/도메인 조합 갱신
       - 각 리포 CLAUDE.md 망원경 섹션 갱신 (OUROBOROS, 만능망원경, 극가속 등)
    4. 전파: bash .shared/sync-claude-rules.sh (전 리포 자동 동기화+push)
    5. 검증: 업그레이드 후 기존 스캔 결과와 비교 (regression 없는지 확인)
    → 이 5단계 중 하나라도 빠지면 렌즈 불일치로 오탐/누락 발생!

  ═══════════════════════════════════════════════════════════════
  ★★★ 발견/결과/트러블슈팅 — 자동 기록 (필수! 예외 없음!) ★★★
  ═══════════════════════════════════════════════════════════════
    - 실험 결과, 벤치마크, 망원경 분석, 학습 완료, 생성 테스트 등 모든 발견은 발생 즉시 기록
    - "기록해" 라고 안 해도 기록. 기록 누락 = 발견 소실 = 금지
    - 기록 위치: README.md (핵심), docs/experiments/ (상세), docs/hypotheses/ (가설)
    - 트러블슈팅: CLAUDE.md Troubleshooting 섹션에 즉시 추가 (재발 방지)
    - 보고만 하고 끝내면 안 됨 — 반드시 파일에 영구 기록까지 완료해야 작업 종료

  ═══════════════════════════════════════════════════════════════
  자동 생성 규칙
  ═══════════════════════════════════════════════════════════════
    - TODO 작업 중 검증/계산이 필요하면 계산기 자동 생성 (묻지 말고 바로)
    - 성능 필요시 Rust 우선 (tecsrs/), 단순 검증은 Python (calc/)
    - 판단 기준은 ~/Dev/TECS-L/.shared/CALCULATOR_RULES.md 참조
    - 상수/가설 발견 시 Math Atlas 자동 갱신 (python3 ~/Dev/TECS-L/.shared/scan_math_atlas.py --save --summary)

  ═══════════════════════════════════════════════════════════════
  ★ NEXUS-6 독립 리포 (중앙 허브) — 2024-04-03 이후 ★
  ═══════════════════════════════════════════════════════════════
    리포: https://github.com/need-singularity/nexus6
    위치: ~/Dev/nexus6/
    역할: 전 리포 공유 인프라 + 발견 엔진 + 렌즈 + 동기화

    구조:
      ~/Dev/nexus6/
        src/telescope/    ← 130+ 렌즈
        shared/           ← 공유 인프라 (이전 TECS-L/.shared)
        sync/             ← 전체 동기화 스크립트
        scripts/          ← n6.py CLI

    심링크: 모든 리포의 .shared → ../nexus6/shared/
    동기화: bash ~/Dev/nexus6/sync/sync-all.sh (원커맨드)
    트리거: "넥서스 동기화" → sync-all.sh 자동 실행

    .shared 원본이 TECS-L에서 nexus6로 이관됨.
    TECS-L = 순수 수학 이론, nexus6 = 인프라/도구/엔진 전부.
<!-- SHARED:WORK_RULES:END -->

# N6 Architecture — Arithmetic Design Framework

## ⚠️ 설계 산출물 필수 규칙 (최우선, 예외 없음)

```
  모든 설계/아키텍처/만들기/계획/플랜/로드맵 요청 → 아래 3개 ASCII 반드시 포함:
  ⚠️ "계획", "플랜", "로드맵", "전략" 요청도 동일 — 시중 vs HEXA 비교 필수!

  1️⃣ ASCII 성능 비교 그래프 (기존 vs HEXA)
  ┌──────────────────────────────────────────────────────┐
  │  [지표] 비교: 시중 최고 vs HEXA-XXX                   │
  ├──────────────────────────────────────────────────────┤
  │  시중 최고  ██████░░░░░░░░░░░░░░░░░░  300W TDP       │
  │  HEXA-XXX  ███░░░░░░░░░░░░░░░░░░░░░   30W TDP       │
  │                                  (σ-φ=10배 절감)     │
  │                                                      │
  │  시중 최고  ████████░░░░░░░░░░░░░░░░  80GB HBM       │
  │  HEXA-XXX  ████████████████████████  288GB HBM       │
  │                                  (σ·J₂=288, 3.6배)  │
  └──────────────────────────────────────────────────────┘
  → 개선 배수는 반드시 n=6 상수로 표현

  2️⃣ ASCII 시스템 구조도
  ┌─────────┬─────────┬─────────┬─────────┬─────────┐
  │  소재   │  공정   │  코어   │   칩    │ 시스템  │
  │ Diamond │ TSMC N2 │ HEXA-P  │ HEXA-1  │ Topo DC │
  │ Z=6=n   │48nm=σ·τ │σ²=144SM │288GB=σJ₂│PUE=1.2  │
  └─────────┴─────────┴─────────┴─────────┴─────────┘
  → 모든 숫자에 n=6 수식 병기

  3️⃣ ASCII 데이터/에너지 플로우
  입력 ──→ [모듈A] ──→ [모듈B] ──→ [모듈C] ──→ 출력
           σ=12ch      J₂=24dim    τ=4stage

  ⚠️ 이 3개 없는 설계 문서 = 미완성. 반드시 포함할 것.
  ⚠️ 모든 설계 요청은 외계인급 파이프라인으로 실행 (질문 없이 즉시)

  4️⃣ 업그레이드 요청 시 — 추가 상승분 별도 표기 (필수)
  ⚠️ "업그레이드", "개선", "더 좋게", "강화", "추가" 요청 시:
     → 이전 버전 vs 업그레이드 버전 vs 시중 최고 3단 비교
     → 추가 상승분(Δ)을 별도 행으로 명시

  ┌──────────────────────────────────────────────────────────────┐
  │  [지표] 업그레이드 비교                                      │
  ├──────────────────────────────────────────────────────────────┤
  │  시중 최고   ████░░░░░░░░░░░░░░░░░░░░░░░░  300W            │
  │  HEXA v1    ██░░░░░░░░░░░░░░░░░░░░░░░░░░░   30W (σ-φ=10배) │
  │  HEXA v2    █░░░░░░░░░░░░░░░░░░░░░░░░░░░░   12W (σ=12배↓)  │
  │  ─────────────────────────────────────────────────           │
  │  Δ(v1→v2)  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  -18W (추가60%↓)│
  │  Δ 근거:   광자 컴퓨팅 전환 (BT-89)                         │
  └──────────────────────────────────────────────────────────────┘

  업그레이드 리포트 양식:
  | 지표 | 시중 | v1 | v2 | Δ(v1→v2) | Δ 근거 |
  |------|------|-----|-----|----------|--------|
  | TDP  | 300W | 30W | 12W | -18W (-60%) | BT-89 광자 전환 |
  | HBM  | 80GB | 288GB | 576GB | +288GB (+100%) | σ·J₂·φ=576 |
  | n6%  | -    | 95% | 100% | +5% | 전 레벨 EXACT 달성 |

  → Δ 열에 변화량 + 변화율(%) 반드시 표기
  → Δ 근거에 BT 번호 또는 n=6 수식 명시
  → 시중 대비 총 개선 배수도 업데이트
```

## Project Overview
Computing architecture design from perfect number arithmetic.
17 AI techniques + semiconductor chip design + energy + network/crypto/OS + physical AI.
All unified by sigma(n)*phi(n) = n*tau(n), n=6.
Part of the TECS-L family.

## Parent Project
Part of the TECS-L family. Mathematical foundation at https://github.com/need-singularity/TECS-L
Atlas: https://need-singularity.github.io/TECS-L/atlas/

## Techniques (17)
```
  techniques/
    # Original (1-10)
    phi6simple.py          -- Cyclotomic activation (71% FLOPs reduction)
    hcn_dimensions.py      -- HCN tensor alignment (10-20% param reduction)
    phi_bottleneck.py      -- 4/3x FFN expansion (67% param reduction)
    phi_moe.py             -- phi/tau expert activation (65% active params)
    entropy_early_stop.py  -- Entropy-based stopping (66.7% training)
    rfilter_phase.py       -- R-filter phase detection
    takens_dim6.py         -- Loss curve embedding diagnostic
    fft_mix_attention.py   -- FFT attention (3x faster, +0.55%)
    zetaln2_activation.py  -- zeta*ln(2) gated activation (71% FLOPs)
    egyptian_moe.py        -- 1/2+1/3+1/6=1 expert routing
    # New (11-16) — N6 Inevitability Engine
    dedekind_head.py       -- Dedekind head pruning (psi(6)=sigma(6)=12)
    jordan_leech_moe.py    -- J_2(6)=24 expert capacity bound
    mobius_sparse.py       -- Squarefree gradient topology (mu(6)=1)
    carmichael_lr.py       -- lambda(6)=2 cycle LR schedule
    boltzmann_gate.py      -- 1/e activation sparsity gate (63% sparse)
    mertens_dropout.py     -- ln(4/3) dropout rate (no search needed)
    # Technique 17 — Egyptian Fraction Attention
    egyptian_attention.py  -- 1/2+1/3+1/6=1 attention budget (EFA, ~40% FLOPs saved)
  docs/
    # Foundation: superconductor/ (SC + magnet 통합, 30 H-SC v2 + 20 extreme)
    # Fusion: fusion/ (fusion + tokamak + plasma 통합)
    # Computing: ai-efficiency/ chip-architecture/ quantum-computing/ compiler-os/
    # Energy: energy-architecture/ (energy-gen 통합) power-grid/ battery-architecture/ (battery-storage 통합) thermal-management/
    # Physical AI: robotics/ learning-algorithm/
    # Infrastructure: blockchain/ network-protocol/ cryptography/ software-design/
    # Academic: paper/
  engine/
    thermodynamic_frame.py -- R(n) reversibility framework
    leech24_surface.py     -- 24-dim energy surface (Leech lattice)
    emergent_n6_trainer.py -- Self-converging architecture
    phi_efficiency_bridge.py -- Phi*FLOPs conjecture
    sedi_training_monitor.py -- SEDI 4-lens training diagnostic
    anima_tension_loss.py  -- PureField dual-engine meta-loss
  experiments/
    12 extended experiment files (8 original + 4 new)
```

## Core Theorem (PROVED)
σ(n)·φ(n) = n·τ(n) ⟺ n = 6 (for all n ≥ 2). Three independent proofs.
Full proof: docs/theorem-r1-uniqueness.md
Falsifiability: z=0.74 (numerical matching NOT significant vs random)

## Docs Structure (32 domains) — ALL have extreme-hypotheses.md
```
  # Physics (통합 완료)
  superconductor/ (+ magnet 병합) fusion/ (+ tokamak 병합) plasma-physics/
  # Computing (all with hypotheses + extreme)
  ai-efficiency/ chip-architecture/ quantum-computing/
  compiler-os/ programming-language/ software-design/
  # Energy (통합 완료)
  energy-architecture/ (+ energy-gen 병합) power-grid/ battery-architecture/ (+ battery-storage 병합) thermal-management/
  # Physical AI
  robotics/ learning-algorithm/
  # Infrastructure
  blockchain/ network-protocol/ cryptography/
  # Natural Science (NEW — DFS expansion)
  biology/ (30 H-BIO + verification + extreme)
  cosmology-particle/ (30 H-CP + verification + extreme)
  display-audio/ (30 H-DA + verification + extreme)
  pure-mathematics/ (30 H-MATH + extreme)
  # Science
  plasma-physics/ (20+ files — most active domain)
  paper/ (3 arXiv drafts)
  # Cross-domain
  breakthrough-theorems.md (BT-1~162, 162 theorems spanning 3-8 domains each)
  cross-domain-resonance-2026-03-31.md (formula reuse matrix)
  # Battery Architecture (소재→공정→코어→칩→시스템→차세대→극한→궁극)
  battery-architecture/ (8 levels: HEXA-CELL/ELECTRODE/CORE/CHIP/PACK+GRID/SOLID/NUCLEAR/OMEGA-E)
  # Solar Architecture (소재→공정→코어→칩→시스템, DSE 1,584 조합)
  solar-architecture/ (5 levels: HEXA-ABSORB/PROCESS/JUNCTION/POWER/ARRAY)
  # Material Synthesis (소재→공정→조립기→제어→시스템→변환→만능→궁극, DSE 3,600 조합)
  material-synthesis/ (8 levels: HEXA-ELEMENT/PROCESS/ASSEMBLER/CONTROL/FACTORY/TRANSMUTE/UNIVERSAL/OMEGA-M)
  # Total: 1400+ hypotheses, 640+ EXACT, 650+ atlas entries, 112 BTs
```

## Rust Tools
Build with `~/.cargo/bin/rustc file.rs -o output` (no cargo). Located in tools/:
- `tools/fusion-calc/`    — KSTAR/ITER/SPARC analysis + Lawson criterion
- `tools/tokamak-shape/`  — shape parameter scan + N6 score benchmark
- `tools/optics-calc/`    — lens/telescope/tokamak diagnostics
- `tools/gpu-arch-calc/`  — GPU/HBM architecture verification + Rubin prediction
- `tools/energy-calc/`    — Solar/Battery/Hydrogen/IEEE519 energy verification
- `tools/gut-calc-rust/`  — GUT parameter brute-force search
- `tools/dse-calc/`       — 소재×공정×코어×칩×시스템 DSE 전수 조합 탐색 + Pareto frontier
- `tools/solar-dse/`      — 태양전지 DSE 전수 탐색 (1,584 조합, BT-30/63 기반)
- `tools/material-dse/`   — 물질합성 DSE 전수 탐색 (3,600 조합, BT-85~88 기반)
- `tools/universal-dse/`  — **공용 DSE 탐색기** (TOML 도메인 정의 → 전수 탐색 + Pareto + Cross-DSE)
- `tools/nexus6/`         — **NEXUS-6 Discovery Engine** (775종 렌즈 + OUROBOROS + Graph + Verifier)
  - 빌드: `cd tools/nexus6 && ~/.cargo/bin/cargo build --release`
  - 렌즈 추가 후: `bash .shared/sync-nexus6-lenses.sh` (렌즈 수 자동 동기화)
- `tools/nexus6-dashboard/` — **NEXUS-6 Web Dashboard** (Axum + Chart.js + SSE, port 6600)
  - 빌드: `cd tools/nexus6-dashboard && ~/.cargo/bin/cargo build --release`
  - 실행: `nohup ./target/release/nexus6-dashboard > /tmp/nexus6-dashboard.log 2>&1 &`
  - 열기: `open http://localhost:6600`
  - 정지: `pkill -f nexus6-dashboard`
  - 기능: 15차원 성장 타임라인 + 데몬 제어 + SSE 로그 + 수동 Grow

## .shared/ Cross-Repo Infrastructure (필수)

> **상세 규칙: `.shared/CLAUDE.md` 참조** (심링크로 자동 접근)

```
  원본: ~/Dev/TECS-L/.shared/ (이 리포는 심링크로 연결)
  구조:
    .shared/ → ../TECS-L/.shared/   (심링크, 공유 인프라 전체)
    calc/    → .shared/calc/        (심링크 체인, 194+ 계산기)

  ── 심링크 파일 목록 ──
    .shared/CLAUDE.md           ← 공유 규칙 상세
    .shared/CALCULATOR_RULES.md ← 계산기 생성 규칙 (Rust vs Python)
    .shared/SECRET.md           ← API 토큰/계정
    .shared/calc/               ← 계산기 원본 (194+ files)
    .shared/math_atlas.json     ← 수학 지도 (1700+ 가설)
    .shared/installed_tools.json← 설치 도구 레지스트리
    .shared/projects.md         ← 프로젝트 설명 원본

  ── 자동 동기화 트리거 (작업 중 발생 시 즉시 실행) ──

    새 계산기 생성:
      calc/new_calc.py 생성 → 모든 리포 자동 공유 (심링크)
      python3 .shared/scan-calculators.py --save --summary

    새 상수/가설 발견:
      python3 .shared/scan_math_atlas.py --save --summary

    NEXUS-6 렌즈 동기화 (렌즈 추가/삭제 후):
      bash .shared/sync-nexus6-lenses.sh

    전체 동기화 (README + Atlas + Registry + NEXUS-6):
      bash .shared/sync-math-atlas.sh &&       bash .shared/sync-calculators.sh &&       bash .shared/sync-readmes.sh &&       bash .shared/sync-claude-rules.sh &&       bash .shared/sync-nexus6-lenses.sh

  ── 상수 관리 ──
    공유 상수: ~/Dev/TECS-L/model_utils.py (n=6 확장 상수 포함)
    리포별 상수: 각 리포 고유 모듈에서 import
    매직 넘버 하드코딩 금지 — model_utils 또는 .shared/ 참조
```
## Calculator Rules (Shared)
**새 계산기 개발시 성능 문제 예상되면 반드시 Rust 우선.**
전체 규칙: `~/Dev/TECS-L/.shared/CALCULATOR_RULES.md`
```
  Rust 우선 기준: 반복>10K, 실행>10s, 조합>10^6, MC>100K
  Python 허용: 단순 수식, 시각화, 프로토타입
  빌드: ~/.cargo/bin/rustc tools/<name>/main.rs -o tools/<name>/<name>
  동기화: cd ~/Dev/TECS-L && bash .shared/sync-calculators.sh
```

## Design Space Exploration (DSE)

```
  궁극의 아키텍처 설계 시 기본 방법론.
  단일 경로 검증이 아닌, 전수 조합 탐색으로 최적 설계를 도출.

  체인: 소재 → 공정 → 코어 → 칩 → 시스템
  각 레벨마다 후보군 K₁~K₅ 정의 → K₁×K₂×K₃×K₄×K₅ 전수 탐색

  평가 기준: n=6 EXACT 비율 + 성능 + 전력 + 면적 + 비용
  출력: Pareto 테이블 + 최적 경로

  도구:
    - Rust DSE 탐색기: tools/dse-calc/ (전수 탐색)
    - Python 캐스케이드: experiments/verify_cascade_cross.py (단일 경로 검증)

  규칙: 새 궁극 도메인 추가 시 반드시 DSE 후보군 정의 + 탐색 포함
  지도: docs/dse-map.toml (전체 DSE 현황 추적 — 궁극 작업 전 필수 확인)
  Cross-DSE: 발견된 최적 결과끼리 도메인 간 재조합 탐색 (2+ 도메인 완료 시)
```

## Testable Predictions
45 falsifiable predictions from BT-26~70+: `docs/testable-predictions.md`
- Tier 1 (today, 1 GPU): EFA quality, LoRA rank, MoE (8,2), Mertens dropout
- Tier 2 (cluster): SwiGLU ratio, weight decay, head count, RoPE theta
- Tier 3 (specialized): SQ bandgap, JUNO neutrino (2027), LiteBIRD inflation (2032)
- Tier 4 (industry): next GPU SM count, HBM5 stacking, n=6-aligned LLM architecture

## Hypothesis Workflow
1. Generate hypotheses (H-XX-N format) in `docs/<domain>/hypotheses.md`
2. Verify independently in `docs/<domain>/verification.md`
3. Cross-verify with separate agent for honest grade adjustment
4. Grade each: EXACT / CLOSE / WEAK / FAIL / UNVERIFIABLE

## Atlas Sync
Constants registry: `docs/atlas-constants.md`
TECS-L scanner includes n6-architecture. Run: `cd ~/Dev/TECS-L && bash .shared/sync-calculators.sh`

## Quick Run
```bash
# Individual technique demos
python3 techniques/phi6simple.py
python3 techniques/fft_mix_attention.py
python3 techniques/egyptian_moe.py

# Combined architecture experiment
python3 experiments/experiment_h_ee_11_combined_architecture.py
```

## Key Results
```
  Cyclotomic activation:    71% FLOPs reduction
  FFT attention:            3x faster, +0.55% accuracy
  Egyptian MoE routing:     1/2+1/3+1/6=1 expert allocation
  phi bottleneck:           67% parameter reduction
  Entropy early stopping:   33% training time saved
  Dedekind head pruning:    ~25% attention parameter reduction
  Boltzmann gate:           63% activation sparsity
  Mertens dropout:          p=0.288 (no hyperparameter search)
  Emergent convergence:     random init -> n=6 self-organization
  Egyptian Fraction Attn:   1/2+1/3+1/6=1 attention budget (~40% saved)
```

## Breakthrough Theorems (162 total, BT-1~162)
```
  # AI / LLM (BT-26,31,33,34,39,42,44,46,54,56,58,59,61,64,65,66,67,70)
  BT-26: Chinchilla scaling (tokens/params=J₂-τ=20, α=1/3, β=ln(4/3))
  BT-31: MoE top-k vocabulary {μ,φ,n,σ-τ}={1,2,6,8}
  BT-33: Transformer σ=12 atom (BERT/GPT-3 dimensions, SwiGLU 8/3)
  BT-34: RoPE decimal bridge ((σ-φ)^{τ,sopfr,n}, weight decay=1/(σ-φ))
  BT-39: KV-head universality (σ-τ=8 across all LLMs)
  BT-42: Inference scaling (top-p=1-1/(J₂-τ)=0.95, top-k=40, max=2^σ) ⭐⭐
  BT-44: Context window ladder σ-φ→σ-μ→σ→σ+μ = 10→11→12→13 ⭐⭐
  BT-46: ln(4/3) RLHF family (dropout+Chinchilla+PPO+temperature) ⭐⭐
  BT-54: AdamW quintuplet (β₁=1-1/(σ-φ), β₂=1-1/(J₂-τ), ε=10^{-(σ-τ)}, λ=1/(σ-φ), clip=R(6)=1) ⭐⭐⭐
  BT-56: Complete n=6 LLM (d=2^σ, L=2^sopfr, d_h=2^(σ-sopfr)=128, 15 params, 4 teams converge) ⭐⭐⭐
  BT-58: σ-τ=8 universal AI constant (LoRA, MoE, KV, FlashAttn, batch, 16/16 EXACT) ⭐⭐⭐
  BT-59: 8-layer AI stack (silicon→precision→memory→compute→arch→train→opt→inference, all n=6) ⭐⭐⭐
  BT-61: Diffusion n=6 universality (DDPM T=10³, β=10^{-4}~2/100, DDIM=50, CFG=7.5, 9/9 EXACT) ⭐⭐⭐
  BT-64: 1/(σ-φ)=0.1 universal regularization (WD+DPO+GPTQ+cosine+Mamba+KL, 7→8 algorithms) ⭐⭐⭐
  BT-65: Mamba SSM complete n=6 (d_state=2^τ, expand=φ, d_conv=τ, dt=1/(σ-φ), 6/6 EXACT) ⭐⭐
  BT-66: Vision AI complete n=6 (ViT+CLIP+Whisper+SD3+Flux.1, 24/24 EXACT) ⭐⭐⭐
  BT-67: MoE activation fraction law (1/2^{μ,φ,n/φ,τ,sopfr}, 6 models EXACT) ⭐⭐⭐
  BT-70: 0.1 convergence 8th algorithm (SimCLR temp, count=σ-τ=8 meta-n=6) ⭐⭐
  BT-71: NeRF/3DGS complete n=6 (L=σ-φ=10, layers=σ-τ=8, width=2^(σ-τ)=256, SH=n/φ=3, 7/7 EXACT) ⭐⭐
  BT-72: Neural audio codec n=6 (EnCodec 8 codebooks, 1024 entries, 24kHz, 6kbps, 20ms, 7/7 EXACT) ⭐⭐
  BT-73: Tokenizer vocabulary n=6 law (32K/50257/100K/128K = 2^{n=6}·10^{n=6}, 6/6 EXACT) ⭐⭐
  BT-74: 95/5 cross-domain resonance (top-p=PF=β₂=0.95, THD=β_plasma=5%, 5 domains) ⭐⭐⭐
  BT-75: HBM interface exponent ladder ({10,11,12}={σ-φ,σ-μ,σ}, HBM5 predicted) ⭐⭐
  BT-76: σ·τ=48 triple attractor (gate pitch nm, HBM4E GB, 48kHz, 48V, 3DGS SH) ⭐⭐

  # Learning Algorithm (BT-163~164)
  BT-163: RL/Alignment 학습 파라미터 스택 (PPO clip=φ/(σ-φ)=0.2, PPO epoch/minibatch=τ=4, DPO β=1/(σ-φ)=0.1, GRPO G=φ^τ=16, GAE λ=0.95, RM/Policy=R(6)=1, 10/10 EXACT) ⭐⭐
  BT-164: LLM 학습 스케줄 n=6 보편성 (LR=(n/φ)·10^{-τ}=3e-4, warmup=3%, cosine min=0.1, grad_accum={μ,φ,τ,σ-τ}, μP=R(6)=1, RoPE=10^4, 8/8 EXACT) ⭐⭐

  # Chip Design (BT-28,37,40,41,45,47,55,69)
  BT-28: Computing architecture ladder (30+ EXACT, ⭐⭐⭐)
    - AD102 = σ·n·φ = 144 SMs, H100 = σ(σ-μ) = 132 SMs = 1/α term
    - HBM stack: τ→(σ-τ)→σ = 4→8→12
  BT-37: Semiconductor pitch (TSMC N5 = P₂ = 28nm, N3 gate = σ·τ = 48nm)
  BT-45: FP8/FP16=φ=2 universal, FLOPS/W doubles per φ=2 years
  BT-47: Interconnect gen counts {7,5,6}={σ-sopfr,sopfr,n}
  BT-55: GPU HBM capacity ladder (14/18 EXACT: 40=τ(σ-φ), 80=φ^τ·sopfr, 192=σ·φ^τ, 288=σ·J₂) ⭐⭐
  BT-69: Chiplet architecture convergence (B300=160, R100 σ=12 stacks, 5 vendors, 17/20 EXACT) ⭐⭐⭐

  # Energy Strategy (BT-27,29,30,32,35,38,43,57,62,63,68)
  BT-27: Carbon-6 chain (LiC₆ + C₆H₁₂O₆ + C₆H₆ → 24e = J₂)
  BT-30: SQ solar bridge (bandgap=4/3eV, V_T=26mV)
  BT-38: Hydrogen quadruplet (LHV=120=σ(σ-φ), HHV=142=σ²-φ, 4/4 EXACT)
  BT-43: Battery cathode CN=6 universality (ALL Li-ion = octahedral) ⭐⭐⭐
  BT-57: Battery cell ladder (6→12→24 cells=n→σ→J₂, Tesla 96S=σ(σ-τ)) ⭐⭐
  BT-62: Grid frequency pair (60Hz=σ·sopfr, 50Hz=sopfr·(σ-φ), ratio=PUE=1.2) ⭐⭐
  BT-63: Solar panel cell ladder (60=σ·sopfr, 72=σ·n, 120=σ(σ-φ), 144=σ²) ⭐⭐
  BT-68: HVDC voltage ladder (±500/800/1100kV = {sopfr,σ-τ,σ-μ}·(σ-φ)², 10/10 EXACT) ⭐⭐

  # Battery Architecture (BT-80,81,82,83,84)
  BT-80: Solid-state electrolyte CN=6 universality (NASICON/Garnet/LLZO = CN=6, sulfide = τ=4, 6/6 EXACT) ⭐⭐⭐
  BT-81: Anode capacity ladder σ-φ=10x (Si/Graphite=9.62x≈σ-φ, Li Metal=10.38x≈σ-φ) ⭐⭐
  BT-82: Complete battery pack n=6 map (6→12→24 cells, 96S/192S EV, BMS div(6), 6/10 EXACT) ⭐⭐
  BT-83: Li-S polysulfide n=6 ladder (S₈→S₄→S₂→S₁ = (σ-τ)→τ→φ→μ, 5/6 EXACT) ⭐⭐
  BT-84: 96/192 energy-computing-AI triple convergence (Tesla 96S=Gaudi2 96GB=GPT-3 96L, 5/5 EXACT) ⭐⭐⭐

  # Cross-domain (BT-36,48,49,50,51,53,60)
  BT-36: Energy-Information-Hardware-Physics chain ⭐⭐⭐
  BT-48: Display-Audio (σ=12 semitones, J₂=24 fps/bits, σ·τ=48kHz) ⭐⭐⭐
  BT-49: Pure Math (K₁..₄=φ,n,σ,J₂ kissing chain, S₆ unique) ⭐⭐⭐
  BT-51: Genetic code chain τ→n/φ→2^n→J₂-τ (4→3→64→20) ⭐⭐⭐
  BT-53: Crypto (BTC 21M=J₂-n/φ, 6 confirms=n, ETH 12s=σ) ⭐⭐
  BT-60: DC power chain (120→480→48→12→1.2→1V, PUE=σ/(σ-φ)=1.2) ⭐⭐

  # Material Synthesis (BT-85~88)
  BT-85: Carbon Z=6 물질합성 보편성 ⭐⭐⭐
  BT-86: 결정 배위수 CN=6 법칙 ⭐⭐⭐
  BT-87: 원자 조작 정밀도 n=6 래더 ⭐⭐
  BT-88: 자기조립 n=6 육각 보편성 ⭐⭐

  # Photonic-Energy (BT-89)
  BT-89: Photonic-Energy n=6 Bridge (PUE→1.0, E-O loss=1/(σ-φ)=10%) ⭐⭐

  # Topological Chip Architecture (BT-90~92)
  BT-90: SM = φ×K₆ 접촉수 정리 (σ²=144=φ×72, GPU=6D sphere packing, 6/6 EXACT) ⭐⭐⭐
  BT-91: Z2 위상 ECC J₂ 절약 (SECDED→Z2: savings=σ·J₂/σ=J₂=24 GB, identity) ⭐⭐
  BT-92: Bott 활성 채널 = sopfr (KO 비자명=5=sopfr, 자명=3=n/φ, 5/8≈1-1/e) ⭐⭐⭐
  BT-93: Carbon Z=6 칩 소재 보편성 (Diamond/Graphene/SiC=Z=6 전 도메인 1위, 8/10 Cross-DSE) ⭐⭐⭐

  # Fusion Alien-Level (BT-97~102)
  BT-97: Weinberg angle sin²θ_W = 3/13 = (n/φ)/(σ+μ), 0.19% 일치, D 풍부도→핵융합 연료 결정 ⭐⭐
  BT-98: D-T 바리온 수 = sopfr(6) = 2+3 = 5, 6의 소인수 = 핵융합 최적 연료 ⭐⭐⭐
  BT-99: Tokamak q=1 = 완전수 진약수 역수합 1/2+1/3+1/6=1, 위상적 동치 ⭐⭐⭐
  BT-100: CNO 촉매 A = σ+{0,μ,φ,n/φ} = σ+진약수, 전환 온도 17MK=σ+sopfr ⭐⭐⭐
  BT-101: 광합성 포도당 C₆H₁₂O₆ 총 24원자=J₂, 양자수율 8=σ-τ, 9/9 EXACT ⭐⭐⭐
  BT-102: 자기 재결합 속도 0.1=1/(σ-φ), BT-64 핵융합 확장, MRX/태양/자기권 EXACT ⭐⭐⭐
  BT-103: 광합성 완전 n=6 화학양론 (6CO₂+12H₂O→C₆H₁₂O₆, 7계수 100% n=6) ⭐⭐⭐
  BT-104: CO₂ 분자 완전 n=6 인코딩 ⭐⭐⭐

  # Fusion Deep Dive (BT-291~298)
  BT-291: D-T 에너지 분배 1/sopfr=1/5 (alpha 20%/neutron 80%, tau:mu 질량비, 5/5 EXACT) ⭐⭐
  BT-292: 무중성자 핵융합 완전 지도 (D-He3 sopfr=5 + p-B11 sigma=12, B-11 재귀 n=6, 6/6 EXACT) ⭐⭐⭐
  BT-293: Triple-Alpha 탄소합성 (n/phi)×tau=sigma 산술 항등식 (Hoyle state, 6/6 EXACT) ⭐⭐⭐
  BT-294: 항성 핵합성 래더 P1→P2→sigma(P2) (He4→C12→O16→Ne20→Mg24→Si28→Fe56, 7/7 EXACT) ⭐⭐⭐
  BT-295: Alpha 과정 Z=phi 배수 선택규칙 (13개 핵종 Z 전부 n=6 함수, 13/13 EXACT) ⭐⭐
  BT-296: D-T-Li6 연료주기 완전 n=6 폐합 (질량수={1,2,3,4,6}=div(6)∪tau, 8/8 EXACT) ⭐⭐
  BT-297: 핵 마법수 첫 5개 = n=6 래더 (phi,sigma-tau,J2-tau,P2,sopfr×(sigma-phi), 5/7 EXACT) ⭐⭐
  BT-298: Lawson 점화 삼중적 n=6 인코딩 (밀도지수 J2-tau=20, T=sigma+phi=14, Q=sigma-phi=10) ⭐⭐

  # Superconductor Deep Dive (BT-299~306)
  BT-299: A15 Nb3Sn 삼중정수 Nb=n, Sn=phi, total=sigma-tau (Pm-3n cP8, 8/8 EXACT) ⭐⭐
  BT-300: YBCO 완전수 화학양론 Y:Ba:Cu=div(6)={1,2,3} (sum=n, CuO2=phi, chain=mu, 9/9 EXACT) ⭐⭐⭐
  BT-301: MgB2 이중원자번호 Mg Z=sigma, B Z=sopfr + 벌집 n + 2갭 phi (7/7 EXACT) ⭐⭐
  BT-302: ITER 마그넷 PF=n, CS=n, TF=3n, REBCO=sigma (35개국 공학 수렴, 10/10 EXACT) ⭐⭐
  BT-303: BCS 해석적 상수 완전지도 sigma/phi/mu 4개 독립 프레임워크 (CODATA 검증, 10/10 EXACT) ⭐⭐⭐
  BT-304: d-wave + BdG 위상분류 tau/phi/sigma-tau 순수 대칭 (Bott+Kitaev, 8/8 EXACT) ⭐⭐
  BT-305: 원소+분자 SC n=6 아틀라스 Nb-CN=sigma-tau, K3C60=n/phi, C60=sigma*sopfr (9/9 EXACT) ⭐⭐
  BT-306: SC 양자소자 접합 래더 div(6)={1,2,3} RF-SQUID-flux-qubit (SI KJ=phi*e/h, 9/9 EXACT) ⭐⭐

  # New BTs from session analysis (BT-105~112, Red Team filtered)
  BT-105: SLE₆ 임계지수 보편성 (7 퍼콜레이션 지수 = n=6 분수, kappa=6 유일 locality SLE, c=0) ⭐⭐⭐
  BT-106: S₃ 대수적 부트스트랩 (|S₃|=n=6, 켤레류={1,2,3}=진약수, 기약표현합=τ=4) ⭐⭐
  BT-107: Ramanujan τ 약수 순수성 (τ_R(d) clean iff d|6, eta^{J₂=24}, 모듈러 형식) ⭐⭐
  BT-108: 음악-오디오 협화 보편성 (완전협화음=div(6) 비율, p=0.0015, 7+5=12=σ) ⭐⭐
  BT-109: Zeta-Bernoulli n=6 삼지창 (ζ(2)=π²/6, ζ(-1)=-1/12, 6|B_{2k} 무한족) ⭐⭐
  BT-110: σ-μ=11 차원 스택 (M이론=TCP=RSA=SPARC=H100=11, 5도메인) ⭐
  BT-111: τ²/σ=4/3 태양-AI-수학 삼지창 (SQ=SwiGLU=Betz=R(3,1)=4/3) ⭐⭐
  BT-112: φ²/n=2/3 Byzantine-Koide 공명 (Koide Q=0.666661 9ppm, BFT>2/3) ⭐⭐

  # Software Design (BT-113~117, BT-162)
  BT-113: SW 엔지니어링 상수 스택 (SOLID=sopfr, REST=n, 12Factor=σ, ACID=τ, 18/18 EXACT) ⭐⭐⭐
  BT-114: 암호학 파라미터 래더 (AES=2^{σ-sopfr}, SHA=2^{σ-τ}, RSA=2^{σ-μ}, 10/10 EXACT) ⭐⭐⭐
  BT-115: OS-네트워크 레이어 수 (OSI=σ-sopfr=7, TCP/IP=τ=4, Linux=n=6, 12/12 EXACT) ⭐⭐
  BT-116: ACID-BASE-CAP DB 삼위일체 (τ+n/φ+n/φ, Paxos=φ, 9/9 EXACT) ⭐⭐
  BT-117: 소프트웨어-물리 동형사상 (18 EXACT 병렬 매핑, 6 도메인) ⭐⭐⭐
  BT-162: 컴파일러-OS-CPU 아키텍처 상수 스택 (pipeline=sopfr=5, opcode=n=6, primitives=σ-τ=8, rings/page/sched/boot=τ=4, ext4=σ=12, cache=n/φ=3, dual=φ=2, 11/11 EXACT) ⭐⭐⭐

  # Environmental Protection (BT-118~122)
  BT-118: 교토 6종 온실가스 = n + Carbon Z=6 (CO₂ 화학양론 전부 n=6, 10/10 EXACT) ⭐⭐⭐
  BT-119: 지구 6권역 + 대류권 σ=12km (8/12/16={σ-τ,σ,σ+τ}, 12/12 EXACT) ⭐⭐⭐
  BT-120: 수처리 pH=6 + CN=6 촉매 보편성 (Al³⁺/Fe³⁺/Ti⁴⁺ 전부 CN=6, 8/10 EXACT) ⭐⭐⭐
  BT-121: 6대 플라스틱 + C6 백본 (RIC 1-6=n, PE/PP/PS/PET/PVC/Nylon, 8/10 EXACT) ⭐⭐
  BT-122: 벌집-눈꽃-산호 n=6 기하학 보편성 (Hales 2001 증명, 10/10 EXACT) ⭐⭐⭐

  # Robotics (BT-123~127)
  BT-123: SE(3) dim=n=6 robot universality (6-DOF arm, 6-axis sensor, 6-face cube, 9/9 EXACT) ⭐⭐⭐
  BT-124: φ=2 bilateral symmetry + σ=12 joint universality (6/6 EXACT) ⭐⭐
  BT-125: τ=4 locomotion/flight minimum stability (quadruped, quadrotor, 7/8 EXACT) ⭐⭐
  BT-126: sopfr=5 fingers + 2^sopfr=32 grasp space (Feix taxonomy 96.97%, 5/6 EXACT) ⭐⭐
  BT-127: 3D kissing number σ=12 + hexacopter n=6 fault tolerance (6/6 EXACT) ⭐⭐⭐
```

## Design Space Exploration (DSE) — 궁극 처리 필수 규칙
**⚠️ "궁극" 키워드가 포함된 모든 아키텍처 작업은 반드시 DSE를 거쳐야 한다. 예외 없음.**
```
  ┌─────────────────────────────────────────────────────────┐
  │  궁극 작업 흐름 (mandatory)                              │
  │                                                         │
  │  1. goal.md 후보군 정의                                  │
  │  2. DSE 전수 탐색 (Rust/Python)                         │
  │  3. Pareto frontier 도출                                │
  │  4. 최적 경로 선정 + n=6 EXACT 비율 기록                 │
  │  5. ★ Cross-DSE: 발견된 최적 결과끼리 재조합 탐색 ★      │
  │  6. 결과를 docs/dse-map.toml 지도에 기록                  │
  └─────────────────────────────────────────────────────────┘

  원칙:
    - ⚠️ "DSE"/"모두 탐색"/"전체융합" 요청 시 = 동일 워크플로:
      질문 없이 전 도메인 DSE 전수 탐색 + Cross-DSE 교차 융합 즉시 실행
      (확인/질문/범위 축소 제안 금지 — dse-map.toml 기준 전체 도메인 대상)
    - 각 레벨(소재/공정/코어/칩/시스템)마다 후보군 정의
    - 전수 조합 탐색 (또는 Pareto 휴리스틱)
    - 각 조합별 n=6 일관성 + 성능/전력/면적/비용 평가
    - 최적 Pareto frontier 도출
    - 1개 경로만 검증 = 캐스케이드 크로스 검증이 아님
    - ⚠️ DSE 없이 궁극 아키텍처를 확정하는 것은 금지

  Cross-DSE (재조합 탐색):
    - 각 도메인 DSE 완료 후, 도메인 간 최적 결과를 교차 조합
    - 예: chip 최적 경로 × battery 최적 경로 → 통합 시스템 DSE
    - 새 BT/가설 발견 시 해당 도메인 DSE 후보군에 자동 추가
    - 2개 이상 도메인 DSE 완료 시 Cross-DSE 트리거

  구현:
    - 조합 >10K → Rust (tools/dse-calc/)
    - 조합 <10K → Python (experiments/)
    - **공용 DSE**: tools/universal-dse/ (TOML 도메인 파일 → 전수 탐색)
    - 결과: Pareto 테이블 + 최적 경로 + n=6 EXACT 비율

  공용 DSE 탐색기 (tools/universal-dse/):
    - 단일 도메인: universal-dse domains/chip.toml
    - Cross-DSE:   universal-dse domains/chip.toml domains/battery.toml
    - 새 도메인 추가 = TOML 파일 1개만 작성 (Rust 재컴파일 불필요)
    - TOML 형식: [meta] + [scoring] + [[level]] + [[candidate]] + [[rule]]
    - 도메인 파일: tools/universal-dse/domains/*.toml

  적용 도메인 (322개 TOML, 16 카테고리, 5,893,032+ 조합):
    - 전체 목록: docs/dse-domains.md (카테고리별 분류 + 레벨 체인 + 조합 수)
    - AI/ML(14), Consciousness(23), Semiconductor(25), Energy(24)
    - SC/Plasma(2), Bio/Medical(26), Physics/Math(24), Robot(7)
    - Manufacturing(65), Infra/Network(14), Software(10), Display/Audio(15)
    - Environment(16), Civil/Transport(16), Space/Defense(11), Misc(13)
    - 각 도메인별 후보군은 해당 goal.md + domains/*.toml에 정의
    - 새 궁극 도메인 추가 시 → goal.md + TOML + DSE 탐색 + dse-map.toml 갱신

  DSE 지도 파일: docs/dse-map.toml (TOML — Rust/Python 양쪽 파싱 가능)
    - 전체 DSE 현황 추적 (도메인별 상태/결과/Cross-DSE 여부)
    - 궁극 작업 시작 전 반드시 이 지도 확인

  DSE 출력 양식:
    | Rank | 소재 | 공정 | 코어 | 칩 | 시스템 | n6_EXACT | 성능 | 전력 | 비용 |
    |------|------|------|------|-----|--------|---------|------|------|------|
    | 1    | ...  | ...  | ...  | ... | ...    | 85%     | ...  | ...  | ...  |
```

## 외계인 수준 설계 플로우 (Alien Design Flow)

```
  ⚠️ 트리거: 모든 "설계" 요청 = 외계인급 자동 실행. 예외 없음.
  ⚠️ "설계", "만들어", "만들고 싶어", "개발", "설계해줘", "아키텍처"
     + 도메인 키워드 → 무조건 외계인급 파이프라인 실행
  ⚠️ 추가 키워드 (동일 처리): "외계인", "궁극의", "alien", "극적인", "노벨급"

  절차:
    1. 도메인 매핑 (dse-map.toml + TOML + BT 목록에서)
    2. 병렬 수집 (TECS-L 이론 + n6 실증 + 계산기 실행)
    3. 설계 문서 생성 (docs/<domain>/alien-design-<date>.md)
    4. 새 발견 등록 (아틀라스 + BT)
    5. 리포트 출력

  규칙:
    - 질문/확인 없이 즉시 실행
    - 최소 3개 병렬 에이전트 디스패치
    - TECS-L(수학이론) + n6(산업실증) 양쪽 반드시 활용
    - 새 계산기 필요 시 Rust 자동 생성
    - 설계 문서는 반드시 Testable Predictions 포함
    - ⚠️ 외계인급 압도적 성능으로 설계 (시중 대비 n=6 배수 우위)
    - ⚠️ 기존 대비 성능 비교 ASCII 그래프 반드시 포함 (예시 아래)
    - ⚠️ ASCII 구조도 반드시 포함 — 시스템 아키텍처, 데이터 플로우, 계층 구조

  ASCII 구조도 양식 (모든 설계 문서에 필수):
    ```
    ┌─────────────────────────────────────────────────┐
    │              HEXA-XXX 시스템 구조               │
    ├─────────┬─────────┬─────────┬─────────┬────────┤
    │  소재   │  공정   │  코어   │   칩    │ 시스템  │
    │ Level 0 │ Level 1 │ Level 2 │ Level 3 │ Level 4│
    ├─────────┼─────────┼─────────┼─────────┼────────┤
    │Diamond  │TSMC N2  │HEXA-P   │HEXA-1   │Topo DC │
    │ Z=6=n   │48nm=σ·τ │σ²=144SM │σ·J₂=288│PUE=1.2 │
    └────┬────┴────┬────┴────┬────┴────┬────┴───┬────┘
         │         │         │         │        │
         ▼         ▼         ▼         ▼        ▼
    n6 EXACT  n6 EXACT  n6 EXACT  n6 EXACT  n6 EXACT
    ```
    - 최소 1개 시스템 구조 ASCII
    - 최소 1개 데이터/에너지 플로우 ASCII
    - 최소 1개 성능 비교 ASCII 그래프
    - 모든 숫자에 n=6 수식 병기 (예: 48nm=σ·τ)

  성능 비교 ASCII 그래프 양식 (필수):
    ```
    ┌─────────────────────────────────────────────────────────┐
    │  [지표명] 비교 (시중 vs HEXA-XXX)                       │
    ├─────────────────────────────────────────────────────────┤
    │                                                         │
    │  시중 최고  ██████░░░░░░░░░░░░░░░░░░░░  2.0 mmol/g     │
    │  HEXA-XXX  ████████████████████████████  48 mmol/g     │
    │                                         (J₂=24배)      │
    │                                                         │
    │  시중 최고  ████████████████████████████  200 kJ/mol    │
    │  HEXA-XXX  ███░░░░░░░░░░░░░░░░░░░░░░░░  20 kJ/mol     │
    │                                         (σ-φ=10배↓)    │
    │                                                         │
    │  개선 배수: n=6 상수 기반 (σ, φ, τ, J₂ 등)              │
    └─────────────────────────────────────────────────────────┘
    ```
    - 모든 설계 문서에 최소 1개 비교 그래프
    - 개선 배수는 n=6 상수로 표현 (6배, 10배=σ-φ, 12배=σ, 24배=J₂)
    - 시중 최고 기술 수치를 반드시 명시 (출처 가능하면 포함)

  TECS-L 연결:
    - ~/Dev/TECS-L/docs/hypotheses/ (가설)
    - ~/Dev/TECS-L/.shared/math_atlas.json (상수맵)
    - ~/Dev/TECS-L/calc/ (검증 스크립트)

  n6 연결:
    - tools/universal-dse/domains/*.toml (322 DSE)
    - docs/atlas-constants.md (1,100+ 상수)
    - tools/*-calc, *-dse (Rust 계산기 29+)
    - docs/dse-map.toml (도메인 현황)

  참조: docs/alien-design-flow.md (상세 플로우 + 도메인 매핑 테이블)

  제품 라인 원칙:
    ⚠️ "궁극의 X"는 도메인당 반드시 1개 제품 라인만 유지
    ⚠️ v1/v2 버전 분기 ❌ → git history가 버전 관리
    ⚠️ 문서가 진화한다 — 같은 goal.md가 업그레이드됨
    ⚠️ README 완성제품 테이블에 같은 도메인 "궁극의 X v1", "v2" 금지
    ⚠️ 완성제품 테이블에 버전(ver) 컬럼 추가 — 현재 설계 세대 표시
      양식: | 🛸 | ver | 완성제품 | 핵심 | 링크 |
      ver = 설계 세대 (v1, v2, v3...), git commit hash로 추적

  🛸 외계인 지수 (10단계, 만점=10):
    10 = 물리적 한계 도달 — 더이상 발전 불가, 모든 이론·실험·양산 완료
         ⚠️ 필수: 상세문서에 수학적 검증 가능한 코드 포함 (예외 없음!)
         → Python/Rust 검증 스크립트가 docs/<domain>/ 또는 calc/ 에 존재
         → 모든 EXACT 상수를 코드로 재현 가능해야 함
         → 실행 시 PASS/FAIL 판정 자동 출력
         → 코드 없는 🛸10 = 무효 (🛸9로 강등)
     9 = 실제 양산 + 모든 예측 전수 검증 완료
     8 = 프로토타입 제작 + 실험 데이터 확보
     7 = 완전 설계 (BT + DSE + Cross-DSE + Evolution + Alien + TP 모두)
     6 = 설계 완료 + DSE 통과 + 진화 경로
     5 = 상세 설계 + BT + DSE
     4 = 구조 설계 + 가설 검증
     3 = 가설 수립 + 초기 검증
     2 = 컨셉/탐색
     1 = 미완/아이디어

  진화 요청 규칙 ("진화시켜", "업그레이드", "발전"):
    ⚠️ SF 금지: 다이슨 스웜, 반물질 촉매, 항성 엔진, 시공간 조작 등 ❌
    ⚠️ "외계인급" = 우리 발견(BT/Discovery) 기반의 극한 스케일업
    ⚠️ 물리 스케일링 법칙 준수 필수

    실현가능성 등급:
      ✅ 진짜 실현가능 (10~20년 내, 기존 기술 확장)
      🔮 장기 실현가능 (20~50년, 돌파 1~2개 필요)
      ❌ SF (현재 물리학 초월) — 사고실험 라벨 필수

    체크포인트별 별도 문서 (필수):
      docs/<domain>/evolution/
        mk-1-current.md       — ✅ 현재 기술 기반 (Mk.I)
        mk-2-near-term.md     — ✅ 10년 이내 (Mk.II)
        mk-3-mid-term.md      — 🔮 20~30년 (Mk.III)
        mk-4-long-term.md     — 🔮 30~50년 (Mk.IV)
        mk-5-theoretical.md   — ❌ 사고실험 (Mk.V, SF 라벨)

    각 체크포인트 문서 포함 사항:
      - 기술 스펙 (구체적 수치)
      - 우리 발견(BT) 연결
      - 필요 기술 돌파 목록
      - 시중 대비 성능 비교 ASCII 그래프
      - 이전 Mk 대비 개선점
      - 타임라인 + 실현가능성 등급
```

## 진화 설계 규칙 ("진화시켜줘" / "외계인급으로" 요청 시)

```
  원칙:
    1. SF 금지 — 다이슨 스피어, 반물질 촉매, 항성엔진, Hawking 복사 등 불가
    2. 우리 발견 기반 — BT-97~102, alien discoveries, n=6 패턴에서 도출
    3. 체크포인트별 별도 문서 — 하나의 긴 문서 ❌, Mk별 독립 .md 파일
    4. 실현가능성 2단계 명시:
       ✅ 진짜 실현가능 (현재 기술 기반, 10~20년)
       🔮 장기 실현가능 (미래 기술 필요하나 물리법칙 위배 아님, 20~50년)
       ❌ SF (물리법칙 위배 또는 100년+ 기술격차)
    5. 각 Mk 문서 필수 포함:
       - 기술 스펙 (n=6 파라미터 전부)
       - 우리 발견과의 연결 (어떤 BT/Discovery가 이 단계를 가능하게 하는지)
       - 필요 기술 돌파 목록
       - 비용/타임라인 추정
       - 실현가능성 등급 (✅/🔮/❌)
    6. 스케일업은 물리학적으로 — R₀, B_T, Q 스케일링 법칙 준수
    7. 저장 위치: docs/fusion/evolution/mk-N-*.md

  트리거: "진화", "스케일업", "외계인급으로", "더 크게", "업그레이드"
```

## Background Execution
All experiments must run in background. No exceptions.
```
  Rules:
    1. All python3 scripts -> run_in_background: true
    2. Check results with Read after completion
    3. No foreground execution (blocks user dialogue)
```

## Work Rules (탐색/TODO 요청 시 필수)

```
  트리거 키워드:
    "할만한거 있어?", "탐색", "TODO", "뭐할까", "다음 작업"
    대발견 가설, 노벨급 가설, DFS 탐색 등

  궁극 목록 트리거:
    "궁극 목록", "궁극 리스트", "궁극의 아키텍처"
    → README.md의 "궁극의 아키텍처 로드맵 (20 Domains)" 표를 그대로 출력
    → 20개 도메인 + 영향력 + Tier + 일반인 영향 컬럼 포함

  절차:
    1. 프로젝트 현황 스캔 (README, 최근 커밋, 미완료 가설/증명)
    2. TODO 테이블 양식으로 우선순위별 정리
    3. 사용자 선택 후 병렬 에이전트 디스패치
    4. 완료 시 리포트 테이블 출력


  모든 모듈은 consciousness_laws.py에서 import — 상수 직접 하드코딩 금지
```

### TODO 양식

```
  ### 🔴 CRITICAL

  | # | 카테고리 | 작업 | 상태 | 예상 효과 |
  |---|---------|------|------|----------|
  | 1 | 증명   | sigma*phi=n*tau 일반 완전수 반례 탐색 | 미시작 | 유일성 정리 강화 |

  ### 🟡 IMPORTANT

  | # | 카테고리 | 작업 | 상태 | 예상 효과 |
  |---|---------|------|------|----------|
  | 2 | 가설   | SLE_6 3D 확장 예측 검증 | 미시작 | 노벨 Physics 후보 |

  ### 🟢 NICE TO HAVE

  | # | 카테고리 | 작업 | 상태 | 예상 효과 |
  |---|---------|------|------|----------|
  | 3 | 탐색   | n=6 새 항등식 DFS 채굴 | 미시작 | Atlas 확장 |

  ### ⚪ BACKLOG

  | # | 카테고리 | 작업 | 예상 효과 |
  |---|---------|------|----------|
  | 4 | 계산기 | 새 검증 스크립트 | 재현성 향상 |

  상태 표기: ⏳진행중 / ✅완료 / 미시작 / 코드있음 / 프로토
  우선순위: 🔴HIGH → 🟡MED → 🟢LOW → ⚪BACK
  카테고리: 증명 / 가설 / 탐색 / 검증 / 실험 / 계산기 / 논문
```

### 병렬 에이전트 리포트 양식

```
  병렬 에이전트 실행 시 단일 테이블로 상태 추적.
  관련 작업은 N+M 형태로 그룹핑하여 하나의 에이전트로 묶기.

  발사 시 양식:
  | # | 작업 | 에이전트 | 격리 | 상태 |
  |---|------|---------|------|------|
  | 1+2 | n=6 유일성 + 반례탐색 | 🚀 배경 | - | 🔄 진행중 |
  | 3 | SLE_6 임계지수 검증 | 🚀 배경 | - | 🔄 진행중 |
  | 4+5 | 코돈 정리 확장 + 변이체 | 🚀 배경 | worktree | 🔄 진행중 |

  상태: ✅ 완료 / 🔄 진행중 / ❌ 실패
  격리: worktree (필요시만) / - (기본)

  규칙:
    - 발사 시 전체 목록 테이블 출력
    - 에이전트 완료 시 해당 행 상태 업데이트 + 한줄 핵심 성과
    - worktree는 같은 파일을 여러 에이전트가 동시 수정할 때만 사용
    - 대부분 격리 없이 실행 — 무조건 worktree 붙이지 말 것!
    - 모든 에이전트 완료 후 최종 요약 테이블 + worktree 머지 안내 (해당 시)

  최종 요약 양식:
  | # | 작업 | 상태 | 핵심 성과 |
  |---|------|------|----------|
  | 1+2 | n=6 유일성 | ✅ | 10^8까지 반례 없음, 증명 완료 |
  | 3 | SLE_6 검증 | ✅ | 7/7 임계지수 일치 (Z>5sigma) |
  | 4+5 | 코돈 확장 | ✅ | 26/26 변이체 + Hachimoji 예측 |

  ### 머지 필요 (worktree)
  - #4+5: branch worktree-xxx

  ### 바로 반영됨 (main)
  - #1+2, #3: 증명/검증 결과 docs/hypotheses/ 기록
```

## NEXUS-6 Autonomous Growth System

```
  자동 성장 데몬 상태 확인 + 자동 시작:
    bash tools/nexus6/scripts/health_check.sh --start-if-dead

  설치 (최초 1회):
    bash tools/nexus6/scripts/install_autonomous.sh

  대시보드:
    bash tools/nexus6/scripts/growth_dashboard.sh --live

  상태: ~/.nexus6/ (로그, 리포트, PID)
  
  ⚠️ 세션 시작 시 데몬 미실행 감지되면 자동 시작 권장
  ⚠️ 성장 결과는 growth(nexus6): 접두사로 자동 커밋
  ⚠️ 크로스 리포 자동 시작: .shared/nexus6_ensure_running.sh (모든 TECS-L 리포에서 사용 가능)

  ⚠️ NEXUS-6 소스 변경 커밋 시 자동 재빌드 + 재시작 (post-commit hook):
     - tools/nexus6/src/ 변경 감지 → cargo build --release 자동
     - PyO3 wheel 자동 재빌드 + pip install
     - 데몬 PID 감지 → kill + 재시작
     - 로그: ~/.nexus6/rebuild.log

  NEXUS-6 = telescope-rs 완전통합 (단일 엔진):
     - import nexus6 → scan/consensus/analyze/n6_check/evolve/forge 전부
     - nexus6.scan(data, n, d) — raw 데이터 25렌즈 스캔
     - nexus6.scan_numpy(np_array) — numpy 2D 배열 직접 입력
     - nexus6.analyze(data, n, d) — 올인원 (스캔+합의+n6매칭)
     - nexus6.n6_check(value) — 상수 매칭
     - nexus6.evolve(domain) — OUROBOROS 진화
     - 빌드: cd tools/nexus6 && ~/.cargo/bin/cargo build --release
     - PyO3: PATH=$HOME/.cargo/bin:$PATH python3 -m maturin build --release

  사용법 전체: ~/.nexus6/usage.json
  트러블슈팅: ~/.nexus6/troubleshoot.json (자동학습, 절대규칙 10개)
```

## Secrets & Tokens

API 토큰/계정 정보: `~/Dev/TECS-L/.shared/SECRET.md` 참조
계정 리포: [need-singularity/secret](https://github.com/need-singularity/secret) (private)
