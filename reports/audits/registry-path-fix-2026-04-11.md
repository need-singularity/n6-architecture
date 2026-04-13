# 레지스트리 경로 드리프트 수정 감사 리포트 (2026-04-11)

> 축: **reports/audits** · n6-architecture · 레지스트리 경로 드리프트 수정
> 규칙 기준: R2(하드코딩 금지) / R5(SSOT) / R14(shared JSON 단일진실) / R18(미니멀) / R25(공용설정 게이트)
> 이전 감사: `reports/audits/synbio-merge-2026-04-11.md` §3-1 / §3-2
> 범위: GO 모드 + 승인된 경로 드리프트 교정 (R18 미니멀)

---

## 1. 배경 — 이전 감사에서 확인된 스테일 포인터

`synbio-merge-2026-04-11.md` §3-1/§3-2 에서 다음 스테일 포인터가 R25 공용설정 게이트로 보류된 상태였음:

| 레지스트리 | 필드 | 스테일 경로 | 실체 |
|---|---|---|---|
| `papers/_registry.json` | sections[tech-industry].products[합성생물학].links[0].path | `docs/synbio/goal.md` | **없음** |
| `papers/_registry.json` | sections[tech-industry].products[합성생물학].links[1].path | `docs/paper/n6-synthetic-biology-paper.md` | **없음** |
| `papers/_registry.json` | `_meta.papers_chunk_c_2026-04-08.papers[9]` | `docs/paper/n6-synthetic-biology-paper.md` | **없음** |
| `nexus/shared/n6/docs/products.json` | sections[tech-industry].products[합성생물학].links[0].path | `docs/synbio/goal.md` | **없음** |
| `nexus/shared/n6/docs/products.json` | sections[tech-industry].products[합성생물학].links[1].path | `docs/paper/n6-synthetic-biology-paper.md` | **없음** |
| `nexus/shared/n6/docs/products.json` | `_meta.papers_chunk_c_2026-04-08.papers[9]` | `docs/paper/n6-synthetic-biology-paper.md` | **없음** |
| `nexus/shared/n6/n6_products.json` | sections[tech-industry].products[합성생물학].links[0].path | `docs/synbio/goal.md` | **없음** |

### 실체 파일 (절대경로 확인 완료)

```
/Users/ghost/Dev/n6-architecture/domains/life/synbio/synbio.md   (38640 B)  # 병합 canonical
/Users/ghost/Dev/n6-architecture/domains/life/synbio/goal.md     (13552 B)  # SSOT
/Users/ghost/Dev/n6-architecture/papers/n6-synthetic-biology-paper.md  (25702 B)  # 논문 본문
```

---

## 2. 백업 (수정 전)

```
reports/audits/_registry-backup-2026-04-11.json              (원본 157741 B)
reports/audits/products-backup-path-fix-2026-04-11.json      (원본 154751 B)
reports/audits/n6_products-backup-path-fix-2026-04-11.json   (원본  99430 B)
```

※ 기존 `reports/audits/products-backup-2026-04-11.json` 은 `products-drift-fix-2026-04-11.md` 의 구조적 드리프트 수정 건 백업이므로 본 건과 별개.

---

## 3. 적용된 수정

### 3-1. `papers/_registry.json`

**(a) 합성생물학 제품 엔트리 `links` 배열** (라인 3207 근방)

변경 전:
```json
"links": [
  { "label": "문서", "path": "docs/synbio/goal.md" },
  { "label": "논문", "path": "docs/paper/n6-synthetic-biology-paper.md" }
]
```

변경 후:
```json
"links": [
  { "label": "도메인", "path": "domains/life/synbio/synbio.md" },
  { "label": "goal",   "path": "domains/life/synbio/goal.md" },
  { "label": "논문",   "path": "papers/n6-synthetic-biology-paper.md" }
]
```

- 경로 `docs/synbio/goal.md` → `domains/life/synbio/goal.md` (label "문서"→"goal")
- 경로 `docs/paper/n6-synthetic-biology-paper.md` → `papers/n6-synthetic-biology-paper.md`
- 신규 링크 추가: `domains/life/synbio/synbio.md` (label "도메인", 병합된 canonical 본문 915 줄)

**(b) `_meta.papers_chunk_c_2026-04-08.papers[9]`** (라인 20)

변경 전:
```
"docs/paper/n6-synthetic-biology-paper.md"
```

변경 후:
```
"papers/n6-synthetic-biology-paper.md"
```

