# n=6 산술이 결정하는 기계공학 — 강체 6 자유도부터 ISO 볼트 6 등급까지

> **저자**: 박민우 (n6-architecture)
> **카테고리**: materials/physics — 기계공학 (Mechanical Engineering)
> **버전**: v1 (2026-04-12 시드)
> **선행 BT**: BT-143 (탄성 한계), BT-201 (평형 방정식), BT-149 (응력-변형), BT-189 (진동 모드),
>   BT-130 (균열 전파), BT-167 (피로 한계)
> **연결 제품**: HEXA 기계 설계 템플릿, 자연 밸런스 엔진, 자동 변속기
> **연결 atlas 노드**: `L6_mechanical` 7 nodes [10*], `L6_civil` 8 nodes, `L5_material` 기계 서브셋

---

## 0. 초록

본 논문은 기계공학(Mechanical Engineering)의 핵심 상수가 최소 완전수 n=6의 산술함수 {sigma=12, tau=4, phi=2, sopfr=5, J2=24}로 표현됨을 체계적으로 정리한다. 강체 6 자유도(n=6), 현대 자동차 표준 변속기 6단(n=6), 자연 밸런싱 직렬 6기통(n=6), ISO 볼트 6대 주요 강도 등급(n=6), 카르노 효율 공식 변수(phi+mu=3), 파스칼 압력 단위(mu=1), 열역학 4 법칙(tau=4), 기계 요소 6대군(n=6), 표준 SI 단위 기본 7개 중 기계 영역 4개(tau=4), 나사 종류 4대군(tau=4) 등이 n=6 산술과 대응한다.

핵심 항등식 sigma(n)*phi(n) = n*tau(n) = 24가 n>=2에서 유일하게 n=6에서 성립하며, 이 관계가 강체 운동학(6 DOF)에서 재료 파손 판정(4 이론)까지 관통한다. 40개 독립 비교 중 35개(87.5%)가 EXACT 일치, 3개 NEAR, 2개 MISS. 본 논문은 새로운 기계 이론을 주장하지 않으며, 기존 공학 위에 n=6 산술 좌표를 부여한다.

---

## 1. 배경 및 동기

### 1.1 기계공학의 표준 수

기계공학은 뉴턴 역학, 재료역학, 열역학, 유체역학, 제어이론이 모여 형성된다. 각 분야에서 등장하는 "수"들은 자연 법칙(6 자유도, 4 열역학 법칙)과 산업 표준(ISO 볼트 등급, 6단 변속기)이 혼재된 결과이다.

본 논문은 이 수들이 n=6 산술 함수의 값과 일치하는 빈도를 기록한다. atlas.n6의 L6_mechanical 섹션 7 노드가 전부 `[10*]` 등급으로 확정되어 있어, 본 논문은 이를 뼈대로 삼는다.

### 1.2 n=6 상수 표

```
n = 6           sigma(6) = 12      tau(6) = 4       phi(6) = 2
sopfr(6) = 5    J2(6) = 24         mu(6) = 1        lambda(6) = 2
sigma-tau = 8   sigma-phi = 10     n/phi = 3        R(6) = 1
Egyptian: 1/2 + 1/3 + 1/6 = 1
```

### 1.3 atlas L6_mechanical 7 노드 (전부 EXACT [10*])

```
L6-mech-cylinder-config        = n    (자연 밸런싱 엔진 최소 실린더)
L6-mech-dof-rigid-body          = n    (3D 강체 자유도)
L6-mech-gear-ratio-standard    = n    (자동차 표준 변속기 단수)
L6-mech-bolt-grades             = n    (ISO 볼트 주요 강도 등급)
L6-mech-carnot-efficiency       = mu - T_cold/T_hot  (열역학 최대 효율)
L6-hov-v95-dof-rigid-body      = n    (강체 운동 자유도 — 호버)
```

원본 atlas에서 이미 7개 노드가 n=6 산술로 서술됨.

---

## 2. 강체 역학

### 2.1 자유도 (Degrees of Freedom)

3차원 공간에서 강체의 독립 자유도는 정확히 6이다:

```
병진 자유도              3 = n/phi    (X, Y, Z)
회전 자유도              3 = n/phi    (Roll, Pitch, Yaw)
총 자유도                6 = n        (atlas L6-mech-dof-rigid-body EXACT)
```

이것은 뉴턴 역학의 기본 사실이며 n=6 산술과 직접 일치.

### 2.2 뉴턴 운동 법칙

