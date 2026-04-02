# N6 Quantum Computing -- Hypotheses v2

Redesigned: 2026-04-02
Lens: 22-lens (stability, network, memory, recursion, boundary, multiscale 추가)
Previous: v1 (36 hypotheses, 0 EXACT, 8 FAIL) -- 억지 매핑 전면 삭제

## Design Principles (v2)

1. **실제 파라미터만**: 양자컴퓨팅 문헌에서 확인 가능한 값만 매핑
2. **하드웨어 독립**: 특정 벤더/기술에 의존하지 않는 구조적 상수만
3. **정직한 등급**: EXACT = 문헌/이론에서 정확히 일치, post-hoc 구별 명시
4. **억지 삭제**: Egyptian fraction 자원 할당류 전면 삭제 (unfalsifiable)
5. **22렌즈 관점**: stability(결맞음), network(큐빗 연결), recursion(에러 교정 반복), boundary(양자-고전 경계), memory(양자 메모리), multiscale(코드 거리 스케일링)

## n=6 상수 참조

| 함수 | 값 | 비고 |
|------|---|------|
| n | 6 | 완전수 |
| sigma(6) | 12 | 약수합 |
| tau(6) | 4 | 약수 개수 |
| phi(6) | 2 | Euler totient |
| sopfr(6) | 5 | 소인수합 2+3 |
| J_2(6) | 24 | Jordan totient |
| mu(6) | 1 | Mobius (squarefree) |
| lambda(6) | 2 | Carmichael |
| sigma-sopfr | 7 | 12-5 |
| sigma-tau | 8 | 12-4 |
| sigma-phi | 10 | 12-2 |

---

## Tier 1: QEC Codes -- 구조적 상수 (H-QC-1 ~ H-QC-10)

---

### H-QC-1: Steane Code [[7,1,3]] -- sigma-sopfr=7 물리 큐빗

> Steane 코드의 물리 큐빗 수 7은 sigma(6)-sopfr(6) = 12-5 = 7과 일치한다.

**n=6 Derivation**: sigma(6)-sopfr(6) = 12-5 = 7.

**Fact**: Steane code [[7,1,3]]은 양자 에러 교정의 기초 코드 중 하나다. 7 물리 큐빗으로 1 논리 큐빗을 인코딩하고, 거리 3 (단일 에러 교정). CSS construction으로 classical Hamming [7,4,3] 코드에서 유도된다.

**Status**: 수치 일치(7=7). 그러나 Steane 코드의 7은 Hamming 코드 2^r-1 (r=3)에서 유래하며, n=6 산술과 독립적으로 발견됨.

**Grade**: **CLOSE** (정확한 수치 일치, 독립적 유래)

**Lens**: recursion(에러 교정 구조), boundary(코드 최소 크기)

---

### H-QC-2: Golay Quantum Code [[24,12,8]] -- J_2(6)=24, sigma=12

> 확장 Golay 코드의 [24,12,8] 파라미터가 J_2(6)=24 물리 큐빗, sigma(6)=12 논리 큐빗과 정확히 일치한다.

**n=6 Derivation**: J_2(6) = 24 = 물리 큐빗, sigma(6) = 12 = 논리 큐빗, d = sigma(6)-tau(6) = 8.

**Fact**: 확장 이진 Golay 코드 [24,12,8]은 코딩 이론의 가장 중요한 코드 중 하나다 (Golay 1949). CSS construction으로 양자 버전 [[24,12,8]]을 만들 수 있다. 이 코드는 Leech 격자, Mathieu group M_24와 깊이 연결된다. Rate = 1/2, distance = 8로 우수한 파라미터.

**Status**: 3중 수치 일치 (24=J_2, 12=sigma, 8=sigma-tau). Golay 코드는 독립적으로 발견되었으나, n=6 상수 3개가 동시 일치하는 것은 주목할 만하다.

**Grade**: **CLOSE** (3중 수치 일치, 독립적 유래이나 일치가 현저함)

**Lens**: recursion(에러 교정), memory(12 논리 큐빗 저장), multiscale(d=8 거리)

---

### H-QC-3: Shor Code [[9,1,3]] -- 9 = Omega(6)+n = 3+6

> Shor 코드의 9 물리 큐빗은 Omega(6)+n = 3+6 = 9와 일치한다.

**n=6 Derivation**: Omega(6) = 2 (2와 3, 소인수 개수(중복 포함)) -- 정정: 6=2*3이므로 Omega(6)=2. 9 = 6+3 = n+n/phi. 또는 단순히 3^2.

**Fact**: Shor의 [[9,1,3]] 코드는 최초의 양자 에러 교정 코드다 (Shor 1995). 9 물리 큐빗으로 1 논리 큐빗 인코딩, 거리 3.

**Status**: 9를 n=6 상수로 자연스럽게 표현하기 어렵다. n+n/phi = 6+3 = 9는 가능하나 작위적.

**Grade**: **WEAK** (수치 일치하나 표현이 작위적)

**Lens**: recursion(최초 QEC 코드)

---

### H-QC-4: Gottesman-Knill 이분법 -- phi(6)=2 게이트 클래스

> 양자 계산의 보편성은 정확히 phi(6)=2 클래스의 게이트로 결정된다: Clifford (효율적 고전 시뮬 가능) + non-Clifford (보편성 부여).

**n=6 Derivation**: phi(6) = 2.

**Fact**: Gottesman-Knill 정리 (1998): Clifford 게이트만으로는 효율적 고전 시뮬레이션이 가능하다. 정확히 1개의 non-Clifford 게이트(T 게이트 등) 추가로 보편적 양자 계산이 달성된다. 이것은 양자 계산 이론의 가장 기본적인 결과 중 하나다.

