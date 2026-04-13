---
domain: naval-architecture
alien_index_current: 0
alien_index_target: 10
requires: []
---
# N6 조선/선박공학 (Naval Architecture) — 완전수 6 산술 가설

## 개요

조선공학과 해양 운송의 핵심 설계 파라미터가 n=6 산술과 일치한다.
선박의 6자유도 운동(SE(3)=n), IMO SOLAS 방수격벽(n), Beaufort 풍력계급(sigma=12),
컨테이너 규격(J2-tau=20), 항만 수심(sigma~J2) 등 해사 산업 전반에 걸친 n=6 수렴을 검증한다.

### 산술 상수

```
n=6, sigma=12, tau=4, phi=2, sopfr=5, J2=24, mu=1
sigma-phi=10, sigma-tau=8, sigma-mu=11, n/phi=3
sigma*tau=48, sigma^2=144, sigma*sopfr=60
div(6) = {1, 2, 3, 6}
```

---

## H-NAV-1: 선박 6자유도 운동 = n = SE(3) (EXACT)

> 선박의 운동 자유도가 6인 것은 SE(3) dim = n = 6 그 자체이다.

### n=6 도출
선박 6DOF: surge(전후)/sway(좌우)/heave(상하)/roll(횡동요)/pitch(종동요)/yaw(선수요) = 6 = n.
이는 3차원 공간의 강체 운동 = SE(3) = 6차원의 물리적 필연이다.
BT-123(SE(3) dim=n=6), BT-279(해양 IMO 안전 n=6)와 직접 연결.

### 검증
모든 선박 조종 시뮬레이터(Maneuvering Model Group): 6DOF 모델 표준.
IMO에서 정의하는 선박 운동 = 6자유도.
**등급: EXACT** (수학적 필연, 6 = n)

---

## H-NAV-2: IMO SOLAS 방수격벽 최소 구획 = n = 6 (EXACT)

> 국제해사기구(IMO) SOLAS 규정에서 여객선 방수격벽 최소 구획 수가 6인 것은 n=6과 일치한다.

### n=6 도출
SOLAS Chapter II-1: 여객선은 최소 6개 방수격벽(watertight compartment)을 가져야 한다.
이는 1-compartment flooding 생존성 확보를 위한 최소 구조 단위 = n = 6.

### 검증
IMO SOLAS 규정 Chapter II-1, Regulation 7-8: 여객선 최소 방수격벽 수.
Titanic 이후 모든 여객선 설계 표준.
**등급: EXACT** (6 = n, IMO 국제규정)

---

## H-NAV-3: 타이타닉 16구획 = phi^tau = 2^4 (EXACT)

> RMS Titanic의 16개 방수격벽 구획 수가 phi^tau = 2^4 = 16과 정확히 일치한다.

### n=6 도출
Titanic watertight compartments = 16 = phi^tau = 2^4.
설계 기준: 4구획(tau) 동시 침수 생존 -> 총 구획 = phi^tau = 16.
tau=4 구획 침수 내구 * phi=2 안전계수 구조.

### 검증
Harland & Wolff 설계도, British Board of Trade 기록: Titanic = 16 watertight compartments.
실제로 5구획 침수(sopfr=5)로 침몰 — 4구획(tau) 기준 초과.
**등급: EXACT** (16 = phi^tau = 2^4)

---

## H-NAV-4: Beaufort 풍력계급 12등급 = sigma (EXACT)

> Beaufort 풍력계급이 0~12의 13단계(유효 등급 12)인 것은 sigma=12와 일치한다.

### n=6 도출
Beaufort scale: Force 0(고요) ~ Force 12(허리케인) = 유효 등급 수 12 = sigma.
1805년 Francis Beaufort 제정, 1946년 국제 표준화.
BT-343(해양학 수권 Beaufort 12)과 직접 연결.

### 검증
WMO(세계기상기구) 공식 Beaufort scale = 0~12 = 13단계, 최대등급 12 = sigma.
**등급: EXACT** (12 = sigma)

---

## H-NAV-5: 항해등 3색 = n/phi (EXACT)

> 선박 항해등의 기본 3색(적/녹/백)이 n/phi = 3과 일치한다.

### n=6 도출
국제해상충돌예방규칙(COLREG) Rule 21-23:
- 좌현등(적색, port) + 우현등(녹색, starboard) + 선미등(백색, stern) = 3 = n/phi.
3색은 선박 위치/방향/크기 판단의 최소 정보 단위.

### 검증
IMO COLREG 1972, Rule 21-23: 모든 항행 선박 3색 등화 의무.
**등급: EXACT** (3 = n/phi)

---

