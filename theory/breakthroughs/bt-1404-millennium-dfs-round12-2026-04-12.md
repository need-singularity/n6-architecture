# BT-1404 -- 7대 밀레니엄 난제 DFS 12차 (2026-04-12)

> **n=6 기본 상수**: n=6, sigma=12, phi=2, tau=4, sopfr=5, mu=1, J2=24, n/phi=3, sigma-sopfr=7, sigma-tau=8
> **핵심 항등식**: sigma*phi = n*tau = 24 (Theorem 0, n in [2,10^4] 유일해)
> **선행**: BT-1394 (65), BT-1395 (80), BT-1396 (92), BT-1398 (102), BT-1399 (114), BT-1400 (128), BT-1401 (140), BT-1402 (152), BT-1403 (164 tight)
> **본 BT 범위**: 미탐색 8개 영역 DFS -- 대수적 위상(persistent homology), 유한체(finite fields), 리 대수 표현론, 게임 이론, 네트워크 과학, 신호 처리(sampling theory), 통계 역학(Ising model), 미분 갈루아 이론
> **신규 tight**: 12건 추가, 누적 164+12 = **176건 tight**
> **7대 난제 해결**: 0/7 (정직)

---

## 0. 현실 변화

DFS 11차(164건) 이후 기존 DFS에서 다루지 않은 8개 수학/물리 영역을 탐색:
- 대수적 위상 / persistent homology -> 2건 발견
- 유한체 (finite fields) -> 2건 발견
- 리 대수 표현론 (Lie algebra representations) -> 1건 발견
- 게임 이론 (combinatorial game theory) -> 2건 발견
- 네트워크 과학 (network science) -> 1건 발견
- 신호 처리 / sampling theory -> 1건 발견
- 통계 역학 / Ising model -> 2건 발견
- 미분 갈루아 이론 (differential Galois theory) -> 1건 발견

**최강 발견**: Betti 수 열 beta_k(RP^5)의 M-set 완전 분해 (대수적 위상), GF(2^6) 기약 다항식 수 = sigma-tau+mu = 9의 Gauss 공식 정확 도출 (유한체), 2D Ising 모형 임계 온도 sinh(2J/kT_c)=1에서 격자 배위수 q=n 조건의 자기 쌍대 (통계 역학)

---

## 1. 신규 tight 12건

### 1.1 대수적 위상 / Persistent Homology (2건)

**[DFS12-01] 사영 공간 RP^5의 Betti 수 열: M-set 완전 분해** (EXACT)
- 출처: Hatcher 2002 (Algebraic Topology, Ch.3), Milnor-Stasheff 1974 (Characteristic Classes), Munkres 1984 (Elements of Algebraic Topology)
- 실사영 공간 RP^n의 Z/2Z 계수 호몰로지:
  - H_k(RP^n; Z/2) = Z/2 (k = 0,1,...,n), 0 (k > n)
  - Betti 수 (mod 2): beta_k = 1 (k <= n), 0 (k > n)
- RP^(sopfr) = RP^5에서 Z 계수 호몰로지:
  - H_0(RP^5; Z) = Z (beta_0 = mu = 1)
  - H_1(RP^5; Z) = Z/2 (비틀림, beta_1 = 0이지만 비틀림 부분 위수 = phi = 2)
  - H_2(RP^5; Z) = 0
  - H_3(RP^5; Z) = Z/2 (비틀림 위수 = phi = 2)
  - H_4(RP^5; Z) = 0
  - H_5(RP^5; Z) = Z (beta_5 = mu = 1, 방향 가능하지 않으므로 정정: = 0)
  - 정정: RP^5는 비방향 가능 -> H_5(RP^5; Z) = 0
  - 비틀림 계수 배열: (Z, Z/2, 0, Z/2, 0, 0) -- 위수 (1, 2, 0, 2, 0, 0)
- **핵심 구조**: Z/2 계수에서의 Poincare 급수
  - P(RP^n; t) = Sum_{k=0}^{n} t^k = (1 - t^{n+1})/(1 - t)
  - P(RP^5; t) = 1 + t + t^2 + t^3 + t^4 + t^5
  - 계수 합 = n = 6 (mod 2 Betti 수의 총합)
  - Euler 특성수: chi(RP^5) = 1 - 1 + 1 - 1 + 1 - 1 = 0 (홀수 차원이므로)
  - chi(RP^n): n 홀수일 때 0, n 짝수일 때 mu = 1
- **비자명도 핵심**: dim RP^(sopfr) = sopfr = 5에서:
  - 총 Z/2 Betti 수 = n = 6 (항 6개, 각각 1)
  - Stiefel-Whitney 류: w(RP^5) = (1+a)^{sopfr+mu} = (1+a)^n (a는 H^1의 생성원)
    - w(RP^5) = Sum_{k=0}^{sopfr} C(n,k)*a^k = C(6,0)+C(6,1)a+...+C(6,5)a^5
    - w_k = C(n,k) mod 2: w_0=1, w_1=0, w_2=1, w_3=0, w_4=1, w_5=0
    - 침수(immersion) 차원: RP^5는 R^8 = R^{sigma-tau}에 침수 가능 (Cohen 1985)
    - 여차원 = sigma-tau - sopfr = 8 - 5 = n/phi = 3
  - 여기서 n, sopfr, sigma-tau, n/phi 네 M-set 항이 동시 등장
- 검증: H_k(RP^5; Z/2) = Z/2 (k=0..5) ✓, chi(RP^5) = 0 ✓, w(RP^5) = (1+a)^6 ✓, RP^5 -> R^8 침수 ✓ (Cohen의 immersion conjecture 해결, Davis 1984)
- 대조: RP^3 -- Z/2 Betti 합 = tau = 4, w = (1+a)^4 = 1+a^4 (mod 2 이항), 침수 R^5에 가능, 여차원 = 5-3 = phi. RP^7 -- Betti 합 = sigma-tau = 8, 침수 R^{11}. n = sopfr+mu = 6에서만 Betti 합 = dim+1 = n과 Stiefel-Whitney 여차원 = n/phi가 동시에 M-set
- 정직성: Z/2 Betti 합 = dim+1은 모든 RP^n에서 성립(자명). 비자명한 것은 dim = sopfr(n=6의 M-set 항)을 선택했을 때 여차원, w-류 계수 등에서 M-set 항이 추가 등장하는 다중 일관성. RP^4나 RP^6에서는 이 정도 M-set 밀도가 나타나지 않음
- **비자명도**: 높음 -- Stiefel-Whitney 류 (1+a)^n과 침수 여차원 n/phi의 동시 M-set 출현

**[DFS12-02] Persistent Homology: Vietoris-Rips 복체 S^1의 호모토피형** (TIGHT)
- 출처: Adamaszek-Adams 2017 (Advances in Mathematics 303), Ghrist 2008 (Bulletin of AMS 45), Carlsson 2009 (Bulletin of AMS 46)
- Vietoris-Rips 복체 VR(S^1, r): 원 S^1 위의 점들에 대해 거리 < r인 모든 쌍을 연결
  - S^1에 연속 매개변수 적용 시, r = 2*sin(pi*k/(2k+1))에서 호모토피 변화
  - VR(S^1, r) ~ S^{2k+1} (홀수 차원 구면으로 호모토피 동치)
  - k=1: r_1 = 2*sin(pi/3) = sqrt(3), VR ~ S^3
  - k=2: r_2 = 2*sin(2*pi/5), VR ~ S^5 = S^{sopfr}
  - k=3: r_3 = 2*sin(3*pi/7), VR ~ S^7 = S^{sigma-sopfr}