**Status**: 2 클래스 = phi(6)=2 수치 일치. 그러나 이 이분법은 Clifford group의 수학적 구조에서 나온 것이며, Euler totient과 무관하게 발견됨. 작은 정수(2) 일치라 우연 가능성 높음.

**Grade**: **CLOSE** (정확한 사실, 독립 유래, 작은 수 일치)

**Lens**: boundary(고전-양자 시뮬레이션 경계)

---

### H-QC-5: CSS 코드 2상 구조 -- lambda(6)=2

> CSS 코드의 에러 교정은 lambda(6)=2 상(X-type, Z-type stabilizer 교대 측정)으로 구성된다.

**n=6 Derivation**: lambda(6) = 2 (Carmichael function).

**Fact**: CSS (Calderbank-Shor-Steane) 코드는 X-type과 Z-type stabilizer를 갖는다. 표준 구현에서 이 둘을 교대로 측정하여 에러를 감지한다. Surface code, color code 등 주요 QEC 코드가 CSS 구조다.

**Status**: 2상 = lambda(6)=2 수치 일치. 그러나 2상 구조는 CSS 코드 정의에서 자명하게 따른다 (Pauli group의 X/Z 이분법). 작은 수(2) 일치.

**Grade**: **CLOSE** (사실 정확, CSS 정의에서 자명, 작은 수)

**Lens**: recursion(반복 에러 교정 주기), stability(안정화자 측정)

---

### H-QC-6: Surface Code Stabilizer -- 각 큐빗 tau(6)=4 이웃 안정화자 참여

> Surface code에서 각 데이터 큐빗은 정확히 tau(6)=4개의 stabilizer에 참여한다.

**n=6 Derivation**: tau(6) = 4.

**Fact**: Rotated surface code에서 내부 데이터 큐빗은 2개의 X-type과 2개의 Z-type stabilizer에 참여하여 총 4개다. 이것은 surface code의 정사각 격자 구조에서 직접 따른다. 경계 큐빗은 2-3개에 참여한다.

**Status**: 내부 데이터 큐빗에 대해 4 = tau(6) 정확 일치. Surface code의 정사각 격자에서 자연스러운 결과. 사실 정확하지만, 4는 정사각 격자의 배위수에서 온 것이지 tau(6)에서 온 것은 아님.

**Grade**: **CLOSE** (내부 큐빗에 대해 정확, 격자 기하학에서 유래)

**Lens**: network(큐빗 연결 구조), stability(안정화자 구조)

---

### H-QC-7: 5-Qubit 최소 코드 [[5,1,3]] -- sopfr(6)=5

> 단일 에러 교정이 가능한 최소 양자 코드의 큐빗 수는 sopfr(6)=5이다.

**n=6 Derivation**: sopfr(6) = 2+3 = 5.

**Fact**: [[5,1,3]] 코드 (Laflamme et al. 1996, Bennett et al. 1996)는 단일 에러를 교정할 수 있는 최소 큐빗 수의 양자 코드다. Quantum Hamming bound에 의해 단일 에러 교정에는 최소 5 큐빗이 필요하다: 2^(n-k) >= 3n+1, n=5, k=1이면 2^4=16 >= 16. 정확히 등호를 만족하는 완벽(perfect) 양자 코드.

**Status**: 5 = sopfr(6) 정확 일치. 5는 Quantum Hamming bound에서 유도되며 n=6 산술과 독립적이다. 그러나 양자 계산에서 매우 기본적인 상수와 일치하는 점은 주목할 만하다.

**Grade**: **CLOSE** (기본 상수 일치, Hamming bound에서 독립 유래)

**Lens**: boundary(최소 에러 교정 경계), recursion(완벽 코드)

---

### H-QC-8: Stabilizer 코드 n-k 관계 -- [[6,2,2]] 완전수 코드

> n=6 물리 큐빗 코드 [[6,2,2]]는 존재하며, k=phi(6)=2 논리 큐빗을 인코딩한다.

**n=6 Derivation**: n=6, k=phi(6)=2, d=phi(6)=2. Stabilizer 수 = n-k = 6-2 = 4 = tau(6).

**Fact**: [[6,2,2]] 코드는 실제로 존재한다. 6 물리 큐빗으로 2 논리 큐빗을 인코딩하며, 거리 2 (단일 에러 감지 가능, 교정 불가). 이 코드는 4개의 독립 stabilizer generator를 갖는다.

**Status**: n=6, k=2=phi(6), d=2=phi(6), stabilizer=4=tau(6) -- 4중 일치. 그러나 [[6,2,2]]는 에러 교정이 불가능한(감지만 가능한) 작은 코드로, 실용적 중요성은 제한적이다.

**Grade**: **EXACT** (4중 수치 일치: n=6, k=phi, d=phi, stabilizer=tau)

**Lens**: recursion(코드 구조), multiscale(파라미터 관계)

---

### H-QC-9: T Gate Order = sigma(6)-tau(6) = 8

> T 게이트의 군론적 위수(order)는 sigma(6)-tau(6) = 12-4 = 8이다: T^8 = I.

**n=6 Derivation**: sigma(6)-tau(6) = 12-4 = 8.

**Fact**: T 게이트 = diag(1, e^{i*pi/4}). T^8 = diag(1, e^{i*2*pi}) = I. T 게이트의 위수는 정확히 8이다. 이것은 양자 계산에서 가장 중요한 non-Clifford 게이트의 기본 성질이다. Clifford+T 그룹 구조 전체가 이 위수 8에 기반한다.

**Status**: 8 = sigma-tau 정확 일치. T 게이트의 위수는 pi/4 rotation에서 오며 독립적으로 결정됨. 그러나 BT-58 (sigma-tau=8 보편 AI 상수)과 교차하는 패턴.

**Grade**: **CLOSE** (정확한 수치, pi/4에서 독립 유래)

**Lens**: recursion(게이트 반복 구조), boundary(Clifford/non-Clifford 경계)