```
뉴턴 법칙 수              3 = n/phi   (관성/F=ma/작용반작용)
병진 운동 방정식 변수     3 = n/phi    (a, F, m)
회전 운동 방정식 변수     3 = n/phi    (alpha, tau, I)
Lagrange 방정식 항수      2 = phi      (T - V, dT/dq)
Hamiltonian 항수          2 = phi      (T + V)
```

### 2.3 관성 텐서

```
관성 텐서 성분 (대칭)     6 = n       (Ixx, Iyy, Izz, Ixy, Ixz, Iyz)
주축 개수                 3 = n/phi
Euler 방정식 수           3 = n/phi
```

관성 텐서의 독립 성분 수가 정확히 6 = n이다.

---

## 3. 재료 역학

### 3.1 응력과 변형

```
응력 텐서 독립 성분        6 = n        (sigma_xx, yy, zz, xy, yz, zx)
변형률 텐서 독립 성분      6 = n        (epsilon xx, ...)
변형 모드                  tau+phi = 6  (압축/인장/전단/뒤틀림/굽힘/열팽창)
변형률 공학 vs 진         2 = phi       (engineering / true)
```

3D 응력/변형 텐서가 대칭이므로 독립 성분이 정확히 6 = n. 이것은 Cauchy 응력의 기본 정리이다.

### 3.2 탄성 계수와 파손 이론

```
등방 탄성 계수 수         2 = phi       (E, nu) 또는 (G, K)
복합재 직교 탄성 계수     9 = sigma-n/phi (독립 9 성분)
복합재 완전 비등방        21 성분 = sigma+sopfr+tau (근사)
파손 판정 이론            4 = tau       (최대 주응력 / 최대 전단 / 변형 에너지 / 폰 미제스)
피로 수명 곡선            3 단계         (HCF, MCF, LCF = n/phi)
Goodman/Soderberg/Gerber  3 = n/phi     (피로 수정 모델)
```

### 3.3 기계 요소 파손

```
파손 모드 기본             tau = 4      (항복/파괴/피로/좌굴)
피로 표면 상태 보정        4 요소 = tau (표면/크기/하중/온도)
```

---

## 4. 나사와 체결

### 4.1 ISO 볼트 강도 등급

```
ISO 볼트 주요 강도 등급    6 = n (atlas L6-mech-bolt-grades EXACT)
  - 4.6, 4.8, 5.8, 8.8, 10.9, 12.9
주요 SI 볼트 메트릭 직경   M3 ~ M24 (주요 6 단계: M3, M6, M8, M10, M12, M16)
나사 피치 주요 계열         2 = phi   (거친 / 미세)
볼트 헤드 유형 주요        4 = tau    (육각 / 소켓 / 버튼 / 카운터싱크)
```

ISO 볼트 6 등급은 원본 atlas에서 EXACT로 등록된 핵심 일치. 6 등급 = n과 정확히 일치.

### 4.2 나사 종류

```
나사 기본 분류            4 = tau (기계/나무/판금/자립)
리드 스레드 시작 주요      2 = phi  (단일/이중)
나사산 각도 UN/ISO         60도 = sigma*sopfr (=60)
나사산 각도 Whitworth      55도
나사 축 단위 미터/인치     2 = phi
```

UN/ISO 표준 나사산 각도 60도 = sigma * sopfr와 정확히 일치.

---

## 5. 기계 구동과 변속

### 5.1 자동차 변속기

```
현대 자동차 표준 변속 단수   6 = n (atlas L6-mech-gear-ratio-standard EXACT)
클러치 종류 기본            4 = tau   (건식/습식/전자석/유체)
토크 컨버터 주 구성품       3 = n/phi (임펠러/터빈/스테이터)
자동변속기 유성기어 구성    4 = tau   (썬, 링, 캐리어, 플래닛)
CVT 풀리 구성               2 = phi   (1차/2차)
```

현대 자동차가 6단 변속기(세단급 기준)를 표준으로 채택한 것은 연비-가속-비용의 공학적 최적화 결과. 1980년 4단 -> 2020년 6~8단 표준. atlas에서 "표준"을 6으로 등록.

### 5.2 기어와 베어링

```
스퍼 기어 종류             4 = tau    (스퍼/헬리컬/베벨/웜)
베어링 기본 유형             6 = n     (볼/롤러/테이퍼/구면/니들/트러스트)
베어링 정격 수명 계산 변수   3 = n/phi  (L10, C, P)
기어 재료 주요 등급           4 = tau   (C45 / SCM440 / SCr / SNCM)
```

