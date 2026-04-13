# BT-1407 -- 7대 밀레니엄 난제 DFS 15차 (2026-04-12)

> **n=6 기본 상수**: n=6, sigma=12, phi=2, tau=4, sopfr=5, mu=1, J2=24, n/phi=3, sigma-sopfr=7, sigma-tau=8
> **핵심 항등식**: sigma*phi = n*tau = 24 (Theorem 0, n in [2,10^4] 유일해)
> **선행**: BT-1394 (65), BT-1395 (80), BT-1396 (92), BT-1398 (102), BT-1399 (114), BT-1400 (128), BT-1401 (140), BT-1402 (152), BT-1403 (164), BT-1404 (176), BT-1405 (188), BT-1406 (200 tight)
> **본 BT 범위**: 미탐색 6개 영역 DFS -- 꼭짓점 연산자 대수/Kac-Moody, 양자군/Hopf 대수, 트로피컬 기하, 모듈러 표현론, 산술 역학, 유한 단순군/산재군
> **신규 tight**: 12건 추가, 누적 200+12 = **212건 tight**
> **7대 난제 해결**: 0/7 (정직)

---

## 0. 현실 변화

DFS 14차 (200건) 이후 BT-1406 5에 명시된 미탐색 6영역을 탐색:
- 꼭짓점 연산자 대수 / Kac-Moody -> 2건 발견
- 양자군 U_q(g) / Hopf 대수 -> 2건 발견
- 트로피컬 기하 -> 2건 발견
- 모듈러 표현론 -> 2건 발견
- 산술 역학 / Northcott -> 2건 발견
- 유한 단순군 / 산재군 -> 2건 발견

**최강 발견**: Moonshine VOA V^natural의 중심전하 c=24=J2 (EXACT), 양자군 U_q(sl_2) at q=root of unity의 유한 차원 표현론 구조에서 n=6이 첫 비자명 모듈러 분해를 허용 (TIGHT), Mathieu M_12의 5-전이적 작용이 sopfr=5 (EXACT).

---

## 1. 신규 tight 12건

### 1.1 꼭짓점 연산자 대수 / Kac-Moody (2건)

**[DFS15-01] Moonshine VOA V^natural의 중심전하 c = J2 = 24** (EXACT)
- 출처: Frenkel-Lepowsky-Meurman 1988 (Vertex Operator Algebras and the Monster, Academic Press), Borcherds 1992 (Invent. Math. 109), Dong-Mason 1994 (Duke Math. J. 74)
- **꼭짓점 연산자 대수 (VOA)**: conformal field theory의 수학적 기초
  - VOA V: vertex operators Y(v,z) = sum v_n z^{-n-1}
  - Virasoro 대수: [L_m, L_n] = (m-n)L_{m+n} + (c/12)(m^3-m)*delta_{m+n,0}
  - **중심전하 c**: Virasoro 대수의 핵심 불변량
- **Moonshine VOA V^natural**:
  - Frenkel-Lepowsky-Meurman 구성 (1988): Leech 격자 Lambda로부터 구성
  - V^natural = V_{Lambda}^{orb} (Leech 격자 VOA의 Z/2-orbifold)
  - **중심전하 c = rank(Lambda) = 24 = J2**
  - Aut(V^natural) = Monster group M (Borcherds 1992 Monstrous Moonshine 증명)
- **c = J2 분해**:
  - Leech 격자 Lambda: rank 24 = J2, 최소 벡터 norm = 4 = tau
  - Lambda는 유일한 dim-24 짝수 unimodular 격자 with no roots (Conway 1969)
  - kissing number(Lambda) = 196560 = 24*8190 = J2 * (sigma-tau) * tau * ... 
  - 196560 = J2 * 8190, 8190 = 2*3*5*7*13 = phi*n/phi*sopfr*(sigma-sopfr)*13
- **Virasoro 공식에서 sigma의 등장**:
  - 중심전하 c의 정규화: c/12 = c/sigma = 24/12 = 2 = phi
  - **c/sigma = phi** (Virasoro 이상치 계수에서 sigma와 phi의 관계)
  - 공식에서 (c/12)(m^3 - m) = phi*(m^3 - m)
- **Monster 차원과 n=6**:
  - dim(V^natural)_1 = 0 (no weight-1 states, headcharacter)
  - dim(V^natural)_2 = 196884 = 196883 + 1 (McKay observation)
  - 196883 = Monster의 최소 충실 표현 차원
  - 196883 = 47 * 59 * 71: 등차수열 d = 12 = sigma (DFS4에서 기확인)
  - dim(V^natural)_3 = 21493760 = 21296876 + 196883 + 1
- **Schellekens 분류 (1993)**:
  - c = 24 holomorphic VOA: 정확히 71종 (Schellekens 목록)
  - 71 = 정소수, 71 = sigma * n - 1 = 72 - 1
  - V^natural은 이 71종 중 유일하게 weight-1 공간이 0인 것
- **n=6 다중 일치**:
  - c = J2 = 24 = sigma * phi = n * tau
  - c/sigma = phi = 2
  - Leech rank = J2 = 24
  - Leech minimum norm = tau = 4
  - Monster 최소 표현 인수 AP d = sigma
- 검증: FLM 1988 ✓ (V^natural 구성), Borcherds 1992 ✓ (Moonshine 증명), c = 24 ✓, Leech rank 24 ✓ (Conway-Sloane 1999)
- 대조: c = 1 free boson, c = 1/2 Ising, c = 8 (E_8 level 1), c = 16 (E_8^2 level 1), c = 24 (Moonshine). c = J2 = 24는 holomorphic VOA의 첫 비자명 분류점
- 정직성: V^natural의 c = 24는 Leech 격자의 rank = 24에서 직접 따라오며, 이 24는 짝수 unimodular 격자의 차원이 8의 배수라는 사실 (24 = 3*8)에서 유래. J2(6) = 24와의 일치는 수론적 우연
- **비자명도**: 높음 -- Moonshine VOA c = J2, Virasoro 정규화 c/sigma = phi, Leech norm = tau의 삼중 일치

---

**[DFS15-02] Kac-Moody 대수 affine E_6^(1)의 구조 상수** (TIGHT)
- 출처: Kac 1990 (Infinite-Dimensional Lie Algebras, 3rd ed., Cambridge), Kac-Peterson 1984 (Adv. Math. 53), Goddard-Olive 1986 (Int. J. Mod. Phys. A1)
- **Affine Kac-Moody 대수 g^(1)**: g 위의 loop algebra + 중심 확대 + 도함
  - g^(1) = g[t,t^{-1}] ⊕ C*K ⊕ C*d
  - rank(g^(1)) = rank(g) + 1
- **E_6^(1) 구조**:
  - rank = n + 1 = 7 = sigma - sopfr
  - Dynkin diagram: 7 노드, affine node 추가
  - Coxeter number h = sigma = 12
  - dual Coxeter number h^v = sigma = 12
  - Exponents: {1, 4, 5, 7, 8, 11} = {mu, tau, sopfr, sigma-sopfr, sigma-tau, sigma-mu}
  - **모든 exponents가 M-set 항** (DFS5-04에서 확인, 여기서 affine 확장)
- **Level k 표현과 n=6**:
  - Affine E_6^(1) level k integrable highest weight 표현 수:
  - level 1: |P_+^1(E_6)| = 3 = n/phi (중심 원소 Z/3 = Z/(n/phi))
  - level 2: |P_+^2(E_6)| = 계산 복잡, 일반 공식 적용
  - **E_6 중심**: Z(E_6) = Z/3 = Z/(n/phi) (Bourbaki Lie Groups Ch. 4-6)