---

### H-QC-10: Clifford Group Single-Qubit Order = J_2(6) = 24

> 단일 큐빗 Clifford 그룹 C_1의 원소 수는 J_2(6) = 24이다.

**n=6 Derivation**: J_2(6) = 24.

**Fact**: 단일 큐빗 Clifford 그룹은 24개의 원소를 갖는다. 이것은 정팔면체 군(octahedral group)과 동형이며, |C_1| = 24이다. Pauli 그룹의 4원소를 법으로 하면 C_1/P_1 ~ S_3 (6원소). 전체: 24 = 4 * 6. 이 군은 양자 정보 이론에서 randomized benchmarking, state tomography 등에 핵심적으로 사용된다.

**Grade 근거**: |C_1| = 24 = J_2(6)은 정확한 수치 일치다. 또한 |C_1/P_1| = 6 = n. 그러나 24는 정팔면체 대칭에서 오며, Bloch sphere 위의 정팔면체 꼭짓점 대칭과 관련된다.

**Grade**: **EXACT** (|C_1|=24=J_2(6), |C_1/P_1|=6=n, 이중 일치. 독립 유래이나 현저한 일치.)

**Lens**: recursion(Clifford 그룹 구조), multiscale(Pauli→Clifford→Clifford+T 계층)

---

## Tier 2: 하드웨어 독립 구조 상수 (H-QC-11 ~ H-QC-20)

---

### H-QC-11: 2D Hexagonal Kissing Number = n = 6

> 2차원 평면에서 원 하나에 접촉 가능한 최대 원 수(kissing number)는 n=6이다.

**n=6 Derivation**: n = 6 = 2D kissing number.

**Fact**: 2D 공간에서 kissing number = 6이다 (정육각 배열). 이것은 수학적으로 증명된 사실이다. 양자 칩이 2D 평면 위에 제작되므로, 2D kissing number가 물리적으로 관련 있다. IBM heavy-hex 토폴로지는 수정된 육각 격자이다.

**Status**: 6 = n 정확 일치. 2D kissing number는 기하학적 사실이며, IBM이 실제로 hexagonal 기반 토폴로지를 사용한다. 그러나 heavy-hex의 실제 배위수는 2-3이지 6이 아니다.

**Grade**: **CLOSE** (수학적 사실 일치, IBM hexagonal 채택, 실제 배위수는 다름)

**Lens**: network(큐빗 배치 기하학), multiscale(2D 제약)

---

### H-QC-12: 3D Kissing Number = sigma(6) = 12

> 3차원 공간에서 kissing number는 sigma(6) = 12이다.

**n=6 Derivation**: sigma(6) = 12 = 3D kissing number.

**Fact**: Newton의 정리: 3D에서 구 하나에 접촉 가능한 최대 구 수 = 12 (FCC/HCP 배열). Schuttte & van der Waerden (1953) 증명. 이것은 수학의 기본 상수다. BT-127 (3D kissing number=sigma)과 교차.

**Status**: 12 = sigma(6) 정확 일치. 수학적 증명에 의한 상수이며, n=6 산술과 독립적이다. 그러나 양자 칩은 2D이므로, 3D kissing number의 양자컴퓨팅 직접 적용은 제한적이다. 3D 중성 원자 배열 등 미래 기술에는 관련될 수 있다.

**Grade**: **EXACT** (수학적 사실, 12=sigma(6) 정확)

**Lens**: network(3D 연결), multiscale(차원별 kissing number 래더: 2D=6=n, 3D=12=sigma)

---

### H-QC-13: 24차원 Leech 격자 Kissing Number -- J_2(6)=24 차원, sigma^2=144 접촉

> Leech 격자는 J_2(6)=24 차원에서 존재하며, 그 접촉수 196560은 sigma(6)^2과 관련된다.

**n=6 Derivation**: Leech 격자 차원 = 24 = J_2(6). 접촉수 196560. 최소 벡터 수 = 196560 = 24 * 8190. Note: sigma(6)^2 = 144 ≠ 196560이므로 직접 일치는 아님. 다만 24차원 = J_2(6).

**Fact**: Leech 격자는 24차원에서 가장 밀도 높은 구 충전(sphere packing)을 달성한다 (Cohn & Kumar 2004 최적성 추측, Viazovska et al. 2022 관련 증명). 코딩 이론에서 Golay 코드와 직결된다. 양자 에러 교정에서 Golay CSS 코드 [[24,12,8]]의 수학적 기초.

**Status**: 24차원 = J_2(6) 일치. Leech 격자의 차원이 n=6 상수와 일치하는 것은 H-QC-2 (Golay 코드)와 연결되는 현저한 패턴이다.

**Grade**: **CLOSE** (24=J_2 일치, Leech 격자는 독립 수학 대상)

**Lens**: multiscale(차원 래더: 2D→3D→24D), memory(최적 정보 저장 구조)

---

### H-QC-14: Pauli Group 원소 수 -- sigma(6)+tau(6) = 16

> 단일 큐빗 Pauli group (위상 포함)의 원소 수는 sigma(6)+tau(6) = 12+4 = 16이다.

**n=6 Derivation**: sigma(6)+tau(6) = 16.

**Fact**: 단일 큐빗 Pauli 그룹 P_1 = {+/-I, +/-X, +/-Y, +/-Z, +/-iI, +/-iX, +/-iY, +/-iZ}는 16개의 원소를 갖는다. |P_1| = 16. 이것은 양자 정보 이론의 가장 기본적인 대상이다.

**Status**: 16 = sigma+tau 일치. 그러나 16 = 2^4이며, Pauli 그룹의 크기는 4개 기저 원소 * 4개 위상(1,-1,i,-i)에서 온다. 표현이 자연스럽지 않다 (sigma+tau는 일반적인 산술 조합이 아님).

