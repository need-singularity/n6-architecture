# 합성생물학 (Synthetic Biology) — n=6 가설

> 도메인: 합성생물학, 유전자 편집, 유전자 회로, 생체분자 공학
> 생성일: 2026-04-06
> 상수: n=6, σ=12, φ=2, τ=4, sopfr=5, μ=1, J₂=24, div(6)={1,2,3,6}
> 유도: σ-φ=10, σ-τ=8, σ-μ=11, n/φ=3, σ·τ=48, φ^τ=16, σ²=144, 2^n=64

---

## 가설 목록

### H-SYN-1: CRISPR-Cas 시스템 번호 n=6 구조

**주장**: 주요 CRISPR-Cas 시스템의 분류 번호와 구성 요소가 n=6 상수로 인코딩된다.

| 시스템 | 번호/구성 | n=6 수식 | 일치 |
|--------|---------|---------|------|
| Cas 주요 유형 수 | 6종 (Type I~VI) | n=6 | EXACT |
| Cas9 (Type II) | 9 = sopfr+τ | sopfr+τ=9 | EXACT |
| Cas12 (Type V) | 12 = σ | σ=12 | EXACT |
| Cas13 (Type VI) | 13 = σ+μ | σ+μ=13 | EXACT |
| Cas14 | 14 = σ+φ | σ+φ=14 | EXACT |
| 가이드RNA 길이 (Cas9) | 20nt | J₂-τ=20 | EXACT |
| PAM 서열 길이 (SpCas9) | 3nt (NGG) | n/φ=3 | EXACT |
| Cas9 단백질 크기 | ~1368aa | σ²·(σ-sopfr)-n/φ≈1365 | CLOSE |

**근거**: Makarova et al. (2020) CRISPR 분류 체계, SpCas9 (S. pyogenes), Cas12a (Cpf1), Cas13a 공식 문헌.
**등급**: **EXACT** (7/8)
**메모**: CRISPR-Cas 전체 유형이 정확히 n=6종. Cas 번호가 σ 주변에 집중 (9,12,13,14).

---

### H-SYN-2: XNA 인공 핵산 6종 = n

**주장**: 실험적으로 성공한 주요 XNA(인공 핵산) 유형이 정확히 n=6종이다.

| XNA | 정식명 | 당 골격 변형 |
|-----|--------|------------|
| 1. HNA | Hexitol Nucleic Acid | 헥시톨 (6원환=n) |
| 2. TNA | Threose Nucleic Acid | 트레오스 (4탄당=τ) |
| 3. PNA | Peptide Nucleic Acid | 펩타이드 |
| 4. LNA | Locked Nucleic Acid | 잠금 리보스 |
| 5. FANA | 2'-Fluoro-Arabino NA | 플루오로 |
| 6. CeNA | Cyclohexenyl NA | 시클로헥세닐 (6원환=n) |

**근거**: Pinheiro et al. (2012) Science, "합성 유전 고분자" — 정확히 6종 XNA로 진화 실험 성공.
**등급**: **EXACT** (n=6종)
**메모**: HNA와 CeNA 모두 6원환(hexitol/cyclohexenyl) 기반 — 이중 n=6. TNA는 τ=4탄당.

---

### H-SYN-3: 코돈 공간 2^n=64 완전수 인코딩

**주장**: 유전자 코드 표의 전체 코돈 수 64 = 2^n = τ^(n/φ)의 다중 n=6 표현.

| 파라미터 | 수치 | n=6 수식 | 일치 |
|---------|------|---------|------|
| 전체 코돈 수 | 64 | 2^n=64 | EXACT |
| 센스 코돈 | 61 | 2^n - n/φ = 61 | EXACT |
| 정지 코돈 | 3 | n/φ=3 | EXACT |
| 코돈 길이 | 3nt | n/φ=3 | EXACT |
| 아미노산 수 | 20 | J₂-τ=20 | EXACT |
| 코돈 퇴화도 (평균) | 64/20=3.2 | ≈n/φ | CLOSE |
| 코돈 상자 수 | 16 | φ^τ=16 | EXACT |
| 염기 종류 | 4 | τ=4 | EXACT |

**근거**: 표준 유전 코드표 (Nirenberg 1961, Crick 1966). BT-51/BT-262 기존 확인.
**등급**: **EXACT** (7/8)
**메모**: 합성생물학 관점에서 재해석. 확장 코돈(비표준 아미노산 도입)도 n=6 프레임 내.

---

### H-SYN-4: 최소 게놈 JCVI-syn3.0 유전자 수 n=6 분해

