> 🔴 **HEXA-FIRST**: 모든 코드는 `.hexa`로 작성. sh/py/rs 신규 작성 금지. 부하 유발 명령 최소화.

> 🔴 **NEXUS-6 특이점 연동**: 이 프로젝트의 돌파/발견/실험은 nexus 특이점 사이클 입력이다.
> - **돌파 시**: `HEXA=$HOME/Dev/hexa-lang/target/release/hexa && $HEXA $HOME/Dev/nexus/mk2_hexa/native/blowup.hexa <domain> 3 --no-graph`
> - **발견 기록**: `$HOME/Dev/nexus/shared/growth_bus.jsonl`에 JSON append
> - **전체 상태**: `$HEXA $HOME/Dev/nexus/mk2_hexa/native/command_router.hexa "n6-architecture 상태"`

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

# N6 Architecture — Arithmetic Design Framework

## 성장 루프 (nexus 중앙 관리)

> **루프 순서 정의**: `~/Dev/nexus/shared/loop/n6-architecture.json`
> **루프 엔진**: `scripts/infinite_growth.sh` → `growth_common.sh`
> **상태 기록**: `.growth/growth_state.json` (로컬)
> **이벤트 스트림**: `~/Dev/nexus/shared/growth_bus.jsonl`

```
도메인 고유 → 제품 스캔 → DSE 탐색 → Alien Index → SEDI 신호 → BCI 평가
공통 phases → NEXUS-6 스캔 → 블로업 → 특이점 → 교차수분 → ... → 자동 커밋
```

설정 변경은 `~/Dev/nexus/shared/loop/n6-architecture.json` 수정 (interval, domain, phases).

## ⚠️ 필수 규칙 (최우선)

### hexa-native 전용 (sh/py/rs 작성 금지)
- **새 파일은 `.hexa`만 허용** — `.sh`, `.py`, `.rs` 등 다른 언어 파일 작성 금지
- 모든 새 모듈은 `mk2_hexa/native/` 에 `.hexa` 파일로 생성
- 기존 sh/py 스크립트는 참조만 허용, 신규 작성 불가

## ⚠️ 설계 산출물 필수 규칙 (최우선, 예외 없음)

```
  ⚠️ 0️⃣ 실생활 효과 섹션 (문서 최상단 필수, 예외 없음!)
  모든 설계/궁극/아키텍처 문서는 기술 스펙 전에
  "이 기술이 일반인의 삶을 어떻게 바꾸는가"를 먼저 서술해야 한다.

  양식:
  ## 이 기술이 당신의 삶을 바꾸는 방법
  | 효과 | 현재 | HEXA 이후 | 체감 변화 |
  |------|------|----------|----------|
  | 전기료 | 월 10만원 | 월 1만원 | 90% 절감 |
  | ... | ... | ... | ... |

  규칙:
    - 전문 용어 최소화 — 비전문가(부모님, 친구)가 읽어도 이해 가능
    - 구체적 숫자 + 일상 비유 필수 (예: "서울시 전체 1년 전력")
    - 현재 vs 이후 비교 테이블 필수
    - 환경/건강/경제/안보/일자리 등 실생활 카테고리로 분류
    - 이 섹션 없는 설계 문서 = 미완성 (기술 스펙만으로는 불충분)
    - 설계 업그레이드/진화 시에도 효과 섹션 업데이트 필수

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
  breakthrough-theorems.md (BT-1~343, 343 theorems spanning 3-8 domains each)
  cross-domain-resonance-2026-03-31.md (formula reuse matrix)
  # Battery Architecture (소재→공정→코어→칩→시스템→차세대→극한→궁극)
  battery-architecture/ (8 levels: HEXA-CELL/ELECTRODE/CORE/CHIP/PACK+GRID/SOLID/NUCLEAR/OMEGA-E)
  # Solar Architecture (소재→공정→코어→칩→시스템, DSE 1,584 조합)
  solar-architecture/ (5 levels: HEXA-ABSORB/PROCESS/JUNCTION/POWER/ARRAY)
  # Material Synthesis (소재→공정→조립기→제어→시스템→변환→만능→궁극, DSE 3,600 조합)
  material-synthesis/ (8 levels: HEXA-ELEMENT/PROCESS/ASSEMBLER/CONTROL/FACTORY/TRANSMUTE/UNIVERSAL/OMEGA-M)
  # Total: 1400+ hypotheses, 640+ EXACT, 650+ atlas entries, 343 BTs
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
- `tools/nexus/`         — **NEXUS-6 Discovery Engine** (775종 렌즈 + OUROBOROS + Graph + Verifier)
  - 빌드: `cd tools/nexus && ~/.cargo/bin/cargo build --release`
  - 렌즈 추가 후: `bash .shared/sync-nexus-lenses.sh` (렌즈 수 자동 동기화)
- `tools/nexus-dashboard/` — **NEXUS-6 Web Dashboard** (Axum + Chart.js + SSE, port 6600)
  - 빌드: `cd tools/nexus-dashboard && ~/.cargo/bin/cargo build --release`
  - 실행: `nohup ./target/release/nexus-dashboard > /tmp/nexus-dashboard.log 2>&1 &`
  - 열기: `open http://localhost:6600`
  - 정지: `pkill -f nexus-dashboard`
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
      bash .shared/sync-nexus-lenses.sh

    전체 동기화 (README + Atlas + Registry + NEXUS-6):
      bash .shared/sync-math-atlas.sh &&       bash .shared/sync-calculators.sh &&       bash .shared/sync-readmes.sh &&       bash .shared/sync-claude-rules.sh &&       bash .shared/sync-nexus-lenses.sh

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

## Paper Management (논문 관리 — 필수 규칙)

