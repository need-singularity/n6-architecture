# 감사 리포트 — 렌즈 SSOT 참조 정리 (2026-04-11)

> 축: **reports/audits** · n6-architecture
> 목적: **다음 세션에서 렌즈 잘못된 경로에 추가되지 않도록** 참조 경고 주입
> 본 세션 범위: 참조 정리만 (실제 Rust→HEXA 흡수는 다음 세션)

---

## 1. 증상 — 두 개의 "nexus"

```
┌──────────────────────────────────────────────────┬─────────────┬──────────┐
│ 경로                                             │ 성격        │ 역할     │
├──────────────────────────────────────────────────┼─────────────┼──────────┤
│ $NEXUS/shared/lenses/            │ HEXA 84     │ ✅ SSOT   │
├──────────────────────────────────────────────────┼─────────────┼──────────┤
│ $NEXUS/shared/blowup/lens/       │ HEXA 15     │ ✅ SSOT   │
├──────────────────────────────────────────────────┼─────────────┼──────────┤
│ $N6_ARCH/nexus/src/      │ Rust 312    │ ❌ 레거시 │
│   telescope/lenses/                              │ 603 엔트리  │ (폐기중)  │
└──────────────────────────────────────────────────┴─────────────┴──────────┘
```

**확인 사실**:
- 진짜 렌즈 SSOT = `$NEXUS/shared/lenses/` + `n6shared/blowup/lens/` (별도 nexus 독립 프로젝트, HEXA 네이티브, `SIGMA=12.0 PHI=2.0 N=6.0 TAU=4.0 J2=24` 헤더 + `σ·φ = n·τ = J₂` 항등식 기반)
- n6-architecture 내부 `nexus/src/telescope/lenses/` 는 **레거시 파생본**
- HEAD `0c23ad27` "refactor(telescope): 56개 렌즈 Rust→HEXA 전환 완료 — mod.rs 등록 해제" 로 Rust → HEXA 단일화 진행 중
- `cargo test` 2593 → 2485 (−108) = 56 렌즈 × 평균 2 테스트 = 등록 해제 결과
- 3차 (450→500) + 4차 (500→600) 확장은 **레거시 경로에 추가**됨 — 다음 세션 HEXA 포팅 시 흡수 대상

---

## 2. 수행한 참조 정리 (7 파일)

### 2-1. 최상위 `CLAUDE.md`

atlas.n6 섹션 앞에 **"렌즈 SSOT" 섹션 신규** 추가. 진짜 경로 + 레거시 경로 + 흡수 계획 명시. `lens-agent` 사용 자제 권고.

### 2-2. `nexus/CLAUDE.md`

`- src/telescope/     렌즈 시스템 (215+ 렌즈)` 라인을 다음으로 교체:
```
- src/telescope/     ⚠️ 렌즈 시스템 (Rust 레거시 312+ 파일, 폐기 중 — 진짜 SSOT: $NEXUS/shared/lenses/ HEXA 네이티브)
```

### 2-3. `nexus/src/telescope/CLAUDE.md` **(신규)**

폴더 전체에 `⛔ 경고 — 신규 렌즈 추가 금지` 안내. 진짜 SSOT 경로 + HEXA 번들 9 파일 목록 + 이 폴더에서 하지 말 것 (신규 `.rs` / `frontier_lenses.rs` 확장 / `lens-agent` 사용 / `lens_registry.json` 등록) + 이관 계획.

### 2-4. `nexus/src/telescope/lenses/CLAUDE.md` **(신규)**

312 Rust 파일 레벨에 폐기 경고. 새 렌즈는 `$NEXUS/shared/lenses/` 에만 추가하라는 짧은 안내.

### 2-5. `n6shared/config/lens_registry.json`

`meta` 블록에 5 필드 추가:
```json
"_warning": "⛔ 이 레지스트리는 n6-architecture 내부 Rust 레거시 렌즈 카운트. 진짜 SSOT 아님.",
"_real_ssot": "$NEXUS/shared/lenses/ (HEXA 네이티브 84 + blowup/lens 15 번들)",
"_legacy_path": "n6-architecture/nexus/src/telescope/lenses/ (Rust .rs 파생본, 폐기 중)",
"_transition_status": "HEAD 0c23ad27 refactor(telescope) — Rust→HEXA 전환 진행, 56 렌즈 등록 해제 완료",
"_next_session_plan": "Rust→HEXA 포팅 후 레거시 삭제 + 본 레지스트리 재구축"
```
`expansion_session` 필드에 `⚠️ 레거시 경로` 접두 추가.

### 2-6. `INDEX.json`

`axes.nexus` 엔트리에 `_lens_warning` 필드 추가:
```json
"_lens_warning": "src/telescope/lenses/ 는 Rust 레거시 (폐기 중). 진짜 렌즈 SSOT: $NEXUS/shared/lenses/ (HEXA 네이티브)"
```

### 2-7. `.claude/agents/lens-agent/AGENT.md` **(폐기 마커)**