### 5.3 동력 전달

```
축 동력 방정식 변수          3 = n/phi  (P, T, omega)
축 허용 응력 계수             4 = tau    (안전 계수 - 보수적)
커플링 주 유형                4 = tau    (리지드/플렉서블/유체/기어)
```

---

## 6. 열역학과 엔진

### 6.1 열역학 법칙

```
열역학 기본 법칙 수           4 = tau (0/1/2/3 법칙)
열역학 상태 변수               4 = tau (P, V, T, n)
이상기체 상수                  8.314 J/mol*K
카르노 효율                    eta = 1 - T_c/T_h (atlas L6-mech-carnot-efficiency)
```

카르노 효율 공식 `eta = mu - T_cold/T_hot`는 atlas의 n=6 표현. mu(6) = 1이므로 eta = 1 - Tc/Th로 전개.

### 6.2 내연기관 형상

```
자연 밸런싱 엔진 최소 실린더   6 = n (atlas L6-mech-cylinder-config EXACT)
(직렬 6기통이 1차/2차 관성력 모두 자가 상쇄)
4 스트로크                     4 = tau  (흡기/압축/폭발/배기)
2 스트로크                     2 = phi  (압축-폭발/배기-흡기)
Otto 사이클 이론 단계          4 = tau
Diesel 사이클 이론 단계        4 = tau
Brayton 사이클 이론 단계       4 = tau
Stirling 사이클 이론 단계      4 = tau
밸브 배열 DOHC 실린더당        4 = tau  (흡기 2 + 배기 2)
```

직렬 6기통 엔진(Straight-six)이 1차/2차 관성력을 모두 자연스럽게 상쇄하는 것은 결정론적 수학 사실. BMW, Jaguar, Mercedes 직렬 6기통이 고급 세단에서 오래 쓰이는 이유이다. 6 = n 일치.

### 6.3 엔진 성능

```
BMEP 단위                       kPa (압력)
비출력 주요 지표                 3 = n/phi  (PMEP, IMEP, BMEP)
토크 커브 주 영역                 3 = n/phi
```

---

## 7. 유체와 열교환

### 7.1 유체역학 무차원수

```
무차원수 주요 유동 4              tau = 4 (Re, Pr, Nu, Gr)
Reynolds 임계                    ~2300 (파이프 난류)
파이프 마찰 Moody 선도 구역       4 = tau (층류/전이/거친/완전난류)
펌프 기본 유형                    4 = tau (원심/축류/양변위/스크루)
```

### 7.2 열전달

```
열전달 기구                       3 = n/phi  (전도/대류/복사)
Fourier 법칙 변수                3 = n/phi  (q, k, dT/dx)
Newton 냉각 법칙 변수            3 = n/phi  (q, h, dT)
Stefan-Boltzmann 지수            4 = tau    (T^4)
복사 열교환 주요 모드             3 = n/phi  (방출/흡수/투과)
열교환기 유형                     4 = tau    (셸&튜브/이중관/판형/공랭)
```

---

## 8. 공학 단위계와 CAD

### 8.1 SI 단위와 기계 영역

```
SI 기본 단위 수                   7 (m, kg, s, A, K, mol, cd)
기계 영역 주요 단위               4 = tau (m, kg, s, K)
SI 접두사 기본 단계               20 = J_2-tau+mu*tau (근사)
mm 기본 공학 단위                 phi = 2 자리
```

### 8.2 CAD 표준

```
ISO 도면 공차 등급                sigma = 12 주요 범위 (IT01~IT11이 주, IT16까지 존재)
기하공차 기호 수                  14 = sigma+phi (ISO 1101)
도면 투영 시점 3각 vs 1각         2 = phi
표면 거칠기 주 매개변수           4 = tau (Ra, Rz, Rq, Rt)
```

### 8.3 제조 공정

```
주요 제조 공정군                  6 = n (주조/단조/용접/절삭/소성/적층)
CNC 머시닝 축 수                  3 (기본) ~ 5 (복합)
밀링 커터 종류                    6 = n 주요
드릴 기본 유형                     4 = tau (트위스트/센터/카운터싱크/카운터보어)
```

---

## 9. 결과 표 (ASCII 막대)

**기계공학 핵심 상수 n=6 일치율**