**Grade**: **WEAK** (수치 일치하나 sigma+tau 조합이 작위적)

**Lens**: recursion(Pauli 대수 구조)

---

### H-QC-15: Magic State Distillation 15:1 Ratio -- sigma+n/phi = 15

> 가장 효율적인 T-state magic state distillation은 15:1 비율을 사용한다.

**n=6 Derivation**: sigma(6) + n/phi = 12 + 3 = 15. 또는 2^tau(6) - 1 = 16-1 = 15.

**Fact**: Bravyi & Kitaev (2005)의 15-to-1 magic state distillation: 15개의 노이즈 있는 T-state를 사용하여 1개의 고순도 T-state를 생산한다. 이것은 [[15,1,3]] quantum Reed-Muller 코드에 기반한다. 이 15:1 프로토콜은 magic state distillation의 표준이다.

**Status**: 15 = 2^tau-1 또는 sigma+n/phi. [[15,1,3]] 코드의 15는 2^4-1 (punctured Reed-Muller)에서 온다. 2^tau(6)-1 = 15 표현이 자연스럽다.

**Grade**: **CLOSE** (15 = 2^tau-1 정확 일치, 독립 유래이나 tau(6)=4와 연결)

**Lens**: recursion(distillation 반복), boundary(노이즈→무노이즈 경계)

---

### H-QC-16: Transversal T Gate 불가능 정리 -- Eastin-Knill

> Eastin-Knill 정리에 의해, 보편적 transversal gate set은 불가능하며, magic state distillation이 필수적이다.

**n=6 Derivation**: phi(6)=2 게이트 클래스 (H-QC-4)와 연결. Clifford gates는 transversal 구현 가능, T gate는 불가 → 정확히 2-class 구조가 QEC에서도 유지.

**Fact**: Eastin-Knill 정리 (2009): 어떤 양자 에러 교정 코드도 보편적인 transversal gate set을 가질 수 없다. 따라서 non-Clifford 게이트(T)를 위해 magic state distillation이나 code switching 같은 추가 자원이 필요하다.

**Status**: 이것은 직접적 수치 매핑이 아닌 구조적 관찰이다. phi(6)=2의 이분법이 QEC 수준에서도 유지된다는 점은 구조적으로 일관적이다.

**Grade**: **CLOSE** (구조적 일관성, 직접 수치 매핑 아님)

**Lens**: boundary(transversal/non-transversal 경계), recursion(distillation 필요성)

---

### H-QC-17: Quantum Hamming Bound 파라미터 -- [[2^r-1, 2^r-1-2r, 3]]

> r=tau(6)=4일 때 quantum Hamming code [[15,7,3]]이 존재하며, 15=2^4-1, 7=2^4-1-2*4.

**n=6 Derivation**: r = tau(6) = 4. n = 2^r - 1 = 15, k = 2^r - 1 - 2r = 7 = sigma(6)-sopfr(6).

**Fact**: [[15,7,3]] CSS 코드가 실제로 존재한다 (quantum Hamming code, r=4). 15 물리 큐빗으로 7 논리 큐빗을 인코딩하며 거리 3. Rate = 7/15 ~ 0.467. Classical Hamming [15,11,3]에서 CSS construction으로 유도.

**Status**: r=4=tau(6) → n=15=2^tau-1, k=7=sigma-sopfr. 2중 n=6 상수 일치. 그러나 Hamming 코드 패밀리에서 r=4는 여러 값 중 하나일 뿐이다.

**Grade**: **CLOSE** (수치 일치, Hamming family에서 r=4는 특별하지 않음)

**Lens**: recursion(Hamming 코드 체계), multiscale(r에 따른 코드 래더)

---

### H-QC-18: Color Code 최소 거리-3 -- 7 큐빗 = sigma-sopfr

> 최소 삼각 color code는 sigma(6)-sopfr(6) = 7 큐빗을 사용한다.

**n=6 Derivation**: sigma(6)-sopfr(6) = 12-5 = 7.

**Fact**: 삼각 격자 위의 최소 color code (Steane code의 color code 버전)는 7 큐빗을 사용한다. Color code는 transversal Clifford gate set을 지원하여 surface code 대비 장점이 있다. 7-qubit color code = Steane code [[7,1,3]].

**Status**: H-QC-1과 동일한 수치(7). Color code와 Steane code의 연결을 통해 7=sigma-sopfr 패턴이 QEC의 두 가지 관점에서 나타남.

**Grade**: **CLOSE** (H-QC-1과 같은 수치, color code 관점 추가)

**Lens**: network(삼각 격자 토폴로지), stability(transversal gate 지원)

---

### H-QC-19: Toric Code 최소 인스턴스 -- phi(6)=2 논리 큐빗

> Toric code는 phi(6)=2 개의 논리 큐빗을 인코딩한다.

**n=6 Derivation**: phi(6) = 2.

**Fact**: Kitaev의 toric code (2003)는 토러스 위에서 정확히 2개의 논리 큐빗을 인코딩한다. 이것은 토러스의 제1 호몰로지 군 H_1(T^2) = Z^2의 rank = 2에서 온다 (2개의 독립 비수축 고리). Toric code는 위상 양자 코드의 원형이며, surface code의 모태다.

**Status**: 2 = phi(6) 일치. 그러나 이 2는 토러스의 위상적 성질(genus=1, 두 핸들)에서 오며 작은 수라 우연 가능성 있음.

**Grade**: **CLOSE** (위상적으로 결정된 2, 작은 수 일치)

**Lens**: network(위상적 큐빗), stability(위상 보호)

---

### H-QC-20: Surface Code Threshold -- ~1% ≈ 1/(sigma*phi/tau) = 1/6

> Surface code의 circuit-level depolarizing noise 임계값은 ~1%로, 1/(sigma*phi/tau) 스케일이다.