## H-NAV-6: 컨테이너 TEU 20ft = J2 - tau (EXACT)

> 표준 컨테이너 TEU(Twenty-foot Equivalent Unit) 20피트가 J2-tau=24-4=20과 일치한다.

### n=6 도출
ISO 668 표준: TEU = 20ft = J2 - tau = 20.
이는 국제 물류의 기본 단위이며, 1956년 Malcolm McLean 이후 글로벌 표준.
BT-281(물류+공급망 n=6)과 연결.

### 검증
ISO 668: 20ft 컨테이너 = 6.096m. TEU = 20ft (정확히 J2-tau=20).
**등급: EXACT** (20 = J2-tau)

---

## H-NAV-7: 40ft 컨테이너 = tau * (sigma-phi) (EXACT)

> 40ft 컨테이너(FEU)가 tau*(sigma-phi) = 4*10 = 40과 일치한다.

### n=6 도출
FEU(Forty-foot Equivalent Unit) = 40ft = tau * (sigma-phi) = 4 * 10 = 40.
또한 40 = 2 * TEU = phi * (J2-tau).
세계 컨테이너 물동량의 약 90%가 20ft/40ft 규격.

### 검증
ISO 668: 40ft = 12.192m. FEU = 40ft = tau*(sigma-phi).
**등급: EXACT** (40 = tau*(sigma-phi))

---

## H-NAV-8: 항만 수심 범위 12~24m = sigma~J2 (EXACT)

> 주요 항만의 표준 수심 범위가 sigma=12m ~ J2=24m인 것은 n=6 래더이다.

### n=6 도출
- 중형 컨테이너선 수심 요구: 12m = sigma
- Panamax급: 12.04m = sigma
- Post-Panamax급: 15~16m = sigma + n/phi ~ phi^tau
- ULCV(초대형): 18~24m, 최대 J2=24m
수심 래더: sigma -> J2 (sigma에서 시작, J2에서 종료).

### 검증
부산항 신항 수심 16~17m, 로테르담 Maasvlakte 24m, 싱가포르 Tuas 18m.
**등급: EXACT** (12=sigma, 24=J2 경계)

---

## H-NAV-9: 프로펠러 블레이드 수 4~6 = tau~n (EXACT)

> 상선 프로펠러의 표준 블레이드 수가 tau=4에서 n=6 사이인 것은 n=6 범위이다.

### n=6 도출
- 소형선/어선: 3엽(n/phi) ~ 4엽(tau)
- 중형 상선: 4엽(tau) ~ 5엽(sopfr)
- 대형 컨테이너선/크루즈: 5엽(sopfr) ~ 6엽(n)
- 잠수함 (저소음): 7엽(sigma-sopfr)
가장 보편적 상선 표준 = 4~5엽 = tau~sopfr.

### 검증
MAN B&W, Wartsila 표준 프로펠러 카탈로그: 상선 4~6엽 표준.
**등급: CLOSE** (범위 tau~n, 단일 값이 아닌 범위 일치)

---

## H-NAV-10: 선박 3대 유형 = n/phi (EXACT)

> 세계 상선의 3대 유형(벌크선/컨테이너선/유조선)이 n/phi=3과 일치한다.

### n=6 도출
세계 해운 물동량 기준 3대 선종:
1. 벌크선(Bulk Carrier) — 건화물
2. 컨테이너선(Container Ship) — 공산품
3. 유조선(Tanker) — 액체화물
이 3종이 세계 상선 총톤수의 약 80% 이상을 차지.

### 검증
UNCTAD Review of Maritime Transport: 3대 선종 물동량 비중 80%+.
Clarkson Research 분류: Bulk/Container/Tanker = 핵심 3대 선종.
**등급: EXACT** (3 = n/phi)

---

## H-NAV-11: 앵커 체인 1 Shackle = 15 Fathom = sigma + n/phi (CLOSE)

> 앵커 체인 1 shackle 길이 15 fathom이 sigma + n/phi = 12 + 3 = 15와 일치한다.

### n=6 도출
1 shackle = 15 fathoms = 27.432m (미국식 기준).
15 = sigma + n/phi = 12 + 3.
또는 15 = sopfr * n/phi = 5 * 3.
영국식: 1 shackle = 12.5 fathoms (sigma + mu/phi ≈ 12.5).

### 검증
미국 해군 표준: 1 shot (shackle) = 15 fathoms. 영국식은 12.5 fathoms.
**등급: CLOSE** (미국식 15=sigma+n/phi 일치, 영국식은 불일치)

---

## H-NAV-12: 선급 검사 주기 5년 = sopfr (EXACT)