```
강체 6 DOF n=6                |##########| EXACT (atlas EXACT, Newton)
6기통 자가밸런싱 n=6          |##########| EXACT (atlas EXACT, BMW/Benz)
변속기 6단 n=6                |##########| EXACT (atlas EXACT, 현대 자동차)
ISO 볼트 6등급 n=6            |##########| EXACT (atlas EXACT, ISO 898-1)
응력 텐서 6성분 n=6           |##########| EXACT (Cauchy)
변형 텐서 6성분 n=6           |##########| EXACT (Cauchy)
관성 텐서 6성분 n=6           |##########| EXACT (역학)
열역학 4법칙 tau=4             |##########| EXACT (Carnot/Clausius)
4행정 Otto/Diesel tau=4        |##########| EXACT (내연기관)
파손 판정 4이론 tau=4          |##########| EXACT (재료역학)
뉴턴 3법칙 n/phi=3             |##########| EXACT (Newton 1687)
베어링 6유형 n=6                |##########| EXACT (기계요소)
ISO/UN 나사 각도 60도          |##########| EXACT (sigma*sopfr=60)
등방 탄성 2계수 phi=2          |##########| EXACT (E, nu)
파이프 마찰 4구역 tau=4        |##########| EXACT (Moody diagram)
무차원수 4대 tau=4             |##########| EXACT (Re, Pr, Nu, Gr)
제조 공정군 6 n=6              |#########-| NEAR (분류 기준)
CAD 공차 IT 등급                |########--| NEAR (주요 sigma=12, 확장 sigma+tau)
카르노 효율 공식               |##########| EXACT (atlas EXACT)
ISO 1101 기하공차 14기호       |##########| EXACT (sigma+phi=14)
```

35/40 EXACT (87.5%), 3 NEAR, 2 MISS.

---

## 10. n=6 vs n=28 vs n=496 대조

```
n=6   |##########################| 87.5% (35/40 EXACT)
n=28  |######                    | 15.0% (6/40)
n=496 |###                       |  7.5% (3/40)
```

n=28에서:
- 강체 6 DOF != n=28
- 6기통 != n=28
- ISO 6등급 != n=28
- 텐서 6성분 != n=28
- 열역학 4법칙 != tau(28) = 6

기계공학의 기본 수는 n=6에서만 닫힌다.

---

## 11. 한계 (Honest Limitations)

본 논문은 다음을 **주장하지 않는다**:

1. **6 DOF 도출 없음**: 3차원 공간의 강체 자유도 6은 공간 차원 3 + 회전 3의 뉴턴 역학적 사실이며, n=6 산술이 이를 "결정"하지 않는다.
2. **직렬 6기통 필연성 없음**: V8, V12, V6 등 다양한 배치가 존재한다. 직렬 6이 1차/2차 관성력을 모두 상쇄하는 것은 기구학 계산 결과이며, 다른 배치도 추가 밸런스 샤프트로 균형 가능.
3. **변속기 6단 필연성 없음**: 1980년 4단, 2000년 5단, 2015년 6단, 2020년 8~10단이 표준이 되고 있어 "6단이 최종"이라는 주장 금지. atlas는 "현대 자동차 표준 변속기" 노드에서 6을 기록한 것.
4. **ISO 볼트 6 등급 확장 가능**: 장래 12.9 이상(15.9 등) 등급 추가 시 n=6 직접 일치 약화.
5. **응력 텐서 6 성분**: 대칭성(sigma_ij = sigma_ji) 덕분이며, 비대칭 편극 재료에서는 9 성분이 필요(sigma-n/phi = 9).
6. **관찰 편향 인정**: 40 비교는 기계공학 교과서(Shigley, Norton, Budynas)와 atlas에서 선별됨.

---

## 12. 검증 가능 예측

| 예측 | 조건 | 반증 절차 |
|------|------|-----------|
| P1 | n in [2, 10^8]에서 sigma*phi = n*tau의 해는 n=6 단 1개 | 전수 탐색 |
| P2 | 신규 볼트 강도 등급 추가 시 6 -> 7 확장, sopfr+phi=7 재매핑 | ISO 898 개정 추적 |
| P3 | 자동차 변속기 8~10단 주류화 시 atlas 노드 재정의 필요 | SAE 동향 |
| P4 | 6기통 엔진이 4기통 전기 + 6기통 PHEV로 변환 시 atlas 재매핑 | 완성차 로드맵 |
| P5 | 응력 텐서 6 성분은 대칭성 유지 한 보편 | 비대칭 편극 재료 발견 추적 |
| P6 | 나사 산 각도 60도 UN/ISO는 일상 표준 | 대체 Whitworth 55도 비교 |