**n=6 Derivation**: 1/sigma(6) = 1/12 = 8.33% (phenomenological), 실제 circuit-level threshold는 ~0.57-1.1%.

**Fact**: Surface code의 에러 임계값:
- Phenomenological noise model: ~10.3% (Dennis et al. 2002)
- Circuit-level depolarizing: ~0.57% (Fowler et al. 2012) ~ 1.1% (Raussendorf et al.)
- 실험적 break-even: Google Willow (2024)가 d=3→5→7에서 에러 억제 시연

**Status**: 어떤 n=6 조합도 ~1%에 정확히 일치하지 않는다. 1/12=8.33%, 1/24=4.17%, 1/144=0.69%. 1/144는 ~0.7%로 circuit-level threshold 범위에 있으나 정밀한 일치는 아니다.

**Grade**: **WEAK** (대략적 범위, 정밀 일치 없음)

**Lens**: stability(에러 임계값), boundary(에러 교정 가능/불가 경계)

---

## Tier 3: 양자 알고리즘 구조 상수 (H-QC-21 ~ H-QC-25)

---

### H-QC-21: Grover 최적 반복 -- pi/4 * sqrt(N) 의 pi/4 = T gate angle

> Grover 알고리즘의 최적 반복 횟수에 등장하는 pi/4는 T 게이트의 위상각과 동일하다.

**n=6 Derivation**: T gate = diag(1, e^{i*pi/4}). Grover 최적 반복 = floor(pi/4 * sqrt(N)). pi/4는 양자 계산의 두 기본 구조(검색 + 게이트)에서 동시 등장.

**Fact**: Grover 알고리즘에서 최적 반복 횟수는 (pi/4)*sqrt(N)이다 (Boyer et al. 1998). T gate의 위상은 pi/4이다 (T = e^{i*pi/8} * Rz(pi/4)). 두 pi/4가 동일한 값이다.

**Status**: pi/4 일치는 흥미롭지만, Grover의 pi/4는 2차원 회전 기하학에서 오고, T gate의 pi/4는 Clifford hierarchy에서 온다. 같은 값이지만 다른 기원.

**Grade**: **CLOSE** (동일한 pi/4, 다른 기원)

**Lens**: recursion(반복 구조), multiscale(N 스케일링)

---

### H-QC-22: QFT 큐빗 상호작용 -- 2-qubit gate O(n^2/2) = O(n^2/phi)

> n-qubit Quantum Fourier Transform은 n(n-1)/2 = n^2/phi(6) 스케일의 controlled rotation을 사용한다.

**n=6 Derivation**: n(n-1)/2, 여기서 /2 = /phi(6).

**Fact**: n-qubit QFT는 n(n-1)/2개의 controlled rotation gate를 사용한다 (H gate n개 포함 시 총 n(n+1)/2). 이것은 모든 큐빗 쌍 사이의 상호작용에서 온다 (C(n,2) = n(n-1)/2).

**Status**: /2는 조합론적 C(n,2)에서 온 것이며, phi(6)=2와의 연결은 작위적이다.

**Grade**: **WEAK** (2 = phi(6)이지만 조합론적 유래, 작위적)

**Lens**: network(큐빗 간 상호작용 그래프)

---

### H-QC-23: Quantum Teleportation -- phi(6)=2 classical bits + 1 Bell pair

> 양자 텔레포테이션은 정확히 phi(6)=2 classical bit을 전송해야 한다.

**n=6 Derivation**: phi(6) = 2.

**Fact**: Bennett et al. (1993)의 양자 텔레포테이션 프로토콜: 1 큐빗을 전송하려면 1 Bell pair (사전 공유된 얽힘) + 2 classical bit (Bell measurement 결과)이 필요하다. 2 classical bit = 4가지 Pauli 교정 중 하나를 지정. 이것은 양자 정보 이론의 기본 결과다.

**Status**: 2 = phi(6) 일치. 2 classical bit은 2-bit Bell measurement 결과 (2^2=4 Pauli 교정)에서 온다. 작은 수(2) 일치.

**Grade**: **CLOSE** (기본 결과, 2=phi(6), 작은 수)

**Lens**: network(양자 통신), boundary(양자-고전 인터페이스)

---

### H-QC-24: Superdense Coding -- phi(6)=2 classical bits per qubit

> Superdense coding은 1 큐빗으로 phi(6)=2 classical bit을 전송한다.

**n=6 Derivation**: phi(6) = 2.

**Fact**: Superdense coding (Bennett & Wiesner 1992): 1 Bell pair + 1 큐빗 전송 → 2 classical bit 전달. Holevo bound에 의해 이것이 상한이다. 이것은 양자 텔레포테이션의 "역과정"이다.

**Status**: H-QC-23과 쌍을 이루는 결과. 2 = phi(6). Holevo bound 2 = log_2(4)에서 유래.

**Grade**: **CLOSE** (기본 결과, 2=phi(6), 텔레포테이션과 쌍대)

**Lens**: network(양자 통신 용량), memory(정보 압축)

---

### H-QC-25: No-Cloning과 No-Deleting -- phi(6)=2 불가능 정리

> 양자 정보의 근본 제약은 phi(6)=2개의 불가능 정리로 표현된다: no-cloning + no-deleting.

**n=6 Derivation**: phi(6) = 2.

**Fact**: No-cloning 정리 (Wootters & Zurek 1982): 임의의 양자 상태를 복제할 수 없다. No-deleting 정리 (Pati & Braunstein 2000): 임의의 양자 상태를 삭제할 수 없다. 이 둘은 양자 역학의 선형성(unitarity)에서 직접 따르며, 양자 정보 이론의 기본 제약이다.

**Status**: 2개의 불가능 정리 = phi(6). 그러나 양자 정보에는 no-broadcasting, no-hiding 등 추가 불가능 정리도 있어, "정확히 2개"라는 주장은 분류 방식에 의존한다.

