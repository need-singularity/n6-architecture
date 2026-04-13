# products.json 드리프트 수정 감사 리포트

- 일자: 2026-04-11
- 대상: `/Users/ghost/Dev/nexus/shared/n6/docs/products.json` (154 KB, 4,862줄)
- 작업자: Claude Opus 4.6 (GO 세션)
- 근거: 이전 감사에서 확인된 `_meta` ↔ 실측 sections 간 구조적 드리프트
- 백업: `/Users/ghost/Dev/n6-architecture/reports/audits/products-backup-2026-04-11.json`

## 1. 발견된 드리프트 (수정 전)

| 필드 | 선언값 | 실측값 | 상태 |
|------|--------|--------|------|
| `_meta.total_products` | 156 | 173 | ❌ -17 누락 |
| `_meta.alien_index_order` (len) | 27 | 34 | ❌ -7 누락 |
| `_meta.last_updated` | 2026-04-10 | 2026-04-11 (오늘) | ❌ 스테일 |
| `sections[id=quantum-computer].products[0].ceiling` | false | true (ufo=10) | ❌ 등급 불일치 |

### 1.1 alien_index_order 누락 섹션 (7개)

실측 sections 에는 존재하나 `alien_index_order` 배열에서 빠진 섹션:

1. `tattoo-removal`
2. `keyboard`
3. `mouse`
4. `manufacturing-quality`
5. `network`
6. `quantum-computer`
7. `horology`

### 1.2 ceiling 플래그 불일치

`sections[id=quantum-computer].products[0]` (HEXA-QUANTUM 양자컴퓨터 아키텍처):

- `ufo`: 10 (천장 등급)
- `ceiling`: false ← ufo=10 이면 반드시 `ceiling=true` 여야 함
- 해당 제품 설명: "20/24 EXACT — NeutralAtom n=6원자, SurfaceCode sigma=12 data qubit, Clifford tau*n=24 gate, Grover sopfr, kissing number BT-49"

## 2. 적용된 수정 (4건)

### 2.1 `_meta.total_products`
```diff
- "total_products": 156,
+ "total_products": 173,
```

### 2.2 `_meta.last_updated`
```diff
- "last_updated": "2026-04-10",
+ "last_updated": "2026-04-11",
```

### 2.3 `_meta.alien_index_order` (hygiene 직후에 7개 삽입)

물리 JSON 선언 순서(실제 sections 배열 순서)와 일치하도록 `hygiene` 직후에 삽입:

```diff
  "hygiene",
+ "tattoo-removal",
+ "keyboard",
+ "mouse",
+ "manufacturing-quality",
+ "network",
+ "quantum-computer",
+ "horology",
  "tech-industry",
```

### 2.4 `sections[quantum-computer].products[0].ceiling`
```diff
  "name": "HEXA-QUANTUM 양자컴퓨터 아키텍처",
  "ufo": 10,
- "ceiling": false,
+ "ceiling": true,
```

※ 섹션 레벨의 `quantum-computer.ceiling` 은 작업 지시 범위 외로 변경하지 않음 (오직 products[0].ceiling 만).

## 3. 수정 후 검증 결과

```
JSON_VALID
total_products:        173
last_updated:          2026-04-11
alien_index_order_len: 34
actual_sections_count: 34
missing_from_index:    []
extra_in_index:        []
HEXA-QUANTUM.ceiling:  True (ufo=10 일치)
actual_total_products: 173 (_meta 일치)
```

- `python3 -m json.tool` 통과 → 문법 유효
- `_meta.total_products == sum(len(s.products))` → **173 == 173** ✅
- `len(_meta.alien_index_order) == len(sections)` → **34 == 34** ✅
- `set(alien_index_order) == set(section ids)` → 완전 일치 (missing/extra 모두 0) ✅
- `products[].ufo==10 ⟹ ceiling==true` 불변식 복원 ✅

## 4. 기존 제품 데이터 보존

작업 지시에 따라 **오직 `_meta` 블록과 `HEXA-QUANTUM.products[0].ceiling` 플래그 1건만** 수정. 개별 섹션의 `bt_exact_pct`, `alien_index`, `products[*].description`, `domains`, `tools`, `links` 등 모든 실데이터는 무변경.

## 5. 규칙 준수

- R8 (데이터 원격 저장소): `n6shared/n6/docs/` 하위 — 직접 수정 허용 범위
- R14 (shared/ JSON 단일진실): 실측과 일치하도록 동기화
- R25 (공용설정 게이트): GO 모드 + 이전 감사 리포트 기반 → 승인 범위
- R0~R27 (common): JSON 유효성 / 백업 / 불변식 복원 모두 준수

## 6. 후속 권장사항

1. `tools/audit/products_drift.hexa` (미존재 시) 신설 → 매일 drift 탐지 자동화
2. `_meta.total_products ↔ sum(sections.*.products)` 불변식을 스키마 검증에 포함
3. `ufo==10 ⟺ ceiling==true` 불변식을 pre-commit 훅으로 강제
4. `alien_index_order` 를 섹션 배열에서 자동 재생성하는 헬퍼 추가 (수동 동기 의존 제거)

---

**수정 필드 요약**: `_meta.total_products`, `_meta.last_updated`, `_meta.alien_index_order`, `sections[quantum-computer].products[0].ceiling` (총 4필드). JSON 유효, 모든 불변식 복원. Total products 156 → **173**.
