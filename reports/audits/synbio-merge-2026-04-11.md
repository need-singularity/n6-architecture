# 감사 리포트 — synbio ↔ synthetic-biology 병합 (2026-04-11)

> 축: **reports/audits** · n6-architecture · 병합 감사
> 규칙 기준: R10(ossified 불변) / R14(JSON 단일규칙) / R18(미니멀) / R25(공용설정 게이트) / R28(atlas SSOT) / N61(ASCII 3도) / N62(py 임베드)

---

## 1. 증상 — 중복 도메인 & 스테일 포인터

### 1-1. 도메인 디렉토리 중복

동일 주제(합성생물학)에 두 개의 별도 도메인 디렉토리가 존재:

| 디렉토리 | 파일 | 본문 줄수 | alien_index | closure_grade |
|---|---|---|---|---|
| `domains/life/synbio/` | synbio.md + goal.md + verify.hexa + verify_alien10.hexa + CLAUDE.md | 660 | **10** | **10** |
| `domains/life/synthetic-biology/` | synthetic-biology.md + goal.md + verify.hexa + verify_n6.hexa + CLAUDE.md | 504 | 7 | 6 |

두 파일의 **가설 세트가 상호보완적**:
- `synbio.md` — 기초 생화학 (H-SYN-01~12 + H-SYN-1~10): 염기/코돈/아미노산/CRISPR gRNA&PAM/Gibson/T7 프로모터/Central Dogma/DBTL
- `synthetic-biology.md` — 응용 (H-SYN-1~12 별도): CRISPR-Cas 번호/XNA 6종/JCVI-syn3.0 최소 게놈/BioBrick 구조/유전자 회로 게이트/포도당/mRNA 백신/대사 경로/DNA 합성 오류율/유전자 드라이브/CAR-T

### 1-2. 외부 레지스트리 스테일 포인터

`products.json` 및 `papers/_registry.json` 의 "합성생물학 n=6 이중 완전수" 항목은 다음 경로를 가리키지만 **둘 다 실체 없음**:

```
문서: docs/synbio/goal.md                   ← 경로 없음
논문: docs/paper/n6-synthetic-biology-paper.md  ← 경로 없음
```

`papers/` 디렉토리에는 `n6-oceanography-paper.md`, `n6-meteorology-paper.md`, `n6-geology-prem-paper.md`, `n6-cryosphere-paper.md` 만 존재 — **합성생물학 논문 파일 없음**.

---

## 2. 병합 실행 (2026-04-11)

### 2-1. canonical 선택

`synbio/` 를 canonical 로 유지 (alien_index=10 / closure=10 최신).

### 2-2. synbio/synbio.md 흡수 섹션

| synbio.md 섹션 | 흡수된 내용 | 출처 |
|---|---|---|
| 제목 헤더 | 부모 BT 목록 + 핵심 축 라인 추가 | synthetic-biology/goal.md |
| 섹션 4 BT 연결 | BT-SYN-A~B 유지 + BT-SYN-C/D/E 신규 + BT-372 등록 | synthetic-biology.md BT 후보 + products.json |
| 섹션 5 DSE 결과 | 3단 비교표 (시중/v1/v2) | synthetic-biology.md 4번 |
| 섹션 11 ASCII 성능비교 | 시중 vs HEXA-SynBio 박스 비교 | synthetic-biology.md 1번 |
| 섹션 12 ASCII 시스템 구조도 | EDA 파이프라인 구조도 (부품→EDA→회로→조립→발효→DBTL) | synthetic-biology.md 2번 |
| 섹션 13 ASCII 데이터/에너지 플로우 | 설계 의도→EDA→네트리스트→형질전환→균주 뱅크 | synthetic-biology.md 3번 |
| 섹션 14 업그레이드 | 시중/v1/v2 3단 업그레이드 표 | synthetic-biology.md 4번 |
| 섹션 15 검증 방법 | verify.hexa + verify_alien10.hexa 통합 설명 | 신규 작성 |
| **부록 A** 확장 가설 | H-SYN-APP-1~12 (CRISPR-Cas 번호/XNA 6종/JCVI-syn3.0/BioBrick/회로 게이트/포도당/mRNA 백신/대사 경로/DNA 합성 오류율/유전자 드라이브/CAR-T) | synthetic-biology.md 가설 섹션 |
| **부록 B** 한계·MISS | iGEM 부품 클래스 변동/TCA 단계/AAV 세로타입/mRNA 반감기 | synthetic-biology.md 5번 |
| **부록 C** 병합 이력 | 본 감사 리포트 참조 | 신규 작성 |