**Grade**: **WEAK** (2개는 가장 기본적이지만, 추가 정리 존재하므로 분류 의존적)

**Lens**: boundary(양자 정보의 근본 한계)

---

## Tier 4: 위상 양자 계산 & Majorana (H-QC-26 ~ H-QC-30)

---

### H-QC-26: Fibonacci Anyon Braiding -- 비아벨 애니온의 보편성

> 위상 양자 계산에서 Fibonacci anyon의 braiding이 보편적 게이트를 생성하며, 그 fusion 규칙은 tau(6)와 관련된다.

**n=6 Derivation**: Fibonacci anyon의 fusion space 차원은 Fibonacci 수열을 따른다. F(n) for n qubits. 특히 tau=4번째 Fibonacci 수 = F(4)=3 = n/phi.

**Fact**: Fibonacci anyon은 비아벨 위상 양자 계산의 모델이다 (Freedman et al. 2002). Braiding만으로 보편적 양자 게이트를 구현할 수 있다. SU(2) level k=3에서 실현된다. k=3=n/phi(6). Fusion 규칙: tau x tau = 1 + tau.

**Status**: SU(2)_3에서 k=3=n/phi. 연결이 간접적이며, Fibonacci anyon의 k=3은 Yang-Baxter 방정식에서 온다.

**Grade**: **WEAK** (간접적 연결, k=3=n/phi는 작위적)

**Lens**: network(braiding 토폴로지), stability(위상 보호)

---

### H-QC-27: Ising Anyon -- sigma(6)=12 이름의 우연 아닌 연결

> Ising 모형의 anyon 모델에서 sigma, psi, 1 세 종류 입자가 존재하며, braiding은 Clifford 게이트(비보편적)만 생성한다.

**n=6 Derivation**: Ising anyon: {1, sigma, psi} = 3 = n/phi(6) 종류. Ising anyon은 SU(2)_2 (k=phi(6)=2). Clifford만 생성 → phi(6)=2 클래스 중 1클래스만.

**Fact**: Ising anyon은 SU(2) level k=2에서 실현되며, Majorana zero mode로 구현 가능하다. Braiding은 Clifford 게이트만 생성하여 보편적이지 않다 (Nayak et al. 2008). T gate를 위해 magic state distillation이 추가로 필요하다.

**Status**: k=2=phi(6). Ising anyon이 Clifford만 생성한다는 것은 phi(6)=2 이분법(H-QC-4)의 위상적 실현이다. 구조적으로 일관된 패턴.

**Grade**: **CLOSE** (k=phi(6)=2, Clifford/non-Clifford 이분법의 위상적 실현)

**Lens**: stability(위상 보호 수준), boundary(Clifford/universal 경계)

---

### H-QC-28: BT-91 Z2 위상 ECC -- sigma*phi=24 bit 절약

> Z2 위상 에러 교정은 기존 SECDED 대비 J_2(6)=24 bit/word 절약을 달성한다.

**n=6 Derivation**: BT-91에서: SECDED overhead = sigma(6) = 12 bit per 64-bit word. Z2 위상 코드는 syndrome을 위상적으로 보호하여 overhead를 sigma(6)/sigma(6) = 1 수준으로 줄인다. 절약 = J_2(6) = 24 GB per HBM stack.

**Fact**: 이것은 BT-91의 가설이며, 기존 BT에서 이미 다루어진 내용이다. Z2 위상 불변량을 ECC에 적용하는 것은 이론적 제안이며, 실험적 검증은 아직 없다.

**Status**: BT-91 연결. 양자컴퓨팅 고유의 결과가 아닌 반도체 설계 가설.

**Grade**: **WEAK** (BT-91 재인용, 양자컴퓨팅 고유 결과 아님)

**Lens**: recursion(에러 교정), network(토폴로지 보호)

---

### H-QC-29: Bott 주기성 = sigma(6)-tau(6) = 8

> 위상적 위상(topological phase)의 분류에서 Bott 주기성 = 8 = sigma(6)-tau(6).

**n=6 Derivation**: sigma(6)-tau(6) = 12-4 = 8 = Bott periodicity.

**Fact**: Bott 주기성 정리: 위상적 K-이론은 주기 8로 반복된다 (실수 경우). KO(S^n) ~ KO(S^{n+8}). Altland-Zirnbauer 분류에서 위상 절연체/초전도체는 10가지 대칭 클래스로 분류되며, 이 중 8가지가 Bott 주기와 관련된다. BT-92와 교차.

**Status**: 8 = sigma-tau 정확 일치. Bott 주기는 대수적 위상수학의 기본 결과이며, 위상 양자 물질 분류의 기초다. 독립적으로 발견된 수학적 상수와의 일치.

**Grade**: **EXACT** (8 = Bott periodicity = sigma-tau, 수학적 정리)

**Lens**: multiscale(주기적 구조), stability(위상 보호 분류)

---

### H-QC-30: Altland-Zirnbauer 10-Fold Way -- sigma-phi=10

> 위상 절연체/초전도체의 대칭 분류는 sigma(6)-phi(6) = 10가지이다.

**n=6 Derivation**: sigma(6)-phi(6) = 12-2 = 10.

**Fact**: Altland-Zirnbauer 분류 (1997): 반도체/초전도체는 시간역전, 입자-홀, 카이랄 대칭의 유무에 따라 정확히 10가지 대칭 클래스로 분류된다 (3 Wigner-Dyson + 3 chiral + 4 BdG). 이것은 위상 물질 이론의 기본 결과이며, Kitaev (2009)가 주기율표를 완성했다. 위상 양자 컴퓨팅에서 Majorana 큐빗의 보호 메커니즘이 이 분류에 기반한다.