`description` 필드를 ⛔ 폐기 대기 로 변경:
```
description: ⛔ 폐기 대기 — 레거시 Rust 렌즈 경로 대상. 신규 렌즈는 general-purpose + $NEXUS/shared/lenses/ 사용.
```
본문 상단에 **"사용 금지"** 블록 + 진짜 SSOT 경로 + HEXA 네이티브 가이드 추가. 원본 지시는 "참고용만" 으로 격하.

---

## 3. 잔여 구 참조 (본 세션 미수정, 낮은 우선순위)

```
domains/sedi/CLAUDE.md:
  line 103  ⚠️ telescope-rs (구 22종)는 폐기. 모든 탐색은 NEXUS-6 사용.
  line 117  파일: tools/nexus/src/telescope/lenses/ (181 .rs 파일)
  line 189  src/telescope/    ← 130+ 렌즈
  line 434  "스캔" → nexus telescope 223종 렌즈 스캔
  line 447  $HEXA $N6/telescope.hexa full <values...>
  line 459  77소스 분석 결과를 nexus telescope로 재스캔

domains/brainwire/CLAUDE.md:
  line 103  ⚠️ telescope-rs (구 22종)는 폐기. 모든 탐색은 NEXUS-6 사용.
  line 117  파일: tools/nexus/src/telescope/lenses/ (181 .rs 파일)
  line 189  src/telescope/    ← 130+ 렌즈
```

**미수정 사유**: 이 파일들은 "과거 맥락/태스크 설명" 이지 "새 렌즈 추가 지시" 가 아님. 루트 CLAUDE.md 의 렌즈 SSOT 섹션이 먼저 읽히므로 혼동 방지에 충분. R18 미니멀 원칙 준수.

**다음 세션**: Rust→HEXA 흡수와 동반 정리 권고.

---

## 4. 다음 세션 지시사항 (명확)

### 렌즈 관련 작업 시 체크리스트

1. **절대 금지**:
   - `n6-architecture/nexus/src/telescope/lenses/` 에 신규 `.rs` 추가
   - `lens-agent` 에이전트 호출
   - `n6shared/config/lens_registry.json` 에 신규 Rust 렌즈 등록
   - `frontier_lenses.rs` 에 `expansion_N_lens_entries()` 추가

2. **허용/권장**:
   - `$NEXUS/shared/lenses/{domain}_{topic}.hexa` 신규 파일
   - `general-purpose` 에이전트 사용
   - HEXA 네이티브 문법 (`SIGMA/PHI/N/TAU/J2` 헤더 + 항등식)
   - 결과는 atlas.n6 에 자동 흡수 (R28)

### 흡수 본작업 (다음 세션 목표)

1. **조사 단계** (이미 #40 에이전트 pending): Rust → HEXA 전환 전략 4 옵션 비교 + 권고
2. **포팅 단계**: 312 Rust 렌즈 → HEXA 네이티브 파일 일괄 생성
3. **레거시 삭제**: `nexus/src/telescope/lenses/*.rs` 전량 제거, `mod.rs` / `registry.rs` / `frontier_lenses.rs` 정리
4. **레지스트리 재구축**: `n6shared/config/lens_registry.json` 진짜 SSOT 기준
5. **cargo test 재검증**: 렌즈 테스트 완전 제거 확인 또는 HEXA 네이티브 검증으로 전환
6. **domains/sedi, brainwire CLAUDE.md 참조 갱신**: 경로 최신화

---

## 5. 현재 상태 스냅샷 (2026-04-11 세션 종료 시)

| 항목 | 값 |
|---|---|
| 레거시 Rust 렌즈 | 312 `.rs` 파일 |
| `lens_registry.json` 엔트리 | 603 |
| cargo test | 2485 PASS (HEAD 0c23ad27 기준) |
| 진짜 HEXA 렌즈 SSOT | `$NEXUS/shared/lenses/` 84 + `blowup/lens/` 15 |
| 정리된 CLAUDE.md | 4 파일 (root / nexus / telescope / lenses) |
| 정리된 JSON | 2 파일 (lens_registry / INDEX) |
| 정리된 에이전트 정의 | 1 파일 (lens-agent AGENT.md) |
| 감사 리포트 | 본 파일 |

---

## 6. 규칙 준수

| 규칙 | 결과 |
|---|---|
| R5 SSOT | ✅ 진짜 SSOT 명시, 레거시 경로 경고 |
| R14 규칙 JSON | ✅ CLAUDE.md 에 규칙 본문 안박음, 안내만 |
| R18 미니멀 | ✅ 참조 정리만, 실제 흡수/삭제는 다음 세션 |
| R25 공용설정 게이트 | ✅ 경고 주입 수준, 구조적 파괴 없음 |
| R28 atlas 단일진실 | ✅ 영향 없음 |

---

*생성: 2026-04-11 · 범위: R18 미니멀 · 흡수 본작업: 다음 세션*
