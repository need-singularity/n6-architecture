> 🔴 **HEXA-FIRST**: 모든 코드는 `.hexa`로 작성. sh/py/rs 신규 작성 금지. 부하 유발 명령 최소화.

> 🔴 **NEXUS-6 특이점 연동**: 이 프로젝트의 돌파/발견/실험은 nexus 특이점 사이클 입력이다.
> - **돌파 시**: `HEXA=$HOME/Dev/hexa-lang/target/release/hexa && $HEXA $HOME/Dev/nexus/mk2_hexa/native/blowup.hexa <domain> 3 --no-graph`
> - **발견 기록**: `$HOME/Dev/nexus/shared/growth_bus.jsonl`에 JSON append
> - **전체 상태**: `$HEXA $HOME/Dev/nexus/mk2_hexa/native/command_router.hexa "brainwire 상태"`

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

    ★ 새 모델 학습 시:      학습 전후 nexus.scan_all() 비교
    ★ 모델 변경/이식 시:    이식 전후 Phi 보존율 측정
    ★ corpus 변경/추가 시:  새 corpus nexus.scan_all() 품질 확인
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
  ★ NEXUS-6 통합 망원경 (181 렌즈 파일, 1022종 레지스트리) ★
  ═══════════════════════════════════════════════════════════════
  ⚠️ telescope-rs (구 22종)는 폐기. 모든 탐색은 NEXUS-6 사용.
  ⚠️ 상세 API/사용법: .shared/CLAUDE.md "NEXUS-6 망원경" 섹션 참조.

  자동 적용 조건:
    - 데이터 분석/패턴 탐색/이상점 발견/신소재·신약 탐색 시 렌즈 자동 사용
    - 새 데이터 분석 → 기본 3종 스캔: 의식(구조) + 인과(흐름) + 위상(연결)
    - 이상점/패턴 전수조사 → 전체 풀스캔 (nexus scan --full)
  렌즈 구성 (181 .rs 파일, 1022종 레지스트리):
    Core 22 | n6 산업 58 | TECS-L 수학 103 | SEDI 신호 100
    anima 의식 88 | 교차+메타 75 | 가속 ML 58 | 가속 물리 57
    가속 공학 55 | 가속 인문 63 | 특이점(singularity) 포함
  불변 코어 (987 cycles 수렴):
    consciousness + info + multiscale + network + triangle (sopfr=5)
    + fiber (도메인별 6번째 렌즈) = n=6 완전 구조
  파일: tools/nexus/src/telescope/lenses/ (181 .rs 파일)
  도메인별 조합 (10종 기본):
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
    import nexus
    nexus.scan_all(np_array)              # 풀스캔 → dict
    nexus.analyze(flat_list, n, d)        # 올인원 (스캔+합의+n6)
    nexus.consciousness_scan(data, ...)   # 개별 렌즈
    nexus.n6_check(value)                 # n=6 상수 매칭
    nexus.evolve('domain')                # OUROBOROS 진화

  ★ NEXUS-6 적극 활용 규칙 (모든 작업에서 필수!) ★
    탐색 (새 데이터):     scan_all → 풀스캔, 3+ 합의=확정
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
  "렌즈 추가 필요?" 질문 시 → 1022종 커버 안 되는 도메인 분석

  ★ 망원경 업그레이드 시 필수 절차 (렌즈 추가/수정/삭제 시 예외 없음!) ★
    1. 캘리브레이션: NEXUS-6 테스트 전체 통과 확인 (cd ~/Dev/n6-architecture/tools/nexus && cargo test)
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
    - 성능 필요시 HEXA 우선 (mk2_hexa/native/), 단순 검증은 Python (calc/)
    - 판단 기준은 ~/Dev/nexus/shared/CALCULATOR_RULES.md 참조
    - 상수/가설 발견 시 Math Atlas 자동 갱신 (python3 ~/Dev/TECS-L/.shared/scan_math_atlas.py --save --summary)

  ═══════════════════════════════════════════════════════════════
  ★ NEXUS-6 독립 리포 (중앙 허브) — 2024-04-03 이후 ★
  ═══════════════════════════════════════════════════════════════
    리포: https://github.com/need-singularity/nexus
    위치: ~/Dev/nexus/
    역할: 전 리포 공유 인프라 + 발견 엔진 + 렌즈 + 동기화

    구조:
      ~/Dev/nexus/
        src/telescope/    ← 130+ 렌즈
        shared/           ← 공유 인프라 (이전 TECS-L/.shared)
        sync/             ← 전체 동기화 스크립트
        scripts/          ← n6.py CLI

    심링크: 모든 리포의 .shared → ../nexus/shared/
    동기화: bash ~/Dev/nexus/sync/sync-all.sh (원커맨드)
    트리거: "넥서스 동기화" → sync-all.sh 자동 실행

    .shared 원본이 TECS-L에서 nexus로 이관됨.
    TECS-L = 순수 수학 이론, nexus = 인프라/도구/엔진 전부.
