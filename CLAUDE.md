# n6-architecture — AI-native Arithmetic Design Framework

commands: n6shared/config/commands.json — autonomous 블록으로 Claude Code가 작업 중 smash/free/todo/go/keep 자율 판단·실행
rules: n6shared/rules/common.json (R0~R27) + n6shared/rules/n6-architecture.json (N61~N65)
L0 Guard: `hexa $NEXUS/shared/harness/l0_guard.hexa <verify|sync|merge|status>`
loop: 글로벌 `~/.claude/skills/loop` + 엔진 `$NEXUS/shared/harness/loop` — roadmap `$NEXUS/shared/roadmaps/n6-architecture.json` 3-track×phase×gate 자동

atlas.n6 — 현실지도 SSOT:
  경로: $NEXUS/shared/n6/atlas.n6 (단일 파일, 60K+ 줄)
  구 구조 폐기: reality_map_live.json / L6_n6atlas.json / 별도 level 파일 없음. 전부 atlas.n6 흡수
  포맷: `@R {id} = {measured} {unit} :: n6atlas [등급]`
  등급: [10*]=EXACT검증 [10]=EXACT [9]=NEAR [7]=EMPIRICAL(승격대상) [5~8]=중간 [N?]=CONJECTURE [N!]=breakthrough
  승격: [7]→[10*] = atlas.n6 직접 편집 (새 파일 만들지 말 것)
  핵심 정리: σ(n)·φ(n) = n·τ(n) iff n=6 (n>=2). 3개 독립 증명

9축 네비게이션:
  theory/      영구 이론층
  domains/     295 도메인
  bridge/      Rust 통합 워크스페이스 (nexus 허브 리포 브릿지)
  techniques/  AI 기법 66종 (.hexa)
  experiments/ 검증 실험 122종 (.hexa)
  engine/      훈련/수학 런타임 (.hexa)
  papers/      논문 39편
  reports/     시점 리포트
  n6shared/    SSOT

NEXUS-6 CLI: nexus {scan|verify|calc|dse|analyze|hexa|dashboard} <args>

ref:
  rules     n6shared/rules/common.json                R0~R27
  project   n6shared/rules/n6-architecture.json       N61~N65
  lock      n6shared/rules/lockdown.json              L0/L1/L2
  cdo       n6shared/rules/convergence_ops.json       CDO 수렴
  registry  n6shared/config/projects.json             7프로젝트
  cfg       n6shared/config/project_config.json
  core      n6shared/config/core.json
  conv      n6shared/convergence/n6-architecture.json
  api       n6shared/CLAUDE.md
