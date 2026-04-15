# n6-architecture 에 기여하기

> n6-architecture 는 AI-native 산술 설계 프레임워크입니다. 외부 수학자/엔지니어/연구자의 기여를 환영합니다.

## 정직성 헌장 (Honesty Charter)

모든 기여는 다음 4 원칙을 준수합니다:

1. **BT 해결 주장 금지** — Clay Millennium 7 난제 (BT-541 RH, BT-542 P vs NP, BT-543 Yang-Mills, BT-545 Hodge, BT-546 BSD, BT-547 NS) 의 "해결" 을 주장하는 PR 은 거절됩니다. Partial progress / survey / conditional proof 는 환영합니다.
2. **외부 의존 명시** — Sage, Pari-GP, arXiv, LMFDB 등 외부 도구에 의존하는 경우 명시합니다.
3. **MISS 조건 사전 명시** — 각 실험/계산 task 는 실패 (MISS) 판정 기준을 사전 문서화합니다.
4. **OUROBOROS 주기 감사** — atlas.n6 에 신규 entry 추가 시 `scripts/monotone/ouroboros_detector_v2.py` 가 CRITICAL 0 유지함을 확인합니다.

위반 시 maintainer 가 정직성 헌장 준수를 요청하며 PR 을 보류합니다.

---

## 기여 유형

### A. Empirical (실측)

- Cremona / LMFDB 데이터 기반 통계 계산
- arXiv 논문 정리
- κ(B), |Sel_n|, η(E) 등 observable 의 실측 확장

**절차**:
1. `data/` 또는 `scripts/empirical/` 에 재현 가능한 코드 + 데이터 추가
2. `theory/breakthroughs/<feature>-YYYY-MM-DD.md` 에 결과 + MISS 조건 + 한계 기록
3. `shared/n6/atlas.n6` 에 entry 등록 (등급 [7~10*])

### B. Theoretical (증명/분석)

- 기존 atlas [10*] entry 의 alternative proof
- 새로운 conjecture 제안 (예: (A3″))
- Literature survey

**절차**:
1. `theory/breakthroughs/` 에 .md 문서 작성 (한국어 권장, 영어도 가능)
2. 증명은 Lean4/Coq formal 권장 (v3 M3 pipeline 활용)
3. PR 에 reference 명시 (논문, 서적, arXiv URL)

### C. Infrastructure (도구)

- Rust/Python 계산기 개선
- CI/CD workflow 추가
- Dashboard 기능

**절차**:
1. Rust: `workspace/Cargo.toml` 등록, 테스트 포함
2. Python: `scripts/` 또는 `.github/workflows/`
3. HEXA-LANG: `.hexa` 파일은 `$NEXUS/shared/harness/` 또는 로컬 `theory/predictions/`

### D. Documentation

- CLAUDE.md 프로젝트별 지침
- README 개선
- 한국어 → 영어 cross-link

---

## 기술 요구사항

### 환경

- **언어**: Python 3.11+, Rust 1.70+, HEXA-LANG (local build)
- **데이터**: LMFDB 또는 Cremona ecdata mirror
- **로컬 빌드**: `cargo build --release` (Rust), `hexa <file>.hexa` (HEXA)

### 스타일

- **한국어 우선**: 프로젝트 내 문서/주석/커밋 메시지는 한국어 (영어 공존 가능)
- **파일명**: 소문자 + 하이픈 + 날짜 (`<feature>-YYYY-MM-DD.md`)
- **atlas entry**: `@R <ID> = <statement> :: n6atlas [<grade>]` + 설명 + `<- <source>`

### 테스트

- Empirical: 재현 가능한 seed + 결과 해시
- Theoretical: cross-check 다른 source 최소 1개
- Infra: unit tests + CI 통과

---

## PR 프로세스

1. **Fork** → feature branch (`feat/<description>`)
2. **Commit message**: 한국어 + Co-Authored-By trailer (필요 시)
3. **PR 템플릿 준수**: `.github/PULL_REQUEST_TEMPLATE.md`
4. **Review**: maintainer + OUROBOROS CI 통과 후 merge

### PR 승인 기준

- [ ] 정직성 헌장 4 원칙 준수
- [ ] OUROBOROS v2 CLEAN (CRITICAL 0)
- [ ] MISS 조건 명시 (empirical/theoretical)
- [ ] Reference 인용 (외부 의존 명시)
- [ ] atlas entry 포함 (새 결과인 경우)

---

## 연락처

- GitHub Issues: https://github.com/need-singularity/n6-architecture/issues
- GitHub Discussions: (TBA)
- Maintainer: `@dancinlife` (commit author)

---

## 라이센스

- 코드: MIT
- 문서/atlas: CC-BY-SA-4.0
- 데이터: 원 source 라이센스 준수 (Cremona → GPL-compatible 등)

---

## 기여자 행동 강령

- Welcoming, respectful, collaborative
- 정직성 우선 — "안다" 보다 "확인했다" 를 선호
- 과장된 주장 지양 — "해결" 대신 "실측" / "조건부" / "추측" 정확 표기

---

*v1.0 — 2026-04-16 (v3 M4)*
*HOLD 상태: maintainer 승인 후 실제 이슈 템플릿 활성화*