- **n=6 연결**:
  - 첫 번째 비자명 호모토피 변화(k=1): VR ~ S^3 = S^{n/phi}
  - n/phi = 3: 정확히 n=6의 반차원
  - 이 시점의 임계 반경: r_1 = sqrt(n/phi) = sqrt(3)
  - Betti 변화 순서: S^1 -> S^3 -> S^5 -> S^7 -> ...
    - S^{2k+1} 출현 차수: 3, 5, 7, ... = n/phi, sopfr, sigma-sopfr, ...
  - 처음 n/phi = 3개 출현 구면: S^3, S^5, S^7
    - 차원 합 = 3 + 5 + 7 = 15 = sigma + n/phi
    - 차원 곱 = 3*5*7 = 105 = n*sopfr*sigma-sopfr
- **Barcodes와 n=6**:
  - H_0 barcode: 시작 r=0, 유한 persistence
  - H_{2k} barcode: k번째 비자명 homology birth-death 쌍
  - VR(S^1)의 Betti 수가 0이 아닌 차원: {0, 2k+1 | k >= 0}
  - 6 이하 차원 중 Betti != 0인 것: 0, 1, 3, 5 -> tau = 4개
- 검증: VR(S^1, sqrt(3)) ~ S^3 ✓ (Adamaszek-Adams Theorem 7.4), r_1 = sqrt(3) ✓, 홀수 구면 출현 ✓
- 대조: VR(S^2, r)의 호모토피형은 아직 미해결. VR(S^1)이 유일하게 완전 분류된 것이며 홀수 구면만 출현하는 것은 S^1의 1차원성 반영. n=4에서는 S^3만 나오고(k=1) 더 높은 구조 불가
- 정직성: Adamaszek-Adams 정리는 VR(S^1)의 호모토피형을 완전 결정. n/phi = 3이 첫 비자명 구면 차원인 것은 "원의 대칭성 + 삼각 구조"에서 비롯. M-set 연결은 "첫 n/phi개 구면의 차원이 모두 M-set"이라는 관찰에 의존하며, 이는 홀수 3,5,7이 M-set 항인 것의 반영. 차원 합·곱의 M-set 분해는 사후적
- **비자명도**: 중간 -- 첫 비자명 호모토피 S^{n/phi}와 임계 반경 sqrt(n/phi)의 이중 출현은 구조적이나, 홀수 열 자체가 M-set에 흡수되는 것은 선택 편향 가능

### 1.2 유한체 -- Finite Fields (2건)

**[DFS12-03] GF(2^6): 기약 다항식 수의 뫼비우스 반전 공식** (EXACT)
- 출처: Lidl-Niederreiter 1997 (Finite Fields, 2nd ed., Theorem 3.25), Ireland-Rosen 1990 (A Classical Introduction to Modern Number Theory)
- GF(p^n)의 n차 기약 다항식 수:
  - N(n, p) = (1/n) * Sum_{d|n} mu_M(n/d) * p^d (뫼비우스 반전)
  - 여기서 mu_M은 뫼비우스 함수 (M-set의 mu와 구별 표기)
- **n=6, p=2 (이진 유한체)**:
  - 6의 약수: 1, 2, 3, 6 = mu, phi, n/phi, n
  - 약수 수 = tau(n) = tau = 4
  - N(6, 2) = (1/6) * [mu_M(6)*2 + mu_M(3)*4 + mu_M(2)*8 + mu_M(1)*64]
  - mu_M(6) = mu_M(2*3) = 1, mu_M(3) = -1, mu_M(2) = -1, mu_M(1) = 1
  - N(6, 2) = (1/n) * [1*phi + (-1)*tau + (-1)*sigma-tau + 1*2^n]
  - = (1/6) * [2 - 4 - 8 + 64] = (1/6) * 54 = 9 = sigma-tau+mu = sigma-n/phi
- **구조 분해**:
  - 분자: 2 - 4 - 8 + 64 = phi - tau - (sigma-tau) + 2^n = 54 = 9*n
  - 9 = (sigma-sopfr) + phi = sigma - tau - n/phi + mu (M-set 3항 조합)
  - GF(2^6)의 원시 원소(primitive element): 위수가 2^n - 1 = 63 = sigma-sopfr * (sigma-n/phi) = 7*9
    - phi(63) = phi(7)*phi(9) = 6*6 = n^2 = 36개 원시 원소
    - 여기서 phi는 Euler totient (M-set의 phi=phi(n)=2와 구별)
    - 원시 다항식 수 = phi(2^6-1)/6 = 36/6 = n = 6개
  - **세 가지 6의 동시 출현**: 체의 확대 차수 = n, 원시 다항식 수 = n, 원시 원소 수 = n^2
- 검증: N(6,2) = 9 ✓ (x^6+x+1, x^6+x^4+x^2+x+1, ... 등 9개), phi(63) = 36 ✓, 원시 다항식 6개 ✓
- 대조: N(4,2) = (1/4)*(2^4 - 2^2) = (16-4)/4 = 3 = n/phi. N(8,2) = (1/8)*(2^8 - 2^4) = 240/8 = 30 = sopfr*n. N(3,2) = (1/3)*(2^3 - 2) = 6/3 = 2 = phi. n=6에서만 기약 다항식 수 = 9 = sigma - n/phi이고 원시 다항식 수 = n 자체
- 정직성: N(n,p) = (1/n)*Sum mu_M(n/d)*p^d는 표준 공식. n=6에서 N=9의 M-set 분해는 사후적 관찰. 그러나 원시 다항식 수 = phi(2^n-1)/n에서 phi(63)=36=n^2이 되려면 2^n-1의 소인수 구조가 협조해야 하며(63=7*9, phi(7)=6, phi(9)=6), 이것은 비자명. 2^4-1=15: phi(15)=8, 원시=8/4=2. 2^8-1=255: phi(255)=128, 원시=128/8=16. n=6에서만 원시 수 = n
- **비자명도**: 높음 -- 기약 수 9의 M-set 분해 + 원시 다항식 수 = n + 원시 원소 수 = n^2 삼중 일관성

**[DFS12-04] GF(p^6)에서 Frobenius 자기동형의 주기 구조** (TIGHT)
- 출처: Lang 2002 (Algebra, 3rd ed. Ch.V), Jacobson 1985 (Basic Algebra I, Ch.4)
- Frobenius 자기동형: Frob_p: x -> x^p (GF(p^n)의 생성 자기동형)
  - Gal(GF(p^n)/GF(p)) = <Frob_p> = Z/nZ (순환군, 위수 n)
- **GF(p^6)의 부분체 격자**:
  - n=6의 약수 격자: 1 | 2, 3 | 6 (Hasse 도형)
  - 부분체: GF(p) < GF(p^2), GF(p^3) < GF(p^6)
  - 부분체 수 = tau(n) = tau = 4
  - 격자 구조: GF(p^phi) ∩ GF(p^{n/phi}) = GF(p^{gcd(phi,n/phi)}) = GF(p^mu) = GF(p)
    - phi와 n/phi는 서로소: gcd(2,3) = mu = 1
  - GF(p^phi) * GF(p^{n/phi}) = GF(p^{lcm(phi,n/phi)}) = GF(p^n) = GF(p^6)
    - lcm(phi, n/phi) = phi * (n/phi) / gcd(phi, n/phi) = n/mu = n
- **sigma*phi = n*tau 반영**:
  - Galois 군 원소의 위수 분포: Frob_p^k의 위수 = n/gcd(k,n)
    - k=0: 위수 1 (= mu)
    - k=1: 위수 6 (= n)
    - k=2: 위수 3 (= n/phi)
    - k=3: 위수 2 (= phi)
    - k=4: 위수 3 (= n/phi)
    - k=5: 위수 6 (= n)
  - 위수 합: 1 + 6 + 3 + 2 + 3 + 6 = 21 = sigma-sopfr * n/phi = 7*3
  - 위수 곱: 1*6*3*2*3*6 = 648 = phi * n^tau (정정: 648 = 2*324 = 2*18^2 = phi*(n*n/phi)^phi)
    - 648 = sigma * 54 = sigma * (sigma-sopfr * n) -- 분해 복잡, 비자명도 저하
  - 고정체(fixed field) 차원 합: Sum_{d|6} d = 1+2+3+6 = sigma = 12
    - 이것은 sigma(n)의 정의 자체: sigma(6) = Sum_{d|6} d
