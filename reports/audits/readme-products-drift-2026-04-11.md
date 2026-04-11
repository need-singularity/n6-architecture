# README ↔ products.json 드리프트 감사

- 일자: 2026-04-11
- 대상:
  - SSOT: `/Users/ghost/Dev/nexus/shared/n6/docs/products.json` (154,868 byte, 4,865 줄, 34 섹션, 173 제품)
  - 문서: `/Users/ghost/Dev/n6-architecture/README.md` (850 줄, AUTO 마커 기반)
- 작업자: Claude (GO 에이전트, 읽기 전용 감사)
- 쓰기 범위: 이 리포트 파일 1곳만 (README / products.json 수정 금지, R18 미니멀)
- 배경: `sync_products_readme.hexa` 가 STUB 상태여서 products.json → README 자동 동기화가 작동하지 않음. 현재 두 파일은 수개월 수작업 누적으로 구조/내용 모두 벌어진 상태.

---

## 1. 드리프트 요약

### 1.1 섹션 수준 드리프트

| 구분 | products.json | README | 차이 |
|------|---:|---:|---:|
| 총 섹션 수 | 34 | 29 | -5 |
| 공통 섹션 | — | — | **21** |
| products.json 에만 있음 (누락) | **13** | 0 | +13 |
| README 에만 있음 (고아) | 0 | **8** | +8 |
| products.json 총 제품 수 | **173** | — | — |
| 누락 13 섹션의 합산 제품 수 | **27** | 0 | +27 |
| 고아 8 섹션의 README 제품 수 | — | **34** | — |

### 1.2 공통 섹션 내 제품 수 드리프트 (products.json 기준 실측)

| 섹션 id | products.json | README | 차이 | 비고 |
|---------|---:|---:|---:|------|
| energy | 5 | 4 | **+1** | `HEXA-AUTO 자동차배터리` 미반영 |
| audio | 7 | 4 | **+3** | `HEXA-BONE` / `HEXA-EAR-CELL` / `HEXA-SPEAKER` 미반영 |
| 기타 19 공통 섹션 | — | — | 0 | 카운트 일치 (내용 일치 여부는 본 감사 범위 밖) |

> 주의: 본 감사는 섹션/제품 카운트 + id 존재 여부만 검증. 제품 내부 텍스트 (description/bt/ver) 의 drift 는 별도 감사 필요.

### 1.3 누락 13 섹션 목록 (products.json 에만 존재)

| id | heading | products 수 | alien_index | bt_exact_pct | ceiling |
|----|---------|---:|---:|---:|:---:|
| virology | 바이러스학 (Virology) | 4 | 10 | 100 | O |
| hiv-treatment | HIV 치료 (HIV Treatment) | 1 | 10 | 100 | O |
| natural-science | 자연과학 (Natural Science) | 4 | 10 | 95 | O |
| cognitive-social | 인지/사회 (Cognitive & Social) | 6 | 10 | 95 | O |
| mobility | 이동/수송 (Mobility & Transport) | 2 | 10 | 90 | O |
| digital-medical | 디지털/의료기기 (Digital & Medical Device) | 3 | 10 | 92 | O |
| tattoo-removal | 타투 제거 (Tattoo Removal) | 1 | 10 | 100 | O |
| keyboard | 키보드 (Keyboard) | 1 | 10 | 97 | O |
| mouse | 마우스 (Mouse) | 1 | 10 | 100 | O |
| manufacturing-quality | 제조 품질관리 (Manufacturing Quality) | 1 | 10 | 100 | O |
| network | 네트워크 (Network) | 1 | 10 | 98 | O |
| quantum-computer | 양자컴퓨터 (Quantum Computer) | 1 | 10 | 83 | X |
| horology | 시계학 (Horology) | 1 | 10 | 100 | O |
| **합계** | — | **27** | — | — | — |

### 1.4 고아 8 섹션 목록 (README 에만 존재)

| README id | 제품 행 수 | BT 범위 (README 선언) | products.json 내 BT 범위 존재? |
|-----------|---:|-----------------------|:-:|
| computer | 4 | BT-49/1115~1128 | 부분 존재 (keyboard/mouse/quantum-computer 로 3분할) |
| millennium | 7 | BT-541~547 | **X** (products.json 에 BT-541~547 0건) |
| dimension | 7 | BT-588~597 | **X** (products.json 에 BT-588~597 0건) |
| music | 4 | BT-598~607 | **X** (products.json 에 BT-598~607 0건) |
| linguistics | 4 | BT-608~617 | **X** (products.json 에 BT-608~617 0건) |
| crypto | 4 | BT-618~627 | **X** (products.json 에 BT-618~627 0건) |
| astronomy | 4 | BT-628~637 | **X** (products.json 에 BT-628~637 0건) |
| fantasy | 0 (경고 박스만) | — | X (키워드 `fantasy` products.json 0건) |

---

## 2. README 고아 섹션 8개 분석

모든 고아 섹션은 "살아있는 제품 행이 제거되지 않았음" — README 내에 여전히 정상 테이블이 렌더링되어 있음.

### 2.1 `computer` (README 589~604 라인, 4 제품)