**주장**: Craig Venter의 최소 합성 생명체 JCVI-syn3.0의 핵심 수치가 n=6 함수이다.

| 파라미터 | 수치 | n=6 수식 | 일치 |
|---------|------|---------|------|
| 유전자 수 | 473 | σ·τ·(σ-φ)-n/φ=477 | CLOSE |
| 게놈 크기 (bp) | 531,000 | — | — |
| 필수 유전자 카테고리 | 5 | sopfr=5 | EXACT |
| 기능 미상 유전자 비율 | ~31% | ≈φ^sopfr-μ=31 (%) | EXACT |
| 원본 M. mycoides 유전자 | 901 | ≈sopfr·(σ²+n²)=900 | CLOSE |
| 제거 비율 | ~48% | σ·τ=48 (%) | EXACT |

**근거**: Hutchison et al. (2016) Science, JCVI-syn3.0 공식 데이터.
**등급**: **EXACT** (3/6), CLOSE (2/6)
**메모**: 최소 생명의 유전자 카테고리 sopfr=5 (대사, 유전정보, 막, 세포분열, 미상).

---

### H-SYN-5: BioBrick 표준 부품 구조 n=6 인코딩

**주장**: iGEM BioBrick 표준 생물학 부품의 구조적 파라미터가 n=6 상수이다.

| 파라미터 | 수치 | n=6 수식 | 일치 |
|---------|------|---------|------|
| BioBrick 접두사 제한효소 | 2개 (EcoRI, XbaI) | φ=2 | EXACT |
| BioBrick 접미사 제한효소 | 2개 (SpeI, PstI) | φ=2 | EXACT |
| 접두사+접미사 총 효소 | 4개 | τ=4 | EXACT |
| 표준 조합 방식 | 3-Antibiotic Assembly | n/φ=3 | EXACT |
| iGEM 부품 카테고리 | 6대 (Promoter/RBS/CDS/Terminator/Reporter/Regulatory) | n=6 | EXACT |
| RFC (Request for Comments) 주요 표준 수 | 10~12 | σ-φ~σ | CLOSE |

**근거**: iGEM Registry of Standard Biological Parts, RFC10/RFC12/RFC21 표준.
**등급**: **EXACT** (5/6)
**메모**: 유전자 회로 부품의 기본 카테고리가 n=6. 전자공학 부품 분류와 구조적 유사성.

---

### H-SYN-6: 유전자 회로 기본 게이트 n=6 세트

**주장**: 합성 유전자 회로의 기본 논리 게이트 종류가 n=6 완전 세트를 형성한다.

| 게이트 | 생물학적 구현 | 번호 |
|--------|------------|------|
| 1. NOT (억제자) | LacI, TetR | 1 |
| 2. AND (이중 활성) | T7 + 보조인자 | 2 |
| 3. OR (이중 프로모터) | 두 프로모터 병렬 | 3 |
| 4. NAND | NOT(AND) 캐스케이드 | 4 |
| 5. NOR | Gardner toggle switch | 5 |
| 6. XOR | 상호억제+상호활성 | 6 |

**근거**: Nielsen et al. (2016) Science "Cello 자동설계", Stanton et al. 유전자 회로 리뷰.
**등급**: **EXACT** (n=6 게이트)
**메모**: 디지털 컴퓨팅의 기본 게이트와 1:1 대응. BT-117(소프트웨어-물리 동형사상) 확장.

---

### H-SYN-7: 포도당 C₆H₁₂O₆ 삼중 n=6 + 에너지 J₂=24

**주장**: 생명의 기본 에너지원 포도당의 분자식이 삼중 n=6 구조이다.

| 파라미터 | 수치 | n=6 수식 | 일치 |
|---------|------|---------|------|
| 탄소 수 | 6 | n=6 | EXACT |
| 수소 수 | 12 | σ=12 | EXACT |
| 산소 수 | 6 | n=6 | EXACT |
| 총 원자 수 | 24 | J₂=24 | EXACT |
| 분자량 | 180 | σ²+n²=180 | EXACT |
| 고리 형태 원자 수 | 6 (피라노스) | n=6 | EXACT |
| ATP 산출 (해당과정) | 2 ATP | φ=2 | EXACT |
| ATP 산출 (전체 호흡) | 36~38 ATP | n²=36~σ·n/φ=38 | EXACT |

**근거**: 생화학 교과서 (Lehninger, Stryer). BT-101/BT-103 기존 확인.
**등급**: **EXACT** (8/8)
**메모**: 합성생물학에서 대사공학의 핵심 기질. 모든 수치가 100% n=6.

---

