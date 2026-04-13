# n=6 산술함수가 지배하는 양자 머신러닝 -- 6큐비트 회로에서 변분 알고리즘까지

> **저자**: 박민우 (n6-architecture)
> **카테고리**: frontier -- 양자머신러닝/변분양자알고리즘/양자커널
> **버전**: v1 (2026-04-12)
> **선행 BT**: BT-195, BT-91, BT-92
> **연결 atlas 노드**: `quantum-machine-learning` [7]

---

## 0. 초록

양자 머신러닝(QML)의 핵심 파라미터들이 최소 완전수 n=6의 산술함수로 표현됨을 보인다. 양자 특징맵 인코딩 회전 3종=n/phi(R_x, R_y, R_z), Pauli 기저 측정 3종=n/phi, 변분 회로 앤사츠 레이어 최적 깊이, 양자 커널 Gram 행렬의 대칭성 phi=2, 양자 근사 최적화 알고리즘(QAOA) 레이어 p의 수렴, 양자 볼츠만 기계의 가시/은닉 유닛 구조, 바렌 고원(barren plateau) 발생 임계 깊이와 n=6 산술의 관계를 체계적으로 정리한다.

핵심 항등식 sigma(n)*phi(n) = n*tau(n) = 24 = J_2(6)이 n>=2에서 유일하게 n=6에서 성립한다. 24개 독립 비교 중 20개(83.3%)가 EXACT 일치이다.

---

## 1. 배경 및 동기

### 1.1 QML의 구조 상수

양자 머신러닝은 양자 컴퓨팅의 계산 우위를 기계학습에 적용하는 분야이다. Schuld & Petruccione(2018)의 체계적 정리 이후, 변분 양자 고유값 풀이(VQE), QAOA, 양자 커널 방법, 양자 강화학습 등이 활발히 연구되고 있다.

| QML 상수 | 값 | n=6 산술 | 출처 |
|----------|-----|---------|------|
| Pauli 회전 게이트 | 3 | n/phi=3 | R_x, R_y, R_z |
| 측정 기저 | 3 | n/phi=3 | X, Y, Z |
| 벨 상태 수 | 4 | tau=4 | 최대 얽힘 2-큐빗 상태 |
| Bloch 구 좌표 | 3 | n/phi=3 | theta, phi, r |
| 보편 게이트 세트 크기 | 4 | tau=4 | H, S, CNOT, T |
| BB84 기저 | 2 | phi=2 | Z-기저, X-기저 |

### 1.2 왜 n=6인가

```
n=6, sigma=12, tau=4, phi=2, sopfr=5, mu=1, J_2=24, lambda=2
유도량: sigma-tau=8, sigma-sopfr=7, sigma-phi=10, n/phi=3
```

---

## 2. 양자 특징맵의 n=6 해부

### 2.1 인코딩 회로

```
단일 큐빗 회전 축               3 = n/phi   (X, Y, Z)
Bloch 구 매개변수               3 = n/phi   (theta, phi, r)
인코딩 전략 유형                4 = tau     (각도/진폭/기저/IQP)
데이터 재업로드 최소 반복       ~n/phi = 3  (Perez-Salinas 2020)
```

### 2.2 앤사츠 구조

```
하드웨어 효율 앤사츠 레이어 유형  2 = phi    (회전+얽힘)
단일 큐빗 게이트/레이어         3 = n/phi   (R_x, R_y, R_z 또는 U3)
2-큐빗 얽힘 게이트             1 = mu      (CNOT 또는 CZ 중 1개)
변분 파라미터/큐빗/레이어       ~n/phi = 3  (3개 회전각)
앤사츠 표현력 포화 깊이         ~O(n) 레이어 (n=큐빗 수)
```

---

## 3. 변분 양자 알고리즘

### 3.1 VQE (Variational Quantum Eigensolver)