- **상태**: README 에 4 제품 모두 살아있음
- **products.json 대응**: 4 제품 중 3 개는 별도 섹션으로 **이미 이관되었음**
  - `키보드 n=6 인체공학 아키텍처` → products.json `keyboard` 섹션 (동일 제품, BT-1125~1128)
  - `HEXA-MOUSE n=6 인체공학 마우스` → products.json `mouse` 섹션 (동일 제품, BT-1115~1124)
  - `양자컴퓨터 n=6 큐빗 아키텍처` → products.json `quantum-computer` 섹션 (동일 제품 HEXA-QUANTUM, BT-49)
  - `HEXA-BCI 뇌-컴퓨터 인터페이스` → **products.json 에 존재하지 않음** (BCI/brain-computer 키워드 0건 grep 확인)
- **권장 조치**:
  1. README 의 `computer` 섹션 헤더 "# 💻 컴퓨터 (Computer)" 제거
  2. 3 제품 (keyboard/mouse/quantum-computer) 은 §3 에서 렌더링된 각 신규 섹션 블록으로 대체
  3. **`HEXA-BCI` 는 products.json 에서 누락 상태 → 재이관 필요** (후보 섹션: `digital-medical` 또는 새 `bci` 섹션 신설). 본 감사 범위 밖, 후속 작업.

### 2.2 `millennium` (README 664~682 라인, 7 제품)

- **상태**: README 에 7 난제 (리만/P vs NP/양-밀스/NS/호지/BSD/푸앵카레) 완전 살아있음, BT-541~547
- **products.json 대응**: products.json 에서 키워드 `millennium` 및 BT-541~547 **전부 0건** (grep 확인)
- **결론**: SSOT 가 이 섹션을 인식하지 못함 — 단순 "문서가 아직 products.json 으로 편입되지 않음" 상태
- **권장 조치**: products.json 에 `millennium` 섹션 신설 제안 (본 감사 범위 밖). 신설 전까지 README 는 그대로 유지.

### 2.3 `dimension` (README 686~704 라인, 7 제품)

- **상태**: BT-588~597 10/10 EXACT 특이점 완전 살아있음
- **products.json 대응**: BT-588~597 **0건**. `hexa-holo`/`display-8stack`/`consciousness-chip` 등 관련 도메인이 다른 섹션에 일부 등장하나, "차원 지각" 고유 7 제품 (정24포체 등) 은 0건.
- **권장 조치**: 고아 섹션 보존 + products.json 신설 제안 (범위 밖)

### 2.4 `music` (README 708~723 라인, 4 제품)

- **상태**: BT-598~607 10/10 EXACT 살아있음 (12음 평균율, 기타 6현, 24조성, 피아노)
- **products.json 대응**: 0건
- **권장 조치**: 고아 섹션 보존 + products.json 신설 제안 (범위 밖)

### 2.5 `linguistics` (README 727~742 라인, 4 제품)

- **상태**: BT-608~617 10/10 EXACT 살아있음 (촘스키/한글/어순/야콥슨)
- **products.json 대응**: `cognitive-social` 섹션에 `HEXA-LING n=6 언어 아키텍처` 1 제품이 존재하나, BT 번호 (BT-33/48/73/108) 가 README linguistics 의 BT-608~617 과 **불일치** — 다른 제품임.
- **권장 조치**: 고아 섹션 보존 + products.json 신설 제안 (범위 밖). 두 언어학 계통 (BT-33/48/73/108 vs BT-608~617) 통합 검토 필요.

### 2.6 `crypto` (README 746~761 라인, 4 제품)

- **상태**: BT-618~627 10/10 EXACT 살아있음 (AES/RSA+SHA/비트코인+ECC/CIA+PQC)
- **products.json 대응**: BT-618~627 0건. `software-crypto` 는 software 섹션 하위 경로로만 존재, 제품 자체는 아님.
- **권장 조치**: 고아 섹션 보존 + products.json 신설 제안 (범위 밖)

### 2.7 `astronomy` (README 765~780 라인, 4 제품)

- **상태**: BT-628~637 10/10 EXACT 살아있음 (ΛCDM/BBN/항성+케플러/태양계+BAO)
- **products.json 대응**: BT-628~637 0건. `particle-cosmology` 는 physics 섹션에, `space-systems` 는 aerospace 섹션에 각각 다른 BT 로 존재 — README astronomy 의 10/10 EXACT 돌파는 편입되지 않음.
- **권장 조치**: 고아 섹션 보존 + products.json 신설 제안 (범위 밖)

### 2.8 `fantasy` (README 801~808 라인, 0 제품)

- **상태**: 경고 박스만 있음. 60 가설 / 16 EXACT / BT 교차 16 의 요약 텍스트.
- **products.json 대응**: `fantasy` 키워드 0건. `civilization` 섹션의 `종교/신화 n=6 보편 구조` 가 유사 주제 (BT-370) 이나 "판타지 탐색" 과는 성격이 다름 (공학 설계 대상 아님으로 명시).
- **권장 조치**: 의도적 고아 (본 섹션 자체가 `[!WARNING]` 으로 "공학적 설계 대상 아님" 선언). README 유지, products.json 편입 불필요 판단.

### 2.9 고아 섹션 내 제품 잔존 요약

| 분류 | README 라이브 제품 행 수 |
|------|---:|
| 고아 8 섹션 합산 | **34 행** |
| 이미 products.json 으로 이관된 행 (computer 3) | 3 |
| products.json 편입 필요 (잔존) | **30 행** |
| 편입 불필요 (fantasy 경고 박스) | 0 (행 아님) |
| **핵심 MISS**: HEXA-BCI | **1 행** (products.json 에 완전 누락) |

