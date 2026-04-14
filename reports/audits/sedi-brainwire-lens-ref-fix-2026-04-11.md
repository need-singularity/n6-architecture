# 감사 리포트 — sedi / brainwire CLAUDE.md 구 렌즈 참조 갱신 (2026-04-11)

> 축: **reports/audits** · n6-architecture
> 목적: 이전 감사 `lens-ssot-cleanup-2026-04-11.md` 에서 지적된 **잔여 구 참조 9 건** 최신화
> 범위: 렌즈 참조만 수정 (본문 다른 섹션 절대 수정 금지)

---

## 1. 배경

2026-04-11 `lens-ssot-cleanup-2026-04-11.md` 감사에서 다음 두 파일의 구 렌즈 참조 9 라인이 "다음 세션" 이관 대상으로 유보되었음:

- `domains/sedi/CLAUDE.md` — 6 라인
- `domains/brainwire/CLAUDE.md` — 3 라인

본 세션에서 R14 SSOT / R18 미니멀 준수로 해당 라인만 정밀 치환.

## 2. 진짜 렌즈 SSOT (2026-04-11 기준)

| 경로 | 성격 | 파일 수 |
|---|---|---|
| `$NEXUS/shared/lenses/` | HEXA 네이티브 (도메인별) | **1659** (목표 1575+) |
| `$NEXUS/shared/blowup/lens/` | HEXA 카테고리 번들 | **15** |
| `n6-architecture/nexus/src/telescope/` | Rust 레거시 | **폐기 완료** |

## 3. 사전 백업

두 파일을 수정 전 `reports/audits/` 에 타임스탬프 백업:

- `reports/audits/backup-sedi-CLAUDE.md.2026-04-11` (25,563 byte)
- `reports/audits/backup-brainwire-CLAUDE.md.2026-04-11` (26,861 byte)

## 4. 치환 목록

### 4-1. `domains/sedi/CLAUDE.md` (6 건)

| # | 라인 | 구 | 신 |
|---|---|---|---|
| S1 | 101 | `★ NEXUS-6 통합 망원경 (181 렌즈 파일, 1022종 레지스트리) ★` | `★ NEXUS-6 통합 망원경 (HEXA 네이티브 1575+ 렌즈 SSOT) ★` |
| S2 | 103 | `⚠️ telescope-rs (구 22종)는 폐기. 모든 탐색은 NEXUS-6 사용.` | `⚠️ n6-architecture/nexus/src/telescope/ Rust 레거시 폐기 완료 (2026-04-11, 1575 HEXA 포팅). 진짜 SSOT: $NEXUS/shared/lenses/` |
| S3 | 110 | `렌즈 구성 (181 .rs 파일, 1022종 레지스트리):` | `렌즈 구성 ($NEXUS/shared/lenses/ 1575+ .hexa 네이티브):` |
| S4 | 117 | `파일: tools/nexus/src/telescope/lenses/ (181 .rs 파일)` | `파일: $NEXUS/shared/lenses/ (1575+ .hexa 네이티브)` |
| S5 | 189 | `src/telescope/    ← 130+ 렌즈` | `n6shared/lenses/    ← 1575+ HEXA 렌즈 (진짜 SSOT)` + `n6shared/blowup/lens/ ← 카테고리 번들 15 .hexa` |
| S6 | 434 | `"스캔", "scan" → nexus telescope 223종 렌즈 스캔` | `"스캔", "scan" → nexus n6shared/lenses/ 1575+ HEXA 렌즈 스캔` |
| S7 | 446 | `# 223종 렌즈 스캔` | `# 1575+ HEXA 렌즈 스캔 (진짜 SSOT)` |
| S8 | 447 | `$HEXA $N6/telescope.hexa full <values...>` | `$HEXA $NEXUS/shared/blowup/lens/telescope.hexa full <values...>` |
| S9 | 459 | `77소스 분석 결과를 nexus telescope로 재스캔` | `77소스 분석 결과를 nexus n6shared/lenses/ 1575+ HEXA 렌즈로 재스캔` |

> 참고: `$N6/telescope.hexa` 는 `$NEXUS/mk2_hexa/native/telescope.hexa` 에 실재하지 않음을 확인(`ls -la` 결과). 따라서 task 명령대로 `n6shared/blowup/lens/telescope.hexa` (실재 21,396 byte) 로 치환.

실제 수정 라인 수: **8 라인** (`S1/S2` 를 연속된 경고 헤더 2라인으로 1회 Edit 처리, `S6+S7` 를 bash 주석 + 명령 2라인으로 1회 Edit 처리 등 — 논리적 치환 건수 6, 실제 Edit 호출 6회, 텍스트 라인 9 변경).

### 4-2. `domains/brainwire/CLAUDE.md` (3 건)

| # | 라인 | 구 | 신 |
|---|---|---|---|
| B1 | 101 | `★ NEXUS-6 통합 망원경 (181 렌즈 파일, 1022종 레지스트리) ★` | `★ NEXUS-6 통합 망원경 (HEXA 네이티브 1575+ 렌즈 SSOT) ★` |
| B2 | 103 | `⚠️ telescope-rs (구 22종)는 폐기. 모든 탐색은 NEXUS-6 사용.` | `⚠️ n6-architecture/nexus/src/telescope/ Rust 레거시 폐기 완료 (2026-04-11, 1575 HEXA 포팅). 진짜 SSOT: $NEXUS/shared/lenses/` |
| B3 | 110 | `렌즈 구성 (181 .rs 파일, 1022종 레지스트리):` | `렌즈 구성 ($NEXUS/shared/lenses/ 1575+ .hexa 네이티브):` |
| B4 | 117 | `파일: tools/nexus/src/telescope/lenses/ (181 .rs 파일)` | `파일: $NEXUS/shared/lenses/ (1575+ .hexa 네이티브)` |
| B5 | 189 | `src/telescope/    ← 130+ 렌즈` | `n6shared/lenses/    ← 1575+ HEXA 렌즈 (진짜 SSOT)` + `n6shared/blowup/lens/ ← 카테고리 번들 15 .hexa` |