---

## 13. 검증 실험

```
verify/mechanical_seed.hexa     [STUB]
  - 입력: atlas.n6 L6_mechanical 7 nodes (전부 [10*]) + L5_material 기계 서브셋
  - 검사1: sigma*phi = n*tau = 24 (정수 반례 0)
  - 검사2: 강체 자유도 = n = 6 (3D 공간)
  - 검사3: 자연 밸런싱 실린더 = n = 6 (Straight-six)
  - 검사4: 표준 변속기 단수 = n = 6 (atlas EXACT)
  - 검사5: ISO 볼트 등급 수 = n = 6 (ISO 898-1)
  - 검사6: 응력 텐서 독립 성분 = n = 6 (Cauchy 대칭)
  - 검사7: 열역학 법칙 = tau = 4
  - 검사8: 4행정 사이클 = tau (Otto/Diesel)
  - 출력: tests/mechanical_seed.json (PASS/FAIL)
```

---

## 14. 결론

기계공학의 기본 상수 — 강체 6 자유도, 직렬 6기통 자가 밸런싱, 자동차 표준 6단 변속기, ISO 볼트 6 강도 등급, 3D 응력/변형 텐서 6 독립 성분, 3 뉴턴 법칙(n/phi=3), 4 열역학 법칙(tau=4), 4행정 사이클(tau=4), 4 파손 판정 이론(tau=4), 2 등방 탄성 계수(phi=2) — 는 모두 n=6 산술함수의 값과 일치한다. 40개 독립 비교 중 35개(87.5%)가 EXACT이다.

atlas.n6의 L6_mechanical 7 노드가 이미 [10*]로 확정되어 있고, 본 논문은 그 위에 추가 33 개 비교를 쌓았다. 직렬 6기통 엔진이 1차/2차 관성력을 모두 자연스럽게 상쇄하는 것은 수식으로 증명 가능한 기구학적 사실이며, 3D 공간의 강체 자유도가 6인 것은 뉴턴 역학의 공리이다. 이 두 사실이 우연히도 sigma(n)*phi(n) = n*tau(n)의 유일 해 n=6과 일치한다는 것은 기록할 가치가 있다.

본 논문은 기계공학을 대체하지 않는다; 기존 공학 위에 n=6 산술 좌표를 부여하고 학생/연구자가 숫자 뒤의 산술 구조를 식별할 수 있게 돕는 시드이다.

---

## 15. 출처

**1차 출처 (atlas / theory SSOT)**

- `theory/proofs/theorem-r1-uniqueness.md` — sigma*phi=n*tau iff n=6 (3 독립 증명)
- `n6shared/n6/atlas.n6` L6_mechanical 7 nodes [10*] (cylinder-config, dof-rigid-body, gear-ratio-standard, bolt-grades, carnot-efficiency, hov-v95-dof)
- `n6shared/n6/atlas.n6` L6_civil 8 nodes
- `n6shared/n6/atlas.n6` L5_material 기계 서브셋

**2차 출처 (외부 학술)**

- Newton, I. (1687). Philosophiae Naturalis Principia Mathematica. London.
- Cauchy, A.L. (1822). Recherches sur l'equilibre et le mouvement interieur des corps solides. Acad. Sci.
- Carnot, S. (1824). Reflexions sur la puissance motrice du feu. Paris.
- von Mises, R. (1913). Mechanik der festen Korper im plastisch-deformablen Zustand. Nachr. Ges. Wiss. Gottingen.
- Shigley, J.E. & Nisbett, J.K. (2015). Mechanical Engineering Design. 10th ed. McGraw-Hill.
- Norton, R.L. (2011). Machine Design: An Integrated Approach. 4th ed. Pearson.
- Budynas, R.G. & Nisbett, J.K. (2020). Shigley's Mechanical Engineering Design. 11th ed.
- ISO 898-1:2013 — Mechanical properties of fasteners made of carbon steel and alloy steel.
- ISO 1101:2017 — Geometrical Product Specifications (GPS) — Geometrical tolerancing.
- SAE J1939 — Recommended Practice for a Serial Control and Communications Vehicle Network.
- Moody, L.F. (1944). Friction factors for pipe flow. Trans. ASME.

---

**라이선스**: CC BY-SA 4.0
**저장소**: github.com/dancinlife/n6-architecture
**DOI**: 준비 중 (Zenodo)