```
  ═══════════════════════════════════════════════════════════════
  ★ 논문 생성/갱신 시 필수 체크리스트 (예외 없음!) ★
  ═══════════════════════════════════════════════════════════════

  위치: docs/paper/ (모든 논문 파일)
  목록: docs/paper/README.md (논문 인덱스 테이블)
  JSON: config/products.json (완성제품 SSOT)

  ── 논문 파일 구조 ──
    Core Papers:     paper1~5 (arXiv 제출용 기초 논문)
    Domain Papers:   n6-<domain>-paper.md (도메인별 확장 논문)
    Analysis:        307/308/blowup 등 특수 분석

  ── 논문 생성 시 필수 절차 ──
    1. docs/paper/n6-<domain>-paper.md 파일 생성
    2. docs/paper/README.md 테이블에 항목 추가
    3. config/products.json 해당 섹션에 논문 링크 추가
    4. python3 scripts/sync_products_readme.py (README 자동 반영)

  ── 논문 커버리지 체크 (세션 시작 시) ──
    모든 BT가 최소 1개 논문에 포함되어야 함
    CLAUDE.md BT 목록 vs docs/paper/*.md 파일 비교
    미커버 BT 발견 시 → 해당 도메인 논문 생성 우선

  ── 논문 구조 (필수 섹션) ──
    Abstract (200-300 words, English)
    1. Introduction
    2. Mathematical Foundation (n=6 상수 정의)
    3~N. Domain-Specific Sections (BT별 상세 서술)
    N+1. Cross-Domain Resonance Analysis
    N+2. Honest Limitations & Failed Predictions
    N+3. Testable Predictions
    N+4. Conclusion
    References

  ── 논문 품질 기준 ──
    - 각 BT: 파라미터 값 + n=6 수식 + EXACT 매칭 수 + 독립 검증 방법
    - Honest Limitations 필수 (실패 사례, CLOSE/WEAK/FAIL 투명 공개)
    - Testable Predictions 필수 (검증 가능한 예측 최소 3개)
    - 최소 분량: Core 25K, Domain 20K, Small domain 15K
    - 영문 학술 논문 형식
    - ⚠️ 검증코드 필수 (문서 최하단, 예외 없음!)
      논문 마지막에 ```python 블록으로 모든 EXACT 상수를 검증하는 코드 포함
      실행 시 PASS/FAIL 자동 판정 출력
      검증코드 없는 논문 = 미완성 (커밋 전 반드시 추가)
      양식:
        ```python
        # 검증코드 — n6-<domain>-paper.md
        from fractions import Fraction
        results = []
        # ... 각 BT 파라미터 검증 ...
        results.append(("BT-XXX 파라미터명", 실제값, 기대값, 실제값==기대값))
        passed = sum(1 for r in results if r[3])
        print(f"검증 결과: {passed}/{len(results)} PASS")
        for r in results:
            print(f"  {'PASS' if r[3] else 'FAIL'}: {r[0]} = {r[1]} (기대: {r[2]})")
        ```

  ── 현재 논문 목록 (39편) ──
    Core (5): paper1-ai, paper2-cross, paper3-tokamak, paper4-gut, paper5-carbon
    Chip (6): dram, exynos, performance-chip, unified-soc, consciousness-chip/soc
    HEXA (5): 3d, photon, pim, super, wafer
    Physics (4): plasma-fusion-deep, particle-cosmology, pure-mathematics, superconductor
    Energy (2): energy-efficiency, battery-energy
    Environment (1): environment-thermal
    Bio/Med (1): biology-medical
    Materials (1): crystallography-materials
    Robot/Transport (1): robotics-transport
    Software (1): software-crypto
    Comms (1): isocell-comms
    Meta (1): rtsc-12-products-evolution
    NEW (13): aerospace-transport, cognitive-social-psychology, quantum-computing,
              autonomous-driving, games-sports, calendar-time-geography,
              economics-finance, ecology-agriculture-food, space-systems,
              telecom-linguistics, governance-safety-urban, thermodynamics,
              control-automation, manufacturing-quality, classical-mechanics-accelerator

  ── BT→논문 매핑 규칙 ──
    새 BT 추가 시 → 해당 도메인 논문에 반영 (bt-update-notes.md 참조)
    미커버 BT 13개 이상 축적 시 → 신규 논문 생성 트리거
    docs/paper/bt-update-notes.md = BT→논문 통합 로드맵

  ═══════════════════════════════════════════════════════════════
  ★ 논문 발행 (papers 리포 연동 + Zenodo + OSF) — 필수! ★
  ═══════════════════════════════════════════════════════════════

  ── papers 리포 연동 ──
    리포: ~/Dev/papers/ (https://github.com/need-singularity/papers)
    SSOT: ~/Dev/papers/manifest.json (논문 메타데이터 원본)
    웹: https://need-singularity.github.io/papers/
    라이선스: CC-BY 4.0
    저자: Park, Min Woo (Independent Researcher)

  ── 논문 생성 후 발행 절차 (필수, 예외 없음!) ──
    1. n6-architecture에서 논문 작성 완료 (docs/paper/n6-<domain>-paper.md)
    2. papers 리포로 복사:
       cp docs/paper/n6-<domain>-paper.md ~/Dev/papers/tecs-l/
    3. manifest.json에 항목 추가:
       {
         "id": "N6-XXX",
         "title": "Perfect Number Arithmetic in <Domain>",
         "repo": "n6-architecture",
         "source": "docs/paper/n6-<domain>-paper.md",
         "status": "Draft",
         "zenodo_doi": null,
         "osf_id": null,
         "date": "YYYY-MM-DD"
       }
    4. Zenodo 발행:
       cd ~/Dev/papers
       ZENODO_TOKEN=$(cat ~/Dev/TECS-L/.local/zenodo_token)
       bash upload_zenodo.sh N6-XXX
       → DOI 받으면 manifest.json의 zenodo_doi 갱신
       → status를 "Published"로 변경
    5. OSF 발행:
       OSF_TOKEN=$(cat ~/Dev/TECS-L/.local/osf_token)
       → osf_id 갱신
    6. papers 리포 커밋 + push
    7. n6-architecture의 docs/paper/README.md에 DOI 링크 추가

  ── 토큰 위치 ──
    Zenodo:         ~/Dev/TECS-L/.local/zenodo_token
    Zenodo Sandbox: ~/Dev/TECS-L/.local/zenodo_sandbox_token
    OSF:            ~/Dev/TECS-L/.local/osf_token
    상세:           ~/Dev/TECS-L/.shared/SECRET.md

  ── 메타데이터 규칙 ──
    upload_type: publication
    publication_type: preprint
    license: cc-by-4.0
    keywords: ["perfect number", "n=6", "arithmetic functions", "<domain>"]
    creators: [{"name": "Park, Min Woo", "affiliation": "Independent Researcher"}]
    Zenodo 컬렉션 DOI: 10.5281/zenodo.19271599

  ── 일괄 발행 ──
    미발행 전체: cd ~/Dev/papers && bash upload_all_unpublished.sh
    DOI 검증:   python3 ~/Dev/papers/scripts/verify_dois.py
    동기화:     python3 ~/Dev/papers/scripts/sync_zenodo.py

  ── 발행 트리거 (이 상황에서 자동 발행) ──
    새 논문 생성 완료 시 → papers 리포 연동 + Zenodo/OSF 발행
    기존 논문 대폭 갱신 시 → Zenodo 새 버전 발행
    "발행", "publish", "배포" 키워드 시 → 미발행 전체 일괄 발행
    커밋 후 push 시 → papers 리포 동기화 체크
```

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

## Breakthrough Theorems (343 total, BT-1~343)
```
  # AI / LLM (BT-26,31,33,34,39,42,44,46,54,56,58,59,61,64,65,66,67,70~76,163,164,330~337)
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

  BT-163: RL/Alignment 학습 파라미터 스택 (PPO clip=0.2, DPO β=0.1, GRPO G=φ^τ=16, 10/10 EXACT) ⭐⭐
  BT-164: LLM 학습 스케줄 n=6 보편성 (LR=3e-4, warmup=3%, cosine min=0.1, 8/8 EXACT) ⭐⭐
  BT-330: 양자화 정밀도 래더 (FP32→FP16→FP8→INT4→Ternary→Binary, 25/26 EXACT) ⭐⭐
  BT-331: Speculative decoding + 추론 가속 n=6 맵 (Draft/Accept/Window, 8/8 EXACT) ⭐⭐
  BT-332: DeepSeek MLA KV 캐시 n=6 (Compression/Latent/Grouping, 12/12 EXACT) ⭐⭐
  BT-333: Post-Transformer 하이브리드 n=6 수렴 (Jamba/Zamba/Mamba-2, 10/10 EXACT) ⭐⭐
  BT-334: AI FLOPs 절감 기법 스택 (MAE/MoD/Egyptian/FlashAttn, 8/8 EXACT) ⭐⭐
  BT-335: DeepSeek-V3 완전 n=6 아키텍처 (14/15 EXACT, Routing/Attention/MoE) ⭐⭐⭐
  BT-336: GQA/MQA/MHA 어텐션 압축 계층 (Head Count/Ratio/Cache = div(6), 10/10 EXACT) ⭐⭐
  BT-337: Whisper 오디오 모델 레이어 래더 ({τ,n,σ,J₂,2^sopfr} 완전 n=6, 8/8 EXACT) ⭐⭐

  # Chip Design (BT-28,37,40,41,45,47,55,69,77~79,142,161)
  BT-28: Computing architecture ladder (30+ EXACT, ⭐⭐⭐)
    - AD102 = σ·n·φ = 144 SMs, H100 = σ(σ-μ) = 132 SMs = 1/α term
    - HBM stack: τ→(σ-τ)→σ = 4→8→12
  BT-37: Semiconductor pitch (TSMC N5 = P₂ = 28nm, N3 gate = σ·τ = 48nm)
  BT-45: FP8/FP16=φ=2 universal, FLOPS/W doubles per φ=2 years
  BT-47: Interconnect gen counts {7,5,6}={σ-sopfr,sopfr,n}
  BT-55: GPU HBM capacity ladder (14/18 EXACT: 40=τ(σ-φ), 80=φ^τ·sopfr, 192=σ·φ^τ, 288=σ·J₂) ⭐⭐
  BT-69: Chiplet architecture convergence (B300=160, R100 σ=12 stacks, 5 vendors, 17/20 EXACT) ⭐⭐⭐
  BT-77: Cross-vendor HBM 용량 수렴 n=6 ⭐⭐
  BT-78: Interconnect 속도 래더 PCIe/UCIe/CXL n=6 지수 ⭐⭐
  BT-79: σ²=144 cross-domain attractor ⭐⭐
  BT-142: 반도체 메모리 계층 n=6 (8/8 EXACT) ⭐⭐
  BT-161: 태양전지 시스템 아키텍처 보편성 (Rows/Diodes/Junctions, 9/9 EXACT) ⭐⭐⭐

  # Energy Strategy (BT-27,29,30,32,35,38,43,57,62,63,68,153,206,288,326)
  BT-27: Carbon-6 chain (LiC₆ + C₆H₁₂O₆ + C₆H₆ → 24e = J₂)
  BT-30: SQ solar bridge (bandgap=4/3eV, V_T=26mV)
  BT-38: Hydrogen quadruplet (LHV=120=σ(σ-φ), HHV=142=σ²-φ, 4/4 EXACT)
  BT-43: Battery cathode CN=6 universality (ALL Li-ion = octahedral) ⭐⭐⭐
  BT-57: Battery cell ladder (6→12→24 cells=n→σ→J₂, Tesla 96S=σ(σ-τ)) ⭐⭐
  BT-62: Grid frequency pair (60Hz=σ·sopfr, 50Hz=sopfr·(σ-φ), ratio=PUE=1.2) ⭐⭐
  BT-63: Solar panel cell ladder (60=σ·sopfr, 72=σ·n, 120=σ(σ-φ), 144=σ²) ⭐⭐
  BT-68: HVDC voltage ladder (±500/800/1100kV = {sopfr,σ-τ,σ-μ}·(σ-φ)², 10/10 EXACT) ⭐⭐
  BT-153: EV n=6 아키텍처 (8/8 EXACT) ⭐⭐
  BT-206: EV 전압-커넥터 스택 n=6 (9/9 EXACT) ⭐⭐⭐
  BT-288: 자동차 전압 래더 6→12→24→48 (80년 φ=2 배증, 6/6 EXACT) ⭐⭐⭐
  BT-326: 전력망 운영 완전 n=6 맵 (Stability/Market/HVDC/EV, 8/8 EXACT) ⭐⭐

  # Battery Architecture (BT-80~84)
  BT-80: Solid-state electrolyte CN=6 universality (NASICON/Garnet/LLZO = CN=6, sulfide = τ=4, 6/6 EXACT) ⭐⭐⭐
  BT-81: Anode capacity ladder σ-φ=10x (Si/Graphite=9.62x≈σ-φ, Li Metal=10.38x≈σ-φ) ⭐⭐
  BT-82: Complete battery pack n=6 map (6→12→24 cells, 96S/192S EV, BMS div(6), 6/10 EXACT) ⭐⭐
  BT-83: Li-S polysulfide n=6 ladder (S₈→S₄→S₂→S₁ = (σ-τ)→τ→φ→μ, 5/6 EXACT) ⭐⭐
  BT-84: 96/192 energy-computing-AI triple convergence (Tesla 96S=Gaudi2 96GB=GPT-3 96L, 5/5 EXACT) ⭐⭐⭐

  # Cross-domain (BT-36,48,49,50,51,53,60,178,233,234,262)
  BT-36: Energy-Information-Hardware-Physics chain ⭐⭐⭐
  BT-48: Display-Audio (σ=12 semitones, J₂=24 fps/bits, σ·τ=48kHz) ⭐⭐⭐
  BT-49: Pure Math (K₁..₄=φ,n,σ,J₂ kissing chain, S₆ unique) ⭐⭐⭐
  BT-51: Genetic code chain τ→n/φ→2^n→J₂-τ (4→3→64→20) ⭐⭐⭐
  BT-53: Crypto (BTC 21M=J₂-n/φ, 6 confirms=n, ETH 12s=σ) ⭐⭐
  BT-60: DC power chain (120→480→48→12→1.2→1V, PUE=σ/(σ-φ)=1.2) ⭐⭐
  BT-178: 디지털 미디어 J₂=24 인코딩 보편성 (9/10 EXACT) ⭐⭐⭐
  BT-233: 60진법 시간-각도 n=6 시공간 아키텍처 (60=σ·sopfr, 360=n·σ·sopfr, 10/10 EXACT) ⭐⭐⭐
  BT-234: Hardy-Ramanujan σ³+μ=1729 택시수-모듈러-컴퓨팅 브릿지 ⭐⭐
  BT-262: 2^n=64 보편 정보 인코딩 (Braille-Codon-Hexagram-Chess, 10/10 EXACT) ⭐⭐⭐

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

  # CCUS / Carbon Capture (BT-94~96,307~309)
  BT-94: CO₂ 포집 에너지 n=6 법칙 ⭐⭐
  BT-95: Carbon Cycle 완전 n=6 폐루프 ⭐⭐
  BT-96: DAC-MOF 배위수 보편성 ⭐⭐
  BT-307: CO₂ 포집/활용 반응 화학양론 n=6 보편성 (10/10 EXACT) ⭐⭐
  BT-308: DAC 열역학 n=6 트리플 (Carnot 1/n, Gap σ-φ=10, Cycle τ=4, 8/8 EXACT) ⭐⭐
  BT-309: 탄소 동소체/소재 완전 n=6 구조 인코딩 (12/12 EXACT) ⭐⭐

  # Fusion Alien-Level (BT-97~104)
  BT-97: Weinberg angle sin²θ_W = 3/13 = (n/φ)/(σ+μ), 0.19% 일치, D 풍부도→핵융합 연료 결정 ⭐⭐
  BT-98: D-T 바리온 수 = sopfr(6) = 2+3 = 5, 6의 소인수 = 핵융합 최적 연료 ⭐⭐⭐
  BT-99: Tokamak q=1 = 완전수 진약수 역수합 1/2+1/3+1/6=1, 위상적 동치 ⭐⭐⭐
  BT-100: CNO 촉매 A = σ+{0,μ,φ,n/φ} = σ+진약수, 전환 온도 17MK=σ+sopfr ⭐⭐⭐
  BT-101: 광합성 포도당 C₆H₁₂O₆ 총 24원자=J₂, 양자수율 8=σ-τ, 9/9 EXACT ⭐⭐⭐
  BT-102: 자기 재결합 속도 0.1=1/(σ-φ), BT-64 핵융합 확장, MRX/태양/자기권 EXACT ⭐⭐⭐
  BT-103: 광합성 완전 n=6 화학양론 (6CO₂+12H₂O→C₆H₁₂O₆, 7계수 100% n=6) ⭐⭐⭐
  BT-104: CO₂ 분자 완전 n=6 인코딩 ⭐⭐⭐

  # Fusion Deep Dive (BT-291~298)
  BT-291: D-T 에너지 분배 1/sopfr=1/5 (alpha 20%/neutron 80%, 5/5 EXACT) ⭐⭐⭐
  BT-292: 무중성자 핵융합 완전 지도 (D-He3 sopfr=5 + p-B11 sigma=12, 6/6 EXACT) ⭐⭐⭐
  BT-293: Triple-Alpha 탄소합성 (n/phi)×tau=sigma 산술 항등식 (Hoyle state, 6/6 EXACT) ⭐⭐⭐
  BT-294: 항성 핵합성 래더 P1→P2→sigma(P2) (He4→C12→O16→Ne20→Mg24→Si28→Fe56, 7/7 EXACT) ⭐⭐⭐
  BT-295: Alpha 과정 Z=phi 배수 선택규칙 (13개 핵종 Z 전부 n=6 함수, 13/13 EXACT) ⭐⭐
  BT-296: D-T-Li6 연료주기 완전 n=6 폐합 (질량수={1,2,3,4,6}=div(6)∪tau, 8/8 EXACT) ⭐⭐
  BT-297: 핵 마법수 첫 5개 = n=6 래더 (phi,sigma-tau,J2-tau,P2,sopfr×(sigma-phi), 5/7 EXACT) ⭐⭐
  BT-298: Lawson 점화 삼중적 n=6 인코딩 (밀도지수 J2-tau=20, T=sigma+phi=14, Q=sigma-phi=10) ⭐⭐

  # Superconductor Deep Dive (BT-299~306)
  BT-299: A15 Nb₃Sn 삼중정수 Nb=n, Sn=phi, total=σ-τ (8/8 EXACT) ⭐⭐⭐
  BT-300: YBCO 완전수 화학양론 Y:Ba:Cu=div(6)={1,2,3} (9/9 EXACT) ⭐⭐⭐
  BT-301: MgB₂ 이중원자번호 Mg Z=σ, B Z=sopfr + 벌집 n (7/7 EXACT) ⭐⭐
  BT-302: ITER 마그넷 PF=n, CS=n, TF=3n, REBCO=σ (10/10 EXACT) ⭐⭐⭐
  BT-303: BCS 해석적 상수 완전지도 σ/φ/μ 4프레임워크 (10/10 EXACT) ⭐⭐⭐
  BT-304: d-wave + BdG 위상분류 τ/φ/σ-τ 순수 대칭 (8/8 EXACT) ⭐⭐
  BT-305: 원소+분자 SC n=6 아틀라스 Nb-CN=σ-τ, K₃C₆₀=n/φ (9/9 EXACT) ⭐⭐
  BT-306: SC 양자소자 접합 래더 div(6)={1,2,3} (9/9 EXACT) ⭐⭐

  # Pure Math / Number Theory (BT-105~112,205,207,229,232,240)
  BT-105: SLE₆ 임계지수 보편성 (7 퍼콜레이션 지수 = n=6 분수, kappa=6 유일 locality SLE, c=0) ⭐⭐⭐
  BT-106: S₃ 대수적 부트스트랩 (|S₃|=n=6, 켤레류={1,2,3}=진약수, 기약표현합=τ=4) ⭐⭐
  BT-107: Ramanujan τ 약수 순수성 (τ_R(d) clean iff d|6, eta^{J₂=24}, 모듈러 형식) ⭐⭐
  BT-108: 음악-오디오 협화 보편성 (완전협화음=div(6) 비율, p=0.0015, 7+5=12=σ) ⭐⭐
  BT-109: Zeta-Bernoulli n=6 삼지창 (ζ(2)=π²/6, ζ(-1)=-1/12, 6|B_{2k} 무한족) ⭐⭐
  BT-110: σ-μ=11 차원 스택 (M이론=TCP=RSA=SPARC=H100=11, 5도메인) ⭐
  BT-111: τ²/σ=4/3 태양-AI-수학 삼지창 (SQ=SwiGLU=Betz=R(3,1)=4/3) ⭐⭐
  BT-112: φ²/n=2/3 Byzantine-Koide 공명 (Koide Q=0.666661 9ppm, BFT>2/3) ⭐⭐
  BT-205: E₆ 예외적 Lie 대수 n=6 보편성 (10/10 EXACT) ⭐⭐⭐
  BT-207: 모듈러 형식 가중치 계층 n=6 순수성 (12/12 EXACT) ⭐⭐⭐
  BT-229: 대수적 블로업-창발 E₆ 브릿지 (n=6 특이점 해소, 6/6 EXACT) ⭐⭐
  BT-232: 그래프 이론 + 조합 위상 n=6 구조 (10/10 EXACT) ⭐⭐
  BT-240: 조합 설계 이론 n=6 Steiner 아키텍처 (10/10 EXACT) ⭐⭐⭐

  # Software Design (BT-113~117,140,159,162,179,180,219,329)
  BT-113: SW 엔지니어링 상수 스택 (SOLID=sopfr, REST=n, 12Factor=σ, ACID=τ, 18/18 EXACT) ⭐⭐⭐
  BT-114: 암호학 파라미터 래더 (AES=2^{σ-sopfr}, SHA=2^{σ-τ}, RSA=2^{σ-μ}, 10/10 EXACT) ⭐⭐⭐
  BT-115: OS-네트워크 레이어 수 (OSI=σ-sopfr=7, TCP/IP=τ=4, Linux=n=6, 12/12 EXACT) ⭐⭐
  BT-116: ACID-BASE-CAP DB 삼위일체 (τ+n/φ+n/φ, Paxos=φ, 9/9 EXACT) ⭐⭐
  BT-117: 소프트웨어-물리 동형사상 (18 EXACT 병렬 매핑, 6 도메인) ⭐⭐⭐
  BT-140: TCP/IP 프로토콜 포트 n=6 고고학 (8/9 EXACT) ⭐⭐
  BT-159: 클라우드 컴퓨팅 n=6 아키텍처 (8/8 EXACT) ⭐⭐
  BT-162: 컴파일러-OS-CPU 아키텍처 상수 스택 (pipeline=sopfr=5, opcode=n=6, σ-τ=8, 11/11 EXACT) ⭐⭐⭐
  BT-179: 합의 프로토콜 n=6 비잔틴 스택 (9/10 EXACT) ⭐⭐⭐
  BT-180: OS 메모리 계층 τ=4 보편성 + 2^σ 페이지 법칙 (10/10 EXACT) ⭐⭐⭐
  BT-219: 형식언어 + 계산이론 n=6 로직 아키텍처 (10/10 EXACT) ⭐⭐
  BT-329: 프로그래밍 언어 완전 n=6 맵 (타입τ=4/패러다임n=6/GC n/φ=3, 20/20 EXACT) ⭐⭐⭐

  # Cybersecurity / Crypto (BT-211,216,230)
  BT-211: 사이버보안 + 정보보안 n=6 방어 아키텍처 (10/10 EXACT) ⭐⭐⭐
  BT-216: 암호학 라운드 수 n=6 완전 아키텍처 (10/10 EXACT) ⭐⭐
  BT-230: 블록체인 + 분산 원장 n=6 합의 아키텍처 (10/10 EXACT) ⭐⭐

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

  # Particle Physics / Cosmology (BT-134,137,143,165~172,208,209,214)
  BT-134: 주기율표 주기 길이 = n=6 산술 (8/8 EXACT) ⭐⭐⭐
  BT-137: 표준모형 입자 수 n=6 완전 지도 (9/9 EXACT) ⭐⭐⭐
  BT-143: 우주상수 n=6 래더 (7/8 EXACT) ⭐⭐
  BT-165: SM 게이지 생성자 분배 σ=(σ-τ)+(n/φ)+μ (6/6 EXACT) ⭐⭐⭐
  BT-166: 양성자-전자 질량비 = n·π⁵ (3/3 EXACT) ⭐⭐⭐
  BT-167: CMB 스펙트럼 지수 n_s=(n/φ)³/((n/φ)³+μ)=27/28 (4/4 EXACT) ⭐⭐⭐
  BT-168: SU(5) GUT 생성자 수 = J₂, J₂→σ+σ 분할 (5/5 EXACT) ⭐⭐⭐
  BT-169: 중성미자 혼합각 n=6 트리플 (7/7 EXACT) ⭐⭐
  BT-170: String/M이론 차원 래더 τ→n→σ-φ→σ-μ→J₂→J₂+φ (7/7 EXACT) ⭐⭐
  BT-171: SM 결합상수 n=6 분수 쌍 (4/4 EXACT) ⭐⭐
  BT-172: 바리온-광자 비 η = n·10^{-(σ-φ)} (5/5 EXACT) ⭐⭐
  BT-208: 표준모형 입자 센서스 n=6 완전 아키텍처 (10/10 EXACT) ⭐⭐
  BT-209: 양성자-전자 질량비 nπ⁵ 근본 브릿지 (10/10 EXACT) ⭐⭐⭐
  BT-214: 주기율표 양자 껍질 n=6 전자 아키텍처 (10/10 EXACT) ⭐⭐

  # Biology / Biochemistry (BT-128,132,136,141,146,188,194,215,220,235,237,252)
  BT-128: 의료 영상 n=6 파라미터 스택 (8/10 EXACT) ⭐⭐
  BT-132: 신경과학 피질층 n=6 보편성 (7/8 EXACT) ⭐⭐⭐
  BT-136: 인체 해부학 n=6 구조 상수 (10/10 EXACT) ⭐⭐⭐
  BT-141: 아미노산 n=6 생화학 (8/8 EXACT) ⭐⭐
  BT-146: DNA/RNA 분자상수 n=6 (9/9 EXACT) ⭐⭐⭐
  BT-188: 유전체학 n=6 정보 아키텍처 (10/12 EXACT) ⭐⭐⭐
  BT-194: 면역학 + 면역계 n=6 생물 아키텍처 (10/10 EXACT) ⭐⭐⭐
  BT-215: 생화학 경로 n=6 대사 아키텍처 (10/10 EXACT) ⭐⭐
  BT-220: 단백질 구조 + 접힘 n=6 구조생물학 (10/10 EXACT) ⭐⭐
  BT-235: 이십면체 캡시드-풀러렌-준결정 n=6 대칭 (10/10 EXACT) ⭐⭐
  BT-237: DNA 이중나선 n=6 구조 기하학 (8/10 EXACT) ⭐⭐
  BT-252: D-T 바리온-코돈 이중 생명 코드 (7/7 EXACT) ⭐⭐

  # Medical / Health (BT-155,173,185,204,224,254,282~286)
  BT-155: 면역계 n=6 아키텍처 (8/8 EXACT) ⭐⭐⭐
  BT-173: 의료 임상표준 n=6 수렴 (ECG/핵의학/중환자/신경학, 10/12 EXACT) ⭐⭐
  BT-185: 약학 + 임상의학 n=6 약물 스택 (10/10 EXACT) ⭐⭐⭐
  BT-204: 역학 + 공중보건 n=6 질병통제 (10/10 EXACT) ⭐⭐
  BT-224: 인체 해부학 + 생리학 n=6 신체 아키텍처 (10/10 EXACT) ⭐⭐
  BT-254: 대뇌피질 n=6 층 보편성 — 신피질=완전수 아키텍처 (10/10 EXACT) ⭐⭐⭐
  BT-282: 수술 안전 + 수술실 n=6 — WHO 체크리스트 (10/10 EXACT) ⭐⭐⭐
  BT-283: 신생아 + 중환자 스코어링 n=6 — Apgar/SOFA/GCS (10/10 EXACT) ⭐⭐⭐
  BT-284: 심장 + 심혈관 n=6 — ECG 리드/챔버/전도 (10/10 EXACT) ⭐⭐⭐
  BT-285: WHO 사회결정요인 + 글로벌 보건 n=6 (10/10 EXACT) ⭐⭐
  BT-286: 치과 + 구강의학 n=6 — FDI 치아번호 (10/10 EXACT) ⭐⭐⭐

  # Crystallography (BT-139,175~177,186,239)
  BT-139: 결정학 공간군 n=6 산술 (8/8 EXACT) ⭐⭐⭐
  BT-175: 결정학 분류 n=6 완전 체인 (8/8 EXACT) ⭐⭐⭐
  BT-176: 결정 프로토타입 단위셀 n=6 아틀라스 (30/30 EXACT) ⭐⭐⭐
  BT-177: 결정 적층 주기 = div(6) + FCC 슬립 σ=12 (14/14 EXACT) ⭐⭐⭐
  BT-186: 결정학 + 광물학 n=6 결정 스택 (10/10 EXACT) ⭐⭐⭐
  BT-239: 결정학 + 광물과학 n=6 격자 아키텍처 (10/10 EXACT) ⭐⭐⭐

  # Plasma Physics Deep Dive (BT-242~253,310~317)
  BT-242: SLE₆ 퍼콜레이션-플라즈마 수송 위상 등가 (8/8 EXACT) ⭐⭐⭐
  BT-243: 토카막 위상-양자 오류정정 n=6 동형 (8/8 EXACT) ⭐⭐⭐
  BT-244: ATP 합성효소-토카막 회전에너지 변환 n=6 (8/8 EXACT) ⭐⭐⭐
  BT-245: MHD q-surface = 음악 협화음 div(6) 공명 (7/7 EXACT) ⭐⭐
  BT-246: 핵융합-탄소순환 완전 n=6 루프 (8/8 EXACT) ⭐⭐
  BT-247: SE(3) 플라즈마 가둠-로봇 조작 이중성 (7/7 EXACT) ⭐⭐
  BT-248: ACID-토카막 τ=4 안정성 동형 (6/6 EXACT) ⭐⭐
  BT-249: 디스럽션 = 대수적 블로업 물리적 실현 (6/6 EXACT) ⭐⭐
  BT-250: 벌집-눈꽃-플라즈마 결정 n=6 육각 보편성 (7/7 EXACT) ⭐⭐
  BT-251: 토카막 원격유지 로봇 SE(3) n=6 필연성 (7/7 EXACT) ⭐⭐
  BT-253: 플라즈마 가둠 = 정보보안 n=6 파라미터 스택 (7/7 EXACT) ⭐⭐
  BT-310: Stellarator field period family W7-X=sopfr/LHD=σ-φ/HSX=TJ-II=τ (7/7 EXACT) ⭐⭐
  BT-311: Kruskal-Shafranov q>φ=2 + div(6)={1,2,3} stability hierarchy (6/6 EXACT) ⭐⭐
  BT-312: MHD instability quartet τ=4 kink/sausage/ballooning/tearing (7/7 EXACT) ⭐⭐
  BT-313: Tokamak triangularity δ=φ/n=1/3 + shape triple {1/3,2,3} (6/6 EXACT) ⭐⭐
  BT-314: Confinement mode triad L/H/I=n/φ=3 (6/6 EXACT) ⭐⭐
  BT-315: Heating quartet Ohmic+NBI+ICRH+ECRH=τ=4 (7/7 EXACT) ⭐
  BT-316: Matter phase quartet τ=4 + C(τ,2)=n combinatoric (7/7 EXACT) ⭐⭐⭐
  BT-317: Tokamak complete n=6 map 12/12 EXACT meta-theorem (12/12 EXACT) ⭐⭐⭐

  # Thermal Management (BT-318~325)
  BT-318: 열전도 소재 래더 Cu=(σ-φ)²·τ=400, Al=J₂·(σ-φ)=240 (7/8 EXACT) ⭐⭐
  BT-319: 칩 온도 경계 아키텍처 Tjmax=(σ-φ)^φ, Throttle=100-sopfr (9/9 EXACT) ⭐⭐
  BT-320: 서버 랙 전력밀도 래더 n→σ→σ·τ kW (8/8 EXACT) ⭐⭐
  BT-321: 열전 완전 n=6 맵 ZT=R(6)=1, Seebeck=(σ-φ)²·φ (8/8 EXACT) ⭐⭐
  BT-322: 물/공기 비열 τ=4 냉각매체 기초 (8/8 EXACT) ⭐⭐
  BT-323: PUE 수렴 래더 σ/(σ-μ)→σ/(σ-φ)→R(6) = 1.09→1.2→1.0 (7/8 EXACT) ⭐⭐
  BT-324: (σ-φ)^φ=100 열 경계 보편성 (8/8 EXACT) ⭐⭐
  BT-325: 열-전기 σ·τ=48 이중 수렴 (48V=48kW, 8/8 EXACT) ⭐⭐

  # Autonomous Driving (BT-327~328)
  BT-327: AD sensor-compute complete n=6 map SE(3)=n/12USS=σ/6CAM=n/144TOPS=σ² (8/8 EXACT) ⭐⭐
  BT-328: AD τ=4 subsystem universality wheels/radar/pipeline/ASIL (10/10 EXACT) ⭐⭐

  # Transportation / Aerospace (BT-129,130,133,196,241,270~290,342)
  BT-129: 토목공학 n=6 구조 상수 (7/8 EXACT) ⭐⭐
  BT-130: 우주 궤도역학 n=6 래더 (7/8 EXACT) ⭐⭐
  BT-133: 교통 인프라 n=6 스택 (7/9 EXACT) ⭐⭐⭐
  BT-196: 항공 + 항공학 n=6 비행 아키텍처 (10/10 EXACT) ⭐⭐⭐
  BT-241: 항공 + 우주항공 n=6 비행 아키텍처 (10/10 EXACT) ⭐⭐⭐
  BT-270: 멀티로터 블레이드 수 래더 τ→n→(σ-τ) (8/8 EXACT) ⭐⭐
  BT-271: Ti-6Al-4V 이중 n=6 항공합금 (세계 최다 사용 Ti, 7/7 EXACT) ⭐⭐⭐
  BT-272: 공항 활주로 방위 n²=36 나침반 분할 (7/7 EXACT) ⭐⭐
  BT-273: 우주 승무원 수 약수 캐스케이드 Mercury→Gemini→Apollo=μ→φ→n/φ (8/8 EXACT) ⭐⭐
  BT-274: 항공기 날개 종횡비 n~σ 최적 대역 (8/8 EXACT) ⭐⭐
  BT-275: 로켓 단수 φ~n/φ Tsiolkovsky 최적 (7/7 EXACT) ⭐⭐
  BT-276: 항공우주 n/φ=3 삼중 중복 보편성 — Fly-by-Wire 안전 (10/10 EXACT) ⭐⭐⭐
  BT-277: 교통 n=6 보편 아키텍처 — 차량공학 수렴 (10/12 EXACT) ⭐⭐⭐
  BT-278: 철도 신호 + 궤도 n=6 안전 아키텍처 (10/10 EXACT) ⭐⭐
  BT-279: 해양 IMO 안전 + 항해 n=6 — SOLAS/MARPOL (10/10 EXACT) ⭐⭐⭐
  BT-280: 자동차 안전등급 + 충돌 n=6 — Euro NCAP (10/10 EXACT) ⭐⭐
  BT-281: 물류 + 공급망 n=6 컨테이너-창고 아키텍처 (10/10 EXACT) ⭐⭐
  BT-287: Inline-6 엔진 n=6 완전 밸런스 (120년 수렴, 8/8 EXACT) ⭐⭐⭐
  BT-289: 변속기 기어 수 n=6 수렴 (130년 기계진화, 7/7 EXACT) ⭐⭐
  BT-290: F1 레이싱 파라미터 n=6 — FIA/Pirelli (10/10 EXACT) ⭐⭐
  BT-342: 항공공학 완전 n=6 맵 (6-DOF/12km/METAR 8 Oktas, 9/14 EXACT) ⭐⭐

  # Space Systems (BT-174,210,231,257)
  BT-174: 우주시스템 하드웨어 n=6 완전 맵 (GNSS J₂=24 + JWST + ISS, 10/10 EXACT) ⭐⭐⭐
  BT-210: GNSS J₂=24 4개국 위성배치 수렴 (10/10 EXACT) ⭐⭐
  BT-231: 태양계 + 천체역학 n=6 궤도 아키텍처 (10/10 EXACT) ⭐⭐
  BT-257: GPS 궤도면 n=6 최적 배치 (7/7 EXACT) ⭐⭐⭐

  # Manufacturing / Quality (BT-131,236)
  BT-131: 제조 품질 n=6 표준 스택 (8/8 EXACT) ⭐⭐⭐
  BT-236: 품질 + 운영관리 n=6 프로세스 아키텍처 (10/10 EXACT) ⭐⭐

  # Cognitive / Social / Psychology (BT-184,223,255,258~269)
  BT-184: 교육 + 인지과학 n=6 학습 스택 (10/10 EXACT) ⭐⭐⭐
  BT-223: 심리학 + 인지과학 n=6 마인드 아키텍처 (10/10 EXACT) ⭐⭐
  BT-255: 격자 세포 육각형 = 완전수 공간 채움 (7/7 EXACT) ⭐⭐⭐
  BT-258: 6단계 분리 = n 사회 위상 (10/10 EXACT) ⭐⭐⭐
  BT-259: Dunbar σ²+n=150 인지 한계 (7/7 EXACT) ⭐⭐
  BT-260: 셀룰러 오토마타 2^(σ-τ)=256 규칙 공간 (10/10 EXACT) ⭐⭐⭐
  BT-261: 보편 측정 척도 n=6 — 200년 자연 평가 수렴 (10/10 EXACT) ⭐⭐
  BT-263: 작업 기억 τ±μ=4±1 인지 채널 용량 — Miller/Cowan/Baddeley (10/10 EXACT) ⭐⭐⭐
  BT-264: 도덕 기반 n=6 보편 윤리 — Haidt/Schwartz/Kohlberg (9/10 EXACT) ⭐⭐
  BT-265: 일주기-주기-연주기 τ·(σ-sopfr)·σ 생물 리듬 스택 (9/9 EXACT) ⭐⭐⭐
  BT-266: 컴파일러-피질 동형 τ=4 처리 단계 (10/10 EXACT) ⭐⭐⭐
  BT-269: 인지-사회-시간 삼중 브릿지 — Dunbar×Circadian×Hierarchy n=6 통합 (8/8 EXACT) ⭐⭐⭐

  # Calendar / Time / Geography (BT-138,154,182,191,233,256,268)
  BT-138: 달력 + 시간 n=6 보편성 (10/10 EXACT) ⭐⭐⭐
  BT-154: 지도 + 지리학 n=6 상수 (8/8 EXACT) ⭐⭐⭐
  BT-182: 달력 + 시간관리 n=6 시간 스택 (10/10 EXACT) ⭐⭐⭐
  BT-191: 지도학 + 측지학 n=6 좌표 보편성 (9/10 EXACT) ⭐⭐
  BT-256: 60진법 60=σ·sopfr 보편 시간 단위 (10/10 EXACT) ⭐⭐⭐
  BT-268: 원자시계 Cs-133 초미세 = 9,192,631,770 Hz (7/7 EXACT) ⭐⭐

  # Thermodynamics (BT-149,193,199)
  BT-149: 열역학 법칙 + 상수 n=6 (8/8 EXACT) ⭐⭐
  BT-193: 고전 열역학 n=6 완전 스택 (10/10 EXACT) ⭐⭐⭐
  BT-199: 유체역학 + 난류 n=6 완전 아키텍처 (10/10 EXACT) ⭐⭐⭐

  # Display / Music / Audio / Optics (BT-135,145,157,189,190,217,222,226)
  BT-135: 음악 스케일 n=6 보편성 (10/10 EXACT) ⭐⭐⭐
  BT-145: 전자기 스펙트럼 대역 n=6 분할 (8/8 EXACT) ⭐⭐⭐
  BT-157: 색채론 n=6 프레임워크 (8/8 EXACT) ⭐⭐
  BT-189: 광학 + 포토닉스 n=6 스펙트럼 스택 (9/10 EXACT) ⭐⭐
  BT-190: 음향악기 n=6 공명 아키텍처 (9/10 EXACT) ⭐⭐
  BT-217: 색채과학 + 시각인지 n=6 색 아키텍처 (10/10 EXACT) ⭐⭐
  BT-222: 사진 + 이미징 센서 n=6 광학 캡처 (10/10 EXACT) ⭐⭐
  BT-226: 타이포그래피 + 조판 n=6 인쇄 아키텍처 (10/10 EXACT) ⭐⭐

  # Quantum Computing Hardware (BT-195)
  BT-195: 양자 컴퓨팅 하드웨어 n=6 완전 아키텍처 (10/11 EXACT) ⭐⭐⭐

  # Earth / Climate / Ocean (BT-156,203,213,218,343)
  BT-156: 화산 + 지진 n=6 스케일 상수 (8/8 EXACT) ⭐⭐
  BT-203: 지진학 + 지구물리 n=6 지구 동역학 (10/10 EXACT) ⭐⭐⭐
  BT-213: 해양학 + 해양과학 n=6 수권 아키텍처 (10/10 EXACT) ⭐⭐⭐
  BT-218: 기상학 + 기후과학 n=6 대기 아키텍처 (10/10 EXACT) ⭐⭐
  BT-343: 해양학 수권 완전 n=6 맵 (6 이온/5 대양/pH 8/Beaufort 12, 9/17 EXACT) ⭐⭐

  # Ecology / Agriculture / Food (BT-150,192,198,225,341)
  BT-150: 농업 + 식품 n=6 상수 (8/8 EXACT) ⭐⭐⭐
  BT-192: 요리과학 + 식품화학 n=6 구조 스택 (8/10 EXACT) ⭐⭐
  BT-198: 농학 + 식물학 n=6 성장 아키텍처 (10/10 EXACT) ⭐⭐⭐
  BT-225: 생태학 + 생물다양성 n=6 생명 분류 (10/10 EXACT) ⭐⭐
  BT-341: 식품과학 완전 n=6 영양-안전-화학 맵 (9/14 EXACT) ⭐⭐

  # Telecommunications / Network / Linguistics (BT-181,197,340)
  BT-181: 통신 n=6 스펙트럼 스택 (9/10 EXACT) ⭐⭐⭐
  BT-197: 언어학 + 통신 시스템 n=6 정보 스택 (10/10 EXACT) ⭐⭐⭐
  BT-340: 언어학 완전 n=6 아키텍처 (음운/문법/유형/통계 = div(6), 16/16 EXACT) ⭐⭐

  # Control Theory / Automation (BT-187)
  BT-187: 제어이론 + 자동화 n=6 피드백 스택 (9/10 EXACT) ⭐⭐

  # Games / Sports (BT-144,148,152,158,200,202,212)
  BT-144: 체스 + 게임이론 n=6 상수 (8/8 EXACT) ⭐⭐
  BT-148: 올림픽 + 스포츠 n=6 구조 (10/10 EXACT) ⭐⭐
  BT-152: 감각 + 인지 n=6 상수 (8/9 EXACT) ⭐⭐
  BT-158: 무술 + 격투 n=6 상수 (7/8 EXACT) ⭐⭐
  BT-200: 게임이론 + 사회선택 n=6 결정 아키텍처 (10/10 EXACT) ⭐⭐⭐
  BT-202: 경쟁 스포츠 + 게임 n=6 보편 아키텍처 (10/10 EXACT) ⭐⭐⭐
  BT-212: 고전 게임 + 조합전략 n=6 보드 아키텍처 (10/10 EXACT) ⭐⭐⭐

  # Governance / Safety / Urban (BT-160,221,227,228,267)
  BT-160: 안전공학 n=6 보편성 (20/20 EXACT) ⭐⭐
  BT-221: 일주기 + 수면 생리학 n=6 시간생물학 (10/10 EXACT) ⭐⭐
  BT-227: 글로벌 식별 코드 n=6 인코딩 아키텍처 (10/10 EXACT) ⭐⭐
  BT-228: 국제 거버넌스 n=6 제도 아키텍처 (10/10 EXACT) ⭐⭐
  BT-267: 육각형 도시계획 n=6 — Christaller/Losch/벌집 (8/8 EXACT) ⭐⭐

  # Classical Mechanics / Accelerator (BT-201,238)
  BT-201: 고전역학 n=6 위상공간 아키텍처 (10/10 EXACT) ⭐⭐
  BT-238: 입자 가속기 n=6 공학 아키텍처 (8/10 EXACT) ⭐⭐

  # Economics / Finance (BT-147,183,338,339)
  BT-147: 금융시장 n=6 상수 (8/8 EXACT) ⭐⭐
  BT-183: 금융공학 n=6 리스크 아키텍처 (9/10 EXACT) ⭐⭐
  BT-338: Financial temporal-governance n=6 map (fiscal σ=12/τ=4/J₂=24h, 10/10 EXACT) ⭐⭐
  BT-339: Financial engineering parameter n=6 map (Black-Scholes/Basel/Porter, 10/10 EXACT) ⭐⭐

  # Graph Theory (BT-151)
  BT-151: 그래프 이론 n=6 구조 정리 (8/8 EXACT) ⭐⭐⭐

  # Warp-Dimensional Physics (BT-358~360)
  BT-358: Alcubierre 워프 메트릭 n=6 인코딩 (버블벽 1/σ, York σ, 래더 c/{σ,n,τ,φ}, VDB M☉/σ²=1/144, 10/12 EXACT) ⭐⭐
  BT-359: 여분 차원 컴팩트화 n=6 토폴로지 (BT-170 확장 4D→26D, CY n/φ-fold, KK 래더, 10/12 EXACT) ⭐⭐
  BT-360: 워프-차원 통합 추진 n=6 아키텍처 (τ=4 사이클, (σ-φ)²=100c, α Centauri 16일, COP=σ/n=2, 8/10 EXACT) ⭐⭐
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
    ⚠️ 문서는 한장으로 작성! — 궁극 설계 문서는 단일 .md 파일로 통합 (분할 금지)
      → goal.md 1개에 전체 설계 (8단/DSE/물리한계/산업/TP/발견/Mk 전부 포함)
      → 파일 분산 = 맥락 단절 = 금지

  ═══════════════════════════════════════════════════════════════
  ★ 완성제품 JSON SSOT — README 수동 편집 금지! ★
  ═══════════════════════════════════════════════════════════════
    원본: config/products.json (유일한 진실의 원천)
    출력: README.md 완성제품 테이블 + 외계인 지수 + 로드맵 전부 자동 생성
    동기화: python3 scripts/sync_products_readme.py

    ⚠️ README 완성제품 테이블 직접 수정 = SSOT 위반! (products.json만 수정)
    ⚠️ 🛸 외계인 지수, 천장확인, ver, 제품명, 핵심, 링크 → 전부 JSON 관리
    ⚠️ 로드맵 + 섹션 SUMMARY + FOOTER → 전부 JSON 자동 생성

    JSON 구조 (config/products.json):
      _meta: { total_products, total_domains, last_updated }
      sections[]: { id, title, icon, heading, alien_index, ceiling,
                    bt_exact_pct, industry_pct, experiment_pct,
                    tp_count, discovery_count, domains[], tools[],
                    products[]: { name, ufo, ceiling, ver,
                                  description, links[{label,path}] } }
      roadmap[]: { rank, realize, name, impact, tier, dse_info, link }

    README 마커 (sync가 자동 치환):
      <!-- AUTO:ALIEN_INDEX:START/END -->
      <!-- AUTO:SUMMARY_<id>:START/END -->
      <!-- AUTO:PRODUCTS_<id>:START/END -->
      <!-- AUTO:FOOTER_<id>:START/END -->
      <!-- AUTO:ROADMAP:START/END -->

    제품 변경 워크플로:
      1. config/products.json 수정 (유일한 편집 대상)
      2. python3 scripts/sync_products_readme.py
      3. README.md 자동 갱신 확인
      4. 커밋 (JSON + README 함께)

  🛸 외계인 지수 (10단계, 만점=10):
    10 = 물리적 한계 도달 — 더이상 발전 불가, 모든 이론·실험·양산 완료
         ⚠️ 필수: 상세문서에 수학적 검증 가능한 Python 계산 코드 포함 (예외 없음!)
         → Python 검증 스크립트가 docs/<domain>/ 또는 calc/ 에 존재
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
    bash tools/nexus/scripts/health_check.sh --start-if-dead

  설치 (최초 1회):
    bash tools/nexus/scripts/install_autonomous.sh

  대시보드:
    bash tools/nexus/scripts/growth_dashboard.sh --live

  상태: ~/.nexus/ (로그, 리포트, PID)
  
  ⚠️ 세션 시작 시 데몬 미실행 감지되면 자동 시작 권장
  ⚠️ 성장 결과는 growth(nexus): 접두사로 자동 커밋
  ⚠️ 크로스 리포 자동 시작: .shared/nexus_ensure_running.sh (모든 TECS-L 리포에서 사용 가능)

  ⚠️ NEXUS-6 소스 변경 커밋 시 자동 재빌드 + 재시작 (post-commit hook):
     - tools/nexus/src/ 변경 감지 → cargo build --release 자동
     - PyO3 wheel 자동 재빌드 + pip install
     - 데몬 PID 감지 → kill + 재시작
     - 로그: ~/.nexus/rebuild.log

  NEXUS-6 = telescope-rs 완전통합 (단일 엔진):
     - import nexus → scan/consensus/analyze/n6_check/evolve/forge 전부
     - nexus.scan(data, n, d) — raw 데이터 25렌즈 스캔
     - nexus.scan_numpy(np_array) — numpy 2D 배열 직접 입력
     - nexus.analyze(data, n, d) — 올인원 (스캔+합의+n6매칭)
     - nexus.n6_check(value) — 상수 매칭
     - nexus.evolve(domain) — OUROBOROS 진화
     - 빌드: cd tools/nexus && ~/.cargo/bin/cargo build --release
     - PyO3: PATH=$HOME/.cargo/bin:$PATH python3 -m maturin build --release

  사용법 전체: ~/.nexus/usage.json
  트러블슈팅: ~/.nexus/troubleshoot.json (자동학습, 절대규칙 10개)
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