### 3-2. `/Users/ghost/Dev/nexus/shared/n6/docs/products.json`

**(a) 합성생물학 제품 엔트리 `links` 배열** (라인 3141 근방)

`_registry.json` 과 동일하게 3-1 (a) 형태로 수정. 신규 링크 3개.

**(b) `_meta.papers_chunk_c_2026-04-08.papers[9]`** (라인 20)

`_registry.json` 과 동일하게 3-1 (b) 형태로 수정.

### 3-3. `/Users/ghost/Dev/nexus/shared/n6/n6_products.json`

합성생물학 제품 엔트리 `links` 배열 (라인 2664 근방). 기존 단일 `"문서"` 링크 1개만 존재. 3개 링크 (도메인/goal/논문) 로 확장:

```json
"links": [
  { "label": "도메인", "path": "domains/life/synbio/synbio.md" },
  { "label": "goal",   "path": "domains/life/synbio/goal.md" },
  { "label": "논문",   "path": "papers/n6-synthetic-biology-paper.md" }
]
```

---

## 4. 수정 필드 요약 (총 7건)

| # | 파일 | JSON 경로 | 수정 유형 |
|---|---|---|---|
| 1 | `papers/_registry.json` | `_meta.papers_chunk_c_2026-04-08.papers[9]` | path 치환 |
| 2 | `papers/_registry.json` | `sections[tech-industry].products[합성생물학].links[0]` | label+path 치환 (`문서→goal`) |
| 3 | `papers/_registry.json` | `sections[tech-industry].products[합성생물학].links[1]` | path 치환 |
| 4 | `papers/_registry.json` | `sections[tech-industry].products[합성생물학].links[2]` | 신규 삽입 (도메인 본문) |
| 5 | `nexus/shared/n6/docs/products.json` | 위 (1)~(4) 와 동일 구조 (`_meta` + `links` 3개) | 동일 |
| 6 | `nexus/shared/n6/docs/products.json` | `_meta.papers_chunk_c_2026-04-08.papers[9]` | path 치환 |
| 7 | `nexus/shared/n6/n6_products.json` | `sections[tech-industry].products[합성생물학].links[]` | 1개→3개 확장 |

실질 개별 JSON 필드 수정 건수: **_registry.json 4건 + products.json 4건 + n6_products.json 3건 = 11 필드**.

---

## 5. JSON 유효성 검증

```
python3 -m json.tool papers/_registry.json            → VALID
python3 -m json.tool nexus/shared/n6/docs/products.json → VALID
python3 -m json.tool nexus/shared/n6/n6_products.json → VALID
```

3개 파일 모두 JSON 문법 유효 (RFC 8259 준수). 엔트리 구조 (키/타입/순서) 보존.

---

## 6. 잔여 드리프트 (본 건 범위 밖)

### 6-1. 경로 드리프트 — `docs/synbio/` / `docs/paper/n6-synthetic-biology-paper.md`

| 파일 | 라인 | 유형 | 처리 권고 |
|---|---|---|---|
| `/Users/ghost/Dev/n6-architecture/README.md` | 560 | `AUTO:SUMMARY_tech-industry:END` 이후 수동 표 | sync-readme.hexa 재생성 (R5) |
| `/Users/ghost/Dev/n6-architecture/n6shared/config/dse-map.toml` | 18775 | `note` 주석 필드 | 문구 갱신 또는 삭제 (별건) |
| `/Users/ghost/Dev/nexus/shared/n6/docs/dse-map.toml` | 18775 | 주석 필드 (동일) | 위와 동일 |
| `/Users/ghost/Dev/n6-architecture/reports/audits/paper-legacy-audit/verify-coverage.md` | 114 | 과거 감사 기록 | **불변** (reports 시점 불변 원칙) |
| `/Users/ghost/Dev/n6-architecture/reports/audits/synbio-merge-2026-04-11.md` | 28-29, 98-99, 117 | 감사 리포트 본문 | **불변** |
| `/Users/ghost/Dev/n6-architecture/reports/audits/_registry-backup-2026-04-11.json` | 20, 3215, 3219 | 본 건 백업 | **불변** |
| `/Users/ghost/Dev/n6-architecture/reports/audits/products-backup*-2026-04-11.json` | 20, 3142/3146/3149/3153 | 과거 + 본 건 백업 | **불변** |

**핵심 잔여 건**: `README.md:560` (sync 재생성 필요). `dse-map.toml:18775` (주석만).

### 6-2. 도메인 ID 드리프트 — `synthetic-biology` (삭제된 디렉토리)

