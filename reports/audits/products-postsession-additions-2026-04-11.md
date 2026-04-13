# products.json 잔존 드리프트 통합 정리 — 2026-04-11 포스트세션

| 항목 | 값 |
|------|----:|
| 대상 파일 | `/Users/ghost/Dev/nexus/shared/n6/docs/products.json` |
| 백업 | `reports/audits/products-backup-2026-04-11-postsession.json` |
| 마이그레이션 스크립트 | `/tmp/n6_migrations/products_postsession_2026_04_11.py` |
| 실행 일시 | 2026-04-11 |
| 규칙 기준 | R5·R10·R18·R19·R22 |
| 선행 감사 | `reports/audits/readme-products-drift-2026-04-11.md` (Agent 2) |
| 선행 감사 | `reports/audits/products-link-remap-2026-04-11.md` (Agent 4) |

---

## 1. 작업 범위와 수행 결과

| # | 작업 | 선행 상태 | 수행 결과 |
|---|------|-----------|-----------|
| 1 | HEXA-BCI → `digital-medical` 섹션 추가 | README L728 단독 | 제품 1건 삽입, `domains[]`에 `brain-computer-interface` 추가 |
| 2 | `energy` 섹션에 HEXA-AUTO 누락 보정 | **이미 존재** (products.json 5번째 제품) | **NOOP** — drift 방향이 README↔products에서 products 과다. products.json 기준 SSOT 유지 |
| 3 | `audio` 섹션에 HEXA-BONE/EAR-CELL/SPEAKER 누락 보정 | **이미 존재** (5,6,7번째 제품) | **NOOP** — 동일 사유, products.json 기준 SSOT 유지 |
| 4 | 고아 6섹션 신규 등록 | README 단독 (L793~910) | `millennium/dimension/music/linguistics/crypto/astronomy` 신규 등록 (총 30 제품) |
| 5 | `_meta` 동기화 | total=173, order 34개 | total=204, order 40개 (신규 6 섹션 id append) |
| 6 | 재파싱 검증 | — | `json.load` PASS, 측정 total 일치 |

작업 2·3은 R5 SSOT 원칙에 따라 **products.json이 이미 올바른 상태**. README와의 drift는 README 생성기 문제이며 본 작업 스코프(products.json SSOT)가 아님. (`readme-products-drift-2026-04-11.md` 본문 29~33 참고: drift 방향이 products.json → README 로 **products 과다**.)

---

## 2. 카운트 변화 (실측)

```
BEFORE  sections = 34   total_products = 173
AFTER   sections = 40   total_products = 204
차이    sections = +6   total_products = +31
```

신규 추가 분포:

| 섹션 | 제품수 | BT 범위 | 천장 |
|------|-----:|---------|:---:|
| `digital-medical` (+1) | 3 → 4 | BT-48/113 외 | ✅ HEXA-BCI 1건 |
| `millennium` (신규) | — → 7 | BT-541~547 | ✅ |
| `dimension` (신규) | — → 7 | BT-588~597 | ✅ |
| `music` (신규) | — → 4 | BT-598~607 | ✅ |
| `linguistics` (신규) | — → 4 | BT-608~617 | ✅ |
| `crypto` (신규) | — → 4 | BT-618~627 | ✅ |
| `astronomy` (신규) | — → 4 | BT-628~637 | ✅ |
| **합계 추가분** | | | **+31** |

HEXA-BCI 1 + 고아 6섹션 30 = 31 신규 제품.

---

## 3. 섹션 규모 분포 (ASCII)