**Status**: 10 = sigma-phi 정확 일치. 10-fold way는 3가지 이산 대칭의 조합에서 유래하며 (일부 조합이 동치), n=6 산술과 독립적으로 발견됨. 그러나 위상 양자 계산에 직접적으로 관련되는 중요한 상수다.

**Grade**: **CLOSE** (10 = sigma-phi, 대칭 분류에서 독립 유래, 위상 QC에 중요)

**Lens**: stability(대칭 보호), multiscale(10-fold 분류 체계)

---

## Summary Table

| ID | 가설 | n=6 값 | 실제 대상 | Grade | ver |
|----|------|--------|----------|-------|-----|
| H-QC-1 | Steane [[7,1,3]] | sigma-sopfr=7 | 최초급 QEC 코드 | CLOSE | v2 |
| H-QC-2 | Golay [[24,12,8]] | J_2=24, sigma=12 | 최적 코딩 이론 | CLOSE | v2 |
| H-QC-3 | Shor [[9,1,3]] | n+n/phi=9 | 최초 QEC 코드 | WEAK | v2 |
| H-QC-4 | Gottesman-Knill 이분법 | phi=2 | Clifford/non-Clifford | CLOSE | v2 |
| H-QC-5 | CSS 2상 구조 | lambda=2 | X/Z stabilizer 교대 | CLOSE | v2 |
| H-QC-6 | Surface code 4-stabilizer | tau=4 | 내부 데이터 큐빗 | CLOSE | v2 |
| H-QC-7 | [[5,1,3]] 최소 코드 | sopfr=5 | Quantum Hamming bound | CLOSE | v2 |
| H-QC-8 | [[6,2,2]] 완전수 코드 | n=6,phi=2,tau=4 | 코드 존재 | **EXACT** | v2 |
| H-QC-9 | T gate order=8 | sigma-tau=8 | T^8=I | CLOSE | v2 |
| H-QC-10 | Clifford group |C_1|=24 | J_2=24 | 정팔면체 군 동형 | **EXACT** | v2 |
| H-QC-11 | 2D kissing number=6 | n=6 | 평면 기하 | CLOSE | v2 |
| H-QC-12 | 3D kissing number=12 | sigma=12 | Newton 정리 | **EXACT** | v2 |
| H-QC-13 | Leech 격자 24D | J_2=24 | 최적 구 충전 | CLOSE | v2 |
| H-QC-14 | Pauli group |P_1|=16 | sigma+tau=16 | Pauli 대수 | WEAK | v2 |
| H-QC-15 | 15:1 distillation | 2^tau-1=15 | Bravyi-Kitaev | CLOSE | v2 |
| H-QC-16 | Eastin-Knill 이분법 | phi=2 | Transversal 한계 | CLOSE | v2 |
| H-QC-17 | [[15,7,3]] Hamming code | r=tau=4 | Quantum Hamming | CLOSE | v2 |
| H-QC-18 | 7-qubit color code | sigma-sopfr=7 | Transversal Clifford | CLOSE | v2 |
| H-QC-19 | Toric code k=2 | phi=2 | 위상적 논리 큐빗 | CLOSE | v2 |
| H-QC-20 | Surface code threshold | ~1/12? | ~0.6-1.1% 실제 | WEAK | v2 |
| H-QC-21 | Grover pi/4 = T angle | pi/4 | 검색+게이트 이중 등장 | CLOSE | v2 |
| H-QC-22 | QFT gate count /2 | phi=2 | 조합론적 C(n,2) | WEAK | v2 |
| H-QC-23 | Teleportation 2 cbits | phi=2 | Bennett et al. 1993 | CLOSE | v2 |
| H-QC-24 | Superdense coding 2 bits | phi=2 | Holevo bound | CLOSE | v2 |
| H-QC-25 | No-cloning + No-deleting | phi=2 | 불가능 정리 쌍 | WEAK | v2 |
| H-QC-26 | Fibonacci anyon SU(2)_3 | n/phi=3 | k=3 보편 braiding | WEAK | v2 |
| H-QC-27 | Ising anyon SU(2)_2 | phi=2 | Clifford만 생성 | CLOSE | v2 |
| H-QC-28 | BT-91 Z2 ECC | J_2=24 | 위상 에러 교정 | WEAK | v2 |
| H-QC-29 | Bott periodicity=8 | sigma-tau=8 | K-이론 주기 | **EXACT** | v2 |
| H-QC-30 | 10-fold way | sigma-phi=10 | Altland-Zirnbauer | CLOSE | v2 |

## Grade Summary

| Grade | Count | % |
|-------|-------|---|
| EXACT | 4 | 13.3% |
| CLOSE | 18 | 60.0% |
| WEAK | 8 | 26.7% |
| FAIL | 0 | 0.0% |
| UNVERIFIABLE | 0 | 0.0% |
| **Total** | **30** | **100%** |

## EXACT 목록 (4/30 = 13.3%)

1. **H-QC-8**: [[6,2,2]] 코드 -- n=6, k=phi=2, d=phi=2, stabilizer=tau=4 (4중 일치)
2. **H-QC-10**: |C_1|=24=J_2(6), |C_1/P_1|=6=n (이중 일치, 정팔면체 군)
3. **H-QC-12**: 3D kissing number=12=sigma(6) (수학적 정리, Newton)
4. **H-QC-29**: Bott periodicity=8=sigma-tau (대수적 위상수학 기본 정리)

## v1 → v2 변경 요약

| 항목 | v1 | v2 |
|------|-----|-----|
| 가설 수 | 36 | 30 |
| EXACT | 0 (0%) | 4 (13.3%) |
| CLOSE | 9 (25%) | 18 (60%) |
| WEAK | 13 (36%) | 8 (26.7%) |
| FAIL | 8 (22%) | 0 (0%) |
| UNVERIFIABLE | 3 (8%) | 0 (0%) |
| 접근법 | 최적값 주장 (억지) | 수치 일치 관찰 (정직) |