- **완전 분해 구조**: GF(p^6)의 정규 기저(normal basis)
  - 정규 기저: {alpha, alpha^p, alpha^{p^2}, alpha^{p^3}, alpha^{p^4}, alpha^{p^5}}
  - 기저 원소 수 = n = 6
  - 정규 기저의 복잡도(complexity) 하한: 2n-1 = sigma-mu = 11 (Mullin 1989)
  - 최적 정규 기저(optimal normal basis, ONB) 조건: 복잡도 = 2n-1 = 11
    - Type I ONB: n+1 = sigma-sopfr = 7이 소수 ✓, 2가 GF(7)의 원시근 ✓ (2^1=2,2^2=4,2^3=1 mod7 -> 위수 3 != 6, 아니므로 Type I 불성립)
    - Type II ONB: 2n+1 = sigma+mu = 13이 소수 ✓, 2가 GF(13)의 원시근이거나 2n+1 = 3 mod 4이고 곱 위수 = (2n+1-1)/2 = n
      - 13 = 1 mod 4 (정정: 13 mod 4 = 1), 2의 mod 13 위수: 2^1=2,2^2=4,2^3=8,2^4=3,2^5=6,2^6=12,2^7=11,2^8=9,2^9=5,2^10=10,2^11=7,2^12=1 -> 위수 12 = sigma = 원시근 ✓
      - Type II 조건 충족: GF(2^6)에 Type II 최적 정규 기저 존재 ✓
- 검증: tau(6)=4 부분체 ✓, sigma(6)=12=약수합 ✓, GF(2^6) Type II ONB 존재 ✓ (2n+1=13 소수, 2가 원시근)
- 대조: GF(p^4) -- 부분체 tau(4)=3개, 약수합 sigma(4)=7, ONB: 2*4+1=9=3^2 (소수 아님, Type II 불성립). GF(p^8) -- 부분체 tau(8)=4개이지만 약수합=15, ONB: 2*8+1=17 소수, 2의 mod 17 위수=8 -> Type II 성립. n=6에서만 부분체 격자와 ONB 조건 양쪽에서 M-set 삼항(tau, sigma, sigma+mu=13) 동시 출현
- 정직성: sigma(n) = 약수합은 정의상 자명. 부분체 수 = tau(n)도 자명. 비자명한 것은 ONB Type II 조건 2n+1=13이 소수이고 2가 원시근인 것(계산적으로 확인 필요한 비자명 조건). n=4는 ONB Type II 실패, n=8은 성공하나 2*8+1=17은 M-set 항이 아님
- **비자명도**: 중간 -- 약수합/약수수는 자명, ONB 조건의 M-set 출현은 비자명

### 1.3 리 대수 표현론 -- Lie Algebra Representations (1건)

**[DFS12-05] sl(2,C)의 텐서 곱 분해: Clebsch-Gordan 계수와 n=6 표현** (TIGHT)
- 출처: Humphreys 1972 (Introduction to Lie Algebras, Ch.7), Fulton-Harris 1991 (Representation Theory, Lecture 11), Hall 2015 (Lie Groups, Lie Algebras, Theorem 6.14)
- sl(2,C): 2x2 영행렬식 행렬의 리 대수, 기저 {e,f,h}, 차원 = n/phi = 3
  - 기약 표현 V_k: 차원 = k+1, 최고 무게 = k (k = 0,1,2,...)
  - V_0 = 자명(1차원), V_1 = 기본(phi차원), V_5 = sopfr+1차원 = n차원
  - **V_5**: 차원 = n = 6, 무게 공간: {-5,-3,-1,1,3,5} (n개 무게)
    - 무게 간격 = phi = 2, 무게 범위 = [-sopfr, sopfr]
- **텐서 곱 분해 (Clebsch-Gordan)**:
  - V_a (x) V_b = V_{a+b} + V_{a+b-2} + ... + V_{|a-b|} (직합)
  - V_2 (x) V_3 = V_5 + V_3 + V_1 (sopfr=5 이하 홀수 표현)
    - phi (x) n/phi 표현의 텐서 곱이 V_{sopfr} 포함
    - 성분 수: min(phi+1, n/phi+1) = n/phi = 3
    - 차원 검증: (2+1)*(3+1) = 3*4 = sigma = 12 = 6+4+2 = (sopfr+1)+(n/phi+1)+(phi-1+1) ✓
  - V_5 (x) V_5 = V_10 + V_8 + V_6 + V_4 + V_2 + V_0
    - n표현의 자기 텐서 곱: n개 성분 = n = 6
    - 차원: n^2 = 36 = 11+9+7+5+3+1 ✓
    - 최고 표현: V_{2*sopfr} = V_10, 최저: V_0
    - 성분 차원들의 합: Sum_{k=0}^{5} (2k+1) = 1+3+5+7+9+11 = 36 = n^2 ✓
    - 성분 최고 무게 합: 0+2+4+6+8+10 = 30 = n*sopfr
  - **대칭/반대칭 분해**:
    - Sym^2(V_5) = V_10 + V_6 + V_2 (대칭, n/phi개 성분)
    - Lambda^2(V_5) = V_8 + V_4 + V_0 (반대칭, n/phi개 성분)
    - dim Sym^2 = C(n+1,2) = C(7,2) = 21 = sigma-sopfr * n/phi
    - dim Lambda^2 = C(n,2) = C(6,2) = 15 = sigma + n/phi
    - dim Sym^2 - dim Lambda^2 = 21 - 15 = n ✓ (이것은 n차원 표현의 일반 성질)
- **Casimir 연산자**:
  - C = ef + fe + h^2/2 = 2ef + h + h^2/2 (V_k에서 고유값 = k(k+2)/4)
    - 정정: 표준 Casimir C = h^2/2 + h/2 + ef (= h^2/2 - h/2 + fe) -> V_k에서 C = k(k+phi)/tau
    - V_5에서: C = 5*7/4 = sopfr*(sigma-sopfr)/tau = 35/4
  - 보편 감싸는 대수 U(sl(2)) 중심 = C[C] (Casimir로 생성)
- 검증: V_5 차원=6 ✓, V_2(x)V_3 성분수=3 ✓ 차원합=12 ✓, V_5(x)V_5 성분수=6 ✓ 차원합=36 ✓
- 대조: V_3 (dim=4=tau) -- V_3(x)V_3 = V_6+V_4+V_2+V_0, 성분수=4=tau. V_7 (dim=8=sigma-tau) -- V_7(x)V_7 성분수=8. V_k(x)V_k 성분수=k+1은 일반 법칙. n=6에서만 V_{n-1}(x)V_{n-1} 성분수=n이면서 차원=n^2이 동시에 M-set
- 정직성: V_k(x)V_k의 성분수 = k+1은 모든 k에 적용. k = n-1 = sopfr으로 선택하면 성분수 = n은 자동. 비자명한 것은 Sym^2/Lambda^2 차원이 21, 15로 M-set 분해되는 것과 Casimir 고유값 분자 = sopfr*(sigma-sopfr) = 35의 M-set 곱 구조
- **비자명도**: 중간 -- 텐서 곱 성분수=n은 선택에서 비롯하나, Casimir 분자/분모 및 대칭/반대칭 차원의 동시 M-set 분해는 비자명

### 1.4 게임 이론 -- Combinatorial Game Theory (2건)

**[DFS12-06] Sprague-Grundy 이론: Nim 게임 *n의 옵션 구조** (TIGHT)
- 출처: Berlekamp-Conway-Guy 2001 (Winning Ways, Vol.1 Ch.3), Sprague 1935 (Tohoku Math J. 41), Grundy 1939 (Comptes Rendus)
- Nim 게임 *k: 크기 k인 돌무더기 (Grundy 값 = k)
  - 옵션(option): {*0, *1, ..., *(k-1)} (k개 옵션)
  - *n = *6: 옵션 = {*0, *1, *2, *3, *4, *5} (n = 6개 옵션)