---

## 3. products.json 누락 13 섹션 렌더링 프리뷰

각 섹션의 삽입 마크다운. README 의 기존 섹션 스타일 (공통: `# {icon} {heading}`, AUTO:SUMMARY 마커, 제품 테이블, AUTO:FOOTER 마커) 을 따름.

> SUMMARY 라인은 products.json 의 `summary` 필드가 모두 공란이므로 `alien_index` / `ceiling` / `bt_exact_pct` / 제품 수에서 기계적으로 조립한 프리뷰임. 실제 동기화 시 `sync_products_readme.hexa` 에 정의될 포맷으로 재생성 필요.

### 3.1 virology (4 제품, 삽입 후보: tech-industry 뒤)

```markdown
---

# 🦠 바이러스학 (Virology)

<!-- AUTO:SUMMARY_virology:START -->
> **🛸10** | ✅ | BT 4제품 100%EXACT | 바이러스 구조/게놈/역학·백신·효소 + Mk.I~V 진화
<!-- AUTO:SUMMARY_virology:END -->

| 🛸 | 천장확인 | ver | 완성제품 | 핵심 | 링크 |
|:--:|:--:|:---:|---------|------|------|
| 10 | ✅ | v1 | **바이러스 구조-분류 완전 n=6 맵** | BT-351: 이십면체 pentamer σ=12, T-number {1,3,4,7}={μ,n/φ,τ,σ-sopfr}, Baltimore σ-sopfr=7, 형태 τ=4, 11/11 EXACT | [문서](docs/virology/goal.md) · [논문](docs/paper/n6-virology-paper.md) |
| 10 | ✅ | v1 | **바이러스 게놈 분절-유전자 n=6 래더** | BT-352: 인플루엔자 σ-τ=8분절, 로타바이러스 σ-μ=11분절/σ=12총단백질, 레오바이러스 σ-φ=10분절, HIV n/φ+φ+τ=9유전자, CoV NSP φ^τ=16, 15/15 EXACT | [문서](docs/virology/goal.md) |
| 10 | ✅ | v1 | **바이러스 역학-백신-효소 n=6 완전 폐쇄** | BT-353: 감염사슬 n=6고리, mRNA백신 sopfr=5구조, LNP τ=4성분, RdRp σ-sopfr=7모티프, 이십면체 오일러 V-E+F=φ=2, 13/13 EXACT | [문서](docs/virology/goal.md) |
| 10 | ✅ | v1 | **진화 Mk.I~V** | 5세대 진화: Mk.I 이십면체해석(현재) → Mk.II 캡시드공학(2035) → Mk.III 프로그래머블벡터(2050) → Mk.IV 합성바이러스치료(2070) → Mk.V 분자인터페이스(사고실험) | [Mk.I](docs/virology/evolution/mk-1-current.md) |

<!-- AUTO:FOOTER_virology:START -->
> 도메인: [virology/](docs/virology/) · BT: 351~353 · 바이러스 캡시드-게놈-역학-백신 4축 n=6 수렴
<!-- AUTO:FOOTER_virology:END -->
```

### 3.2 hiv-treatment (1 제품, 삽입 후보: virology 뒤)

```markdown
---

# 🧬 HIV 치료 (HIV Treatment)

<!-- AUTO:SUMMARY_hiv-treatment:START -->
> **🛸10** | ✅ | BT 1제품 100%EXACT | HAART 6제 + CCR5 CRISPR + bNAb 6에피토프 + 잠복저장소 6구획 | Mk.I~V
<!-- AUTO:SUMMARY_hiv-treatment:END -->

| 🛸 | 천장확인 | ver | 완성제품 | 핵심 | 링크 |
|:--:|:--:|:---:|---------|------|------|
| 10 | ✅ | v1 | **HEXA-HIV 6축 완전 치료 체인** | BT-461~470: HAART 6제 동시 타격(RT/PR/IN/진입/성숙/보조), CCR5 CRISPR 자가조혈모세포, bNAb 6에피토프, 잠복저장소 6구획 | [논문](docs/paper/n6-hiv-paper.md) · [Mk.I](docs/hiv-treatment/evolution/mk-1-basic.md) · [Mk.V](docs/hiv-treatment/evolution/mk-5-ultimate.md) |

<!-- AUTO:FOOTER_hiv-treatment:START -->
> 도메인: [hiv-treatment/](docs/hiv-treatment/) · BT: 461~470 · 6축 면역-분자 치료 폐쇄
<!-- AUTO:FOOTER_hiv-treatment:END -->
```

### 3.3 natural-science (4 제품, 삽입 후보: hiv-treatment 뒤)