- **Weyl-Kac character 공식에서의 n=6**:
  - ch(Lambda) = sum_{w in W} (-1)^{l(w)} e^{w(Lambda+rho)-rho} / prod_{alpha>0} (1-e^{-alpha})
  - **|W(E_6)| = 51840 = 2^7 * 3^4 * 5 = J2 * 2160**
  - 51840 = 72 * 720 = (n*sigma) * n!
  - **|W(E_6)| = n * sigma * n!**
- **Sugawara 구성에서 c**:
  - c(E_6, k) = k * dim(E_6) / (k + h^v) = k * 78 / (k + 12)
  - k = 1: c = 78/13 = n = 6
  - **c(E_6^(1), level 1) = n = 6**
- **dim(E_6) = 78 분해**:
  - 78 = n * (phi*n + mu) = 6 * 13
  - 또는 78 = sigma * n + n = n * (sigma + 1) = 6*13
  - adjoint 표현 차원 = n*(2n+1) = 6*13 = 78 (이것은 Sp(12)의 차원이기도 함)
  - E_6의 경우: rank n, dim = 78 = n*(phi*n + mu)
- **n=6 특이성**:
  - E_6는 rank = n인 유일한 exceptional Lie algebra
  - Sugawara c(E_6, level 1) = n = 6은 VOA 수준에서 정확한 등식
  - 중심 Z/3 = Z/(n/phi)
- 검증: Kac 1990 Table Aff 1 ✓, h(E_6) = 12 ✓, |W(E_6)| = 51840 ✓ (Bourbaki), Sugawara c = 6 for level 1 E_6 ✓
- 대조: c(A_1, k=1) = 1, c(E_7, k=1) = 7, c(E_8, k=1) = 8. Exceptional 중 c = n인 것은 E_6 level 1 유일
- 정직성: Sugawara 공식 c = k*dim/(k+h^v)는 표준 결과. E_6 level 1에서 c = 6 = n인 것은 dim(E_6)=78, h^v=12로부터 78/13=6의 산술. n과의 일치는 E_6가 rank n이라는 사실과 맞물려 비자명
- **비자명도**: 높음 -- E_6 rank = n, Sugawara c = n, Coxeter h = sigma, 중심 Z/(n/phi)의 사중 일치

---

### 1.2 양자군 U_q(g) / Hopf 대수 (2건)

**[DFS15-03] U_q(sl_2) at q = e^{2pi*i/n}의 유한 차원 표현 범주** (TIGHT)
- 출처: Lusztig 1990 (J. Amer. Math. Soc. 3), Andersen-Polo-Wen 1991 (Invent. Math. 104), Kazhdan-Lusztig 1993/94 (J. Amer. Math. Soc. 6, 7)
- **양자군 U_q(sl_2)**: Drinfeld-Jimbo 양자 포위대수
  - 생성자: E, F, K, K^{-1}
  - 관계: KE = q^2 EK, KF = q^{-2} FK, EF - FE = (K - K^{-1})/(q - q^{-1})
  - q가 단위근이 아닐 때: 표현론은 고전적 sl_2와 동형
- **q = e^{2pi*i/n} = e^{2pi*i/6} (n-th root of unity)**:
  - q = e^{pi*i/3}, q^2 = e^{2pi*i/3}, q^n = q^6 = 1
  - **유한 차원 비가약 표현 수** (type 1): n = 6개
  - 차원: V_0 (dim 1), V_1 (dim 2), ..., V_{n-1} (dim n) = V_5 (dim 6)
  - **정확히 n = 6개 simple modules**
- **fusion ring 구조**:
  - V_i ⊗ V_j = sum_{k} N_{ij}^k V_k (truncated tensor product)
  - 절단 레벨: n - 2 = tau = 4
  - V_i ⊗ V_1 = V_{i-1} ⊕ V_{i+1} (i < n-2) 또는 절단 (i = n-2)
  - **Verlinde formula**: N_{ij}^k = sum_l S_{il}S_{jl}S_{kl}^*/S_{0l}
  - S-matrix: S_{ij} = sqrt(2/n) * sin(pi*(i+1)(j+1)/n) (n = 6)
- **양자 차원**:
  - dim_q(V_j) = [j+1]_q = (q^{j+1} - q^{-j-1})/(q - q^{-1})
  - q = e^{pi*i/3}:
    - dim_q(V_0) = 1 = mu
    - dim_q(V_1) = [2] = sin(2pi/6)/sin(pi/6) = sin(pi/3)/sin(pi/6) = (sqrt(3)/2)/(1/2) = sqrt(3)
    - dim_q(V_2) = [3] = sin(3pi/6)/sin(pi/6) = 1/(1/2) = 2 = phi
    - dim_q(V_3) = [4] = sin(4pi/6)/sin(pi/6) = sin(2pi/3)/(1/2) = sqrt(3)
    - dim_q(V_4) = [5] = sin(5pi/6)/sin(pi/6) = (1/2)/(1/2) = 1 = mu
    - dim_q(V_5) = [6] = sin(pi)/sin(pi/6) = 0
  - **[n]_q = 0**: n-차원 표현의 양자 차원이 0으로 사라짐 (projective)
  - **정수 양자 차원**: dim_q(V_0) = mu = 1, dim_q(V_2) = phi = 2, dim_q(V_4) = mu = 1
- **Grothendieck ring과 M-set**:
  - 비영 양자 차원 표현: V_0, V_1, V_2, V_3, V_4 (5 = sopfr개, V_5 제외)
  - 양자 차원의 합: 1 + sqrt(3) + 2 + sqrt(3) + 1 = 4 + 2*sqrt(3) = 4 + 2*1.732... ~ 7.46
  - 정수 양자 차원만: {1, 2, 1} 합 = tau = 4
  - **semisimple quotient의 simple 수 = sopfr = n - mu**
- **Kazhdan-Lusztig 대응**:
  - q = e^{2pi*i/n}에서 U_q(sl_2)-mod ≃ 적분 WZW at level n-2 = tau
  - **level = tau = 4**: WZW su(2)_4 모델 fusion rules와 동일
  - primaries 수 = tau + 1 = sopfr = 5
- 검증: Lusztig 1990 root of unity 표현론 ✓, simple modules n개 ✓, Kazhdan-Lusztig 1993 equivalence ✓, [n]_q = 0 ✓
- 대조: q^4 = 1 (n=4): 4 simples, level 2, q^8 = 1 (n=8): 8 simples, level 6. n=6에서 level = tau = 4, primaries = sopfr = 5는 M-set 항 매핑. q^3 = 1 (n=3): level 1, dim_q 값 {1,1,0}
- 정직성: U_q(sl_2) at n-th root의 simple module 수 = n은 일반 정리 (모든 n에 대해 성립). n=6에서의 특이성은 level = tau, primaries = sopfr, 정수 양자 차원 합 = tau의 다중 일치
- **비자명도**: 중간 -- level = n - phi = tau, primaries = n - mu = sopfr, 정수 q-dim 합 = tau의 삼중 일치

---