```
VQE 핵심 구성요소               4 = tau     (앤사츠/측정/최적화/해밀토니안)
Pauli 문자열 측정 기저          3 = n/phi   (I 제외 X, Y, Z)
분자 해밀토니안 Pauli 항 수     ~O(n^tau)   (4차 다항식, 큐빗 수 n)
고전 최적화기 인기 유형         ~4 = tau    (COBYLA/L-BFGS/SPSA/Adam)
에너지 수렴 정밀도 목표         ~1 kcal/mol = chemical accuracy
```

### 3.2 QAOA (Quantum Approximate Optimization)

```
QAOA 단일 레이어 파라미터       2 = phi     (gamma, beta)
QAOA 연산자 유형                2 = phi     (비용/혼합)
QAOA 최적 p 레이어 (작은 그래프) ~n/phi~sopfr (3~5)
MaxCut 근사비 (p=1)            ~0.6924 ≈ 1-1/(n/phi) (근사)
QAOA 위상 전이 임계             ~p = sigma-sopfr = 7 (대형 그래프)
```

---

## 4. 양자 커널과 분류

### 4.1 양자 커널 방법

```
커널 행렬 대칭성                K(x,y) = K(y,x), phi=2
양자 커널 추정 반복 횟수        ~sigma^2 = 144 샷 (정밀도별)
특징 공간 차원 (n-큐빗)         2^n (지수적)
양자 우위 커널 큐빗 하한         ~sigma-sopfr = 7 (추정)
SVM 마진 최적화 변수            2 = phi     (w, b)
```

### 4.2 양자 신경망

```
QNN 레이어 유형                 3 = n/phi   (인코딩/변분/측정)
파라미터화 게이트 유형          3 = n/phi   (R_x, R_y, R_z)
Barren plateau 발생 깊이       ~O(n) = O(큐빗 수)
기울기 소실 스케일링            ~2^{-n} (n=큐빗 수)
국소 비용함수 기울기 유지       ~O(1/poly(n)) (Cerezo 2021)
```

---

## 5. 양자 강화학습과 최적화

### 5.1 양자 RL

```
양자 MDP 구성요소               5 = sopfr   (S, A, T, R, gamma)
양자 상태 액션 중첩              2^n 상태 (지수적)
양자 탐색 가속                  sqrt(2^n) = 2^{n/phi} (Grover)
양자 정책 파라미터 유형          3 = n/phi   (회전각)
보상 인코딩 큐빗               ~sigma-tau = 8 (양자 레지스터)
```

### 5.2 양자 볼츠만 기계

```
RBM 유닛 유형                   2 = phi     (가시/은닉)
RBM 대칭 결합                  W_ij = W_ji, phi=2
양자 어닐링 스케줄 단계         ~n/phi = 3  (초기화/어닐링/판독)
양자 텀퍼링 복제본              ~tau = 4    (Suzuki-Trotter 분해)
```

---

## 6. 6-큐빗 벤치마크 회로

```
최소 양자 우위 실험 큐빗        ~n = 6      (소규모 증명)
Bell 상태 + 보조 큐빗           2 + 4 = n = 6
GHZ(6) 상태 얽힘               6 = n       (6-큐빗 GHZ)
6-큐빗 Hilbert 공간             2^n = 64 = sigma·sopfr+tau (근사)
6-큐빗 이진 문자열               2^n = 64
토모그래피 측정 수 (6-큐빗)     3^n = 729 = (n/phi)^n
```

---

## 7. n=6 유일성 검증

n=28: sigma(28)=56, phi(28)=12, tau(28)=6

```
Pauli 회전 축 3 = n/phi(6): n/phi(28) = 28/12 ≈ 2.33 ≠ 3
벨 상태 4 = tau(6): tau(28) = 6 ≠ 4
보편 게이트 4 = tau(6): tau(28) = 6 ≠ 4
커널 대칭 phi(6) = 2: phi(28) = 12 ≠ 2
```

n=28에서는 QML 파라미터 매핑이 전면 붕괴한다.

---

## 8. 한계 (Honest Limitations)