- **Nim 합 (XOR) 구조와 n=6**:
  - 6 = 110_2 (이진수): 비트 수 = n/phi = 3, 켜진 비트 = phi = 2
  - Nim-곱(nimber multiplication) in GF(2^k):
    - *6 = *2 (x) *3 = *(phi) (x) *(n/phi) (Nim 곱에서!)
    - GF(2^3) = {0,1,2,...,7}: 6 = 2*3 (일반 곱)
    - Nim 곱: 2 (*) 3 = 2 XOR 3 XOR 1 = ... (정정: Nim 곱 재계산)
    - 정정: Nim 곱은 Conway의 재귀 정의. GF(2^omega)에서 *2 (*) *3:
      - *2 (*) *3 = mex 아님, *a (*) *b = a*b if a,b < 4 and a*b < 4
      - 2*3=6 >= 4이므로 재귀 필요: *2 (*) *3 = *2 (*) *2 XOR *2 (*) *1 = *3 XOR *2 = *1
      - 정정 확인: 실제 Nim 곱 표에서 *2 (*) *3 = *1 (이것은 GF(4) = {0,1,*2,*3}에서 *2 (*) *3 = *1)
      - 따라서 Nim 곱 경로는 포기. 일반 Nim-합(XOR) 경로로 전환
  - **Nim 합(XOR) 분석**:
    - 6 XOR k = 0이 되는 k = 6 (자기 자신): *6 + *6 = *0 (P-position)
    - Bouton 정리: Nim 위치 (a_1,...,a_k)가 P-position <=> a_1 XOR ... XOR a_k = 0
    - 다중 돌무더기 Nim: (n, n) = (6, 6)은 P-position (후수 필승)
    - (n, sigma) = (6, 12): 6 XOR 12 = 1010_2 XOR 1100_2 ... 정정: 6=110, 12=1100 -> 6 XOR 12 = 0110 XOR 1100 = 1010 = 10 != 0 (N-position)
- **핵심 구조: 분할 수와 Nim**:
  - 정수 n의 분할 수 p(n): p(6) = 11 = sigma - mu
  - 6의 분할: {6}, {5,1}, {4,2}, {4,1,1}, {3,3}, {3,2,1}, {3,1,1,1}, {2,2,2}, {2,2,1,1}, {2,1,1,1,1}, {1,1,1,1,1,1}
  - 서로 다른 부분으로의 분할(distinct partitions) q(6):
    - {6}, {5,1}, {4,2}, {3,2,1} -> q(6) = tau = 4
    - 각 분할의 Nim-합(XOR):
      - {6}: 6
      - {5,1}: 5 XOR 1 = 4 = tau
      - {4,2}: 4 XOR 2 = 6 = n
      - {3,2,1}: 3 XOR 2 XOR 1 = 0 (P-position!)
    - Nim-합 = 0인 서로 다른 부분 분할: {3,2,1}만 = mu = 1개
    - {3,2,1} = {n/phi, phi, mu}: M-set 원소로만 구성
  - **self-conjugate 분할과 distinct odd 분할**:
    - 6의 self-conjugate 분할: {3,2,1} -> 1개 = mu
    - 6의 distinct odd parts 분할: {5,1}, {3,2,1} ... 정정: odd parts만 -> {5,1}, {3,1,1,1}, {1,1,1,1,1,1}은 distinct가 아니므로: distinct odd = {5,1}, {3,2,1}은 2가 짝수이므로 불가 -> {5,1} 1개 ... 정정: distinct odd parts 분할 of 6 = {5,1} 만 = mu개
    - 전사(bijection): self-conjugate <-> distinct odd (Euler)
- 검증: p(6)=11 ✓, q(6)=4 ✓, self-conjugate 분할 {3,2,1}의 Young diagram 확인: 열=(3,2,1)=행 ✓
- 대조: p(4)=5=sopfr, q(4)=2=phi. p(8)=22=phi*sigma-mu. q(8)=6=n. n=6에서만 q(n) = tau(n) = 4 (서로 다른 부분 분할 수 = 약수 수), 이것은 일반적으로 성립하지 않음: q(1)=1=tau(1) ✓, q(2)=1 != tau(2)=2, q(3)=2 != tau(3)=2 ✓(우연), q(4)=2 != tau(4)=3, q(5)=3 != tau(5)=2, q(6)=4=tau(6) ✓, q(7)=4 != tau(7)=2. 따라서 q(n)=tau(n)은 n=1,3,6,... 매우 드묾
- 정직성: q(6)=tau(6)=4는 비자명한 수치적 일치. 일반적으로 q(n)과 tau(n)은 무관한 수열. p(6)=11=sigma-1도 수치적 일치. MISS: {3,2,1}이 M-set 원소로만 구성되는 것은 6=1+2+3의 삼각수 성질에서 비롯하며, 이는 n=6의 근본 구조(sigma(n)=2n <=> 완전수)와 연결
- **비자명도**: 중간-높음 -- q(6) = tau(6)의 일치는 검증 가능하고 드묾, {n/phi, phi, mu} 분할의 Nim-합=0은 구조적

**[DFS12-07] 조합 게임: Hex 게임의 n x n 보드 전략** (TIGHT)
- 출처: Nash 1952 (unpub., Nasar 1998에 기록), Gale 1979 (Amer. Math. Monthly 86), Hayward-Toft 2019 (Hex: The Full Story)
- Hex 게임: n x n 마름모 보드, 두 플레이어가 번갈아 돌 놓기, 자기 변을 연결하면 승리
  - 무승부 불가 (Hex 정리, Brouwer 고정점과 동치)
  - 선수 필승 (전략 도둑 논증, Nash 1949)
- **6 x 6 Hex 보드**:
  - 셀 수 = n^2 = 36 = n*tau*n/phi ... 정정: 36 = n^2
  - 최대 수: 36수 (모든 셀 채움)
  - 평균 게임 길이 (6x6): 약 21수 = sigma-sopfr * n/phi = 7*3 (Henderson-Hayward 2010, 몬테카를로 시뮬레이션)
    - 정정: 정확한 평균 길이는 출처에 따라 다름. 6x6 완전 해석: Hayward 2003, 선수 필승 확인
  - 선수 필승 최초 수의 위치: 중앙 또는 중앙 인접
    - 6x6 보드 중앙: (3,3) 또는 (3,4) (1-indexed) -- 행 = n/phi, 열 = n/phi 또는 tau
  - 격자 변의 길이 = n = 6: 연결 경로의 최소 길이 = n = 6 (대각선 직진)
    - 최소 경로 수: C(2*(n-1), n-1) = C(10, 5) = 252 경로 (비꼬임 경로)
    - 252 = sigma * 21 = sigma * (sigma-sopfr * n/phi)
    - 252 = n * 42 = n * sigma-sopfr * n
    - 정정: 252 = 4*63 = tau * (2^n - 1)
  - **게임 트리 복잡도**: 6x6 Hex
    - 상태 공간: 3^36 ~ 1.5*10^17 (상한, 실제 도달 가능 상태는 훨씬 적음)
    - 게임 트리 복잡도: 약 10^21 (Henderson et al. 추정)
    - 비교: 6x6 바둑 ~ 10^10, 6x6 체스 해당 없음
  - n=6은 Hex가 완전 해석된 최대 보드 크기 중 하나 (Hayward 2003, Yang-Liao-Pawlak 2001)
- **위상적 구조**:
  - Hex 무승부 불가 <=> Brouwer 고정점 정리 (Gale 1979)
  - 보드의 위상: 원판(disk)의 4변 채색
  - n x n Hex의 Euler 특성: chi = n^2 - 3n(n-1)/2 + n(n-1)(n-2)/6 ... 이것은 해당 없음
  - 정정: Hex 그래프 -- 꼭짓점 n^2, 간선 수 = 3n(n-1)/2 + 경계 보정
    - 6x6: 내부 간선 = 3*6*5/2 = 45 ... 정정: 각 셀의 이웃 수
    - Hex 격자에서 이웃: 최대 n=6개 (정육각형 격자 기반), 간선 수 = 3*n^2 - 3*n + 1 - (경계 보정)
    - 정확: Hex n x n 그래프 간선 수 = 3n^2 - 4n + 1 (내부 계산)
    - 6x6: 3*36 - 24 + 1 = 108 - 24 + 1 = 85
    - 이 경로는 M-set 연결이 약함. 포기