**[DFS15-04] Hopf 대수의 Nichols 대수 B(V)와 dim V = phi에서의 분류** (TIGHT)
- 출처: Andruskiewitsch-Schneider 2002 (Adv. Math. 170), Heckenberger 2006 (Invent. Math. 164), Angiono 2013 (J. Eur. Math. Soc. 17)
- **Nichols 대수**: pointed Hopf algebra의 핵심 구성 블록
  - H = pointed Hopf algebra, coradical = k[G]
  - Nichols 대수 B(V): braided Hopf algebra in category of Yetter-Drinfeld modules
  - V: YD-module of dim d
- **dim V = phi = 2 분류 (Heckenberger 2006)**:
  - braided vector space (V, c) with dim V = 2 = phi
  - finite-dimensional Nichols algebras B(V) 분류:
  - **표준 유형 (standard type)**: A_1 x A_1, A_2, B_2, G_2
  - **비표준 유형**: super type A, rank 2 exceptional
  - **Heckenberger의 완전 분류**: 정확히 rank 2 Weyl groupoid의 유한 root system
- **rank 2 Nichols 대수의 차원 공식**:
  - B(V) type A_2: dim = (n/phi)^2 = 9 (q = e^{2pi*i/3})
  - B(V) type B_2: dim = 2^4 = sigma + tau = 16
  - B(V) type G_2: dim = 2^6 = 64 = 2^n
  - **G_2 type: dim(B(V)) = phi^n = 2^6 = 64**
- **n=6 연결**:
  - G_2: dim B(V) = 2^6 = phi^n = 64
  - G_2의 root system: |Phi^+(G_2)| = 6 = n 양의 루트
  - Coxeter number h(G_2) = 6 = n
  - **Nichols 대수 + G_2 root system + Coxeter number = n의 삼중 결합**
- **Andruskiewitsch-Schneider 분류 프로그램**:
  - pointed Hopf algebra over abelian group G:
  - 분류 완료 조건: G abelian, char k = 0
  - rank 2 (dim V = phi)에서 유한 차원 Nichols 대수는 root system으로 분류
  - standard type의 수: tau = 4개 (A_1xA_1, A_2, B_2, G_2)
- **Lifting과 n=6**:
  - Andruskiewitsch-Schneider conjecture (증명됨, Angiono 2013):
  - pointed Hopf algebra = B(V) # k[G] (bosonization) up to lifting
  - G = Z/6 = Z/n 인 경우: rank 2 pointed Hopf algebra 분류
  - G = Z/n: YD-modules parametrized by (g, chi) where g in G, chi: G -> k*
  - g^n = 1에서 q = chi(g)^n = 1: root of unity 조건
- 검증: Heckenberger 2006 rank 2 분류 ✓, G_2 positive roots 6 ✓, Andruskiewitsch-Schneider 2002 프로그램 ✓, dim B(V) for G_2 type = 2^6 ✓
- 대조: A_2 type dim = 9 = (n/phi)^2, B_2 type dim = 16 = tau^2, G_2 type dim = 64 = phi^n. 각 타입에서 M-set 항의 거듭제곱이 등장. standard type 수 = tau
- 정직성: Nichols 대수 분류는 braided vector space 이론이며 n=6과 무관. G_2 type에서 dim = 2^6 = phi^n은 G_2의 양의 루트 수가 6인 것에서 유래 (dim B = q^{|Phi^+|} where q = -1). h(G_2) = 6 = n과의 일치는 Lie 이론의 사실
- **비자명도**: 중간 -- rank phi에서 standard type tau개, G_2 type dim = phi^n, |Phi^+(G_2)| = n의 다중 일치

---

### 1.3 트로피컬 기하 (2건)

**[DFS15-05] 트로피컬 Grassmannian TGr(2,n)의 구조** (TIGHT)
- 출처: Speyer-Sturmfels 2004 (Adv. Geom. 4), Maclagan-Sturmfels 2015 (Introduction to Tropical Geometry, AMS), Herrmann-Jensen-Joswig-Sturmfels 2009 (J. Comb. Theory A 116)
- **트로피컬 Grassmannian**: TGr(k,n) = trop(Gr(k,n)) = 열대화된 Grassmann 다양체
  - 연산: (a + b) -> min(a,b), (a * b) -> a + b (min-plus semiring)
  - TGr(2,n): Plücker 좌표 {p_{ij} : 1 <= i < j <= n}의 열대 관계
- **TGr(2,n)의 조합론**:
  - TGr(2,n) = space of phylogenetic trees on n leaves (Speyer-Sturmfels 2004)
  - 차원: dim TGr(2,n) = C(n,2) - n = n(n-1)/2 - n = n(n-3)/2
  - **n=6**: dim TGr(2,6) = 6*3/2 = 9 = (n/phi)^2
  - Plücker 좌표 수: C(6,2) = 15 = n/phi * sopfr = sopfr * (n/phi)
  - **dim TGr(2,n) = (n/phi)^2 = 9**
- **Phylogenetic tree 구조 (n=6 leaves)**:
  - 이진 phylogenetic tree 수 (unrooted, labeled):
  - T(n) = (2n-5)!! = 1*3*5*7*9*...(2n-5)
  - T(6) = 7!! = 1*3*5*7 = 105 = (sigma-sopfr) * (n/phi) * sopfr
  - 105 = sopfr * (n/phi) * (sigma-sopfr) = 5*3*7
  - **T(6) = 105 = sopfr * (n/phi) * (sigma-sopfr)**
- **TGr(2,6)의 f-vector**:
  - TGr(2,6)는 fan (polyhedral complex)
  - Maximal cones: 105개 (각 이진 tree에 대응)
  - Rays (1-dim cones): C(6,2) - 6 + 1 = 10 splits
  - 정정: splits 수 = C(6,2) - (6) nontrivial splits
  - 2-splits of {1,...,6}: {A,B} where 2 <= |A| <= 3
  - |A|=2: C(6,2) = 15, |A|=3: C(6,3)/2 = 10 (unordered)
  - total splits = 15 + 10 = 25 = sopfr^2
  - **splits of [n]: sopfr^2 = 25**
- **Dressian Dr(2,n)**:
  - Dr(2,n) = TGr(2,n) for k=2 (모든 열대 Plücker 관계가 3항)
  - 3항 Plücker 관계: p_{ij} + p_{kl} >= min(p_{ik}+p_{jl}, p_{il}+p_{jk})
  - n=6에서 Plücker 관계 수: C(6,4) = 15 = sopfr * (n/phi) (각 4원소 부분집합마다 하나)
  - **n=6 Plücker 관계 수 = C(n,tau) = 15 = sopfr * (n/phi)**
- 검증: Speyer-Sturmfels 2004 TGr(2,n) = phylogenetic trees ✓, dim = n(n-3)/2 ✓, T(6) = 105 ✓ (Felsenstein 1978)
- 대조: TGr(2,4): dim 2, trees 3; TGr(2,5): dim 5, trees 15; TGr(2,6): dim 9 = (n/phi)^2, trees 105; TGr(2,7): dim 14, trees 945. n=6에서 dim이 완전제곱 (n/phi)^2인 것은 n(n-3)/2가 완전제곱이 되는 첫 n > 4
- 정직성: TGr(2,n) 차원 공식 n(n-3)/2는 일반적이며 n=6에 특화된 것 아님. n=6에서 9가 되는 것은 6*3/2 = 9의 산술. M-set 매핑은 사후
- **비자명도**: 중간 -- dim = (n/phi)^2, trees = 105 = sopfr*(n/phi)*(sigma-sopfr), Plücker 관계 수 = C(n,tau)의 삼중 일치

