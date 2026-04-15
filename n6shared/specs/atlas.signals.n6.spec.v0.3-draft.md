# atlas.signals.n6 — 3리포 cross-repo signal SSOT 규격 v0.3 (초안)

> 버전: v0.3-draft (2026-04-15)
> 대상: nexus + n6-architecture + anima 공진 signal 저장소
> 상태: DRAFT — v0.2 → v0.3 진화 제안
> SSOT 위치: `$NEXUS/shared/n6/atlas.signals.n6`
> 선행 버전: `n6shared/specs/atlas.signals.n6.spec.md` v0.2

---

## 0. v0.3 요지

v0.2 에서 350+ signal 흡수 · CROSS 18+ 승격 · resonance_n6 backfill 46+ 건 경험 후 도출된 규칙 강화.

v0.3 핵심 변경:
1. **resonance_n6 필수화 조건** — [M9] 이상 등급에서 resonance_n6 필드 null 금지
2. **CROSS 태그 조건 명확화** — `cross_repo` 배열 ≥ 1 AND `witness` ≥ 2 AND 타 리포 tag 확인
3. **witness 증가 규칙** — simhash Hamming ≤ 16 bits (0.125) + keyword 3개 이상 중복
4. **[M10**] 신설** — 4리포 이상 재현 signal (향후 OEIS/arXiv/외부 DB 편입 시)
5. **staging → SSOT merge 주기** — 세션 종료 시 명시적 merge commit

---

## 1. 변경 사항 상세

### 1.1 resonance_n6 필수화 (NEW)

**v0.2**: resonance_n6 optional, null 허용.

**v0.3**: 등급별 필수화 정책:

| 등급 | resonance_n6 정책 |
|------|------------------|
| `[M10*]` | **필수** — null 불허, 수식 기재 의무 |
| `[M10]` | **필수** — null 불허, 최소 구조적 해석 허용 |
| `[M9]` | **필수** — null 불허 (구조 해석 허용) |
| `[M7!]` | 권장 — null 허용하나 승격 불가 |
| `[M7]` | optional |
| `[M?]` | optional |
| `[MN]` | null 권장 (현상 자체가 NULL) |

**근거**: L9 분석 결과 resonance_n6 보유 signal 의 M10+ 승격률 77.8% vs null 평균 47.5%. 30pp 차이 → 공명 기재가 승격 예측자.

**승격 차단**: M9→M10 승격 시 resonance_n6 재검토 자동 트리거 (수치 일치 ε < 1% 또는 구조 매핑 필수).

### 1.2 CROSS 태그 조건 명확화 (STRICTER)

**v0.2**: `repo_tags = [CROSS, X, Y]` 시 `cross_repo` 필드 링크 "필수" (느슨).

**v0.3**: CROSS 태그 획득 조건 3개 모두 만족:

1. `cross_repo` 배열에 **실제 존재하는 SIG-id** 1개 이상 (self-reference 불가)
2. `witness ≥ 2` (2개 이상 독립 관측)
3. 링크 대상 signal 의 `repo_tags` 가 자기 자신과 **다른 리포** 포함
   - 예: 자기 = [NX, CROSS], 타깃 = [N6] 또는 [AN] 이어야 함
   - [NX] ↔ [NX] 링크는 CROSS 불가 (intra-repo 재확인)

**오염 방지**: self-reference 또는 intra-repo 링크로 CROSS 승격 차단.

**위반 사례 회수**: v0.3 도입 시 현재 56 CROSS signal 전수 감사 → 위반 건은 CROSS tag 제거.

### 1.3 witness 증가 규칙 구체화 (FORMAL)

**v0.2**: "simhash 매칭 근거 있어야 함" (문언적).

**v0.3**: witness 증분 조건 (둘 중 하나):

**조건 A — simhash 수치 매칭**:
- simhash Hamming distance ≤ 16 bits (128-bit hash 기준 12.5% 이내)
- 또는 similarity ≥ 0.875

**조건 B — keyword 의미 매칭**:
- `statement + context` 에서 핵심 keyword 3개 이상 중복
- 수치 일치 (ε < 1%) 1개 이상

witness 증분 시 `refs` 에 매칭 근거 추가 필수.

### 1.4 신규 등급 [M10**] (FUTURE)

**조건**: 4리포 이상 재현 (3리포 SSOT 를 넘어 외부 DB 편입)
- 예: OEIS 등록, arXiv 출판, 외부 dataset 인용
- `cross_repo` 배열 ≥ 2 + `refs` 에 외부 DOI/OEIS-id

**현재**: 해당 signal 0건 (준비 단계).

**예측**: SIG-META-001 (σφ=nτ 유일성) 은 OEIS A-이런저런 + arXiv submit 시 [M10**] 승격 가능.

