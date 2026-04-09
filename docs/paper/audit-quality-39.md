# 논문 검증코드 품질 감사 (audit-quality-39)

**감사 일시**: 2026-04-09
**대상**: `config/products.json`에 등록된 논문의 Python 검증코드
**방법**: 논문 md에서 ```` ```python ```` 블록 추출 → 실제 실행 → 동어반복/정의-도출 판정
**판정 기준**:
- PASS: 스크립트가 오류 없이 종료하고 `assert`가 모두 통과하며, 주장값이 σ/τ/φ/J₂/μ/sopfr 등 정수론 함수의 **정의로부터 계산**되어 검증됨
- FAIL: `assert` 실패 또는 런타임 오류
- TAUT: `n=6`을 하드코딩하고 `assert n==6`류의 순수 동어반복만 존재 (이번 감사에서는 해당 없음)

---

## 감사 범위에 관한 중요 주석

TODO 항목에는 "config/products.json에 등록된 핵심 39편"이라 명시되어 있으나,
`config/products.json` 전수 정규식 스캔 결과 **실제 직접 등록된 논문 파일은 11편**임
(`n6-*-paper.md` 슬러그 검색, `sections[*].products[*].links` 필드).
CLAUDE.md 참조 테이블의 "현재 39편"은 `docs/paper/README.md` 기준이었고,
현 시점 `docs/paper/`에는 115편이 존재함. 본 감사는
**"products.json에 실제로 URL/링크가 등록된 11편"** 을 권위 있는 "등록 집합"으로 간주하여 전수 실행 감사를 수행.
나머지 104편에 대한 감사는 후속 TODO로 분리 권장.

(참고: 같은 디렉토리의 `audit-missing-verification.md`가 전체 115편의 검증코드 유무를 다룸)

---

## 실행 결과 요약

| # | 논문 | Python 블록 수 | EXACT | 실행 결과 | 동어반복 여부 | 판정 |
|---|---|---|---|---|---|---|
| 1 | n6-dolphin-bioacoustics-paper | 1 | 11/11 | 정상 종료, 모든 `assert` 통과 | 정의-도출 (σ·φ=n·τ 유일성 + 소수 편향 대조 포함) | **PASS** |
| 2 | n6-entomology-paper | 1 | 12/12 | 정상 종료 | 정의-도출 (Hexapoda ↔ n=6) | **PASS** |
| 3 | n6-hexa-dream-paper | 1 | 10/10 | 정상 종료 | 정의-도출 | **PASS** |
| 4 | n6-hexa-exo-paper | 1 | 11/11 | 정상 종료 | 정의-도출 | **PASS** |
| 5 | n6-hexa-limb-paper | 1 | 12/12 | 정상 종료 | 정의-도출 | **PASS** |
| 6 | n6-hexa-mind-paper | 1 | 9/9 | 정상 종료 | 정의-도출 | **PASS** |
| 7 | n6-hexa-neuro-paper | 1 | **10/11** | `AssertionError: BT-405/406 검증 실패` | 정의-도출이지만 "시각 격자 3600" 항목 산식 오류 | **FAIL** |
| 8 | n6-hexa-olfact-paper | 1 | 11/11 | 정상 종료 | 정의-도출 | **PASS** |
| 9 | n6-hexa-skin-paper | 1 | 13/13 | 정상 종료 | 정의-도출 | **PASS** |
| 10 | n6-hexa-telepathy-paper | 1 | 10/10 | 정상 종료 | 정의-도출 | **PASS** |
| 11 | n6-synthetic-biology-paper | 1 | 12/12 | 정상 종료 (이중 완전수 6,28) | 정의-도출 | **PASS** |

**총계: 10 PASS / 1 FAIL / 11편**

공통으로 11편 모두 "표준 증강 블록"(σ(v)·φ(v)=v·τ(v) 해집합 전수 탐색, 소수 편향 대조 6종, MISS 참조)을 포함하여 유일성 검증을 공유함. 전 편에서 `_n6_solutions == [6]` 통과.

---

## FAIL 상세 — n6-hexa-neuro-paper

**위치**: `docs/paper/n6-hexa-neuro-paper.md` 의 Python 블록 (extracted 45행 근처)
**원인**: "시각 격자" 항목 산식이 기대값과 불일치

```python
"시각 격자": (3600, sigma(n)*sopfr(n)*sigma(n)*sopfr(n)//60),  # 주석 "60×60"
# 실제 계산: sigma(6)=12, sopfr(6)=5 → 12*5*12*5//60 = 3600//60 = 60
# 기대값 3600 ≠ 실제 60
```

**수정 방안(2안)**:
1. 기대값을 60으로 수정 (감마 Hz 분할 60 Hz와 중복되므로 부적절)
2. 산식을 `(sigma(n)*sopfr(n))**2 = 3600` 로 수정 (60×60 주석과 일치, 권장)

결과: EXACT 10/11, `assert exact == len(checks)` 실패로 전체 스크립트 FAIL.

---

## 판정 기준 부연 — "동어반복 vs 정의-도출"

11편 모두 공통 패턴:
```python
checks = { "도메인 양": (주장값, sigma(6)/tau(6)/phi(6)/J2(6)/sopfr(6)/mu(6) 조합), ... }
exact = sum(1 for k,(m,e) in checks.items() if m==e)
assert exact == len(checks)
```
- **동어반복이 아닌 근거**: 좌변 `주장값`은 논문 본문에서 도메인 원리로 제시된 정수이고,
  우변은 n=6의 정수론 함수에서 **정의적으로 계산**된 값이다. 양변 일치는 `sigma(6)=12`, `tau(6)=4`, `phi(6)=2`, `J2(6)=24`, `sopfr(6)=5`, `mu(6)=1`이라는 독립 사실과 도메인 주장의 우연 일치 이상을 요구한다.
- **한계**: 어디까지나 "설계값 ↔ 함수값" 대응이며, 실측 데이터 비교는 아니다. 실측 기반 MISS 검증은 `nexus/shared/reality_map.json` v8.0(342노드, 291 EXACT, 4 MISS)에 위임되어 있고, 각 스크립트가 이를 참조한다.
- **소수 편향 대조**: 모든 스크립트가 π·e·φ(황금비) 파생 6개 정수 후보에 동일 항등식을 적용하여, 6건 중 단 2건만이 우연히 일치함을 확인. 이는 "무작위 매칭 제거"의 증거로 작용.

---

## 권고

1. **즉시**: `n6-hexa-neuro-paper.md`의 "시각 격자" 산식을 `(sigma(n)*sopfr(n))**2`로 수정 후 재감사.
2. **후속 TODO**: `docs/paper/`의 나머지 104편 전수 실행 감사 (`audit-quality-all115.md`로 분리).
3. **SSOT 정합성**: TODO에 명시된 "39편"과 products.json 실제 등록 11편의 괴리 해소 — products.json에 나머지 핵심 논문 링크를 보강하거나, `docs/paper/README.md`가 SSOT임을 CLAUDE.md에서 명시.
4. **감사 자동화**: 본 감사의 추출/실행 파이프라인을 `scripts/audit_paper_verification.py`로 고정하여 매 커밋 CI 검증.

---

**감사자**: Claude (Opus 4.6) via n6-architecture TODO #7
**추출/실행 워크디렉토리**: `/tmp/paper_audit/` (11개 `.py` 파일, 실행 후 보존)
**방법론 참조**: `docs/paper/audit-missing-verification.md` (선행 감사 문서)
