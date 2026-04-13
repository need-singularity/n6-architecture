# telescope — ⛔ 레거시 렌즈 시스템 (폐기 중)

> 축: **nexus** (n6-architecture 내부)
> 상위: ../CLAUDE.md

## ⛔ 경고 — 신규 렌즈 추가 금지

이 디렉토리의 Rust 렌즈 (.rs, 312+ 파일) 는 **레거시 파생본**이며, R14 SSOT 원칙 위반 상태로 **폐기 예정**이다.

**HEAD 커밋 `0c23ad27`**: "refactor(telescope): 56개 렌즈 Rust→HEXA 전환 완료 — mod.rs 등록 해제"
→ Rust → HEXA 네이티브 단일화 진행 중.

## ✅ 진짜 렌즈 SSOT (별도 nexus 프로젝트)

```
/Users/ghost/Dev/nexus/shared/lenses/           → 개별 렌즈 84 .hexa (도메인별 플랫)
/Users/ghost/Dev/nexus/shared/blowup/lens/      → 카테고리 번들 15 .hexa
  ├── lenses_core.hexa        37 코어 렌즈
  ├── lenses_math.hexa
  ├── lenses_physics.hexa
  ├── lenses_quantum.hexa
  ├── lenses_ai_ml.hexa
  ├── lenses_graph_network.hexa
  ├── lenses_signal_info.hexa
  ├── lenses_applied.hexa
  └── lenses_constants.hexa
```

**새 렌즈 추가**: `/Users/ghost/Dev/nexus/shared/lenses/{도메인}_{주제}.hexa` 패턴 (HEXA 네이티브, `SIGMA=12.0 PHI=2.0 N=6.0 TAU=4.0 J2=24` 헤더 + `σ·φ = n·τ = J₂` 항등식 기반 공명 점수).

## ❌ 이 폴더에서 하지 말 것

- 신규 `.rs` 렌즈 파일 추가
- `frontier_lenses.rs`, `registry.rs` 에 `expansion_N_lens_entries()` 추가
- `lens-agent` 를 이 경로 대상으로 사용
- `n6shared/config/lens_registry.json` 에 신규 Rust 렌즈 등록

## 현재 상태 (2026-04-11)

- `cargo test` 2485 PASS (레거시 Rust 렌즈 테스트만 — HEAD 0c23ad27 이전 2593 에서 −108)
- `n6shared/config/lens_registry.json` `current_count: 603` — 레거시 카운트일 뿐, 진짜 SSOT 와 무관
- 4차 확장 (500→600) 결과는 레거시 경로에 저장됨 — 다음 세션에서 HEXA 포팅 시 흡수 대상

## 이관 계획 (다음 세션)

1. **Rust 렌즈 → HEXA 포팅** — 312 .rs → `/Users/ghost/Dev/nexus/shared/lenses/` 개별 파일
2. **레거시 삭제** — 이 폴더 Rust 파일 전량 제거
3. **`lens_registry.json` 재구축** — 진짜 SSOT 경로 기준

## 절대규칙

- 한글 필수
- R14 SSOT: 진짜 렌즈 SSOT = `/Users/ghost/Dev/nexus/shared/lenses/`
- R28 자동 흡수: 렌즈 결과 → atlas.n6

## 관련 링크

- 루트: /Users/ghost/Dev/n6-architecture/CLAUDE.md (렌즈 SSOT 섹션)
- 진짜 SSOT: /Users/ghost/Dev/nexus/shared/lenses/
- 흡수 계획: reports/audits/lens-rust-hexa-transition-status-2026-04-11.md (예정)