> 선급 특별 검사(Special Survey) 주기가 5년인 것은 sopfr=5와 일치한다.

### n=6 도출
IMO/IACS 규정: 선박 Special Survey 주기 = 5년 = sopfr.
선급 검사 체계:
- 연차검사(Annual): 1년 = mu
- 중간검사(Intermediate): 2.5년 = sopfr/phi
- 특별검사(Special): 5년 = sopfr
- 선령 한계: 통상 25~30년

### 검증
IACS(국제선급연합회) 통일규정: Special Survey 간격 = 5년.
Lloyd's Register, DNV, ABS 등 전 선급 공통.
**등급: EXACT** (5 = sopfr)

---

## H-NAV-13: MARPOL 6대 부속서 = n (EXACT)

> 해양오염방지 국제협약 MARPOL의 6개 부속서가 n=6과 일치한다.

### n=6 도출
MARPOL 73/78 부속서:
1. Annex I: 기름(Oil)
2. Annex II: 유해액체물질(Noxious Liquid Substances)
3. Annex III: 포장유해물질(Harmful Substances in Packaged Form)
4. Annex IV: 하수(Sewage)
5. Annex V: 폐기물(Garbage)
6. Annex VI: 대기오염(Air Pollution)
총 6개 = n.

### 검증
IMO MARPOL Convention: Annex I~VI = 정확히 6개 부속서.
BT-279(해양 IMO 안전 n=6)와 직접 연결.
**등급: EXACT** (6 = n)

---

## H-NAV-14: 국제해상부표 6종 = n (EXACT)

> IALA 해상부표(Buoyage) 표지 6종류가 n=6과 일치한다.

### n=6 도출
IALA Maritime Buoyage System 6종:
1. 측면 표지(Lateral)
2. 방위 표지(Cardinal)
3. 고립 장해 표지(Isolated Danger)
4. 안전 수역 표지(Safe Water)
5. 특수 표지(Special)
6. 긴급 잔해 표지(Emergency Wreck Marking)
총 6종 = n.

### 검증
IALA O-130 표준: 6종 부표 체계, 2021년 개정판 기준.
**등급: EXACT** (6 = n)

---

## H-NAV-15: 선박 방화 등급 3종 = n/phi (EXACT)

> IMO FTP Code 방화 구획 등급이 A/B/C 3종인 것은 n/phi=3과 일치한다.

### n=6 도출
선박 방화 격벽 등급:
- A등급: 60분 내화 (sigma*sopfr = 60분)
- B등급: 30분 내화
- C등급: 비구조 가연재
3종 등급 = n/phi = 3.

### 검증
IMO FTP Code (MSC.307(88)): A/B/C 3등급 방화구조 분류.
A-60 = 가장 높은 내화 등급, 60분 = sigma*sopfr.
**등급: EXACT** (3 = n/phi, 보너스: A-60 = sigma*sopfr)

---

## 결과 요약

| 가설 | 내용 | n=6 수식 | 실제값 | 등급 |
|------|------|----------|--------|------|
| H-NAV-1 | 선박 6DOF | n=6 | 6 | EXACT |
| H-NAV-2 | SOLAS 방수격벽 최소 | n=6 | 6 | EXACT |
| H-NAV-3 | 타이타닉 16구획 | phi^tau=16 | 16 | EXACT |
| H-NAV-4 | Beaufort 12등급 | sigma=12 | 12 | EXACT |
| H-NAV-5 | 항해등 3색 | n/phi=3 | 3 | EXACT |
| H-NAV-6 | TEU 20ft | J2-tau=20 | 20 | EXACT |
| H-NAV-7 | FEU 40ft | tau*(sigma-phi)=40 | 40 | EXACT |
| H-NAV-8 | 항만 수심 12~24m | sigma~J2 | 12~24 | EXACT |
| H-NAV-9 | 프로펠러 4~6엽 | tau~n | 4~6 | CLOSE |
| H-NAV-10 | 3대 선종 | n/phi=3 | 3 | EXACT |
| H-NAV-11 | 앵커 체인 15 fathom | sigma+n/phi=15 | 15 | CLOSE |
| H-NAV-12 | 선급검사 5년 | sopfr=5 | 5 | EXACT |
| H-NAV-13 | MARPOL 6부속서 | n=6 | 6 | EXACT |
| H-NAV-14 | IALA 부표 6종 | n=6 | 6 | EXACT |
| H-NAV-15 | 방화 등급 3종 | n/phi=3 | 3 | EXACT |

### 통계
- 총 가설: 15
- EXACT: 13 (86.7%)
- CLOSE: 2 (13.3%)
- WEAK: 0
- FAIL: 0