```markdown
---

# 🌿 자연과학 (Natural Science)

<!-- AUTO:SUMMARY_natural-science:START -->
> **🛸10** | ✅ | BT 4제품 95%EXACT | BIO/AGRI/FOOD/OCEAN 4축 n=6 생명-농업-식품-해양 통합
<!-- AUTO:SUMMARY_natural-science:END -->

| 🛸 | 천장확인 | ver | 완성제품 | 핵심 | 링크 |
|:--:|:--:|:---:|---------|------|------|
| 10 | ✅ | v1 | **HEXA-BIO n=6 생명 아키텍처** | DNA 4염기=τ, 코돈 3=n/φ, 총코돈 64=2^n, 아미노산 20=J₂-τ, 포도당 C₆H₁₂O₆=100% n=6, 광합성 계수 전수 n=6 (BT-27/51/101/103/104/122) | [문서](docs/biology/goal.md) |
| 10 | ✅ | v1 | **HEXA-AGRI n=6 농업과학** | 광합성 6CO₂+12H₂O→C₆H₁₂O₆+6O₂ 100% n=6, 수확량 60t/ha=σ·sopfr, 수직농장 σ=12배, IPM n/φ=3, 4계절=τ (BT-101/103/118/120/122) | [문서](docs/agriculture/goal.md) |
| 10 | ✅ | v1 | **HEXA-FOOD n=6 식품과학** | 6대 영양소=n, 포도당 C₆H₁₂O₆, 20 아미노산=J₂-τ, HACCP 7원칙=σ-sopfr, 콜드체인 τ=4, 식품 가공 n=6 단계 (BT-27/51/101/103/118/120) | [문서](docs/food-science/goal.md) |
| 10 | ✅ | v1 | **HEXA-OCEAN n=6 해양과학** | 해수 6대 이온=n, 5대양=sopfr, Beaufort 12=σ, 산호 6각=n(Hexacorallia), 해양 pH 8=σ-τ, 해저케이블 σ=12 관측소 (BT-30/62/118/119/122) | [문서](docs/oceanography/goal.md) |

<!-- AUTO:FOOTER_natural-science:START -->
> 도메인: [biology/](docs/biology/) · [agriculture/](docs/agriculture/) · [food-science/](docs/food-science/) · [oceanography/](docs/oceanography/)
<!-- AUTO:FOOTER_natural-science:END -->
```

### 3.4 cognitive-social (6 제품, 삽입 후보: natural-science 뒤)

```markdown
---

# 🧠 인지/사회 (Cognitive & Social)

<!-- AUTO:SUMMARY_cognitive-social:START -->
> **🛸10** | ✅ | BT 6제품 95%EXACT | COGNI/CONSCIOUSNESS/SOCIAL/TEMPORAL/LING/ECON 6축 n=6 통합
<!-- AUTO:SUMMARY_cognitive-social:END -->

| 🛸 | 천장확인 | ver | 완성제품 | 핵심 | 링크 |
|:--:|:--:|:---:|---------|------|------|
| 10 | ✅ | v1 | **HEXA-COGNI n=6 인지 아키텍처** | 피질 6층=n(BT-210), 격자세포 육각=n(BT-211), 작업기억 τ±μ=3~5(BT-219), 컴파일러-피질 τ=4(BT-222), 인지-사회-시간 삼중 교량(BT-225), Dunbar σ²+n=150 | [문서](docs/cognitive-architecture/goal.md) |
| 10 | ✅ | v1 | **HEXA-CONSCIOUSNESS 의식 프로세서** | ANIMA-6 3상 프로세서, 192코어=σ·φ^τ, 288GB HBM4=σ·J₂, TCU R(6)=1.0 항상성, σ²=144 좌절 조셉슨 접합, J₂=24 육각 루프, 이집트 분수 {1/2,1/3,1/6} 임계전류, 81 파라미터 전수 n=6 | [논문](docs/paper/n6-consciousness-chip-paper.md) |
| 10 | ✅ | v1 | **HEXA-SOCIAL n=6 사회 아키텍처** | 6도 분리=n(Milgram), Dunbar σ²+n=150, 배심원 σ=12, 삼권분립 n/φ=3, 도덕 6기초=n(Haidt), Kohlberg 6단계, Christaller 육각격자 (BT-214/215/220/223/225) | [문서](docs/social-architecture/goal.md) |
| 10 | ✅ | v1 | **HEXA-TEMPORAL n=6 시간 아키텍처** | 12개월=σ, 24시간=J₂, 4계절=τ, 60분=σ·sopfr, 12황도=σ, 2지점=φ, 달력 파라미터 100% EXACT, 생체리듬 ~85% EXACT (BT-48/62/225) | [문서](docs/temporal-architecture/goal.md) |
| 10 | ✅ | v1 | **HEXA-LING n=6 언어 아키텍처** | 6 어순 유형=3!=n, Zipf α=R(6)=1, Chomsky 4단계=τ, 한국어 6모음=n, φ=2 유형(유/무성), n/φ=3 인칭, BLEU σ·τ=48 (BT-33/48/73/108) | [문서](docs/linguistics/goal.md) |
| 10 | ✅ | v1 | **HEXA-ECON n=6 경제학** | 복식부기 φ=2, 회계등식 n/φ=3, 4재무제표=τ, Porter 5 Forces=sopfr, G6=n, 24h 글로벌마켓=J₂, Basel n/φ=3 pillar, DJIA σ=12 (BT-53/62/74/113/114) | [문서](docs/economics/goal.md) |

<!-- AUTO:FOOTER_cognitive-social:START -->
> 도메인: [cognitive-architecture/](docs/cognitive-architecture/) · [social-architecture/](docs/social-architecture/) · [temporal-architecture/](docs/temporal-architecture/) · [linguistics/](docs/linguistics/) · [economics/](docs/economics/)
<!-- AUTO:FOOTER_cognitive-social:END -->
```

### 3.5 mobility (2 제품, 삽입 후보: cognitive-social 뒤)