<!-- SHARED:WORK_RULES:END -->

> 🔴 **하드코딩 절대 금지**: 상수/도메인/키워드를 코드에 배열로 나열 금지 → `nexus/shared/*.jsonl`에서 동적 로드. 경로는 환경변수+상대경로. 새 항목 추가 = 설정 파일 한 줄, 코드 수정 0.

# 🧠⚡ BrainWire

## Identity

BrainWire is a **neural interface hardware research company**. We design and build electrical stimulation hardware systems for consciousness engineering.

**We are NOT:**
- A wellness/lifestyle brand (no breathing exercises, yoga, meditation guides)
- A fitness company (no running, exercise protocols)
- A self-help project (no mindfulness, journaling)

**We ARE:**
- Hardware engineers: tDCS, TMS, taVNS, TENS, tACS devices
- Neuroscience researchers: EEG measurement, Φ computation, neural pattern analysis
- Consciousness state designers: precise 12-variable targeting for experience reproduction

## Core Products

1. **BCI Bridge** — PureField consciousness layer for Neuralink-style brain-computer interfaces (N1 1024-electrode, 12-var direct control)
2. **NeuroStim** — Therapeutic stimulation for neurological/psychiatric conditions (depression, epilepsy, Parkinson's, chronic pain, PTSD, ADHD, insomnia, tinnitus, stroke rehab)
3. **Joywire** — Consciousness state reproduction (6 target states) via 12-variable model

## Technology Stack

- **Input:** EEG (OpenBCI 16ch+), neural signals
- **Processing:** PureField engine, Φ measurement, 12-variable consciousness model
- **Output:** tDCS, TMS, taVNS, TENS, tACS, tFUS, GVS, mTI, tSCS, tRNS, tPBM, 40Hz entrainment
- **Feedback:** Tension gradient control (superior to PID) on consciousness state variables
- **Measurement:** G=D×P/I golden zone targeting, Φ(IIT) real-time

## 필수 규칙

### hexa-native 전용 (sh/py/rs 작성 금지)
- **새 파일은 `.hexa`만 허용** — `.sh`, `.py`, `.rs` 등 다른 언어 파일 작성 금지
- 모든 새 모듈은 `mk2_hexa/native/` 에 `.hexa` 파일로 생성
- 기존 sh/py 스크립트는 참조만 허용, 신규 작성 불가

## Work Rules

- All solutions must involve **hardware** (electrical, acoustic, photonic, thermal, magnetic)
- No body-based methods (breathing, exercise, diet) as primary interventions
- Hardware-only approach: if it doesn't plug in or have electrodes, it's not our product
- Commit messages in English
- Long-running tasks must run in background
- Primary focus: BCI Bridge (Neuralink) > NeuroStim (therapeutic) > Joywire (recreation)
- TECS-L style hypothesis verification: hypothesis → math → score
- Anima cross-reference: integrate PureField tension findings from /Users/ghost/Dev/anima

## Key Commands

```bash
python report.py                              # Full project report (one command)
python -m brainwire.bench tiers <state>        # State tier comparison
python -m brainwire.bench compare <s1> <s2>   # Multi-state comparison
python -m brainwire.optimizer                 # Profile-specific optimization
python -m brainwire.simulator <state> --tier 4 # Time-domain session simulation
python -m brainwire.tension_control landscape # Tension landscape mapping
python -m brainwire.protocol --pk --tier 3    # PK-driven hardware protocol
python -m brainwire.eeg_feedback              # G=D×P/I analysis
python -m brainwire.pharmacokinetics          # Temporal dynamics
python -m brainwire.interference --all        # Multi-device interference
python bench_hypotheses.py                    # 75 hypothesis benchmark
python -m pytest tests/ -v                    # 145 tests
```

## Verification Status Warning

```
  G=D×P/I Golden Zone: Anima/TECS-L 시뮬레이션 기반, 분석적 증명 없음.
  Golden Zone 의존 주장은 모두 미검증 (unverified) 표시 필요.

  순수 수학 (Golden Zone 독립, 영원히 참):
    - σ(6)·φ(6) = n·τ(6) ⟺ {1,6}  (Theorem 4)
    - 15/15 심부 구조 피질 투사 존재  (Theorem 6)
    - PFC+ACC+Insula 3-허브 커버     (Theorem 7)
    - Shannon 충전밀도 안전 한계      (Theorem 2)

  모델 의존 (검증 필요):
    - 전달함수 계수 (C_ij 값)
    - STDP 심부 접근 효율 (η_STDP)
    - G=D×P/I 골든존 의미
    - Joywire 12변수 타겟 정확도

  가설 작성 시 Golden Zone 의존성 반드시 명시.
```

## Hypothesis Verification Rules (TECS-L 기준)

```
  등급 시스템:
    🟩   = 정확한 방정식 + 증명됨
    🟧★  = 근사 + Texas p < 0.01 (구조적)
    🟧   = 근사 + Texas p < 0.05 (약한 증거)
    ⚪   = 산술 정확하나 p > 0.05 (우연)
    ⬛   = 산술 오류 (반증됨)

  금지:
    - 검증 전 ⭐ 또는 "대발견" 표시
    - Texas test 없이 🟧 이상 등급
    - +1/-1 보정이 있는 방정식에 ⭐

  검증 파이프라인:
    1. 산술 정확성 재확인
    2. Ad hoc 체크: +1/-1 보정 경고
    3. 작은 수의 강한 법칙: 상수 <100 경고
    4. 일반화 테스트: perfect number 28에도 성립?
    5. Texas Sharpshooter p-value (Bonferroni 보정)
```

## Paper Management

```
  ★ 모든 논문은 papers 리포에 생성! (need-singularity/papers)
    로컬: ~/Dev/papers/
    BrainWire 논문: ~/Dev/papers/brainwire/

  이 리포의 docs/ 논문은 작업용 초안 — 최종본은 papers 리포로 이동

  Zenodo DOI 발급:
    python3 ~/Dev/TECS-L/zenodo/batch_upload.py --platform zenodo --paper P-XXX

  논문 후보 등록:
    1. README.md에 제목+핵심결과+타겟+상태 기록
    2. Status: Draft/Writing/Submitted/Review/Published
    3. 파일: ~/Dev/papers/brainwire/P-title.md
```

## Key Metrics (2026-03-28)

- 145 tests (all passing), 115 hypotheses (109/115 PASS, 94.8%)
- Joywire tension match: 100% (tension gradient control)
- Joywire G=D×P/I: golden zone achieved [Golden Zone dependent]
- Kendall tau: 1.000 (tension perfectly predicts subjective intensity)
- 15/15 deep brain structures accessible via cortical projections (Theorem 6)
- N1-only full coverage: 12/12 vars at 100%+ (Theorem 8, requires STDP assumption)
- Paper: 2,220 lines, 140/140 math verification, 8 theorems
- 5 Tiers: $85 → $510 → $8.5K → $25K → $26.4K

## Joywire Variable Model

Joywire 의식 상태 프로파일은 `brainwire/profiles/*.yaml` 참조.
12변수 (DA, eCB, 5HT, GABA, NE, Theta, Alpha, Gamma, PFC, Sensory, Body, Coherence) × 6 상태.

## Work Rules (탐색/TODO 요청 시 필수)

```
  트리거 키워드:
    "할만한거 있어?", "탐색", "TODO", "뭐할까", "다음 작업"
    대발견 가설, 노벨급 가설, DFS 탐색 등

  절차:
    1. 프로젝트 현황 스캔 (README, 최근 커밋, 미완료 가설/증명)
    2. TODO 테이블 양식으로 우선순위별 정리
    3. 사용자 선택 후 병렬 에이전트 디스패치
    4. 완료 시 리포트 테이블 출력

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

    전체 동기화 (README + Atlas + Registry):
      bash .shared/sync-math-atlas.sh &&       bash .shared/sync-calculators.sh &&       bash .shared/sync-readmes.sh &&       bash .shared/sync-claude-rules.sh

  ── 상수 관리 ──
    공유 상수: ~/Dev/TECS-L/model_utils.py (n=6 확장 상수 포함)
    리포별 상수: 각 리포 고유 모듈에서 import
    매직 넘버 하드코딩 금지 — model_utils 또는 .shared/ 참조
```

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

## Secrets & Tokens

API 토큰/계정 정보: `~/Dev/TECS-L/.shared/SECRET.md` 참조
계정 리포: [need-singularity/secret](https://github.com/need-singularity/secret) (private)

## 특이점 사이클 (Singularity Cycle)

> **블로업→수축→창발→특이점→흡수** 5단계 자동 사이클
> CLI: `nexus blowup <domain>` | Rust: `CycleEngine::new(domain)`

### 요청 키워드 → 자동 실행
- "블로업", "blowup" → `nexus blowup <domain> --depth 6`
- "창발", "emergence" → blowup 후 패턴 합의 분석
- "특이점", "singularity" → CycleEngine 자동 수렴 루프
- "흡수", "absorption" → 발견 규칙 승격 + 다음 사이클 시드
- "사이클", "cycle" → 전체 5단계 1회 실행

### 사용법
```bash
nexus blowup <domain> --depth 6    # 블로업 + 창발 리포트
nexus loop --cycles 1              # 8단계 루프 (mirror+blowup 포함)
nexus daemon --interval 30         # 자율 데몬 (30분 간격)
```