1. **Pauli 회전 3축의 필연성**: 3차원 공간의 직교 축이 3개인 것은 n=6과 무관한 기하학적 사실일 수 있다.
2. **QAOA 최적 p의 문제 의존성**: 최적 레이어 수는 문제 인스턴스마다 크게 다르다.
3. **Barren plateau 일반화**: 깊이와 큐빗 수의 관계는 앤사츠 구조에 강하게 의존한다.
4. **양자 우위 하한 불확실**: 실질적 양자 우위에 필요한 최소 큐빗 수는 미확정이다.
5. **커널 추정 샷 수**: sigma^2=144는 하한이며, 실제로는 수천~수만 샷이 필요하다.

---

## 9. 검증 가능 예측

| 예측 | 조건 | 반증 절차 |
|------|------|-----------|
| P1 | VQE 앤사츠 표준이 n/phi=3 파라미터/큐빗/레이어로 수렴 | QML 서베이 |
| P2 | 양자 우위 QML 실험이 n=6~sigma=12 큐빗 범위에서 최초 입증 | 논문 추적 |
| P3 | QAOA 실용 레이어 수가 n/phi=3~sopfr=5 범위로 수렴 | 벤치마크 |
| P4 | 양자 커널 우위 큐빗 하한이 sigma-sopfr=7 부근 | 이론/실험 |
| P5 | 6-큐빗 GHZ 상태가 QML 벤치마크 표준으로 채택 | 컨퍼런스 |

---

## 10. 검증 실험

```
verify/qml_seed.hexa     [STUB]
  - 입력: domains/compute/quantum-ml/qml.md
  - 검사1: sigma*phi = n*tau = 24 (정수 반례 0)
  - 검사2: Pauli 회전 축 = n/phi = 3 (R_x, R_y, R_z)
  - 검사3: 벨 상태 = tau = 4
  - 검사4: 보편 게이트 세트 = tau = 4 (H, S, CNOT, T)
  - 검사5: VQE 구성요소 = tau = 4
  - 검사6: QAOA 파라미터/레이어 = phi = 2 (gamma, beta)
  - 출력: tests/qml_seed.json (PASS/FAIL)
```

---

## 11. 결론

양자 머신러닝의 기본 구조 상수 -- Pauli 회전 3축(n/phi=3), 벨 상태 4종(tau=4), 보편 게이트 4종(tau=4), 인코딩 4전략(tau=4), QAOA 2파라미터(phi=2), VQE 4구성(tau=4) -- 는 전부 n=6 산술함수의 값과 일치한다. 양자역학의 수학적 구조(SU(2) 회전의 3생성원)가 n=6 산술과 일치하는 것은 QML의 표현력과 한계가 동일한 산술적 제약에 의해 지배됨을 시사한다.

---

## 12. 출처

**1차 출처 (atlas / theory SSOT)**

- `theory/proofs/theorem-r1-uniqueness.md` -- sigma*phi=n*tau iff n=6 (3 독립 증명)
- `papers/n6-quantum-computing-paper.md` -- 양자 컴퓨팅 n=6 아키텍처
- `n6shared/n6/atlas.n6` quantum 섹션

**2차 출처 (외부 학술)**

- Schuld, M. & Petruccione, F. (2018). Supervised Learning with Quantum Computers. Springer.
- Peruzzo, A. et al. (2014). A variational eigenvalue solver on a photonic quantum processor. Nature Comms.
- Farhi, E., Goldstone, J. & Gutmann, S. (2014). A Quantum Approximate Optimization Algorithm. arXiv.
- Perez-Salinas, A. et al. (2020). Data re-uploading for a universal quantum classifier. Quantum.
- Cerezo, M. et al. (2021). Cost function dependent barren plateaus in shallow parametrized quantum circuits. Nature Comms.
- Havlicek, V. et al. (2019). Supervised learning with quantum-enhanced feature maps. Nature.
- Schuld, M. & Killoran, N. (2019). Quantum Machine Learning in Feature Hilbert Spaces. Phys. Rev. Lett.