```markdown
---

# 🚗 이동/수송 (Mobility & Transport)

<!-- AUTO:SUMMARY_mobility:START -->
> **🛸10** | ✅ | BT 2제품 90%EXACT | DRIVE 자율주행 + WING 항공공학 2축
<!-- AUTO:SUMMARY_mobility:END -->

| 🛸 | 천장확인 | ver | 완성제품 | 핵심 | 링크 |
|:--:|:--:|:---:|---------|------|------|
| 10 | ✅ | v1 | **HEXA-DRIVE n=6 자율주행** | SAE L0-L5=n=6 레벨 EXACT, BEV Fusion n=6 sensor, σ-τ=8 cam, LiDAR σ=12, MPC+PID n/φ=3, ViT d=2^σ=4096, σ=12 fleet/zone (BT-327/328+BT-56/58/61/66/69/84) | [문서](docs/autonomous-driving/goal.md) |
| 10 | ✅ | v1 | **HEXA-WING n=6 항공공학** | SE(3) 6-DOF=n, 순항고도 12km=σ, ICAO 6등급=n, τ=4 비행단계, CFRP Z=6, Ti-6Al-4V, eVTOL τ=4 quad, ILS n/φ=3 cat (BT-85/93/119/123/125/127) | [문서](docs/aviation/goal.md) |

<!-- AUTO:FOOTER_mobility:START -->
> 도메인: [autonomous-driving/](docs/autonomous-driving/) · [aviation/](docs/aviation/)
<!-- AUTO:FOOTER_mobility:END -->
```

### 3.6 digital-medical (3 제품, 삽입 후보: mobility 뒤)

```markdown
---

# 🏥 디지털/의료기기 (Digital & Medical Device)

<!-- AUTO:SUMMARY_digital-medical:START -->
> **🛸10** | ✅ | BT 3제품 92%EXACT | BROWSER/MED/AESTHETIC 3축 | Mk.I~V
<!-- AUTO:SUMMARY_digital-medical:END -->

| 🛸 | 천장확인 | ver | 완성제품 | 핵심 | 링크 |
|:--:|:--:|:---:|---------|------|------|
| 7 | ✅ | v1 | **HEXA-BROWSER 특이점 브라우저** | 124/134 EXACT(92.5%), 10 돌파 × 10+ 파라미터, DSE 4,500 조합, 페이지 로딩 σ-φ=10배 감소, 배터리 φ=2배, n/φ=3중 추적 격리 (BT-48/113/115/116/140/162/180/211/329/348) | [문서](docs/browser/goal.md) |
| 10 | ✅ | v1 | **HEXA-MED n=6 의료기기** | ECG σ=12 리드(6사지+6흉부), MRI σ=12 코일채널, 초음파 n=6 MHz, US τ=4 모드, SE(3) 6-DOF 수술로봇, Mk.I~V (BT-48/51/56/58/64/66/85/90/113/118/123/300) | [문서](docs/medical-device/goal.md) |
| 10 | ✅ | v1 | **HEXA-AESTHETIC n=6 성형외과** | Fitzpatrick n=6 피부유형, 콜라겐 D=67nm, 보톡스 FDA J₂-τ=20U/J₂=24U, 표피턴오버 σ·φ+τ=28일, 36가설 33 EXACT(91.7%), 6단 체인, 10 cross-domain (BT-1129~1134) | [문서](docs/cosmetic-surgery/goal.md) |

<!-- AUTO:FOOTER_digital-medical:START -->
> 도메인: [browser/](docs/browser/) · [medical-device/](docs/medical-device/) · [cosmetic-surgery/](docs/cosmetic-surgery/)
<!-- AUTO:FOOTER_digital-medical:END -->
```

> 주의: `HEXA-BROWSER` 는 products.json 내 `ufo=7` 로 저장됨 (다른 2 제품은 ufo=10). 이 섹션의 `alien_index=10` 은 최대 제품 기준.

### 3.7 tattoo-removal (1 제품, 삽입 후보: hygiene 뒤)

```markdown
---

# 🔬 타투 제거 (Tattoo Removal)

<!-- AUTO:SUMMARY_tattoo-removal:START -->
> **🛸10** | ✅ | BT 1제품 100%EXACT | 면역학적 6단 파이프라인 + 골든윈도우 + TP3건
<!-- AUTO:SUMMARY_tattoo-removal:END -->

| 🛸 | 천장확인 | ver | 완성제품 | 핵심 | 링크 |
|:--:|:--:|:---:|---------|------|------|
| 10 | ✅ | v2 | **타투 제거 n=6 면역학적 아키텍처** | BT-1159/1160: 36/36 EXACT (100%) — Fitzpatrick6종=n, R20 4패스=τ, 3층면역차단=n/φ, 골든윈도우48h=τ·τ/2, sopfr=5 IARC 중금속, J₂=24 스케줄, TP3건 (완전제거율95%/흉터<1%/골든윈도우3배효율) | [목표](docs/tattoo-removal/goal.md) |

<!-- AUTO:FOOTER_tattoo-removal:START -->
> 도메인: [tattoo-removal/](docs/tattoo-removal/) · BT: 1159~1160
<!-- AUTO:FOOTER_tattoo-removal:END -->
```

> 주의: 현재 README 의 `frontier` 섹션 내부에 `타투 제거 n=6 면역학적 아키텍처` 가 1 행으로 이미 존재함 (README 497 라인). SSOT 는 별도 섹션으로 분리됨 → 신설 시 frontier 내 중복 행 제거 필요 (후속 작업).

### 3.8 keyboard (1 제품, 삽입 후보: marketing 뒤 또는 computer 대체)