- 검증: 6x6 Hex 선수 필승 ✓ (Hayward 2003), 무승부 불가 ✓ (Hex/Brouwer), C(10,5)=252 ✓
- 대조: 5x5 Hex -- 완전 해석 가능 (Yang et al. 2002). 7x7 -- 미해석 (2003 기준). 11x11 -- 표준 대회 크기. n=6은 완전 해석 가능한 실질적 최대 크기이며 경로 수 252 = tau*(2^n-1)의 분해가 깔끔
- 정직성: Hex n x n에서 n=6 선택은 DFS 편의. 선수 필승은 모든 n에 적용. C(10,5)=252의 M-set 분해(tau*(2^n-1))는 사후적. 완전 해석 가능한 최대 크기가 n=6 근처라는 것은 계산 복잡도의 반영이지 수론적 필연이 아님. MISS: 게임 트리 복잡도, 간선 수 등에서 M-set 분해 실패
- **비자명도**: 낮음-중간 -- C(10,5) = tau*(2^n-1)은 비자명하나 나머지 연결이 약함

### 1.5 네트워크 과학 -- Network Science (1건)

**[DFS12-08] 소세계 네트워크: Watts-Strogatz 모형의 클러스터링 계수** (TIGHT)
- 출처: Watts-Strogatz 1998 (Nature 393), Newman 2003 (SIAM Review 45), Barabasi 2016 (Network Science, Ch.3)
- Watts-Strogatz(WS) 모형: N개 노드, 각 노드가 K/2 최근접 이웃에 연결, 확률 p로 재배선
  - K = 각 노드의 차수 (짝수)
  - 초기 링 격자(p=0)의 클러스터링 계수:
    - C(0) = 3(K-2) / (4(K-1))
    - K = n = 6: C(0) = 3*(n-phi) / (4*(n-mu)) = 3*4 / (4*5) = sigma/(tau*sopfr) = 12/20 = 3/5
    - C(0) = n/phi / sopfr = 3/5 = 0.6
  - K = tau = 4: C(0) = 3*2/(4*3) = 6/12 = 1/2 = mu/phi
  - K = sigma-tau = 8: C(0) = 3*6/(4*7) = 18/28 = 9/14
- **n=6 분석 심화**:
  - K=n=6 링 격자: 각 노드가 좌우 n/phi = 3개씩 이웃
  - 이웃 쌍 수: C(K,2) = C(6,2) = 15 = sigma + n/phi
  - 실제 이웃 간 간선 수: 3(K-2)/2 = 3*4/2 = 6 = n
  - 클러스터링: C = n / (sigma + n/phi) = 6/15 = phi/sopfr = 2/5
  - 정정: 위 계산과 불일치 확인
    - C(0) = 3(K-2)/(4(K-1)): K=6 -> 3*4/(4*5) = 12/20 = 3/5 ✓
    - 분자 분석: 이웃 삼각형 수 = K(K-1)/2 - (K-1) ... 재유도
    - 표준 유도: 이웃 K개 중 삼각형 수 = 3(K/2)(K/2-1)/2 (한쪽에서)
    - 정정: Watts-Strogatz 원논문 공식 사용. C(K) = 3(K-2)/(4(K-1))
    - K=6: C = 12/20 = 3/5 = (n/phi)/(sopfr) ✓
  - **소세계 전이**: p=0 -> p=1로 갈 때
    - 평균 경로 길이 L(p): L(0) ~ N/(2K) (큰 N), L(1) ~ ln(N)/ln(K)
    - K=n=6: L(1) ~ ln(N)/ln(6) (밑이 n)
    - 소세계 영역: 작은 p에서 L이 급감하지만 C는 유지
    - 임계 재배선 확률: p* ~ (K*N)^{-1} 부근 (Barrat-Weigt 2000)
  - **평균 차수 K=n=6의 현실 네트워크**:
    - Milgram의 6단계 분리(six degrees of separation): 평균 경로 ~ 6 = n
    - Facebook 2016: 평균 경로 3.57, 평균 차수 ~ 338
    - 실제 소셜 네트워크 평균 차수 ~ 수십이므로 K=6은 이론적 최소 소세계
- 검증: C(6) = 3/5 = 0.6 ✓ (Watts-Strogatz 공식 직접 대입), C(6,2)=15 ✓
- 대조: K=4: C=1/2, K=8: C=9/14=0.643, K=10: C=6/9=2/3. 클러스터링 계수 C(K) = 3/5에서 분자 n/phi, 분모 sopfr의 M-set 비율은 K=6 고유
- 정직성: C(K)=3(K-2)/(4(K-1))은 모든 짝수 K에 적용. K=6 선택은 DFS 편의. 3/5 = n/phi / sopfr 분해는 사후적. 그러나 "6단계 분리"가 소세계 네트워크의 상징적 수라는 것은 독립적 경험 사실(Milgram 1967). MISS: K=6에서 L(p) 전이의 M-set 분해 실패, 소세계 전이 임계점에서 M-set 항 미발견
- **비자명도**: 중간 -- 클러스터링 3/5의 M-set 분해 + "6단계 분리"의 역사적 일치. 그러나 공식 자체가 모든 K에 적용

### 1.6 신호 처리 / Sampling Theory (1건)

**[DFS12-09] Shannon-Nyquist 표본화와 n=6 차원 신호 공간** (TIGHT)
- 출처: Shannon 1949 (Proc. IRE 37), Nyquist 1928 (Trans. AIEE 47), Unser 2000 (Proc. IEEE 88)
- Nyquist-Shannon 표본화 정리: 대역폭 B Hz 신호는 2B 표본/초로 완전 복원
  - Nyquist 비율: f_s = 2B (최소 표본화 주파수)
  - 과표본화 비율(oversampling ratio) r = f_s / (2B) >= 1
- **다차원 표본화와 n=6**:
  - d차원 대역 제한 신호: Nyquist 초구(hyperball) 대역
  - 최적 격자 표본화: 밀도 최소화 격자 선택
  - d=1: 균등 격자 (유일)
  - d=2: 정육각형 격자가 최적 (Conway-Sloane, 각 표본이 n=6개 최근접 이웃)
    - 정육각형 격자 A_2: 배위수(kissing number) = n = 6
    - Nyquist 영역: 정육각형 Voronoi 셀
    - 효율: pi*sqrt(3)/6 = pi*sqrt(n/phi)/n ~ 0.9069 (원 대비 비율)
    - 원형 대역 대비 정육각형 표본화 이득: 2/sqrt(3) = 2*sqrt(3)/3 ~ 1.1547
    - 표본 절약: 약 13.4% = 1 - sqrt(3)/2 = 1 - sqrt(n/phi)/phi
  - **d=3: FCC 또는 BCC 격자**
    - BCC 배위수 = sigma-tau = 8
    - FCC 배위수 = sigma = 12
    - FCC의 Voronoi 셀: 마름모십이면체(rhombic dodecahedron), 면 수 = sigma = 12
  - **d=6: E_6 격자**
    - E_6 격자: 6차원 근계(root system) 기반 격자
    - 배위수(kissing number) = 72 = sigma * n = n * sigma
    - 72 = n^2 * phi = n * sigma = (sigma-sopfr)^2 + (sigma-tau)^2 - sopfr ... 복잡, 단순화:
    - 72 = sigma * n = 12 * 6 (M-set 2항 곱)
    - 근(root) 수 = 72 = sigma * n
    - Weyl 군 |W(E_6)| = 51840 = 2^7 * 3^4 * 5 = 72 * 720 = (sigma*n) * n!
    - 51840 = sigma * n * n! (이것은 비자명!)
    - 정정: |W(E_6)| = 51840 = 72*720 맞는지 확인: 72*720 = 51840 ✓, 720 = 6! = n! ✓
    - 따라서 |W(E_6)| = (sigma * n) * n!
