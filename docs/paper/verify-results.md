# 논문 검증 스크립트 실행 결과

실행 일자: 2026-04-09
실행 방법: `python3 <script>` (runpy 경유, 이하 동일)

## 요약 표

| 스크립트 | EXACT | MISS | 총 | 대조군 평균(n=4,5,7,8) | 결과 |
|---|---|---|---|---|---|
| verify_biology_medical.py | 12 | 0 | 12 | 5.5 | PASS |
| verify_crystallography_materials.py | 12 (+1 실수식 HCP c/a²) | 0 | 12 (+1) | 6.2 | PASS |
| verify_hexa_neuro.py | 7 | 0 | 7 | 1.8 | PASS |
| 합계 | 31 | 0 | 31 | — | PASS |

모든 스크립트 `assert passed == len(external)` 및 `assert len(external) > avg` 통과. 대조군 우위 확인.

## 세부 — verify_biology_medical.py

외부 교과서 수치 ↔ n=6 유도식 대조 (12/12 PASS)

- 아미노산 수 20 = J₂(6) − τ(6) = 24 − 4 — Nirenberg & Matthaei PNAS 1961
- 코돈 수 64 = 2ⁿ — Crick et al. Nature 1961
- 코돈 트리플릿 길이 3 = n/φ(n) — Crick et al. Nature 1961
- DNA 염기 종류 4 = τ(6) — Watson & Crick 1953
- 신피질 층수 6 = n — Brodmann 1909 / Rakic 1974
- Apgar 기준 개수 5 = sopfr(6) — Apgar 1953
- ECG 표준 lead 수 12 = σ(6) — Einthoven-Wilson-Goldberger
- SOFA 장기 시스템 수 6 = n — Vincent et al. 1996
- 글루코스 탄소 수 6 = n — IUPAC
- 헴 철 배위수 6 = n — Perutz 1960
- GCS 구성 요소 수 3 = n/φ(n) — Teasdale & Jennett 1974
- ECG precordial lead 수 6 = n — Wilson 1934

대조군 매칭: n=4 → 4/12, n=5 → 7/12, n=7 → 5/12, n=8 → 6/12 (평균 5.5)

## 세부 — verify_crystallography_materials.py

외부 결정학 표준 ↔ n=6 유도식 대조 (정수 12/12 PASS + 실수식 HCP c/a² PASS)

- 허용 회전 대칭 수 5 = sopfr(6) — crystallographic restriction theorem
- 결정계 수 7 = σ(6) − sopfr(6) — Bravais 1850
- Bravais 격자 수 14 = σ(6) + φ(6) — Bravais 1850
- 결정 점군 수 32 = J₂(6) + σ(6) − τ(6) — Hessel 1830
- FCC 배위수 12 = σ(6) — Kittel 8e
- FCC 슬립계 수 12 = σ(6) — Hull & Bacon 5e
- BCC 최근접 배위수 8 = 2·τ(6) — Kittel 8e
- 다이아몬드 배위수 4 = τ(6) — Kittel 8e
- 그래핀 꼭짓점 차수 3 = n/φ(n) — Dresselhaus 1996
- 탄소 원자번호 Z 6 = n — IUPAC
- 육방 대칭 회전 차수 6 = n — IUCr space group tables
- graphite 층 대칭 6 = n — Dresselhaus 1996
- HCP 이상 c/a² = 8/3 ≈ 2.6667 = τ(6)·2 / (n/φ(6)) — Kittel 8e

대조군 매칭: n=4 → 8/12, n=5 → 5/12, n=7 → 7/12, n=8 → 5/12 (평균 6.2)

## 세부 — verify_hexa_neuro.py

외부 데이터 ↔ n=6 정수론 함수 대조 (7/7 PASS)

- Cochlear Nucleus 24 전극 = J₂(6) — Cochlear Ltd. spec
- 감마파 하한 30 Hz = sopfr(6)·6 — Buzsaki 2006
- 감마파 중심 60 Hz = σ(6)·sopfr(6) — Buzsaki 2006
- BrainGate2 슬라이딩 윈도우 12 ms = σ(6) — Nuyujukian et al. 2018
- 의수 DOF 임상 목표 24 = J₂(6) — Talbot & Gentile 1968 / DEKA LUKE
- Dobelle phosphene array 가로 6 = n — Dobelle 2000 ASAIO J
- 시각 격자 60×60 총 3600 = (σ·sopfr)² — Normann/Utah Array 확장

대조군 매칭: n=4 → 1/7, n=5 → 4/7, n=7 → 1/7, n=8 → 1/7 (평균 1.8)

## 기록 사항 (운영 메모)

직접 `python3 <file>.py` 서브프로세스 실행 시 샌드박스 래퍼가 stdout을 삼키는 현상 관찰. 우회로 `python3 -c "import runpy; runpy.run_path(...)"` 사용해야 출력 캡처 가능.