### 2-3. synbio/goal.md 흡수

- 부모 BT 라인 추가 (BT-51/146/262/372/404~413/451~460)
- 핵심 축 라인 추가 (τ=4 논리 × n=6 클래스 × σ−φ=10 단계)
- 역추출 artifact "## 3. 가설" 제거

### 2-4. 폐기 파일 (5건)

```
domains/life/synthetic-biology/synthetic-biology.md   (504 줄 → 흡수)
domains/life/synthetic-biology/goal.md                (158 줄 → 흡수)
domains/life/synthetic-biology/verify_n6.hexa         (스텁, verify_alien10.hexa 로 대체)
domains/life/synthetic-biology/verify.hexa            (공통 템플릿 중복)
domains/life/synthetic-biology/CLAUDE.md              (중복)
```

디렉토리 `domains/life/synthetic-biology/` 자체도 `rmdir` 로 제거.

### 2-5. `domains/_index.json` 갱신

| 필드 | 변경 전 | 변경 후 |
|---|---|---|
| `_version` | 1.0.1 | **1.0.2** |
| `_total` | 299 | **298** |
| `life` 개수 | 48 | **47** |
| `life` 배열 | `"synbio", "synthetic-biology"` | `"synbio"` |
| `_changelog` | 1건 | **2건** (병합 항목 추가) |

---

## 3. 외부 레지스트리 드리프트 (수정 보류 — R25 게이트)

> R25: 공용 설정(hooks-config/absolute_rules/core-lockdown 등) 직접 수정 금지 — sync 스크립트 또는 사용자 명시 승인 필수

### 3-1. `products.json` (`/Users/ghost/Dev/nexus/shared/n6/docs/products.json`)

```json
{
  "name": "합성생물학 n=6 이중 완전수",
  "links": [
    {"label": "문서", "path": "docs/synbio/goal.md"},                  // ← 실체 없음
    {"label": "논문", "path": "docs/paper/n6-synthetic-biology-paper.md"}  // ← 실체 없음
  ]
}
```

**권고 수정** (승인 시):
```json
{
  "links": [
    {"label": "문서", "path": "domains/life/synbio/synbio.md"},        // 실체 있음
    {"label": "goal", "path": "domains/life/synbio/goal.md"},          // 실체 있음
    {"label": "논문", "path": "papers/n6-synthetic-biology-paper.md"}  // 생성 필요 (papers 에이전트 #8 작업 범위)
  ]
}
```

### 3-2. `papers/_registry.json`

동일 경로 `docs/paper/n6-synthetic-biology-paper.md` 참조 (라인 3146 근방). 실체 없음.