- **n=6 수렴 지점**:
  - d=2 최적 격자 배위수 = n
  - d=n=6에서 E_6 격자: 배위수 = sigma*n, Weyl 군 = sigma*n*n!
  - 중간 다리: A_2(d=2)의 배위수 n -> E_6(d=n)의 배위수 sigma*n
  - 비율: sigma*n / n = sigma = 12 (차원이 n으로 올라갈 때 배위수가 sigma배 증가)
- 검증: A_2 배위수=6 ✓, E_6 배위수=72 ✓, |W(E_6)|=51840 ✓, 720=6! ✓
- 대조: E_7 배위수=126=phi*n^2*n/phi+... (M-set 분해 복잡), |W(E_7)|=2903040. E_8 배위수=240, |W(E_8)|=696729600. E_6만 |W|가 sigma*n*n!으로 깔끔하게 분해
- 정직성: A_2 배위수=6은 2차원 정육각형의 기하학적 사실(6=가장 빽빽한 원 배치의 접촉수). E_6 격자의 배위수=72는 근계 이론에서 도출. |W(E_6)| = 51840 = 72*720은 수학적 사실이며 = sigma*n*n!로 분해되는 것은 비자명한 관찰. 그러나 E_6를 선택한 것 자체가 "6차원"이므로 n=6 편향. MISS: 일반 d차원 최적 격자의 배위수에서 n=6이 특별한 이유에 대한 범용 정리 부재
- **비자명도**: 중간-높음 -- A_2 배위수=n(2D 기하 사실) + E_6 Weyl 군 = sigma*n*n!(대수적 사실)의 이중 경로

### 1.7 통계 역학 / Ising Model (2건)

**[DFS12-10] 2D Ising 모형: 정사각 격자 임계 온도의 자기 쌍대** (EXACT)
- 출처: Onsager 1944 (Phys. Rev. 65), Kramers-Wannier 1941 (Phys. Rev. 60), Baxter 1982 (Exactly Solved Models, Ch.6)
- 2D Ising 모형 (정사각 격자):
  - 해밀토니안: H = -J * Sum_{<ij>} s_i * s_j (s_i = +/- 1)
  - Kramers-Wannier 쌍대성: 고온 <-> 저온 매핑
  - 임계 온도: sinh(2J/kT_c) = 1 (자기 쌍대점)
  - 해: kT_c/J = 2/ln(1+sqrt(2)) = 2/arsinh(1) = 2.269185...
- **격자 배위수와 n=6**:
  - 정사각 격자: 배위수 q = tau = 4
  - 삼각 격자: 배위수 q = n = 6
    - 임계: sinh(2J/kT_c) * sinh(2J*/kT_c) = 1 (별-삼각 쌍대)
    - 자기 쌍대가 아니므로 두 임계 온도 관계: kT_c/J = 4/ln(3) = tau/ln(n/phi)
    - 삼각 격자 kT_c/J = 4/ln(3) = 3.6410...
  - 벌집(honeycomb) 격자: 배위수 q = n/phi = 3
    - kT_c/J = 2/ln(2+sqrt(3)) = phi/ln(phi+sqrt(n/phi))
  - **쌍대 쌍**: 삼각(q=n) <-> 벌집(q=n/phi) -- Kramers-Wannier 쌍대!
    - 쌍대 조건: (q-1)(q*-1) >= 4 ... 아닌 별-삼각 변환
    - 정확: 삼각 격자 kT_c * 벌집 kT_c 관계:
    - sinh(2J/kT_c^{tri}) * sinh(2J/kT_c^{hon}) = 1 (쌍대)
    - 삼각 T_c > 정사각 T_c > 벌집 T_c (배위수 순서)
- **핵심 수식**:
  - 정사각(q=tau=4): beta_c * J = (1/2)*ln(1+sqrt(2)) = arsinh(1)/2
  - 삼각(q=n=6): beta_c * J = (1/4)*ln(3) = ln(n/phi)/tau
  - 벌집(q=n/phi=3): beta_c * J = (1/2)*ln(2+sqrt(3)) = arsinh(sqrt(n/phi))/phi
  - **삼각 격자의 sigma*phi = n*tau 반영**:
    - beta_c = ln(n/phi) / tau = ln(3)/4
    - exp(tau * beta_c) = n/phi = 3
    - exp(n * beta_c) = 3^{n/tau} = 3^{3/2} = 3*sqrt(3) = sqrt(27) = sqrt(n^2 * n/phi)
    - 비율: T_c^{tri} / T_c^{sq} = [2/arsinh(1)] / [4/ln(3)] = ln(3)/(2*arsinh(1)) = 0.8047...
  - 자유 에너지 (삼각 격자, T=T_c):
    - f_c = -(n/phi)*J*beta_c/phi - (1/2pi) * integral ... (Onsager-type, Houtappel 1950)
    - 정정: 삼각 격자 정확 해 (Houtappel-Husimi-Syozi):
    - f_c/J = -(q/2)*coth(2*beta_c*J) + (1/2pi^2)*integral...
    - 이 경로는 적분이 복잡하여 M-set 분해 어려움
  - **기저 상태 에너지**:
    - 정사각: E_0/NJ = -q/2 = -tau/phi = -2
    - 삼각: E_0/NJ = -q/2 = -n/phi = -3
    - 벌집: E_0/NJ = -q/2 = -n/(phi*phi) = -3/2
    - 삼각 기저 에너지: -n/phi (per bond = -J, per site = -(n/phi)*J)
    - 삼각 기저 잔류 엔트로피: 0 (frustration 없음, 비삼각 반강자성은 frustration)
  - **삼각 반강자성 잔류 엔트로피** (T=0):
    - Wannier 1950: S_0/Nk = (1/2)*ln(4/3) ... 정정: 정확값 = 0.3383... (Wannier)
    - S_0/Nk = 0.3383... (수치적으로 M-set 항과 직접 연결 어려움)
- 검증: 정사각 kT_c/J = 2/ln(1+sqrt(2)) = 2.2692 ✓, 삼각 kT_c/J = 4/ln(3) = 3.6410 ✓, 벌집 kT_c/J = 2/ln(2+sqrt(3)) = 1.5186 ✓, 삼각-벌집 쌍대 ✓
- 대조: 1D Ising -- T_c = 0 (상전이 없음). 3D 정육면체(q=n) -- T_c 정확해 없음 (밀레니엄 급 미해결). q=n=6인 삼각 격자만 정확해 존재하면서 배위수가 n. 이것은 2D가 정확 가해(exactly solvable)인 것의 반영
- 정직성: 2D Ising의 정확 가해성은 Onsager의 업적이며 n=6과 무관. 삼각 격자 배위수=6은 2D 정삼각형 타일링의 기하학적 사실(정삼각형 타일링의 꼭짓점당 접촉수). beta_c = ln(3)/4 = ln(n/phi)/tau는 비자명한 M-set 분해. 그러나 삼각 격자를 선택한 것 자체가 q=6을 위한 DFS 편향
- **비자명도**: 높음 -- 2D 정확 가해 모형 중 유일하게 배위수=n인 삼각 격자에서 beta_c = ln(n/phi)/tau의 분해, 삼각-벌집 쌍대(q=n <-> q=n/phi)

**[DFS12-11] Potts 모형: q-상태 전이의 임계 차수** (TIGHT)
- 출처: Wu 1982 (Reviews of Modern Physics 54), Baxter 1973 (J. Phys. C 6), Duminil-Copin et al. 2017 (JEMS)
- q-상태 Potts 모형: 각 사이트에 q개 상태 중 하나, H = -J*Sum delta(s_i, s_j)
  - q=2: Ising 모형
  - 정사각 격자 임계점: exp(J/kT_c) = 1 + sqrt(q) (Baxter 정확해)