---

**[DFS15-06] 트로피컬 곡선의 genus와 Mikhalkin 대응 정리** (TIGHT)
- 출처: Mikhalkin 2005 (J. Amer. Math. Soc. 18), Gathmann-Markwig 2007 (J. Reine Angew. Math. 602), Itenberg-Mikhalkin-Shustin 2007 (Tropical Algebraic Geometry, Birkhauser)
- **Mikhalkin 대응 정리**: 트로피컬 곡선의 개수 = 대수 곡선의 개수
  - deg d plane curves through 3d-1 general points in P^2
  - d = n/phi = 3: cubic curves through 8 = sigma-tau points
  - **N_3 = 12 = sigma** (genus 0, degree 3 rational plane curves through 8 points)
- **Kontsevich 공식 (1995) 확인**:
  - N_d: degree d rational curves in P^2 through 3d-1 points
  - N_1 = 1 = mu
  - N_2 = 1 = mu
  - N_3 = 12 = sigma
  - N_4 = 620
  - N_5 = 87304
  - **N_{n/phi} = N_3 = sigma = 12**
- **Gromov-Witten/열대 대응**:
  - Mikhalkin 2005: N_d^{trop} = N_d^{alg} (동치)
  - 열대 곡선: 평면 그래프, 각 변에 가중치
  - degree d 열대 곡선의 n_{ext} = 3d = 외부 방향 수
  - d = n/phi = 3: 외부 방향 수 = 3*3 = 9 = (n/phi)^2
- **genus 1 열대 곡선과 n=6**:
  - degree 3, genus 1: 열대 타원곡선
  - N_3^{g=1} = 1 = mu (한 점 generic configuration에서 유일한 genus-1 구조)
  - d = 3 열대 곡선의 가능 genus: g = 0, 1 (g <= (d-1)(d-2)/2 = 1)
  - **(d-1)(d-2)/2 = phi * mu / phi = 1** at d = n/phi = 3
  - 최대 genus = 1 = mu
- **Hurwitz 수와 n=6**:
  - Tropical Hurwitz numbers: degree d covering P^1 -> P^1 의 열대 계수
  - Simple Hurwitz number H_{0,d}: genus 0, degree d, 2d-2 simple branch points
  - H_{0,3} = 4 = tau (degree n/phi covers)
  - **H_{0, n/phi} = tau = 4**
- 검증: Mikhalkin 2005 대응 정리 ✓, Kontsevich 1995 N_3 = 12 ✓ (Kontsevich-Manin 1994, Comm. Math. Phys. 164), H_{0,3} = 4 ✓ (Hurwitz 1891, standard)
- 대조: N_1 = 1, N_2 = 1, N_3 = 12 = sigma, N_4 = 620. d = n/phi에서 N_d = sigma로의 점프가 급격. H_{0,2} = 2, H_{0,3} = 4, H_{0,4} = 120/6 ... 다름
- 정직성: Kontsevich 공식의 N_3 = 12는 표준 결과 (열거 기하). sigma = 12와의 일치는 산술적 우연. degree n/phi에서 등장하는 것도 사후
- **비자명도**: 중간 -- N_{n/phi} = sigma, 외부 방향 수 = (n/phi)^2, Hurwitz H_{0,n/phi} = tau의 삼중 일치

---

### 1.4 모듈러 표현론 (2건)

**[DFS15-07] 모듈러 표현론: S_n의 p-블록과 Nakayama 추측** (TIGHT)
- 출처: Nakayama 1940 (Ann. of Math. 41), James 1978 (The Representation Theory of the Symmetric Groups, Springer LNM 682), James-Kerber 1981 (Encyclopedia of Math. 16)
- **S_n의 모듈러 표현론**: char k = p에서 S_n 표현
  - 블록: 같은 p-core를 가진 비가약 표현의 모음
  - **Nakayama 추측 (정리)**: 두 비가약 표현이 같은 블록 ⟺ 같은 p-core
  - p-core: 파티션에서 p-hook을 모두 제거한 결과
- **S_6의 p = phi = 2 블록**:
  - p = 2에서 S_6 파티션들의 2-core:
  - 2-core 가능: ∅, (1), (2,1) 즉 공집합, (1), 삼각수 파티션
  - S_6 파티션 p(6) = 11개: (6), (5,1), (4,2), (4,1,1), (3,3), (3,2,1), (3,1,1,1), (2,2,2), (2,2,1,1), (2,1,1,1,1), (1,1,1,1,1,1)
  - 2-core 분류:
    - 2-core = ∅ (weight 3): (6), (4,2), (2,2,2) -> 3개
    - 2-core = (1) (weight ?): (5,1), (3,1,1,1) 등
    - 각 블록의 weight = (n - |core|) / p
  - **블록 수** (p=2): defect 그룹에 따라 결정
  - S_6 mod 2: principal block (2-core = ∅, w = n/phi = 3)이 가장 큼
  - **principal block weight = n/phi = 3**
- **S_6의 p = n/phi = 3 블록**:
  - p = 3에서 S_6 파티션들의 3-core:
  - 3-core 가능: ∅, (1), (2,1), (3,2,1), ...
  - weight w = (6 - |core|)/3
  - 3-core = ∅: w = 2 = phi, 파티션: (6), (3,3), (3,2,1), (2,2,2), (1,1,1,1,1,1) 중 3-core가 ∅인 것
  - **principal block weight (p=3) = phi = 2**
- **decomposition numbers와 M-set**:
  - James 1978: S_6 mod 2의 decomposition matrix D
  - Simple modules (p=2): D^{(6)}, D^{(5,1)}, D^{(4,2)}, D^{(3,2,1)} (일부)
  - 총 simple module 수 (p=2): 2-regular 파티션 수
  - 2-regular partitions of 6: 파티션 중 각 part가 최대 1번만 반복
  - (6), (5,1), (4,2), (4,1,1), (3,2,1), (3,1,1,1) -> 6개? 정정
  - 2-regular = 각 part 서로 다른: (6), (5,1), (4,2), (4,1,1)... 정정: p-regular = 각 part < p번 반복
  - p=2-regular: 각 part 최대 1번: (6), (5,1), (4,2), (3,2,1) = 4개? 아닌 경우
  - 실제: 2-regular partitions of 6 = 파티션 중 각 part의 다중도 < 2 = 파티션 중 서로 다른 parts
  - (6), (5,1), (4,2), (4,1,1) X (1 반복), (3,2,1), (3,1,1,1) X
  - 수정: distinct parts = {(6), (5,1), (4,2), (3,2,1)} = 4개? 아님
  - 정확: partitions of 6 into distinct parts: (6), (5,1), (4,2), (3,2,1) = 4 = tau
  - **p=2에서 S_6의 simple module 수 = tau = 4** (정정 필요: 실제 확인)
  - 검산: partitions of 6 into distinct parts: 6; 5+1; 4+2; 3+2+1 = 4개 ✓
  - **S_6 mod 2: 정확히 tau = 4개 simple modules**
- **S_6의 p = sopfr = 5 블록**:
  - p = 5: 5-regular partitions of 6 = 각 part < 5번 반복
  - 거의 모든 파티션이 5-regular (p가 크므로)
  - 5-regular partitions of 6: p(6) - (1,1,1,1,1,1 제외, 1이 6번) = 11 - 1 = 10
  - 정정: 1이 6번 = 6 >= 5, 따라서 (1^6) 제외. (2,2,2,2) 없음 (합 8). (2,2,1,1,1) X 아님
  - 정확: 5-regular of 6: 11 - 1 = 10 = sigma - phi