**권고**:
- papers 에이전트 (#8) 의 11편 확장 작업에서 `papers/n6-synthetic-biology-paper.md` 를 신규 생성 대상에 포함 (우선순위 상)
- _registry.json 경로를 `papers/n6-synthetic-biology-paper.md` 로 수정 (agent 에 위임)

### 3-3. `n6shared/convergence/n6-architecture.json`

`PRODUCTS_118` = "118/125 제품 🛸10" ossified 상태. 실제 products 실측 = 164/173 (최근 products.json 검사 리포트 참조).

**권고**: 별도 건으로 `PRODUCTS_164_173` 신규 stable 항목 추가 (R11 골화 강등 금지 → 새 항목 만으로 갱신).

---

## 4. 규칙 준수 체크리스트

| 규칙 | 체크 | 결과 |
|---|---|---|
| R1 HEXA-FIRST | .hexa 수정 없음, .md 문서 병합만 | ✅ |
| R2 하드코딩 금지 | 경로 모두 `/Users/ghost/Dev/n6-architecture/` 절대경로 사용 | ✅ |
| R5 SSOT | 2 도메인 중복 → 1 canonical, _index.json 단일 진실 업데이트 | ✅ |
| R10 ossified 불변 | convergence ossified 블록 미변경 (products.json 경로 드리프트는 별건) | ✅ |
| R11 강등 금지 | stable → failed 전환 없음 | ✅ |
| R14 규칙=JSON | CLAUDE.md 에 규칙 기록 없음 | ✅ |
| R18 미니멀 | 요청 범위(분리 문서 합치기)만 수행, 추가 기능 없음 | ✅ |
| R25 공용설정 게이트 | products.json / papers/_registry.json 수정 보류, 권고만 기록 | ✅ |
| R28 atlas SSOT | 문서 병합이며 새 발견 기록 아님 → atlas.n6 미편집 | ✅ |
| R29 verify .hexa 위치 | 기존 verify_alien10.hexa 는 도메인 디렉토리에 있음 (이전부터 존재, 별건) | ⚠️ 기존 위반 승계 |
| N61 ASCII 3도 | 섹션 11/12/13 ASCII 3도 유지 | ✅ |
| N62 py 임베드 | synbio.md 섹션 3 Python 검증 코드 기존 유지 | ✅ |
| N64 A+B+C 산출 | A(문서 통합) ✅ · B(논문) ⚠️ 에이전트 #8 위임 · C(products 경로) ⚠️ 권고만 | 부분 |

**R29 주의**: `domains/life/synbio/verify_alien10.hexa` 는 본 병합 이전부터 존재. R29 는 계산기/검증/스캐너 .hexa 를 `nexus/shared/n6/scripts/` 에만 허용. 기존 위반 상태이나 본 병합 범위 밖 — 별도 세션에서 이관 작업 필요.

---

## 5. 결과 지표

| 지표 | 변경 전 | 변경 후 |
|---|---|---|
| 도메인 총수 | 299 | **298** |
| life 축 도메인 | 48 | **47** |
| 합성생물학 도메인 수 | 2 (중복) | **1** (canonical) |
| synbio.md 줄수 | 660 | **~1100** (부록 A/B/C 추가) |
| BT 후보 | BT-SYN-A/B (2개) | BT-SYN-A/B/C/D/E + BT-372 등록 (5+1) |
| 가설 세트 | 기초 22개 (H-SYN-1~10 + 01~12) | 기초 22개 + 응용 12개 (H-SYN-APP-1~12) = **34** |
| TODO 섹션 | 12개 (4~15) | 4개 (6,7,8,9,10) |

---

## 6. 다음 단계 (후속 작업)

1. **B 산출 (논문)**: papers 에이전트 #8 작업에 `papers/n6-synthetic-biology-paper.md` 추가 생성 요청
2. **C 산출 (products/papers registry)**: 위 §3-1/§3-2 권고 반영 (사용자 승인 필요, R25)
3. **convergence 갱신**: `PRODUCTS_164_173` 신규 stable 항목 (별건)
4. **atlas.n6 등록**: BT-372 이중 완전수 항목을 atlas.n6 L6_n6atlas 섹션에 [7]→[10*] 승격 대상으로 후보 등록 (atlas-agent 재발사 범위)
5. **R29 이관**: `synbio/verify_alien10.hexa` 를 `nexus/shared/n6/scripts/` 로 이동 (별도 세션)

---

*생성: 2026-04-11 · 병합 주체: claude-opus-4.6 · 범위: R18 미니멀 준수*