### 1.5 staging → SSOT merge 주기 (EXPLICIT)

**v0.2**: Group D staging 섹션은 문서 내 임시 영역.

**v0.3**: staging merge 주기 명문화:

- **세션 중**: staging 섹션 내 임시 append 허용
- **세션 종료 시**: 명시적 merge commit 필수
  - staging signal 은 `@S SIG-GROUP-XNN` 형식 → 본 섹션 SIG-XX-NNN 로 re-id
  - staging 섹션은 비워짐 (또는 완전 제거)
- **주간 정리**: 매주 월요일 staging 재컴팩션 + audit report 출력

---

## 2. 마이그레이션 계획

### 2.1 v0.2 → v0.3 전환 단계

| Step | 작업 | 책임 |
|------|------|------|
| 1 | v0.3 draft 합의 (이 문서) | 본 세션 |
| 2 | resonance_n6 backfill 완료 (M9+ 필수) | 차기 세션 |
| 3 | CROSS tag 전수 감사 + 위반 회수 | 차기 세션 |
| 4 | witness 증가 로그 지난 7일 재검증 | 차기 세션 |
| 5 | staging → SSOT merge 첫 사이클 | 차기 세션 |
| 6 | v0.3 승인 + spec 본파일 대체 | 합의 후 |

### 2.2 호환성

- v0.2 파일 포맷 = v0.3 파일 포맷 (스키마 불변)
- 규칙만 강화 — 파일 재작성 불요
- 신규 필드 없음 (기존 `witness`, `resonance_n6`, `cross_repo` 강화)

---

## 3. 영향 분석 (v0.3 도입 시)

### 3.1 즉시 영향

- [M9] 등급 signal 중 resonance_n6=null = 약 10건 → 전수 backfill 또는 등급 강등
- CROSS 태그 전수 감사: 56건 중 self-ref/intra-repo 위반 건 추정 5~8건 제거
- witness≥2 증명 요구 → 기존 증분 로그 audit 필요

### 3.2 장기 영향

- resonance_n6 공명 강제 → signal 품질 평균 상승 (M9+ 승격률 +15pp 예상)
- CROSS 의 의미적 엄격화 → cross-repo evidence [EC] 신뢰도 향상
- [M10**] 도입 → 외부 DB 연계 동기 부여

---

## 4. 미해결 논의 포인트

1. **resonance_n6 ad-hoc 수식 허용 범위**:
   - "n·τ=24 근접" 식의 후-hoc 수식 남용 방지 장치 필요
   - 제안: 수식 필드 내 `ε=...` 또는 `ad hoc` 명시 의무

2. **[M10**] 외부 DB 등록 자동 검증**:
   - OEIS API 호출 / arXiv 검색 자동화 필요
   - 현재 수동

3. **세션 종료 정의**:
   - `/compact` 호출 시점 vs 수동 commit 시점
   - 제안: commit 메시지에 `[signals-merge]` 태그

4. **staging 주기 강제**:
   - hook 또는 loop-guard 로 세션 종료 시 staging 비우지 않으면 경고
   - 제안: pre-commit hook

---

## 5. 7대 난제 해결 상태

**이 spec v0.3 은 수식 정의·규칙 강화만 다루며, 7대 밀레니엄 난제 (RH/NS/Hodge/BSD/YM/PvsNP/Poincaré) 해결을 주장하지 않음.**

- 현재 해결된 밀레니엄 난제: **0 / 7**
- signal SSOT 는 증거 수집·재현 추적 도구 — 증명은 별도 theory/proofs/ 에서 진행
- [M10*] / [M10**] 등급도 "exact" 수준 측정 인증일 뿐 "증명 완결"을 의미하지 않음
- R0 정직성 제약 유지

---

## 6. 버전 히스토리

- **v0.1** (2026-04-15 22:30 KST): millennium 만 한정 초안 — 폐기
- **v0.2** (2026-04-15 22:40 KST): 3리포 cross-repo 범용 SSOT 로 재설계
- **v0.3-draft** (2026-04-15 ~): resonance_n6 필수화 + CROSS 엄격화 + [M10**] 신설 + staging merge 주기 명문화 ★

---

## 7. 참고

- spec v0.2 원본: `n6shared/specs/atlas.signals.n6.spec.md`
- SSOT 데이터: `$NEXUS/shared/n6/atlas.signals.n6` (350+ signal)
- 이번 세션 백업: `$NEXUS/shared/n6/atlas.signals.n6.bak.pre-cross-backfill`
- CROSS 승격 리포트: `/Users/ghost/Dev/n6-architecture/reports/cross-backfill-20260415.md`