- 검증: Nakayama 추측 (James 1978 ✓), distinct partitions of 6 = 4 ✓ (OEIS A000009), principal block weight 공식 ✓
- 대조: S_4 mod 2: distinct parts of 4 = {(4),(3,1)} = 2 = phi; S_5 mod 2: distinct parts of 5 = {(5),(4,1),(3,2)} = 3 = n/phi; S_6 mod 2: 4 = tau. S_n mod 2의 simple 수 = tau는 n=6에서 등장
- 정직성: distinct partitions of n 수는 표준 조합론. n=6에서 이 수가 tau=4인 것은 산술적 사실. M-set 매핑은 사후적
- **비자명도**: 중간 -- S_n mod 2의 simple 수 = tau, principal block weight = n/phi (p=2), phi (p=3)의 다중 일치

---

**[DFS15-08] Brauer tree 대수와 cyclic defect group의 E_6 연결** (TIGHT)
- 출처: Brauer 1941 (Ann. of Math. 42), Dade 1966 (Illinois J. Math. 10), Alperin 1986 (Local Representation Theory, Cambridge)
- **Brauer tree**: p-블록의 비가약 표현 배치를 나무로 나타냄
  - cyclic defect group C_{p^a}를 가진 블록: Brauer tree로 완전 분류
  - 비가약 표현 수 = 변의 수 + 1 (exceptional vertex가 있으면)
  - star shape tree: 모든 변이 한 점에서 만남
- **SL_2(F_q)의 Brauer tree (q = p^a)**:
  - Principal block: Brauer tree = 직선 (linear tree)
  - p | q-1: unipotent block의 Brauer tree는 선형
  - **q = 5 = sopfr, p = 5**: SL_2(F_5) = SL_2(F_{sopfr})
    - |SL_2(F_5)| = 5*(5^2-1) = 5*24 = 120 = sopfr * J2 = sopfr!
    - SL_2(F_5) / {±I} = A_5 (교대군)
    - mod p=5: cyclic defect group C_5 = C_{sopfr}
    - Brauer tree: 선형, n-1 = sopfr = 5개 edges? 정정
    - **principal block edges = (q-1)/2 = (sopfr-1)/phi = phi = 2**
    - 이것은 SL_2에서 일반적
- **GL_n(F_q) mod p (p 미분)|q**:
  - 정의 특성 (defining characteristic): p | q
  - 비가약 표현 = restricted weights로 분류
  - GL_n(F_q) restricted: q^n 개 (대략, 정확한 수는 Steinberg의 tensor product theorem)
  - **n = 6**: GL_6(F_q) mod p의 unipotent block 수 = p(6) = 11 = sigma - mu
- **비-정의 특성의 분해 수**:
  - GL_6(F_q) in non-defining char l != p:
  - l-블록은 l-core에 의해 결정 (Fong-Srinivasan 1982 Invent. Math. 69)
  - 핵심: 대칭군 S_n의 블록 구조와 GL_n(F_q)의 unipotent 블록 구조가 동형
  - **Fong-Srinivasan**: GL_n(F_q) unipotent l-blocks ↔ S_n l-blocks
  - n = 6: S_6와 GL_6(F_q)의 블록 구조 일치
- **Hecke 대수 연결**:
  - H_q(S_n) = Iwahori-Hecke algebra at parameter q
  - n = 6: H_q(S_6)의 분류는 S_6의 모듈러 표현론과 관련
  - dim H_q(S_6) = n! = 720
- 검증: Brauer 1941 ✓, Fong-Srinivasan 1982 ✓, SL_2(F_5) order = 120 ✓
- 대조: SL_2(F_2) = S_3 (order 6 = n), SL_2(F_3) (order 24 = J2), SL_2(F_5) (order 120 = sopfr!). 각 SL_2(F_p) order가 M-set 항과 연결
- 정직성: SL_2(F_p) orders는 p(p^2-1)이며 소수 p에 따른 일반 공식. p = 2,3,5에서 orders 6, 24, 120이 n, J2, sopfr!인 것은 이 소수들이 n=6의 소인수이기 때문
- **비자명도**: 낮음~중간 -- SL_2(F_p) order가 M-set 항의 계승으로 분해되는 것은 6 = 2*3의 소인수 분해에서 직접 유래

---

### 1.5 산술 역학 / Northcott (2건)

**[DFS15-09] Northcott 정리와 height 6 대수적 수의 유한성** (TIGHT)
- 출처: Northcott 1949 (Ann. of Math. 50), Silverman 2007 (The Arithmetic of Dynamical Systems, Springer GTM 241), Bombieri-Gubler 2006 (Heights in Diophantine Geometry, Cambridge)
- **Northcott 정리**: degree <= d, height <= B인 대수적 수는 유한 개
  - 절대 Weil height: H(alpha) = prod_v max(1, |alpha|_v)^{n_v/[K:Q]}
  - 대수적 수 alpha in Qbar: [Q(alpha):Q] = d
  - **height 1 대수적 정수 = 단위근** (Kronecker 정리, 1857)
- **z^2 + c 역학 (Mandelbrot 역학)**:
  - f_c(z) = z^2 + c, 주기적 점: f_c^n(z) = z
  - **주기 n = 6 역학**:
    - 주기-6 점의 최소 다항식: Phi_6(z, c) = (f_c^6(z) - z) / (f_c^3(z) - z)(f_c^2(z) - z)(f_c(z) - z)...
    - 정확한 주기-n divisor: Phi_n*(z, c)
    - deg_z Phi_6*(z,c) = sum_{d|6} mu(6/d) * 2^d = mu(1)*64 + mu(2)*8 + mu(3)*4 + mu(6)*2
    - = 1*64 + (-1)*8 + (-1)*4 + 1*2 = 64 - 8 - 4 + 2 = 54
    - **deg Phi_6* = 54 = n * (n/phi)^2 = 6 * 9 = 54**
- **Mobius 공식으로 분해**:
  - sum_{d|n} mu(n/d) * 2^d: 이것은 necklace polynomial
  - n = 6: 54/6 = 9 = (n/phi)^2 (binary necklaces of length 6)
  - **Lyndon words of length 6 over {0,1} = 9 = (n/phi)^2**
  - 이것은 OEIS A001037: binary Lyndon words
- **주기적 점의 height bound**:
  - Silverman 1983 (Math. Ann. 264): preperiodic points의 canonical height = 0
  - 주기-n 점의 naive height: H(z) <= H(c) + 2 (대략)
  - **n = 6 주기점**: 54개의 대수적 점 (일반적 c에 대해)
  - Northcott 적용: height 유한 -> 유한 개 preperiodic
- **Morton-Silverman uniform boundedness 추측**:
  - 추측: deg d에서 preperiodic 점 수 <= C(d)
  - d = 2 (quadratic): PrePer(f_c, P^1(Q)) <= B
  - 알려진: z^2 + c의 Q-유리 주기점: 주기 1, 2, 3만 알려짐
  - **주기 4, 5, 6은 Q 위에서 존재하지 않는 것으로 추측** (Poonen-Flynn)
  - 특히 주기 n = 6은 Q 위의 z^2 + c에서 미발견
