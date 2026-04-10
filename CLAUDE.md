<!-- L0 CORE — 수정 금지 -->
# n6-architecture — Arithmetic Design Framework

🔴 HEXA-FIRST: 모든 코드 .hexa, 부하 명령 최소화
🔴 NEXUS-6 연동: 돌파 → blowup.hexa <d> 3, 발견 → growth_bus.jsonl, 상태 → command_router.hexa
🔴 하드코딩 금지: 상수/도메인/키워드 → nexus/shared/*.jsonl 동적로드, 코드 배열 금지
🔴 데이터 로컬 금지: .jsonl/constants/discovery_log → ~/Dev/nexus/shared/ (R8)

R14: shared/ JSON 단일진실, 규칙 본문은 absolute_rules.json만. 이 파일은 참조만.

핵심 정리: σ(n)·φ(n) = n·τ(n) ⟺ n = 6 (n≥2). 3개 독립 증명.
              docs/theorem-r1-uniqueness.md | reality_map v8.0 (342노드, 291 EXACT, z=9.04)
부모 리포: TECS-L 수학 이론의 산업 실증
모토: 17 AI 기법 + 반도체 + 에너지 + 네트워크/암호/OS + 물리 AI

ref:
  rules     shared/config/absolute_rules.json    R1~R21 + N6/N61~N65
  lock      shared/config/core-lockdown.json     L0 22 / L1 / L2
  registry  shared/config/projects.json
  cfg       shared/config/project_config.json    CLI/설계문서/논문/진화/Rust
  core      shared/config/core.json              시스템맵+14명령
  conv      shared/convergence/n6-architecture.json
  loop      shared/loop/                          자율 데몬
  bt        docs/breakthrough-theorems.md         343 정리 BT-1~343
  dse       docs/dse-map.toml                     322 도메인
  papers    config/products.json                  116편 (docs/paper/ 135파일)
  atlas     docs/atlas-constants.md               1100+ 상수
  lenses    tools/nexus/src/telescope/lenses/     181 .rs (1022종)
  techniques techniques/                          17 AI 기법
  flow      docs/alien-design-flow.md
  predict   docs/testable-predictions.md          45건
  api       nexus/shared/CLAUDE.md
  secret    ~/Dev/TECS-L/.shared/SECRET.md        API 토큰

NEXUS-6:
  cli       nexus scan <d> | --full | nexus verify <v>
  api       nexus.scan_all() / .analyze() / .n6_check() / .evolve()
  consensus 3+후보 / 7+고신뢰 / 12+확정

명령:
  못박아줘    L0 등록 (core-lockdown.json)
  블로업/돌파 9-phase 특이점 (blowup.hexa)
  go          전체 TODO 백그라운드 병렬
  설계/궁극의 외계인급 설계 파이프라인
  동기화      sync-all.sh 전 리포

Rust 도구 (tools/, build: ~/.cargo/bin/rustc file.rs -o out, nexus만 cargo):
  fusion-calc tokamak-shape optics-calc gpu-arch-calc energy-calc gut-calc-rust
  dse-calc solar-dse material-dse universal-dse
  nexus (cargo build --release)  nexus-dashboard (port 6600 Axum+Chart.js)

Quick run:
  python3 techniques/phi6simple.py
  python3 techniques/fft_mix_attention.py
  python3 experiments/experiment_h_ee_11_combined_architecture.py

주요 결과:
  Cyclotomic activation     71% FLOPs↓
  FFT attention             3x speed, +0.55% acc
  Egyptian MoE              1/2+1/3+1/6=1 라우팅
  phi bottleneck            67% param↓
  Entropy early stop        33% 학습시간↓
  Boltzmann gate            63% 활성희소
  Mertens dropout           p=0.288 (탐색불요)
  Egyptian Fraction Attn    ~40% FLOPs↓
  Emergent convergence      랜덤→n=6 자기조직화

설계 산출물 필수 5요소 (없으면 미완성):
  1. 실생활 효과 섹션 (최상단) — 삶을 어떻게 바꾸는가
  2. ASCII 성능 비교 — 시중 vs HEXA, 개선 배수는 n=6 상수
  3. ASCII 시스템 구조도 — 소재→공정→코어→칩→시스템, n=6 수식 병기
  4. ASCII 데이터/에너지 플로우
  5. 업그레이드 시 — 시중 vs v1 vs v2 3단 + delta 행
  + Python 검증코드 (정의에서 도출, 동어반복 금지)

논문 관리 (상세 docs/paper/README.md):
  loc      docs/paper/n6-<domain>-paper.md
  ssot     config/products.json
  sync     python3 scripts/sync_products_readme.py
  필수섹션 Abstract+Foundation+Domain+Limitations+TestablePredictions+검증코드
  발행     ~/Dev/papers/ → Zenodo/OSF
  현재     39편, 검증코드 없는 논문 = 미완성

진화 설계 (Mk.I~V):
  - SF 금지 (다이슨 스웜/반물질 등) — BT 기반 극한 스케일업만
  - docs/<domain>/evolution/mk-{1..5}-*.md
  - ✅ 진짜(10~20y) | 🔮 장기(20~50y) | ❌ SF(라벨)
  - 각 Mk: 스펙+BT연결+필요돌파+ASCII+타임라인
  - 도메인당 1제품라인 (v1/v2 분기 금지, git history가 버전)

todo: "todo"|"할일" → $HOME/Dev/hexa-lang/target/release/hexa $HOME/Dev/nexus/mk2_hexa/native/todo.hexa n6-arch