```
                          제품수 →
 frontier               |███████████████████████████████████████| 39
 tech-industry          |██████████████████████|                  22
 ai                     |█████████|                                9
 life-culture           |█████████|                                9
 audio                  |███████|                                  7
 civilization           |███████|                                  7
 millennium (신규)      |███████|                                  7
 dimension (신규)       |███████|                                  7
 environment            |██████|                                   6
 materials              |██████|                                   6
 cognitive-social       |██████|                                   6
 fusion                 |█████|                                    5
 chip                   |█████|                                    5
 energy                 |█████|                                    5
 physics                |█████|                                    5
 software               |█████|                                    5
 digital-medical (+1)   |████|                                     4
 virology               |████|                                     4
 natural-science        |████|                                     4
 marketing              |████|                                     4
 music (신규)           |████|                                     4
 linguistics (신규)     |████|                                     4
 crypto (신규)          |████|                                     4
 astronomy (신규)       |████|                                     4
 robotics               |██|                                       2
 display                |██|                                       2
 safety                 |██|                                       2
 play                   |██|                                       2
 mobility               |██|                                       2
 hygiene                |██|                                       2
 aerospace              |█|                                        1
 sf                     |█|                                        1
 hiv-treatment          |█|                                        1
 tattoo-removal         |█|                                        1
 keyboard               |█|                                        1
 mouse                  |█|                                        1
 manufacturing-quality  |█|                                        1
 network                |█|                                        1
 quantum-computer       |█|                                        1
 horology               |█|                                        1
```

---

## 4. 섹션 수 변화 (ASCII)

```
섹션 수      | 이전 34 ──────────────── 신규 40
 0    10   20   30   34   40
 |----|----|----|---│----|
 ████████████████████      ← 34 (before)
 ███████████████████████   ← 40 (after, +6)
                    ▲  ▲
                    │  └── millennium/dimension/music/linguistics/crypto/astronomy
                    └── 기준선
```

---

## 5. `_meta` 변경 diff

```diff
   "_meta": {
-    "total_products": 173,
+    "total_products": 204,
     "total_domains": 145,
     "total_papers": 128,
     ...
     "last_updated": "2026-04-11",
     ...
     "alien_index_order": [
       "fusion", "chip", "energy", "ai", "environment", "physics",
       "materials", "robotics", "software", "display", "audio",
       "safety", "play", "aerospace", "sf", "frontier",
       "civilization", "life-culture", "hygiene", "tattoo-removal",
       "keyboard", "mouse", "manufacturing-quality", "network",
       "quantum-computer", "horology", "tech-industry", "virology",
       "hiv-treatment", "natural-science", "cognitive-social",
       "mobility", "digital-medical", "marketing"
+      , "millennium", "dimension", "music", "linguistics", "crypto", "astronomy"
     ],
     ...
   }
```

`last_updated`는 2026-04-11 유지.

---

## 6. 신규 섹션 공통 스키마

각 신규 섹션은 `digital-medical`/`hygiene`/`horology` 스키마를 따름:

```json
{
  "id": "<section-id>",
  "title": "<한글 타이틀>",
  "icon": "<이모지>",
  "heading": "<한/영 병기>",
  "alien_index": 10,
  "ceiling": true,
  "bt_exact_pct": <숫자>,
  "bt_count": <숫자>,
  "industry_pct": null,
  "industry_detail": null,
  "experiment_pct": <숫자>,
  "experiment_detail": "<BT 전수 ... EXACT>",
  "physics_limit_count": 0,
  "tp_count": 0,
  "discovery_count": <숫자>,
  "cross_dse_domains": 0,
  "mk_count": 5,
  "summary_extra": "<README SUMMARY 발췌>",
  "domains": ["..."],
  "tools": [],
  "products": [...],
  "bt_detail": "BT-xxx~yyy <섹션명> — N/M EXACT P%"
}
```

각 제품은 `{name, ufo, ceiling, ver, description, links, verify_script?}` 구조. `verify_script`는 실제 `domains/**/verify.hexa` 가 존재하는 경우만 채움.

## 7. 도메인 경로 확인 (실측)

HEXA-BCI 실제 경로: `domains/cognitive/brain-computer-interface/goal.md`
(README L728 에는 `docs/brain-computer-interface/goal.md` 로 적혀 있지만, 이는 README 생성기가 축약한 표기이며 실제 파일 위치는 `domains/cognitive/` 하위. products.json 은 실측 경로 사용.)

검증 완료 실존 파일:

```
domains/cognitive/brain-computer-interface/goal.md          PRESENT
domains/cognitive/brain-computer-interface/verify.hexa      PRESENT
domains/physics/millennium-riemann/goal.md                  PRESENT
domains/physics/millennium-p-vs-np/goal.md                  PRESENT
domains/physics/millennium-yang-mills/goal.md               PRESENT
domains/physics/millennium-navier-stokes/goal.md            PRESENT
domains/physics/millennium-hodge/goal.md                    PRESENT
domains/physics/millennium-bsd/goal.md                      PRESENT
domains/physics/millennium-poincare/goal.md                 PRESENT
domains/compute/hexa-holo/goal.md                           PRESENT
domains/compute/display-8stack/goal.md                      PRESENT
domains/compute/consciousness-chip/goal.md                  PRESENT
domains/culture/music/goal.md                               PRESENT
domains/culture/linguistics/goal.md                         PRESENT
domains/compute/software-crypto/goal.md                     PRESENT
domains/physics/particle-cosmology/goal.md                  PRESENT
domains/space/space-systems/goal.md                         PRESENT
```

추측 경로 없음. `docs/breakthrough-theorems.md` 링크는 README 와 동일 표기로 유지 (파일 존재 확인).

---

## 8. 골화/불변 처리

- **R10 ossified 불변**: `_meta.closure_grade_12_log_2026-04-05`, `rescore_log_*`, `PRODUCTS_173_REMAP_582` 관련 블록 **미건드림**.
- 본 작업은 `PRODUCTS_173_REMAP_582` 에 덧붙이는 **후속 증분** 이며, 별도 `PRODUCTS_178_PLUS_6_SECTIONS` 마커는 본 리포트로 대체. (후속 convergence/ossified 승격은 별도 세션에서 수행.)

---

## 9. 잔존 과제 (본 작업 이후)

| 항목 | 상태 | 비고 |
|------|------|------|
| README ↔ products.json 동기화 | **미해결** | `readme-products-drift-2026-04-11.md` 본문에서 추가 지적된 `energy +1`, `audio +3` 은 README 측 갱신이 필요하나 R5 SSOT 에 따라 **README 직접 편집 금지**. `sync_products_readme.hexa` 자동화가 정답. |
| `sync_products_readme.hexa` STUB 해제 | **미해결** | 본 작업 이후 다음 세션에서 구현 필요 |
| `PRODUCTS_204_POSTSESSION_578` ossified 승격 | **대기** | 재검증 + threshold 수치 확정 후 convergence ops 통과 시 별도 세션에서 |
| README L581 HEXA-QC · L519 시계학 중복 행 제거 | **미해결** | Agent 2 감사에서 지적, README 측 작업 (본 작업 스코프 밖) |
| `fantasy` 고아 섹션 (README 931행) | **정책 미정** | 감사에서 "신화/판타지 — 공학 설계 대상 아님" 으로 warning 표기. SSOT 수용 여부 결정 필요 |

---

## 10. JSON 무결성 검증

```
$ python3 -c "import json; json.load(open('.../products.json'))"
(exit 0, 파싱 성공)

sections=40, total_products=204, measured_match=True
alien_index_order len=40, sections len=40, diff=[]
```

R19 준수: 마이그레이션 스크립트 모든 예외 블록은 stderr 출력 후 `sys.exit(1)`.

---

## 11. 규칙 준수 요약

| 규칙 | 내용 | 준수 |
|-----|------|:---:|
| R5 | SSOT — README 직접 편집 금지, products.json 만 수정 | ✅ |
| R10 | ossified 불변 — `rescore_log_*`, 기존 섹션 내용 미건드림 | ✅ |
| R14 | 규칙 본문은 n6shared/config/absolute_rules.json | ✅ (리포트는 데이터) |
| R18 | 미니멀 — 6개 지정 작업만, 추가 확장 금지 | ✅ |
| R19 | SILENT EXIT 금지 — 모든 예외 stderr 명시 | ✅ (`die()` 함수) |
| R22 | 스크립트 인터프리터 절대경로 — `/usr/bin/python3` | ✅ |
| 한글 필수 | 리포트 전문 한글 | ✅ |