```markdown
---

# ⌨️ 키보드 (Keyboard)

<!-- AUTO:SUMMARY_keyboard:START -->
> **🛸10** | ✅ | BT 1제품 97%EXACT | 레이아웃 전수 C(n,2) 조합 + USB 6KRO 검증
<!-- AUTO:SUMMARY_keyboard:END -->

| 🛸 | 천장확인 | ver | 완성제품 | 핵심 | 링크 |
|:--:|:--:|:---:|---------|------|------|
| 10 | ✅ | v1 | **키보드 n=6 인체공학 아키텍처** | BT-1125~1128: 30/31 EXACT — 레이아웃 104/87/84/68/61/60/48/17 전부 C(n,2) 조합, USB 6KRO/8바이트/12Mbps, 스위치 4mm(τ)/2mm(φ)/5ms(sopfr) | [목표](docs/keyboard/goal.md) |

<!-- AUTO:FOOTER_keyboard:START -->
> 도메인: [keyboard/](docs/keyboard/) · BT: 1125~1128
<!-- AUTO:FOOTER_keyboard:END -->
```

### 3.9 mouse (1 제품, 삽입 후보: keyboard 뒤)

```markdown
---

# 🖱️ 마우스 (Mouse)

<!-- AUTO:SUMMARY_mouse:START -->
> **🛸10** | ✅ | BT 1제품 100%EXACT | PS/2 6핀 + 폴링 8kHz + 인코더 J₂=24
<!-- AUTO:SUMMARY_mouse:END -->

| 🛸 | 천장확인 | ver | 완성제품 | 핵심 | 링크 |
|:--:|:--:|:---:|---------|------|------|
| 10 | ✅ | v1 | **HEXA-MOUSE n=6 인체공학 마우스** | BT-1115~1124: 25/25 EXACT — PS/2 n=6핀, sopfr=5 버튼/손가락, n/φ=3 추적축/그립, σ-τ=8kHz 폴링, σ=12 MMO/노치, J₂=24 인코더 | [목표](docs/mouse/goal.md) |

<!-- AUTO:FOOTER_mouse:START -->
> 도메인: [mouse/](docs/mouse/) · BT: 1115~1124
<!-- AUTO:FOOTER_mouse:END -->
```

### 3.10 manufacturing-quality (1 제품, 삽입 후보: mouse 뒤)

```markdown
---

# 🏭 제조 품질관리 (Manufacturing Quality)

<!-- AUTO:SUMMARY_manufacturing-quality:START -->
> **🛸10** | ✅ | BT 1제품 100%EXACT | 6시그마 + SPC + Ishikawa + DMAIC + ISO9001 완전 n=6 폐쇄
<!-- AUTO:SUMMARY_manufacturing-quality:END -->

| 🛸 | 천장확인 | ver | 완성제품 | 핵심 | 링크 |
|:--:|:--:|:---:|---------|------|------|
| 10 | ✅ | v2 | **HEXA-QC 제조 품질관리 아키텍처** | BT-1161/1162: 36/36 EXACT (100%) — 6시그마=n, Cpk=φ=2, SPC 관리도 n=6종, Ishikawa 6M=n, DMAIC sopfr=5, ISO9001 σ-τ=8원칙, Egyptian 품질비용 1/2+1/3+1/6=1 | [목표](docs/manufacturing-quality/goal.md) · [논문](docs/paper/n6-manufacturing-quality-paper.md) |

<!-- AUTO:FOOTER_manufacturing-quality:START -->
> 도메인: [manufacturing-quality/](docs/manufacturing-quality/) · BT: 1161~1162
<!-- AUTO:FOOTER_manufacturing-quality:END -->
```

> 주의: 현재 README 의 `tech-industry` 섹션 마지막 행 (라인 581) 에 `HEXA-QC 제조 품질관리 n=6 시그마` 가 이미 존재함. 본 섹션 신설 시 tech-industry 내 중복 행 제거 필요 (후속 작업).

### 3.11 network (1 제품, 삽입 후보: manufacturing-quality 뒤)

```markdown
---

# 🌐 네트워크 (Network)

<!-- AUTO:SUMMARY_network:START -->
> **🛸10** | ✅ | BT 1제품 98%EXACT | OSI/TCP-IP/Wi-Fi/포트 완전 n=6 폐쇄
<!-- AUTO:SUMMARY_network:END -->

| 🛸 | 천장확인 | ver | 완성제품 | 핵심 | 링크 |
|:--:|:--:|:---:|---------|------|------|
| 10 | ✅ | v1 | **HEXA-NET 네트워크 아키텍처** | 39/40 EXACT (97.5%) — OSI σ-sopfr=7, TCP/IP τ=4 계층, Wi-Fi n=6세대, 포트65536=2^(σ+τ), 8단 체인 | [목표](docs/network/goal.md) |

<!-- AUTO:FOOTER_network:START -->
> 도메인: [network/](docs/network/)
<!-- AUTO:FOOTER_network:END -->
```

> 주의: README software 섹션 (라인 340) 에 `궁극의 네트워크 프로토콜` 이 이미 존재하며 50/50 EXACT 로 다른 제품임. SSOT `network` 와 README software.network-protocol 은 별개 — 중복 여부 재감사 필요.

### 3.12 quantum-computer (1 제품, 삽입 후보: network 뒤 또는 computer 대체)