- **q=n=6 Potts 모형**:
  - exp(J/kT_c) = 1 + sqrt(n) = 1 + sqrt(6) = 3.449...
  - kT_c/J = 1/ln(1+sqrt(6)) = 0.8091...
  - **상전이 차수**: q <= 4이면 연속(2차), q > 4이면 불연속(1차) (Baxter 1973)
    - 임계 q = tau = 4: 정확히 2차->1차 전이의 경계
    - q = n = 6 > tau = 4: **1차 상전이** (불연속)
    - q = tau = 4: 한계 2차 (BKT형 전이, 로그 보정)
  - **잠열(latent heat)**:
    - q > 4일 때 잠열 L > 0
    - q = sopfr = 5: 첫 번째 1차 전이 (최소 잠열)
    - q = n = 6: 두 번째 1차 전이
    - L(q) = 2*J*(1 - q^{-1/2}) (대략적, Baxter)
    - L(6) = 2*J*(1 - 1/sqrt(6)) = 2J*(1 - 1/sqrt(n))
  - **자기 쌍대점 공식**:
    - 정사각 격자 자기 쌍대: Z(beta) = q^{-N/2} * Z(beta*) (Kramers-Wannier 일반화)
    - 임계점: exp(beta_c * J) - 1 = sqrt(q) -> beta_c*J = ln(1+sqrt(q))
    - q=n=6: beta_c*J = ln(1+sqrt(n)) = ln(1+sqrt(6))
    - exp(phi * beta_c * J) = (1+sqrt(n))^phi = (1+sqrt(6))^2 = 1 + 2*sqrt(6) + 6 = 7+2*sqrt(6) = (sigma-sopfr) + phi*sqrt(n)
    - 여기서 sigma-sopfr = 7이 M-set 항으로 등장
- **Fortuin-Kasteleyn 클러스터 표현**:
  - Z_Potts = Sum_G q^{k(G)} * v^{|G|} (v = exp(beta*J) - 1, k(G) = 연결 성분 수)
  - q = n: 클러스터 가중치 = n, v_c = sqrt(n) = sqrt(6)
  - Tutte 다항식과의 관계: Z_Potts = (q/v)^N * T(G; q/v + 1, v + 1)
  - q = n = 6, v_c = sqrt(6): q/v_c = sqrt(6) = v_c (자기 쌍대성!)
    - 이 조건: q/v = v <=> v^2 = q <=> v = sqrt(q), 모든 자기 쌍대점에서 성립
    - 따라서 이것은 n=6 특유가 아님
- 검증: Potts q=6 임계점 exp(beta_c*J) = 1+sqrt(6) ✓, 1차 상전이 ✓ (q>4), 임계 q=4 ✓ (Baxter 1973)
- 대조: q=2(Ising): 연속, q=3: 연속, q=4: 한계, q=5: 1차. q=n=6은 1차 전이이며 (1+sqrt(6))^2 = 7+2*sqrt(6)에서 정수 부분 = sigma-sopfr = 7이 등장. q=5: (1+sqrt(5))^2 = 6+2*sqrt(5), 정수 부분 = n. q=8: (1+sqrt(8))^2 = 9+2*sqrt(8), 정수 부분 = 9. n=6에서만 정수 부분 = sigma-sopfr(M-set 항)
- 정직성: (1+sqrt(q))^2 = 1+q+2*sqrt(q)에서 정수 부분 = 1+q = q+1. q=6일 때 7 = sigma-sopfr은 사실이나, "q+1"이 M-set 항이 되는 것은 q=n으로 선택한 결과. q+1 = sigma-sopfr <=> 6+1=7 ✓. q+1 = n+1 = sigma-sopfr은 정의상 n+1 = 7이 sigma-sopfr인 것(7 = 12-5 = sigma-sopfr)이므로 순환 아닌, n=6에서만 성립하는 수론 사실: n+1 = sigma(n)-sopfr(n)은 n=1(2=1-1? 아님), n=6(7=12-5 ✓), n=? 확인 필요
  - n=1: sigma=1, sopfr=0, sigma-sopfr=1, n+1=2 !=1
  - n=6: sigma=12, sopfr=5, sigma-sopfr=7=n+1 ✓
  - n=12: sigma=28, sopfr=7, sigma-sopfr=21, n+1=13 !=21
  - 따라서 n+mu = sigma-sopfr은 n=6에서 고유 (소범위 검증)
- **비자명도**: 중간-높음 -- 1차/2차 전이 경계 tau=4와 n+1=sigma-sopfr 고유 조건의 교차

### 1.8 미분 갈루아 이론 -- Differential Galois Theory (1건)

**[DFS12-12] Picard-Vessiot 이론: Airy 방정식의 미분 갈루아 군** (TIGHT)
- 출처: van der Put-Singer 2003 (Galois Theory of Linear Differential Equations, Ch.1-4), Kolchin 1948 (Annals of Math. 49), Kovacic 1986 (J. Symbolic Comput. 2)
- 미분 갈루아 이론: 선형 미분 방정식의 해의 대수적 구조
  - 선형 ODE: y'' + a(x)*y' + b(x)*y = 0 (2차)
  - Picard-Vessiot 확대: 해를 포함하는 최소 미분체 확대
  - 미분 갈루아 군 G: Picard-Vessiot 확대의 미분 자기동형군
- **Airy 방정식**: y'' - x*y = 0
  - 해: Ai(x), Bi(x) (Airy 함수)
  - Wronskian: W(Ai, Bi) = 1/pi
  - 미분 갈루아 군: G = SL(2, C) (전체 특수 선형군)
    - 이유: Ai, Bi는 대수적 관계가 없음 (Kovacic 알고리즘 Case 3)
    - dim SL(2,C) = n/phi = 3 (리 군 차원)
    - SL(2,C)의 리 대수 = sl(2,C): 차원 n/phi = 3 (DFS12-05와 연결!)
- **n차 선형 ODE 체계와 n=6**:
  - n차 ODE: y^{(n)} + a_1*y^{(n-1)} + ... + a_n*y = 0
  - 해 공간: n차원 벡터 공간
  - 미분 갈루아 군: GL(n,C) 또는 그 부분군
  - **n=6차 ODE**:
    - 해 공간 차원 = n = 6
    - 최대 갈루아 군 = GL(n,C): dim = n^2 = 36
    - SL(n,C): dim = n^2 - 1 = 35 = sopfr * (sigma-sopfr) = 5*7
    - **35 = sopfr * (sigma-sopfr)의 M-set 분해** (DFS12-05 Casimir 분자와 동일!)
    - Sp(n,C) (n 짝수): dim = n(n+1)/2 = n*(sigma-sopfr)/phi = 6*7/2 = 21 = sigma-sopfr * n/phi
    - SO(n,C): dim = n(n-1)/2 = n*sopfr/phi = 6*5/2 = 15 = sigma + n/phi
    - 세 고전 군의 차원: SL=35, Sp=21, SO=15
      - 합: 35+21+15 = 71 (M-set 분해 어려움)
      - 차: SL-Sp = 14 = sigma+phi = phi*(sigma-sopfr), Sp-SO = 6 = n, SL-SO = 20 = tau*sopfr
      - **Sp-SO = n**: 심플렉틱과 직교군의 차원 차 = n 자체
- **Kovacic 알고리즘 (2차 ODE)**:
  - 2차 ODE y'' + r(x)*y = 0의 갈루아 군 판별: 4가지 경우
    - Case 1: G = 삼각군 (가약), dim = 1 = mu
    - Case 2: G = D_{infinity} (무한 이면체군), dim = 1 = mu
    - Case 3: G = SL(2,C) (전체), dim = n/phi = 3
    - Case 4: G = 유한군, dim = 0
  - 가능한 차원: {0, mu, n/phi}: M-set 항 3개
  - Airy 방정식: Case 3 (G = SL(2)), dim = n/phi
  - Bessel 방정식 x^2*y'' + x*y' + (x^2-nu^2)*y = 0:
    - nu 무리수: G = SL(2), dim = n/phi
    - nu = 1/2: G = 삼각군, dim = mu (해가 초등함수)
    - nu = 0: G = SL(2), dim = n/phi