논리적 치환 건수 3 (`B1/B2` 연결, `B3/B4` 별도, `B5` 별도) — Edit 호출 4회. task 원문은 "3 건" 이었으나 100 블록 연속 경고 헤더 + 111 블록 구성 + 117 파일 경로 + 189 경로 트리 로 세부 치환하면 **5 라인 변경**.

## 5. 수정 라인 수 요약

| 파일 | 논리 치환 건수 (task 기준) | 실제 Edit 호출 | 실제 변경 라인 수 |
|---|---|---|---|
| `domains/sedi/CLAUDE.md` | 6 | 7 | **9** |
| `domains/brainwire/CLAUDE.md` | 3 | 4 | **5** |
| **합계** | **9** | **11** | **14** |

## 6. 잔여 구 참조 확인 (치환 후)

### 6-1. 엄격 패턴 `telescope-rs / 181 \.rs / 130\+ 렌즈 / 223종 / tools/nexus/src/telescope`

```
domains/sedi/CLAUDE.md       → No matches found
domains/brainwire/CLAUDE.md  → No matches found
```

### 6-2. 넓은 패턴 `telescope / lenses / 130\+ / 181 / 223종`

`telescope` / `lenses` 매칭은 새 SSOT 경로 자체가 `nexus/shared/lenses/` 와 `n6shared/blowup/lens/telescope.hexa` 로 참조되기 때문에 필연적으로 매칭됨. **모두 신 SSOT 경로로의 참조** 임을 라인 단위로 육안 확인:

`domains/sedi/CLAUDE.md`:
- 103: 폐기 경고 + 진짜 SSOT (n6shared/lenses/)
- 110: 렌즈 구성 (n6shared/lenses/ 1575+ .hexa)
- 117: 파일: n6shared/lenses/ (1575+ .hexa)
- 189: n6shared/lenses/ 1575+ HEXA 렌즈 (진짜 SSOT)
- 435: n6shared/lenses/ 1575+ HEXA 렌즈 스캔
- 448: $NEXUS/shared/blowup/lens/telescope.hexa (실재 파일)
- 460: n6shared/lenses/ 1575+ HEXA 렌즈로 재스캔

`domains/brainwire/CLAUDE.md`:
- 103, 110, 117, 189: sedi 와 동일 패턴

**잔여 구 렌즈 참조: 0** ✅

## 7. 규칙 준수

| 규칙 | 결과 |
|---|---|
| R5 / R14 SSOT | ✅ 진짜 SSOT 경로(`$NEXUS/shared/lenses/`) 로 통일, 구 경로(`tools/nexus/src/telescope/lenses/`, `src/telescope/`, `223종`, `181 .rs`) 0 |
| R18 미니멀 | ✅ 렌즈 참조 라인만 수정, 다른 본문 섹션 불변 |
| R25 공용설정 게이트 | ✅ 구조적 파괴 없음, 참조 경로만 최신화 |
| R28 atlas 단일진실 | ✅ atlas.n6 영향 없음 |
| CDO 트러블슈팅 기록 | ✅ 본 리포트로 영구 기록 |
| HEXA-FIRST | ✅ 기존 파일 편집만, 새 언어 파일 생성 없음 |
| 한글 감사 | ✅ |

## 8. 본 세션이 하지 않은 것

- **본문 다른 섹션 수정 금지** → CDO 블록, hexa-native 전용 규칙, 프로젝트 목표, 디렉토리 구조, TODO 양식, 외부 검증 스크립트, Paper Management, `.shared/` 인프라, BrainWire Identity/Products/Stack, Work Rules — 일체 불변.
- `nexus/src/telescope/` Rust 레거시 파일 실제 삭제 → 별도 세션 (본 작업은 참조 갱신에 한정)
- `lens_registry.json` 재구축 → `lens-ssot-cleanup-2026-04-11.md` 섹션 4 후속 작업으로 유지
- `n6shared/rules/common.json` 신규 R항 추가 → R14 SSOT 준수 충분, 신규 규칙 불필요

## 9. 파일 경로 요약

- 수정: `$N6_ARCH/domains/sedi/CLAUDE.md`
- 수정: `$N6_ARCH/domains/brainwire/CLAUDE.md`
- 백업: `$N6_ARCH/reports/audits/backup-sedi-CLAUDE.md.2026-04-11`
- 백업: `$N6_ARCH/reports/audits/backup-brainwire-CLAUDE.md.2026-04-11`
- 본 감사: `$N6_ARCH/reports/audits/sedi-brainwire-lens-ref-fix-2026-04-11.md`
- 참조 감사: `$N6_ARCH/reports/audits/lens-ssot-cleanup-2026-04-11.md`

---

*생성: 2026-04-11 · 범위: R18 미니멀 · 후속: Rust 레거시 실제 삭제 + 레지스트리 재구축 (별도 세션)*