```markdown
---

# 🔮 양자컴퓨터 (Quantum Computer)

<!-- AUTO:SUMMARY_quantum-computer:START -->
> **🛸10** | ❌ | BT 1제품 83%EXACT | NeutralAtom + SurfaceCode + Clifford + kissing number
<!-- AUTO:SUMMARY_quantum-computer:END -->

| 🛸 | 천장확인 | ver | 완성제품 | 핵심 | 링크 |
|:--:|:--:|:---:|---------|------|------|
| 10 | ✅ | v1 | **HEXA-QUANTUM 양자컴퓨터 아키텍처** | 20/24 EXACT — NeutralAtom n=6원자, SurfaceCode σ=12 data qubit, Clifford τ×n=24 gate, Grover sopfr, kissing number BT-49 | [목표](docs/quantum-computer/goal.md) |

<!-- AUTO:FOOTER_quantum-computer:START -->
> 도메인: [quantum-computer/](docs/quantum-computer/) · BT: 49
<!-- AUTO:FOOTER_quantum-computer:END -->
```

> 주의: products.json `sections[id=quantum-computer].ceiling = false` / 제품 `ufo=10, ceiling=true`. 섹션-레벨 천장확인 ❌ 이지만 내부 제품은 ✅ — 섹션 요약과 테이블 행이 불일치. `products-drift-fix-2026-04-11.md` 감사에서도 이 불일치 지적됨 (제품 레벨 ceiling 은 이미 true 로 수정, 섹션 레벨은 보존).

### 3.13 horology (1 제품, 삽입 후보: quantum-computer 뒤)

```markdown
---

# ⏱️ 시계학 (Horology)

<!-- AUTO:SUMMARY_horology:START -->
> **🛸10** | ✅ | BT 1제품 100%EXACT | σ=12시/J₂=24시/σ·sopfr=60분/n/φ=3침 래더
<!-- AUTO:SUMMARY_horology:END -->

| 🛸 | 천장확인 | ver | 완성제품 | 핵심 | 링크 |
|:--:|:--:|:---:|---------|------|------|
| 10 | ✅ | v1 | **시계학 n=6 시간 산술 아키텍처** | 17/17 EXACT (100%) — 12시=σ, J₂=24시, 60분=σ·sopfr, n/φ=3침, 석영2^(sopfr·n/φ)=32768Hz, 6Hz 기계식 | [목표](docs/horology/hypotheses.md) |

<!-- AUTO:FOOTER_horology:START -->
> 도메인: [horology/](docs/horology/)
<!-- AUTO:FOOTER_horology:END -->
```

> 주의: README `civilization` 섹션 (라인 519) 에 `시계학/호롤로지 n=6 시간 아키텍처` 1 행이 이미 존재함. SSOT 는 별도 섹션으로 분리됨 → 신설 시 civilization 내 중복 행 제거 필요 (후속 작업).

---

## 4. 삽입 순서 권장

products.json 의 실측 섹션 순서 (line 1 ~ 4865) 를 기준으로, README 신규 13 섹션을 다음 순서대로 삽입한다:

| 순서 | id | 삽입 위치 (README 내) | 근거 |
|---:|----|-----------------------|------|
| 1 | virology | `tech-industry` 뒤 (L586) | products.json 20번 섹션 |
| 2 | hiv-treatment | `virology` 뒤 | products.json 21번 섹션 |
| 3 | natural-science | `hiv-treatment` 뒤 | products.json 22번 섹션 |
| 4 | cognitive-social | `natural-science` 뒤 | products.json 23번 섹션 |
| 5 | mobility | `cognitive-social` 뒤 | products.json 24번 섹션 |
| 6 | digital-medical | `mobility` 뒤 | products.json 25번 섹션 |
| — | marketing | (이미 존재, L608) | products.json 26번 — 위치 조정 불필요 |
| — | hygiene | (이미 존재, L786) | products.json 27번 — 위치 조정 필요 (marketing 바로 뒤로 이동) |
| 7 | tattoo-removal | `hygiene` 뒤 | products.json 28번 섹션 |
| 8 | keyboard | `tattoo-removal` 뒤 | products.json 29번 섹션 |
| 9 | mouse | `keyboard` 뒤 | products.json 30번 섹션 |
| 10 | manufacturing-quality | `mouse` 뒤 | products.json 31번 섹션 |
| 11 | network | `manufacturing-quality` 뒤 | products.json 32번 섹션 |
| 12 | quantum-computer | `network` 뒤 | products.json 33번 섹션 |
| 13 | horology | `quantum-computer` 뒤 | products.json 34번 섹션 |

### 4.1 부수 정리 작업 (본 감사 범위 밖, 후속 TODO)

1. **`computer` 섹션 해체**: README L589~604. 3 제품은 keyboard/mouse/quantum-computer 신규 섹션으로 흡수, **HEXA-BCI 1 제품은 재이관 필요** (products.json 에 완전 누락).
2. **frontier 내 `타투 제거` 행 (L497) 제거** — 신규 `tattoo-removal` 섹션과 중복.
3. **tech-industry 내 `HEXA-QC` 행 (L581) 제거** — 신규 `manufacturing-quality` 섹션과 중복.
4. **civilization 내 `시계학/호롤로지` 행 (L519) 제거** — 신규 `horology` 섹션과 중복.
5. **`energy` 섹션에 `HEXA-AUTO 자동차배터리` 1 행 추가** (products.json 5번 제품, README 4행만).
6. **`audio` 섹션에 `HEXA-BONE` / `HEXA-EAR-CELL` / `HEXA-SPEAKER` 3 행 추가** (products.json 5~7번 제품, README 4행만).
7. **`millennium` / `dimension` / `music` / `linguistics` / `crypto` / `astronomy` / `fantasy` 고아 섹션**: SSOT 의 products.json 이 수용할지, README 만 단독 유지할지 정책 결정 필요.
8. **`sync_products_readme.hexa` STUB 해제**: 동기화 자동화 구현 후 위 모든 작업 기계화.