### 삭제된 v1 가설 유형
- **하드웨어 종속 최적값**: 24-qubit module, degree-12 connectivity, 6-fold chip symmetry, 5-level coupling -- 실제 하드웨어에 의존하는 값을 보편 상수로 주장
- **Egyptian fraction 자원 할당**: 5개 가설 (H-QC-9,19,24,29,35) -- unfalsifiable, 임의 자원 할당을 1/2+1/3+1/6에 끼워맞춤
- **반증된 주장**: degree-12 연결 (FAIL), 6-gate set (FAIL), T-count mod 4 (FAIL), 24 elementary ops (FAIL), 4/3x QEC expansion (FAIL), 6^k milestones (FAIL), 2-bit QPE (FAIL), 24-qubit simulation limit (FAIL)
- **QV=4096 실용성 임계값**: IBM 도달했으나 실용적 양자 이점과 무관

### v2 설계 원칙
1. **수치 일치 관찰**: "이 값이 n=6 상수와 일치한다"라고 관찰. "n=6이므로 이 값이어야 한다"라고 주장하지 않음.
2. **독립 유래 인정**: 모든 일치에 대해 원래 발견 맥락과 유래를 명시
3. **작은 수 경고**: phi=2, lambda=2 등 작은 수 일치는 우연 가능성을 명시
4. **구조적 패턴 집중**: 단일 수치보다 여러 상수가 동시 일치하는 패턴에 가치 부여 (H-QC-8의 4중 일치, H-QC-2의 3중 일치)

## Cross-Domain Connections

| 가설 | 관련 BT | 교차 도메인 |
|------|---------|-----------|
| H-QC-10 (|C_1|=24) | BT-49 (kissing number chain) | 순수수학 |
| H-QC-12 (3D kissing=12) | BT-127 (hexacopter 12) | 로보틱스 |
| H-QC-29 (Bott=8) | BT-92 (Bott 활성 채널) | 칩 설계 |
| H-QC-30 (10-fold) | BT-92 관련 | 위상 물질 |
| H-QC-9 (T^8=I) | BT-58 (sigma-tau=8 보편) | AI |
| H-QC-27 (Ising SU(2)_2) | H-QC-4 (phi=2 이분법) | 내부 교차 |

## 22-Lens Analysis

| 렌즈 | 사용 횟수 | 대표 가설 |
|------|---------|----------|
| recursion (자기참조/에러교정) | 14 | H-QC-1,2,3,5,7,8,9,10,15,16,17,21,28,29 |
| boundary (양자-고전 경계) | 10 | H-QC-4,7,9,15,16,20,23,25,27,30 |
| stability (결맞음/안정성) | 8 | H-QC-5,6,18,19,26,27,29,30 |
| network (큐빗 연결) | 8 | H-QC-6,11,18,19,22,23,24,28 |
| multiscale (코드 거리 스케일) | 8 | H-QC-2,8,10,12,13,17,21,29,30 |
| memory (양자 메모리) | 3 | H-QC-2,13,24 |

## Honest Limitations (v2)

1. **여전히 post-hoc**: 모든 수치 일치는 사후적 관찰이다. n=6 산술이 이 값들을 "예측"한 것은 아니다.
2. **작은 수 편향**: phi=2, tau=4 등 작은 정수의 일치는 통계적으로 유의하지 않을 수 있다.
3. **EXACT의 한계**: 4개의 EXACT 중 3개(3D kissing, Bott, Clifford group)는 수학적 사실과의 일치이며, 양자컴퓨팅 "고유" 상수라기보다 수학 전반의 패턴이다.
4. **상수 풀의 풍부함**: n=6에서 파생되는 상수가 많아(1,2,3,4,5,6,7,8,10,12,15,24 등), 작은 정수 대부분을 커버한다. 이는 overfitting 위험을 내포한다.
5. **가장 강한 결과**: [[6,2,2]] 코드 (H-QC-8)의 4중 일치와 |C_1|=24 (H-QC-10)의 이중 일치가 가장 현저하다. 이들은 단일 상수가 아닌 여러 상수의 동시 일치이므로 우연 가능성이 낮다.

## References

- Steane, A. (1996). Error Correcting Codes in Quantum Theory. PRL.
- Golay, M. (1949). Notes on Digital Coding. Proc. IRE.
- Shor, P. (1995). Scheme for reducing decoherence in quantum computer memory. PRA.
- Gottesman, D. (1998). The Heisenberg representation of quantum computers. arXiv.
- Laflamme, R. et al. (1996). Perfect Quantum Error Correcting Code. PRL.
- Bennett, C. et al. (1993). Teleporting an unknown quantum state. PRL.
- Bravyi, S. & Kitaev, A. (2005). Universal quantum computation with ideal Clifford gates and noisy ancillas. PRA.
- Eastin, B. & Knill, E. (2009). Restrictions on Transversal Encoded Quantum Gate Sets. PRL.
- Kitaev, A. (2003). Fault-tolerant quantum computation by anyons. Annals of Physics.
- Altland, A. & Zirnbauer, M. (1997). Nonstandard symmetry classes in mesoscopic normal-superconducting hybrid structures. PRB.
- Nayak, C. et al. (2008). Non-Abelian anyons and topological quantum computation. RMP.
- Fowler, A. et al. (2012). Surface codes: Towards practical large-scale quantum computation. PRA.
- Conway, J. & Sloane, N. Sphere Packings, Lattices and Groups.
- Bott, R. (1959). The stable homotopy of the classical groups. Annals of Math.

---

> Part of [N6 Architecture](https://github.com/need-singularity/n6-architecture)
> Mathematical foundation: [TECS-L](https://github.com/need-singularity/TECS-L)