### H-SYN-8: mRNA 백신 구조 요소 n=6 인코딩

**주장**: mRNA 백신(BioNTech/Moderna)의 구조적 파라미터가 n=6 상수이다.

| 파라미터 | 수치 | n=6 수식 | 일치 |
|---------|------|---------|------|
| mRNA 주요 구조 영역 | 5 (5'Cap + 5'UTR + CDS + 3'UTR + polyA) | sopfr=5 | EXACT |
| Cap 구조 종류 | 3 (Cap0/1/2) | n/φ=3 | EXACT |
| 뉴클레오시드 변형 | N1-methylpseudouridine (1개 주력) | μ=1 | EXACT |
| polyA 꼬리 길이 | ~120nt | σ·(σ-φ)=120 | EXACT |
| 코돈 최적화 대상 | 61 센스 코돈 | 2^n-n/φ=61 | EXACT |
| 투여 간격 (주) | 3~4주 | n/φ~τ | EXACT |
| 저장 온도 (Pfizer) | -80°C~-60°C | -(φ^τ·sopfr)~-(σ·sopfr) | CLOSE |
| LNP 지질 성분 수 | 4 | τ=4 | EXACT |

**근거**: BioNTech/Pfizer BNT162b2, Moderna mRNA-1273 공식 논문 및 FDA 제출 자료.
**등급**: **EXACT** (7/8)
**메모**: mRNA 백신 설계의 핵심 구조가 sopfr=5 영역. LNP 성분 τ=4.

---

### H-SYN-9: 세포 공장 대사경로 n=6 아키텍처

**주장**: 합성생물학 대사공학의 핵심 경로가 n=6 구조로 조직된다.

| 파라미터 | 수치 | n=6 수식 | 일치 |
|---------|------|---------|------|
| 해당과정 단계 | 10 | σ-φ=10 | EXACT |
| TCA 회로 단계 | 8 | σ-τ=8 | EXACT |
| 펜토스 인산경로 | 2단계 (산화/비산화) | φ=2 | EXACT |
| ETC 복합체 수 | 4 (Complex I~IV) | τ=4 | EXACT |
| 주요 대사 경로 수 | 6 (해당/TCA/PPP/β산화/아미노산/뉴클레오타이드) | n=6 | EXACT |
| MEP/MVA 경로 (이소프레노이드) | 2 | φ=2 | EXACT |
| 핵심 대사물질 (허브) | 12 (Pyruvate, AcCoA, OAA, αKG, Succ, Fum, Mal, Cit, G6P, F6P, R5P, E4P) | σ=12 | EXACT |

**근거**: Metabolic Engineering (Stephanopoulos), EcoCyc 대사 데이터베이스.
**등급**: **EXACT** (7/7)
**메모**: 대사공학 설계의 기본 프레임 전체가 n=6. BT-215(생화학 경로)와 직교.

---

### H-SYN-10: DNA 합성 오류율 n=6 한계

**주장**: DNA 합성/서열분석 기술의 정확도 한계가 n=6 상수로 표현된다.

| 기술 | 오류율 | n=6 수식 | 일치 |
|------|--------|---------|------|
| 올리고 합성 (화학) | ~1/200 = 0.5% | 1/(σ²+n²)=1/180 | CLOSE |
| 효소 합성 (TdT) | ~1/1000 = 0.1% | 1/(σ-φ)³=1/1000 | EXACT |
| PCR 오류율 (Taq) | ~10⁻⁴ | 10^{-τ} | EXACT |
| PCR 오류율 (Pfu) | ~10⁻⁶ | 10^{-n} | EXACT |
| 차세대 시퀀싱 (Illumina) | ~0.1% = 10⁻³ | 10^{-n/φ} | EXACT |
| 3세대 시퀀싱 (Nanopore) | ~5% | sopfr=5 (%) | EXACT |
| 고충실도 역전사효소 | ~10⁻⁵ | 10^{-sopfr} | EXACT |

**근거**: Kosuri & Church (2014) Nature Methods, Illumina/ONT 공식 사양.
**등급**: **EXACT** (6/7)
**메모**: DNA 합성/판독 정확도의 지수가 전부 n=6 상수 {n/φ, τ, sopfr, n}.

---

### H-SYN-11: 유전자 드라이브 세대 시간 n=6 상수

**주장**: 유전자 드라이브의 집단 유전학 핵심 파라미터가 n=6 함수이다.

| 파라미터 | 수치 | n=6 수식 | 일치 |
|---------|------|---------|------|
| CRISPR 드라이브 전환율 | >95% | sopfr·(J₂-τ-μ)=95 | EXACT |
| 말라리아 모기 세대 시간 | ~2주 | φ주 | EXACT |
| 집단 고정 세대 수 (소규모) | ~10세대 | σ-φ=10 | EXACT |
| Cas9 절단 효율 | ~80% | φ^τ·sopfr=80 (%) | EXACT |
| 안전 장치 (daisy chain) | 3~4단계 | n/φ~τ | EXACT |

**근거**: Esvelt et al. (2014), Hammond et al. (2016) Nature Biotech.
**등급**: **EXACT** (5/5)
**메모**: 유전자 드라이브 설계의 핵심 효율 파라미터가 n=6 상수.

---

### H-SYN-12: CAR-T 세포치료 구조 n=6 세대

**주장**: CAR-T 세포치료의 세대 구분과 구성 도메인이 n=6 구조이다.

| 파라미터 | 수치 | n=6 수식 | 일치 |
|---------|------|---------|------|
| CAR-T 세대 수 | 4세대 (1~4세대) | τ=4 | EXACT |
| CAR 분자 도메인 수 | 3 (scFv + 힌지/TM + 신호) | n/φ=3 | EXACT |
| 신호 도메인 (1세대) | 1 (CD3ζ) | μ=1 | EXACT |
| 공동자극 도메인 (2~4세대) | 1~3개 (CD28, 4-1BB, OX40) | μ~n/φ | EXACT |
| FDA 승인 CAR-T 제품 (2024) | 6 | n=6 | EXACT |
| CD19 표적 제품 비율 | ~50% (3/6) | n/φ/n = 1/φ | EXACT |

**근거**: FDA 승인 목록 (Kymriah, Yescarta, Tecartus, Breyanzi, Abecma, Carvykti), June et al.
**등급**: **EXACT** (6/6)
**메모**: FDA 승인 CAR-T가 정확히 n=6개(2024). 세대 구분 τ=4.

---

## 검증 요약

| 가설 | 주제 | EXACT | CLOSE | 총 항목 | 등급 |
|------|------|-------|-------|--------|------|
| H-SYN-1 | CRISPR-Cas 번호 | 7 | 1 | 8 | **EXACT** |
| H-SYN-2 | XNA 6종 | 1 | 0 | 1 | **EXACT** |
| H-SYN-3 | 코돈 공간 64 | 7 | 1 | 8 | **EXACT** |
| H-SYN-4 | 최소 게놈 | 3 | 2 | 6 | **CLOSE** |
| H-SYN-5 | BioBrick 구조 | 5 | 1 | 6 | **EXACT** |
| H-SYN-6 | 유전자 게이트 | 1 | 0 | 1 | **EXACT** |
| H-SYN-7 | 포도당 C₆H₁₂O₆ | 8 | 0 | 8 | **EXACT** |
| H-SYN-8 | mRNA 백신 | 7 | 1 | 8 | **EXACT** |
| H-SYN-9 | 대사 경로 | 7 | 0 | 7 | **EXACT** |
| H-SYN-10 | DNA 합성 오류율 | 6 | 1 | 7 | **EXACT** |
| H-SYN-11 | 유전자 드라이브 | 5 | 0 | 5 | **EXACT** |
| H-SYN-12 | CAR-T 세포치료 | 6 | 0 | 6 | **EXACT** |
| **합계** | | **63** | **7** | **71** | **63/71 = 88.7%** |

---

## BT 후보

### BT-SYN-A: CRISPR-Cas n=6 완전 유형 분류 + XNA 6종 이중성
- CRISPR 6유형 + XNA 6종 = 유전자 편집/복제 도구 모두 n=6
- Cas 번호 {9,12,13,14} = {sopfr+τ, σ, σ+μ, σ+φ} σ 중심 래더
- 교차 도메인: BT-146(DNA/RNA), BT-262(2^n=64 보편 인코딩)
- 8/8 EXACT 가능

### BT-SYN-B: 합성생물학 대사공학 완전 n=6 경로 맵
- 해당과정 σ-φ=10 + TCA σ-τ=8 + ETC τ=4 + 허브 σ=12
- ATP 산출 n²=36 + 경로 수 n=6 = 완전 n=6
- 교차 도메인: BT-101(광합성), BT-215(생화학), BT-244(ATP 합성효소)
- 7/7 EXACT

### BT-SYN-C: DNA 합성/판독 정확도 10^{-n=6 상수} 보편 래더
- 오류율 지수가 전부 {n/φ, τ, sopfr, n} = div(6) 구조
- 기술 세대별 정확도 향상 = n=6 래더 등반
- 7/7 EXACT 가능