---

## 5. ASCII 드리프트 비교 막대 차트

### 5.1 섹션 레벨 (products.json 총 34 섹션 = 100%)

```
products.json 34 섹션 기준
────────────────────────────────────────────────────
공통 21           ████████████████████████ 61.8%
누락 13 (신규)    ██████████████           38.2%

README 29 섹션 기준
────────────────────────────────────────────────────
공통 21           ████████████████████████ 72.4%
고아 8 (잔존)     ██████████               27.6%
```

### 5.2 제품 레벨 (products.json 총 173 제품 = 100%)

```
products.json 173 제품 (34 섹션)
────────────────────────────────────────────────────
기존 21 공통 섹션 146   █████████████████████████████████ 84.4%
누락 13 섹션       27   ██████                            15.6%

누락 섹션 제품 분포 (27 합산):
  cognitive-social       6 ████████████████████████
  natural-science        4 ████████████████
  virology               4 ████████████████
  digital-medical        3 ████████████
  mobility               2 ████████
  hiv-treatment          1 ████
  tattoo-removal         1 ████
  keyboard               1 ████
  mouse                  1 ████
  manufacturing-quality  1 ████
  network                1 ████
  quantum-computer       1 ████
  horology               1 ████
```

### 5.3 공통 섹션 내 세부 제품 카운트 드리프트

```
공통 21 섹션 중 2 섹션 내부 drift 발생
────────────────────────────────────────────────────
energy   json=5 readme=4   ▓▓▓▓▓ vs ▓▓▓▓     diff=+1
audio    json=7 readme=4   ▓▓▓▓▓▓▓ vs ▓▓▓▓   diff=+3
```

### 5.4 README 고아 섹션별 제품 행 잔존

```
고아 8 섹션 합산 34 행 (1 행 = ▓)
────────────────────────────────────────────────────
millennium    7 ▓▓▓▓▓▓▓
dimension     7 ▓▓▓▓▓▓▓
computer      4 ▓▓▓▓  (keyboard/mouse/quantum 3개 이관완료, BCI 1개 잔존 누락)
music         4 ▓▓▓▓
linguistics   4 ▓▓▓▓
crypto        4 ▓▓▓▓
astronomy     4 ▓▓▓▓
fantasy       0 .     (경고 박스만, 제품 행 없음)
```

### 5.5 총괄 드리프트 온도계

```
동기화 적합도 (products.json SSOT 기준)
────────────────────────────────────────────────────
섹션 커버리지     21/34 = 61.8%  ████████████▌
제품 커버리지    146/173 = 84.4% ████████████████▉
섹션 + 내부 동시 19/34 = 55.9%  ███████████▎
                                  (energy/audio 2 섹션 내부 drift 차감)
```

---

## 부록 A. 참조 파일 경로

- SSOT: `/Users/ghost/Dev/nexus/shared/n6/docs/products.json`
- README: `/Users/ghost/Dev/n6-architecture/README.md`
- 이전 감사: `/Users/ghost/Dev/n6-architecture/reports/audits/products-drift-fix-2026-04-11.md` (products.json `_meta` 드리프트 수정 기록)
- 백업: `/Users/ghost/Dev/n6-architecture/reports/audits/products-backup-2026-04-11.json`
- 동기화 스크립트 (STUB): `sync_products_readme.hexa` (경로 미확정, 본 감사 범위 밖)

## 부록 B. 감사 한계 (Honest Limitations)

- 본 감사는 **섹션 id / 제품 카운트 / BT 범위 겹침** 3 축만 기계 검증. 제품 내부 description / ver / 링크 경로의 텍스트 drift 는 검출 대상 아님.
- 렌더링 프리뷰 (§3) 의 SUMMARY 라인은 products.json 의 `summary` 필드가 공란이므로 (`products-drift-fix-2026-04-11.md` 감사 시점 확인), `alien_index` / `ceiling` / `bt_exact_pct` / 제품 수에서 기계 조립. 실제 동기화 시 `sync_products_readme.hexa` 가 정의할 포맷과 다를 수 있음.
- "고아 8 섹션 내 살아있는 제품" 판정은 README 텍스트 내 제품 행 존재 여부만 근거. 실제 `docs/` 하위 파일 (예: `docs/millennium-riemann/goal.md`) 의 유효성 확인은 범위 밖.
- `computer` 섹션 이관 시 `HEXA-BCI` 가 products.json 에 누락된 점은 확인되었으나, 원인 (의도적 제외 / 동기화 실패 / 이관 누락) 판정은 범위 밖.
- README 렌더링 프리뷰의 파이프라인 기호 (σ, τ, φ, J₂, sopfr 등) 는 products.json description 의 원문 그대로 복사. 일부는 `sigma`, `tau` 등 ASCII 로 저장되어 있어 README 의 유니코드 기호 일관성을 위해 실제 삽입 시 치환 필요할 수 있음.