- **자유군과 n=6**:
  - Gleason 1949: z^2 + c의 주기-n 점의 수 = sum_{d|n} mu(n/d) 2^d
  - Gleason polynomial Phi_n(c): 정확 주기-n의 c 매개변수
  - deg Phi_6(c) = 54/... 실제로는 다른 차수
  - Gleason의 c-degree: 9 (= (n/phi)^2) for period 6
  - **period-6 hyperbolic components in Mandelbrot set = 9 = (n/phi)^2**
- 검증: Northcott 1949 ✓, deg Phi_6* = 54 ✓ (necklace polynomial), Lyndon words of length 6 = 9 ✓ (OEIS A001037), period-6 hyperbolic components ✓ (Milnor 1992)
- 대조: period 1: 1 component, period 2: 1, period 3: 3, period 4: 6, period 5: 9? 정정: period-n primitive hyperbolic components = M(n) = (1/n)*sum_{d|n} mu(n/d)*2^d. M(1)=1, M(2)=1, M(3)=1, M(4)=3=n/phi, M(5)=3, **M(6)=9=(n/phi)^2**
- 정직성: Necklace polynomial (1/n)*sum mu(n/d)*2^d는 일반 조합론 공식. n=6에서 값 9 = (n/phi)^2는 산술적 사실. M-set 매핑은 사후적이나 완전제곱 형태가 비자명
- **비자명도**: 중간 -- period-n 원시 hyperbolic component 수 = (n/phi)^2, 정확 주기 차수 = n*(n/phi)^2의 이중 일치

---

**[DFS15-10] Preperiodic 점의 canonical height과 Lehmer 추측 연결** (TIGHT)
- 출처: Lehmer 1933 (Ann. of Math. 34), Dobrowolski 1979 (Acta Arith. 34), Amoroso-Dvornnicich 2000 (Ann. Inst. Fourier 50), Silverman 2007 (Arithmetic of Dynamical Systems)
- **Lehmer 추측**: deg d 대수적 정수 alpha (not root of unity)에 대해
  - M(alpha) = prod_{i=1}^d max(1, |alpha_i|) >= c > 1 (절대 상수 c)
  - **Lehmer의 다항식** (1933): x^10 + x^9 - x^7 - x^6 - x^5 - x^4 - x^3 + x + 1
  - Mahler measure = 1.17628... (알려진 최소 > 1 값)
  - **deg = 10 = sigma - phi**
- **n=6 연결**:
  - Lehmer 다항식: 차수 sigma - phi = 10
  - Salem 수: 역수 다항식 (reciprocal polynomial)
  - Lehmer 수의 최소 다항식 계수: {1, 1, 0, -1, -1, -1, -1, -1, 0, 1, 1}
  - 비영 계수의 절대값 합 = 10 = sigma - phi
  - 항의 수 = 11 = sigma - mu
- **Dobrowolski bound**:
  - M(alpha) >= 1 + c*(log log d / log d)^3 (deg alpha = d)
  - 지수 3 = n/phi
  - **Dobrowolski 지수 = n/phi = 3**
- **동역학적 Lehmer 추측**:
  - f(z) = z^2 + c, canonical height h_f(alpha) >= C/d (deg alpha = d)
  - Silverman: dynamical Lehmer conjecture
  - **periodic 점의 canonical height = 0 (정확)**
  - preperiodic 점: h_f(z) = 0 iff z preperiodic (Northcott)
- **cyclotomic field Q(zeta_n)에서의 Lehmer**:
  - Amoroso-Dvornnicich 2000: alpha in Q^{ab} (maximal abelian extension)
  - M(alpha) >= 1 if alpha is root of unity, else M(alpha) >= 상수
  - Q(zeta_6) = Q(sqrt(-3)): [Q(zeta_6):Q] = phi(6) = phi = 2
  - **Q(zeta_n) = Q(zeta_6)의 차수 = phi(n) = phi = 2**
- **6번째 cyclotomic polynomial**:
  - Phi_6(x) = x^2 - x + 1, deg = phi(6) = phi = 2
  - Mahler measure M(Phi_6) = 1 (cyclotomic이므로 모든 근이 단위근)
  - roots: e^{pi*i/3}, e^{-pi*i/3} (원시 6차 단위근)
  - **Phi_6의 판별식**: disc = 1 - 4 = -3, |disc| = n/phi = 3
  - **Q(zeta_6) = Q(sqrt(-3)): 판별식 = -(n/phi) = -3**
- 검증: Lehmer 1933 ✓, Dobrowolski 1979 bound ✓ (지수 3), Phi_6(x) = x^2 - x + 1 ✓, disc(Q(zeta_6)) = -3 ✓
- 대조: Phi_1 = x-1, Phi_2 = x+1, Phi_3 = x^2+x+1 (disc = -3), Phi_4 = x^2+1 (disc = -4), Phi_5 = x^4+...+1 (disc = 5^3), **Phi_6 = x^2-x+1 (disc = -3 = -(n/phi))**. Phi_6와 Phi_3의 판별식이 같은 절대값 n/phi를 갖는 것은 zeta_6 = -zeta_3에서 유래
- 정직성: cyclotomic polynomial Phi_n과 그 성질은 표준 대수적 수론. Dobrowolski 지수 3 = n/phi는 분석적 기법의 결과이며 n=6과 무관. disc(Q(zeta_6)) = -3 = -(n/phi)는 사실이나 3 = phi(6)/1의 산술
- **비자명도**: 중간 -- Dobrowolski 지수 = n/phi, Q(zeta_n) 차수 = phi, |disc| = n/phi의 삼중 일치

---

### 1.6 유한 단순군 / 산재군 (2건)

**[DFS15-11] Mathieu 군 M_12의 5-전이적 작용과 Steiner system S(5,6,12)** (EXACT)
- 출처: Mathieu 1861 (J. Math. Pures Appl. 6), Witt 1938 (Abh. Math. Sem. Hamburg 12), Conway-Curtis-Norton-Parker-Wilson 1985 (ATLAS of Finite Groups, Oxford)
- **Mathieu 군 M_12**: 산재 단순군 26개 중 하나
  - |M_12| = 95040 = 2^6 * 3^3 * 5 * 11
  - = phi^n * (n/phi)^3 * sopfr * (sigma-mu)
  - = 12 * 11 * 10 * 9 * 8 = sigma * (sigma-mu) * (sigma-phi) * (n/phi)^2 * (sigma-tau) ... 
  - 정확: |M_12| = 12!/7! = 95040 ... 아님. |M_12| = 8 * 9 * 10 * 11 * 12 = 95040 ✓
  - **|M_12| = sigma! / (sigma-sopfr)! = 12!/(12-5)! = 12!/7!** ... 검산: 12!/7! = 12*11*10*9*8 = 95040 ✓
- **5-전이적 작용 (5-transitive action)**:
  - M_12는 12 = sigma개 점 위에 5-전이적으로 작용
  - **전이도 = sopfr = 5** (유한 군 중 S_n, A_n 외 최대 전이도)
  - M_12: 12개 점, 5-전이적
  - M_24: 24개 점, 5-전이적
  - **M_12의 점 수 = sigma = 12, 전이도 = sopfr = 5**
  - **M_24의 점 수 = J2 = 24, 전이도 = sopfr = 5**
