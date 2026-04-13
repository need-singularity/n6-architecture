# goal.md 축 확장 감사 — 20 → 295

> **작업일**: 2026-04-11
> **배경**: `n6shared/convergence/n6-architecture.json` GOAL_MD_20 (ossified) 에서 "20개 생성 완료" 로 기록되었으나, 실제 파일시스템 조사 결과 **standalone goal.md 파일은 단 1개(`domains/compute/software-design/hexa-ssh/goal.md`, 중첩 경로)만 존재**. 나머지 294개 도메인은 `goal.md` 내용이 `<name>.md` 통합본 내부 `### 출처: goal.md` 섹션으로 흡수되어 있었음.
> **목표**: 295 도메인 SSOT(`domains/_index.json`) 전 도메인에 대해 `domains/<axis>/<name>/goal.md` standalone 파일 생성.

---

## 1. 최종 생성 결과

| 축 | 리스트(`_index.json`) | 폴더 존재 | 생성 완료 | 추출 방식 | 템플릿 방식 |
|----|------|------|------|------|------|
| physics | 37 | 37 | **37** | 37 | 0 |
| life | 48 | 48 | **48** | 47 | 1 |
| energy | 16 | 16 | **16** | 16 | 0 |
| compute | 47 | 47 | **47** | 47 | 0 |
| materials | 19 | 19 | **19** | 19 | 0 |
| space | 8 | 8 | **8** | 8 | 0 |
| infra | 57 | 57 | **57** | 57 | 0 |
| cognitive | 21 | 21 | **21** | 21 | 0 |
| culture | 25 | 25 | **25** | 25 | 0 |
| sf-ufo | 17 | 17 | **17** | 17 | 0 |
| **합계** | **295** | **295** | **295** | **294** | **1** |

> `compute/software-design/hexa-ssh/goal.md` (중첩 경로)은 _index.json 항목이 아닌 서브도메인이며 이전부터 존재. 295 집계에서 제외.
> `compute/` 하위에는 `_index.json` 비등재 서브폴더 4건(`chip-dse-pipeline`, `chip-isa-n6`, `chip-npu-n6`, `chip-rtl-gen`) 존재 — 이번 작업 범위에서 제외(SSOT 기준).

---

## 2. 생성 전략

### 2.1 추출 방식 (Primary: 294/295)

각 도메인의 `<name>.md` 통합본에서 정규식 `### 출처: \`goal\.md\`\n(.*?)(?=\n### 출처: |\Z)` 로 folded goal.md 섹션 역추출 → 축/경로 헤더(2줄) 부착 → standalone `goal.md` 로 기록.

- 장점: 기존 도메인 담당 에이전트가 작성한 풍부한 본문(실생활 효과표, ASCII 구조도, DSE Pareto, BT 연결, 예측 검증표)을 **무손실 보존**.
- 파일 크기 분포: 최소 1.7KB, 최대 168KB, 평균 17KB.
- 상위 통합본 `<name>.md` 는 **변경하지 않음** (R14 SSOT 원칙, folded 원본도 그대로 유지 = 이중 SSOT 의도적 유지).

### 2.2 템플릿 방식 (Fallback: 1/295)

folded goal.md 섹션이 존재하지 않는 도메인은 한글 4섹션 템플릿(① 도메인 목표 ② n=6 정합성 포인트 ③ 검증 지표 ④ 연결 BT·기법·atlas 상수)으로 초안 생성.

- **대상**: `life/womens-intimate-cleanser` (1건)
- 사유: 해당 도메인 `<name>.md` 에 `### 출처: goal.md` 섹션 부재
- 후속 과제: 도메인 담당 에이전트가 구체 상수·BT·기법 연결을 후속 채움

---

## 3. 템플릿 구조 (템플릿 방식 적용 도메인용)

```markdown
# <name> — n=6 설계 목표

> 축: **<axis>** (<axis_ko>) · n6-architecture · 자동 생성 템플릿

## 1. 도메인 목표
## 2. n=6 정합성 포인트 (n=6 / σ=12 / τ=4 / φ=2 / sopfr=5 / J₂=24)
## 3. 검증 지표 (표)
## 4. 연결 BT · 기법 · atlas 상수
```

---

## 4. 검증 샘플 경로

- `/Users/ghost/Dev/n6-architecture/domains/physics/higgs/goal.md` (5.6KB, 추출)
- `/Users/ghost/Dev/n6-architecture/domains/compute/blockchain/goal.md` (7.0KB, 추출)
- `/Users/ghost/Dev/n6-architecture/domains/cognitive/reality-map/goal.md` (추출)
- `/Users/ghost/Dev/n6-architecture/domains/life/womens-intimate-cleanser/goal.md` (1.7KB, 템플릿)

---

## 5. SSOT 갱신 권고

`n6shared/convergence/n6-architecture.json` 의 `GOAL_MD_20` 엔트리를 다음과 같이 갱신 권고:

```json
"GOAL_MD_295": {
  "status": "PASS",
  "value": "295 도메인 standalone goal.md 생성 완료 (294 추출 + 1 템플릿)",
  "threshold": "전 도메인(domains/_index.json) goal.md 존재",
  "note": "2026-04-11 축 확장 20→295. 기존 folded <name>.md 섹션 역추출 방식으로 무손실 분리. 1건(life/womens-intimate-cleanser)은 템플릿 초안",
  "ossified_at": "2026-04-11",
  "promoted_from": "goal_md_expansion"
}
```

---

## 6. 누락/주의 사항 (200단어 이내 사유 보고)

**누락 0건**. 295/295 전부 생성 완료.

**주의 사항 3건**:

1. **convergence 기록과 실제 파일시스템 불일치**: `GOAL_MD_20` 가 "20개 생성 완료" 로 PASS 기록되었으나, 실제 standalone 파일은 1개(중첩 경로 hexa-ssh) 뿐이었음. 이는 folded 방식(`<name>.md` 내부 `### 출처: goal.md` 섹션 흡수)을 "생성" 으로 집계한 역사적 기록 관행 때문으로 추정. 이번 작업으로 standalone 형태로 물리화되어 기록-실체 정합성 회복.

2. **템플릿 초안 1건**: `life/womens-intimate-cleanser` 는 `<name>.md` 에 folded goal.md 섹션이 없어 4섹션 한글 템플릿으로 초안 생성. 도메인 담당 에이전트의 후속 채움 필요 (부모 BT 탐색, 상수 매핑, 검증 시나리오).

3. **이중 SSOT 현상**: 추출된 294 standalone goal.md 는 원본 `<name>.md` 의 folded 섹션과 내용이 **중복**. 이는 R14(단일진실) 관점에서 경계 케이스. 대응: (a) `<name>.md` folded 섹션을 `이 섹션의 단일진실 = goal.md 참조` 스텁으로 교체하는 추가 작업 권고 (차기 세션), 또는 (b) goal.md 를 참조 사본으로 표기 유지. 본 세션에서는 **변경 없음** 으로 판단 (원본 무손실 원칙 우선).