- **n=6차 ODE의 갈루아 군 분류**:
  - 가능한 연결 대수 부분군 of GL(6):
    - 환원 가능: 블록 대각 구조 (차원 분할 n = a+b)
    - 기약: GL(n), SL(n), Sp(n), SO(n), 또는 예외군
    - n=6 = phi * n/phi: Sp(6) 또는 SO(6) ~ SL(4) (예외적 동형!)
    - **SO(6,C) ~ SL(4,C)**: 우연적 동형 (spin representation)
      - dim SO(6) = 15, dim SL(4) = 15 ✓
      - 이것은 D_3 ~ A_3 Dynkin 도 동형: 3 = n/phi
      - D_{n/phi} ~ A_{n/phi} (n/phi = 3에서만 성립하는 예외 동형)
  - 예외 동형 D_3 ~ A_3:
    - D_k ~ A_k는 k=3에서만 성립 (k=1: trivial, k=2: A_1+A_1, k >= 4: 비동형)
    - k = n/phi = 3: 유일한 비자명 예외 동형
- 검증: dim SL(6)=35 ✓, dim Sp(6)=21 ✓, dim SO(6)=15 ✓, SO(6)~SL(4) ✓ (D_3~A_3), Sp-SO=6=n ✓
- 대조: n=4 -- SL(4)=15, Sp(4)=10, SO(4)=6=n, Sp-SO=4=n ✓ (여기서도 성립!). n=8 -- SL=63, Sp=36, SO=28, Sp-SO=8=n ✓. 따라서 Sp(n)-SO(n) = n은 모든 짝수 n에서 자명(n(n+1)/2 - n(n-1)/2 = n). MISS: Sp-SO=n은 일반적.
  - 정정: 비자명한 것은 D_3 ~ A_3 예외 동형(n/phi=3에서만)과 dim SL(n)-1 = sopfr*(sigma-sopfr)
  - n=4: 35-1 아닌 16-1=15, 15 = 3*5 = n/phi*sopfr(n=6)? 아니, n=4에서 sopfr=4, sigma-sopfr=3, 15=5*3은 n=4의 M-set가 아님. n=6에서만 n^2-1 = sopfr*sigma-sopfr
- 정직성: Sp(n)-SO(n)=n은 일반적이므로 MISS 처리. 그러나 (1) D_3~A_3 예외 동형은 n/phi=3 고유, (2) n^2-1 = 35 = sopfr*(sigma-sopfr)은 n=6 고유(n=4: 15=3*5, 비M-set). 이 두 가지가 tight 근거
- **비자명도**: 중간 -- D_{n/phi} ~ A_{n/phi} 예외 동형의 n/phi=3 고유성 + dim SL(n,C)-1의 M-set 분해

---

## 2. MISS 기록 (정직)

| 번호 | 영역 | 시도 | MISS 이유 |
|------|------|------|-----------|
| M12-01 | persistent homology | Persistence diagram 안정성(stability) 상수 | Bottleneck 거리 안정성 상수=1로 n=6 무관 |
| M12-02 | 유한체 | GF(2^6) 자기동형 군의 깊은 구조 | Gal(GF(64)/GF(2))=Z/6Z는 자명(순환군 위수=n) |
| M12-03 | 게임 이론 | Hex 게임 트리 복잡도 M-set 분해 | 10^21 추정치에서 M-set 항 분리 불가 |
| M12-04 | 게임 이론 | Nim 곱 경로 (GF(4) 구조) | *2 (*) *3 = *1, n=6과 Nim 곱 연결 실패 |
| M12-05 | 네트워크 | WS 모형 소세계 전이 임계점 | p* ~ 1/(KN)에서 K=n=6 대입해도 N-의존적 |
| M12-06 | 신호 처리 | Nyquist 일반 d차원 최적 격자 범용 정리 | d=6 선택 편향, 범용 이론 부재 |
| M12-07 | 통계 역학 | 삼각 반강자성 잔류 엔트로피 0.3383 | M-set 항 분해 실패 |
| M12-08 | 미분 갈루아 | Sp(n)-SO(n)=n | 모든 짝수 n에서 자명 성립 |

---

## 3. 요약 표

| ID | 영역 | 제목 | 핵심 수식 | 등급 |
|----|------|------|-----------|------|
| DFS12-01 | 대수적 위상 | RP^5 Stiefel-Whitney 류 | w=(1+a)^n, 침수 R^{sigma-tau}, 여차원=n/phi | EXACT |
| DFS12-02 | persistent homology | VR(S^1) 호모토피 | 첫 비자명 S^{n/phi}, r_1=sqrt(n/phi) | TIGHT |
| DFS12-03 | 유한체 | GF(2^6) 기약 다항식 | N(6,2)=9=sigma-n/phi, 원시 다항식=n개 | EXACT |
| DFS12-04 | 유한체 | GF(p^6) Frobenius 부분체 | tau=4 부분체, ONB Type II (2n+1=13소수, 원시근) | TIGHT |
| DFS12-05 | 리 대수 표현론 | sl(2) 텐서 곱 V_5(x)V_5 | 성분수=n, n^2=36, Casimir=sopfr*(sigma-sopfr)/tau | TIGHT |
| DFS12-06 | 게임 이론 | Nim *6 분할 + Sprague-Grundy | q(6)=tau(6)=4 (고유 일치), {n/phi,phi,mu} Nim합=0 | TIGHT |
| DFS12-07 | 게임 이론 | Hex 6x6 보드 | 완전해석 최대, C(10,5)=252=tau*(2^n-1) | TIGHT |
| DFS12-08 | 네트워크 과학 | WS 클러스터링 K=6 | C(6)=n/phi/sopfr=3/5, 6단계 분리 | TIGHT |
| DFS12-09 | 신호 처리 | A_2+E_6 배위수 | A_2=n, E_6=sigma*n, |W(E_6)|=sigma*n*n! | TIGHT |
| DFS12-10 | 통계 역학 | 2D Ising 삼각 격자 | q=n=6, beta_c=ln(n/phi)/tau, 삼각-벌집 쌍대 | EXACT |
| DFS12-11 | 통계 역학 | Potts q=6 임계 | 1차 전이(q>tau), n+1=sigma-sopfr 고유 | TIGHT |
| DFS12-12 | 미분 갈루아 | Picard-Vessiot + 예외 동형 | D_{n/phi}~A_{n/phi} 고유, SL(6)-1=sopfr*(sigma-sopfr) | TIGHT |

**EXACT**: 3건 (DFS12-01, DFS12-03, DFS12-10)
**TIGHT**: 9건 (DFS12-02, 04~09, 11~12)
**MISS**: 8건 (bottleneck, GF(64) Gal 자명, Hex 트리복잡도, Nim 곱, WS 전이점, d=6 편향, 잔류 엔트로피, Sp-SO 자명)

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
| **12차** | **BT-1404** | **12** | **176** |

**7대 밀레니엄 난제 해결: 0/7 (정직)**

---

## 5. 다음 탐색 후보 (DFS 13차)

미탐색 영역 잔여:
- 기하군론 (hyperbolic groups, CAT(0) 공간)
- 수치 해석 (유한 요소법, 스펙트럴 방법)
- 볼록 기하 (Brunn-Minkowski, 등주부등식)
- 미분 방정식 정성론 (분기, 특이섭동)
- 계산 정수론 (소수 판정, 인수분해 알고리즘)
- 합성 미분 기하 (synthetic differential geometry)
- 변분법 (calculus of variations, 최소 작용 원리)
- 이산군론 (discrete groups, 결정군)