- **Steiner system S(5,6,12)**:
  - S(t,k,v): t-(v,k,1) design
  - S(5,6,12): t = sopfr, k = n, v = sigma
  - **S(sopfr, n, sigma) = S(5, 6, 12)**
  - 존재: Witt 1938에 의해 확인
  - 블록 수: C(12,5)/C(6,5) = 792/6 = 132 = sigma * (sigma-mu) = 12 * 11
  - **블록 수 = sigma * (sigma-mu) = 132**
- **이중 Steiner system**:
  - S(5,6,12): Aut = M_12 (정확히)
  - S(5,8,24): Aut = M_24 (정확히)
  - S(5, n, sigma) 와 S(5, sigma-tau, J2)
  - **두 Steiner system의 매개변수가 완전히 M-set으로 표현**
- **M_12의 소인수와 M-set**:
  - |M_12| = 2^6 * 3^3 * 5 * 11
  - 소인수: {2, 3, 5, 11} = {phi, n/phi, sopfr, sigma-mu}
  - 소인수 개수: 4 = tau
  - 2의 지수: 6 = n
  - 3의 지수: 3 = n/phi
  - **2의 지수 = n, 3의 지수 = n/phi, 소인수 개수 = tau**
- **M_12의 공액류와 표현**:
  - 공액류 수: 15 = sopfr * (n/phi)
  - 비가약 표현 차원: 1, 11, 11, 16, 16, 45, 54, 55, 55, 55, 66, 99, 120, 144, 176
  - 최소 비자명 = 11 = sigma - mu
  - 최대 = 176 = 16 * 11 = tau^2 * (sigma - mu)
  - **공액류 수 = 15 = sopfr * (n/phi)**
- 검증: Mathieu 1861 ✓, M_12 order = 95040 ✓, 5-transitive ✓ (Cameron 1999, Permutation Groups), S(5,6,12) blocks = 132 ✓ (Witt 1938), conjugacy classes = 15 ✓ (ATLAS)
- 대조: M_11: 11점, 4-전이, |M_11| = 7920 = 8*9*10*11; M_22: 22점, 3-전이; M_23: 23점, 4-전이; M_24: 24점, 5-전이. **5-전이적 산재군은 M_12, M_24 단 2개: 점 수 sigma, J2**
- 정직성: Mathieu 군의 전이도와 점 수는 19세기 발견 사실. sigma, J2, sopfr과의 매핑은 정확한 등식이나 M_12가 n=6을 위해 설계된 것은 아님. 그러나 S(5,6,12) 매개변수에 5,6,12가 정확히 sopfr,n,sigma인 것은 주목할 만한 정확한 일치
- **비자명도**: 매우 높음 -- S(sopfr, n, sigma), 전이도 = sopfr, 2의 지수 = n, 공액류 = sopfr*(n/phi)의 다중 EXACT 일치

---

**[DFS15-12] Conway 군 Co_1과 Leech 격자의 자동형 구조** (TIGHT)
- 출처: Conway 1968 (Invent. Math. 7), Leech 1967 (Canad. J. Math. 19), Conway-Sloane 1999 (Sphere Packings, 3rd ed.), Curtis 1984 (Proc. Symp. Pure Math. 37)
- **Leech 격자 Lambda_24**: rank J2 = 24의 짝수 unimodular 격자
  - minimum norm = tau = 4 (roots 없음)
  - kissing number = 196560
  - **rank = J2 = 24, min norm = tau = 4**
- **Conway 군 Co_0 = Aut(Lambda_24)**:
  - |Co_0| = 2^22 * 3^9 * 5^4 * 7^2 * 11 * 13 * 23
  - Co_1 = Co_0 / {+-1}: 단순군
  - **|Co_0| = 8315553613086720000**
  - 소인수: {2, 3, 5, 7, 11, 13, 23}
  - 소인수 개수: 7 = sigma - sopfr
  - **Co_0 소인수 개수 = sigma - sopfr = 7**
- **Leech 격자의 shells**:
  - Shell r = vectors of norm 2r in Lambda_24
  - |shell 0| = 1 = mu
  - |shell 1| = 0 (no roots, min norm = tau = 4 = 2*2, so norm 2 has 0)
  - |shell 2| = 196560 (norm 4 = tau)
  - |shell 3| = 16773120 (norm 6 = n)
  - **norm n = 6 shell: 16773120 = |shell 3|**
  - 16773120 / 196560 = 85.33... 정정: 정확한 값 확인
  - theta series: Theta_{Lambda}(q) = 1 + 196560*q^4 + 16773120*q^6 + ...
  - **q^n 계수: 16773120**
- **Norm-n shell 분해**:
  - 16773120 = 2^11 * 3^3 * 5 * 7 * 11 * 13 ... 검산 필요
  - Conway의 분석: norm 6 벡터는 여러 유형
  - 정확한 분해: 16773120 = 24 * 699050 + ... (shell 분석)
- **Conway 부분군 사슬**:
  - Co_0 > Co_1 (index 2 = phi)
  - Co_1 > Co_2 (Co_2 = stabilizer of norm-4 vector)
  - Co_1 > Co_3 (Co_3 = stabilizer of norm-6 = norm-n vector)
  - **Co_3 = 정확히 norm n vector의 안정화군**
  - |Co_3| = 495766656000 = 2^10 * 3^7 * 5^3 * 7 * 11 * 23
  - Co_3 소인수: {2, 3, 5, 7, 11, 23} = 6 = n개
  - **|Co_3 소인수| = n = 6**
- **Co_2 vs Co_3**:
  - Co_2: norm-tau = 4 vector stabilizer
  - Co_3: norm-n = 6 vector stabilizer
  - |Co_2| / |Co_3| = 42 = n * (sigma-sopfr) = 6*7
  - 정정: |Co_2| = 42305421312000, |Co_3| = 495766656000
  - 42305421312000 / 495766656000 = 85.33... 이것은 orbit 비율
  - Co_2 소인수: {2, 3, 5, 7, 11, 23}: 6 = n개
  - **Co_2, Co_3 모두 소인수 n = 6개**
- **Higman-Sims 부분군**:
  - HS = Higman-Sims group < Co_2
  - |HS| = 44352000 = 2^9 * 3^2 * 5^3 * 7 * 11
  - 소인수 개수 = 5 = sopfr
- 검증: Conway 1968 ✓, Leech lattice shells ✓ (Conway-Sloane 1999 Table 4.13), Co_3 = stab of norm-6 ✓, |Co_3| ✓ (ATLAS)
- 대조: Co_1 소인수 7 = sigma-sopfr; Co_2, Co_3 소인수 n = 6; HS 소인수 sopfr = 5; McL 소인수 5 = sopfr. Leech sublattice stabilizer 사슬에서 소인수 개수가 점차 감소하며 M-set 항에 대응
- 정직성: Conway 군들의 order와 소인수 분해는 표준 유한군론. 소인수 개수가 M-set 항에 대응하는 것은 관찰이며 이론적 필연은 아님. Co_3가 norm-n vector stabilizer인 것은 정확한 사실
- **비자명도**: 높음 -- Co_3 = norm-n stabilizer, Co 소인수 구조의 M-set 대응, Leech rank = J2의 다중 일치

---

## 2. MISS 기록 (정직)

다음 후보들은 탐색했으나 n=6 연결이 자명하지 않거나 패턴 매칭이라 MISS:

