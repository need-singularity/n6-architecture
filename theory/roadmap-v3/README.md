---
id: roadmap-v3-index
date: 2026-04-24
roadmap_task: HONEST-PX-3 (v3 execution scaffold)
grade: [10] skeleton index
parent_design: theory/roadmap-v2/millennium-v3-design-2026-04-15.md
parent_json: shared/roadmaps/millennium.json (v2.3 → v3.0 pending)
license: CC-BY-SA-4.0
status: SCAFFOLD (phase-00 bootstrap only; phase-01+ require authorization)
---

# Millennium 7 난제 로드맵 v3 — 실행 스캐폴드

> **본 디렉토리는 실행 스켈레톤입니다.** v3 설계는
> `theory/roadmap-v2/millennium-v3-design-2026-04-15.md` 에 확정되어 있으며,
> 본 README 는 그 설계를 디렉토리 구조로 구현한 입구입니다.
>
> **Phase 내용은 연구가 필요하므로 본 스캐폴드 단계에서는 생성하지 않습니다.**
> 각 phase 파일은 사용자의 명시적 "authorize phase-NN" 지시 후에만 채워집니다.

---

## 0. 스캐폴드 현황 (2026-04-24 기준)

| 항목 | 상태 |
|------|------|
| 설계 문서 | DONE (`roadmap-v2/millennium-v3-design-2026-04-15.md`) |
| v3 디렉토리 생성 | DONE (본 디렉토리) |
| `phase-00-bootstrap.md` | DONE (본 스캐폴드 커밋) |
| `migration-from-v2.md` | DONE (본 스캐폴드 커밋) |
| `phase-01-E-empirical.md` ~ `phase-13-closure.md` | **PENDING (research required)** |
| `millennium.json` schema_version 승격 (2.3 → 3.0) | PENDING (§5.3 MANDATORY 4건 충족 후) |

---

## 1. v3 구조 (계획)

설계 문서 §1.3 에 따라 v2.3 의 P0~P10/PΩ/PX 는 **그대로 승계** 하고, 신규 3 개 phase 만 본
디렉토리에서 관리합니다.

```
theory/roadmap-v3/
├── README.md                       # 본 파일 (index)
├── phase-00-bootstrap.md           # v2.3 → v3 전환 게이트 (MANDATORY §5.1)
├── migration-from-v2.md            # v2 → v3 승계 매핑
├── phase-11-E-empirical.md         # [PENDING] E track 7 tasks (E1~E7)
├── phase-12-T-theoretical.md       # [PENDING] T track 6 tasks (T1~T6)
├── phase-13-M-meta.md              # [PENDING] M track 5 tasks (M1~M5)
└── phase-omega-v3-closure.md       # [PENDING] v3 종결 메타
```

> 주의: phase-11~13 의 번호는 v2.3 에서 PX 이후 이어지는 연속 번호입니다.
> v3 신규 track 만 본 디렉토리에 생성하며, P0~P10/PΩ/PX 는 `roadmap-v2/` 에 남습니다.

---

## 2. 3-track 요약 (설계 §1.2 재인용, 실행용)

| track | phase 파일 | tasks | cost 합 | BT 연관 |
|-------|-----------|-------|---------|---------|
| **E** Empirical | phase-11-E-empirical.md | E1~E7 | L+L+M | BT-541, 546 |
| **T** Theoretical | phase-12-T-theoretical.md | T1~T6 | L+L+L | BT-542, 545 |
| **M** Meta | phase-13-M-meta.md | M1~M5 | M+L+M | 전 BT |

---

## 3. v3 진입 조건 (MANDATORY, 설계 §5.1 재인용)

- [ ] v2.3 deferred 7 항목 중 ≥ 3 "v3 Phase 배치" 완료 (설계 §2-4 에 배치 완료)
- [ ] v2.3 loop1-7 atlas 18 entries 재확인 (R14 CLEAN 2026-04-15 확정 필요)
- [ ] Monotone drift 체크 baseline 등록 (MONOTONE-PX-1 CLI)
- [ ] BT 해결 수 0/6 재확인

4 건 전부 충족 + 사용자 `go v3` 명시 시 `shared/roadmaps/millennium.json.schema_version` 를
`"3.0"` 으로 승격합니다.

---

## 4. 정직성 헌장 (설계 §6.1 재인용, 본 디렉토리 전반 적용)

1. **BT 해결 주장 금지** — v3 진행 어느 시점에도 "RH 풀었다" 등 claim 금지
2. **외부 의존 노출** — Sage / Lean4 / arXiv API / 외부 수학자 의존 명시
3. **MISS 조건 사전 명시** — 각 task 는 MISS 기준 선규정
4. **OUROBOROS 주기 감사** — 매 phase 종료 시 R14 CLEAN 확인

---

## 5. 관련 파일

- 설계: `theory/roadmap-v2/millennium-v3-design-2026-04-15.md`
- 승계 매핑: `./migration-from-v2.md`
- 전환 게이트: `./phase-00-bootstrap.md`
- 상위 JSON: `shared/roadmaps/millennium.json` (v2.3)

---

*작성: 2026-04-24 (v3 scaffold bootstrap)*
*다음 단계: 사용자 `authorize phase-01` 승인 시 `phase-11-E-empirical.md` 내용 생성 시작*