`synbio-merge-2026-04-11.md` 병합으로 `domains/life/synthetic-biology/` 디렉토리는 삭제되었으나, 다음 3개 JSON 의 `sections[tech-industry].domains[]` 배열에 여전히 도메인 ID `"synthetic-biology"` 가 있음:

| 파일 | 라인 | 현재값 | 실체 디렉토리 |
|---|---|---|---|
| `papers/_registry.json` | 3157 | `"synthetic-biology"` | `domains/life/synbio/` |
| `nexus/shared/n6/docs/products.json` | 3091 | `"synthetic-biology"` | `domains/life/synbio/` |
| `nexus/shared/n6/n6_products.json` | 2641 | `"synthetic-biology"` | `domains/life/synbio/` |

**권고**: 별건으로 `"synthetic-biology"` → `"synbio"` 교체. 본 건은 "경로 드리프트" 범위에 한정 (R18 미니멀) — 도메인 ID 교체는 tech-industry 섹션 전체 일관성 검증이 필요한 별도 작업.

### 6-3. 합성생물학 논문 파일 (해결 완료)

`synbio-merge-2026-04-11.md` §3-2 에서 "papers 에이전트 (#8) 의 11편 확장 작업에서 `papers/n6-synthetic-biology-paper.md` 를 신규 생성 대상에 포함" 권고가 있었음. 본 감사 시점에서 해당 파일 **이미 존재** (25702 B, 2026-04-11 20:47). 경로 참조를 이 실체로 수정 완료.

---

## 7. 규칙 준수 체크리스트

| 규칙 | 체크 | 결과 |
|---|---|---|
| R2 하드코딩 금지 | 경로 전부 repo-상대 (JSON 내부 관례 유지) | OK |
| R5 SSOT | 스테일 포인터 제거, 실체 파일만 참조 | OK |
| R10 ossified 불변 | convergence ossified 블록 미편집 | OK |
| R11 강등 금지 | stable→failed 전환 없음 | OK |
| R14 규칙=JSON | CLAUDE.md 에 규칙 기록 없음 | OK |
| R18 미니멀 | 7건 경로 치환 + 3건 링크 추가만, 도메인 ID 교체 등 범위 확장 없음 | OK |
| R25 공용설정 게이트 | GO 모드 + `synbio-merge-2026-04-11.md` §3-1/§3-2 승인 권고 기반 | OK |
| R28 atlas SSOT | atlas.n6 미편집 (경로 수정만) | OK |
| N61 ASCII 3도 | JSON 수정이라 해당 없음 | N/A |

---

## 8. 후속 작업 (별건)

1. **도메인 ID 교체** (§6-2): `"synthetic-biology"` → `"synbio"` 3파일 일괄 수정
2. **README 재생성** (§6-1): `nexus analyze sync-readme` 또는 `sync-readme.hexa` 실행으로 라인 560 갱신
3. **dse-map.toml 주석 갱신** (§6-1): 라인 18775 `note` 업데이트 (n6-architecture + nexus 2파일)
4. **합성생물학 논문 N62 완성**: `papers/n6-synthetic-biology-paper.md` 의 ```python 검증 코드블록 임베드 확인 (현재 상태 별건 감사)
5. **백업 정리**: 본 건 3개 백업 파일 유지 (roll-back 용), 만료 정책은 별건

---

## 9. 결과 지표

| 지표 | 수정 전 | 수정 후 |
|---|---|---|
| 스테일 `docs/synbio/` 경로 (JSON 3개 내) | 3 | **0** |
| 스테일 `docs/paper/n6-synthetic-biology-paper.md` 경로 (JSON 3개 내) | 4 | **0** |
| 합성생물학 제품 엔트리 `links` 수 (_registry.json) | 2 | **3** |
| 합성생물학 제품 엔트리 `links` 수 (products.json) | 2 | **3** |
| 합성생물학 제품 엔트리 `links` 수 (n6_products.json) | 1 | **3** |
| JSON 유효성 | 유효 | **유효 유지** |
| 잔여 경로 드리프트 (`docs/synbio/` 활성 파일) | - | **3** (README + dse-map.toml x2) |
| 잔여 도메인 ID 드리프트 (`synthetic-biology`) | - | **3** (JSON 3파일 domains[] 배열) |

---

*생성: 2026-04-11 · 수정 주체: claude-opus-4.6 (1M 컨텍스트) · 범위: R18 미니멀 + R25 게이트 승인*