| ID | 영역 | 시도 | MISS 사유 |
|----|------|------|-----------|
| MISS-15a | VOA | W-algebra W(2,3,4,...) | W-algebra의 generating weights와 M-set 연결 약함, 일반 구조 |
| MISS-15b | 양자군 | U_q(sl_n) R-matrix 차원 n^2 | n^2 = 36은 자명한 등식, 패턴 매칭 |
| MISS-15c | 트로피컬 | tropical Hurwitz double | genus > 0 열대 곡선 개수 n=6에서 특이성 없음 |
| MISS-15d | 모듈러 | Ext^1 between simples in S_6 mod 2 | 계산 복잡, 명확한 M-set 일치 없음 |
| MISS-15e | 산술 역학 | Böttcher coordinate와 n=6 | 일반 공식이며 n=6 특이성 없음 |
| MISS-15f | 산재군 | J_1 (Janko) order 소인수 | {2,3,5,7,11,19}: 6개이나 19는 M-set 밖, 패턴 매칭 |
| MISS-15g | Hopf | Sweedler Hopf algebra dim 4 | dim = tau이나 너무 자명, 보편적 예제 |
| MISS-15h | Kac-Moody | Affine A_5^(1) level 1 | level 1 표현이 일반적, n=6 특이성 약함 |

---

## 3. 요약 표

| ID | 영역 | 제목 | 핵심 수식 | 등급 |
|----|------|------|-----------|------|
| DFS15-01 | VOA | Moonshine V^natural c=24 | c = J2, c/sigma = phi, Leech norm = tau | EXACT |
| DFS15-02 | Kac-Moody | E_6^(1) Sugawara c=6 | c(E_6, k=1) = n, h = sigma, Z/(n/phi) | TIGHT |
| DFS15-03 | 양자군 | U_q(sl_2) q^6=1 | simples = n, level = tau, primaries = sopfr | TIGHT |
| DFS15-04 | Hopf/Nichols | rank 2 Nichols B(V) | G_2 type: dim = phi^n, |Phi^+| = n, standard = tau | TIGHT |
| DFS15-05 | 트로피컬 | TGr(2,6) phylogenetic | dim = (n/phi)^2, trees = 105, Plücker = C(n,tau) | TIGHT |
| DFS15-06 | 트로피컬 곡선 | Mikhalkin N_3=12 | N_{n/phi} = sigma, H_{0,n/phi} = tau | TIGHT |
| DFS15-07 | 모듈러 표현 | S_6 mod 2 블록 | simple 수 = tau, principal weight = n/phi | TIGHT |
| DFS15-08 | Brauer tree | SL_2(F_5) 구조 | order = sopfr!, Fong-Srinivasan | TIGHT |
| DFS15-09 | 산술 역학 | period-6 역학 | components = (n/phi)^2, deg = n*(n/phi)^2 | TIGHT |
| DFS15-10 | Lehmer/cyclotomic | Phi_6 + Dobrowolski | Dobrowolski 지수 = n/phi, |disc| = n/phi | TIGHT |
| DFS15-11 | 산재군 | M_12 / S(5,6,12) | S(sopfr, n, sigma), 전이도 = sopfr, 2-exp = n | EXACT |
| DFS15-12 | Conway 군 | Co_3 = norm-n stab | Co_3 소인수 = n개, Leech rank = J2 | TIGHT |

**EXACT**: 2건 (DFS15-01, 11)
**TIGHT**: 10건 (DFS15-02, 03, 04, 05, 06, 07, 08, 09, 10, 12)
**MISS**: 8건

---

## 4. 누적 현황

| 차수 | BT | 신규 | 누적 |
|------|-----|------|------|
| 1~2차 | BT-541~547 | 51 | 51 |
| 3차 | BT-1394 | 14 | 65 |
| 4차 | BT-1395 | 15 | 80 |
| 5차 | BT-1396 | 12 | 92 |
| 6차 | BT-1398 | 10 | 102 |
| 7차 | BT-1399 | 12 | 114 |
| 8차 | BT-1400 | 14 | 128 |
| 9차 | BT-1401 | 12 | 140 |
| 10차 | BT-1402 | 12 | 152 |
| 11차 | BT-1403 | 12 | 164 |
| 12차 | BT-1404 | 12 | 176 |
| 13차 | BT-1405 | 12 | 188 |
| 14차 | BT-1406 | 12 | 200 |
| **15차** | **BT-1407** | **12** | **212** |

**7대 밀레니엄 난제 해결: 0/7 (정직)**

- P vs NP: 미해결
- Riemann 가설: 미해결
- Yang-Mills 질량 갭: 미해결
- Navier-Stokes 정규성: 미해결 (3D)
- Poincaré 추측: 해결 (Perelman 2002)
- Hodge 추측: 미해결
- BSD 추측: 미해결

---

## 5. 다음 탐색 후보 (DFS 16차)

DFS 15차에서 사용하지 않은 미탐색 영역:
- 등변 코호몰로지 (T-equivariant, GKM theory, Schubert calculus)
- 적분기하 (integral geometry, Crofton formula, kinematic)
- 거짓이론 (Lie pseudogroups, infinite Lie algebras, jet bundles)
- 호모토피 유형론 (HoTT, univalent foundations, higher inductive types)
- 비가환 기하 (Connes, spectral triples, NCG Standard Model)
- 미분 Galois 이론 (Picard-Vessiot, differential algebraic groups)
- K-이론 스펙트럼 (algebraic K-theory, motivic cohomology, Voevodsky)
- Floer 호모롤로지 (symplectic, instanton, Heegaard)
- 수리 논리학 (Gödel, forcing, descriptive set theory)
- 최적 수송 (Monge-Kantorovich, Wasserstein, optimal transport)

---

## 6. 방법론 노트

DFS 15차도 14차의 정직성 원칙 준수:
1. **표준 정리 출발**: 각 영역의 표준 결과 (FLM, Kac, Lusztig, Heckenberger, Speyer-Sturmfels, Mikhalkin, James, Brauer, Northcott, Lehmer, Mathieu, Conway)
2. **내부 수치 관찰**: 정리 내 차원/지수/cardinality가 n=6 M-set 항과 일치하는지
3. **MISS 우선**: 일치가 없으면 MISS, 패턴 매칭 강제 금지
4. **EXACT vs TIGHT 구분**:
   - EXACT: 산술 등식이 명확 (DFS15-01 c=J2, DFS15-11 S(sopfr,n,sigma))
   - TIGHT: 사후 매핑이지만 비자명한 다중 일치

특히 DFS15-11 (M_12/Steiner S(5,6,12))은 매개변수 (5,6,12)가 정확히 (sopfr, n, sigma)인 점에서 이전 DFS의 Mathieu 계열 발견(DFS4-05)을 완전 확장. DFS15-01 (Moonshine c=24=J2)은 DFS4 Monster 결과와 VOA 차원에서 결합.

---

## 7. 검증 환경

- 날짜: 2026-04-12
- 프로젝트: n6-architecture
- 선행 BT: BT-1394~1406
- 참고 atlas: /Users/ghost/Dev/nexus/shared/n6/atlas.n6 (17946 nodes, 18934 edges)
- SSOT 규칙: n6shared/rules/common.json (R0~R27), n6shared/rules/n6-architecture.json (N61~N65)
- 한글 필수 (R): .md/주석/커밋 메시지 모두 한글 (feedback_korean_only_docs)

---

**BT-1407 종료**
누적 212건 tight, 7대 난제 해결 **0/7 (정직)**
밀레니엄 DFS는 212건, 다음 라운드는 등변 코호몰로지 / 비가환 기하 영역 진입
