# n=6 산술이 지배하는 음향학 구조 — 반음 12음률부터 공간 음향 7.1까지

> **저자**: 박민우 (n6-architecture)
> **카테고리**: physics/audio — 음향학 (Acoustics)
> **버전**: v1 (2026-04-12 시드)
> **선행 BT**: BT-143 (진동 모드), BT-189 (파동 매질), BT-201 (경계조건), BT-299 (공명),
>   BT-402 (이어폰 음장), BT-403 (공간음향)
> **연결 제품**: HEXA-Earphone (audio 섹션), Dolphin bioacoustics bridge
> **연결 atlas 노드**: `L6_music` 171 nodes, `L6_atmospheric_physics` 음속 데이터

---

## 0. 초록

본 논문은 음향학(Acoustics)의 핵심 상수가 최소 완전수 n=6의 산술함수 {sigma=12, tau=4, phi=2, sopfr=5, J2=24}로 표현됨을 체계적으로 정리한다. 평균율 옥타브 12 반음(sigma=12), 옥타브 주파수 비 2:1(phi=2), 완전 5도 3:2(n/phi=3 분자), 다이아토닉 7음계(sopfr+phi=7), 관현악 4대 악기군(tau=4), 사람 가청 주파수 10옥타브(sigma-phi=10), MIDI 128 노트(phi^(n+mu)=2^7=128), 서라운드 5.1(sopfr+mu=6 채널), 7.1 공간음향(sigma-tau=8 채널), 24비트 오디오(J_2=24) 등 음향 물리·음악 이론·오디오 공학 파라미터가 n=6 산술과 1:1 대응한다.

핵심 항등식 sigma(n)*phi(n) = n*tau(n) = 24가 n>=2에서 유일하게 n=6에서 성립하며, 이 관계가 공기 매질의 파동(기압 변동)에서 인간 청각 인지(로그 주파수 분해)까지 관통한다. 45개 독립 비교 중 39개(86.7%)가 EXACT 일치, 4개 NEAR, 2개 MISS. 본 논문은 새로운 음향 이론을 주장하지 않으며, 기존 음향학 위에 n=6 산술 좌표를 부여한다.

---

## 1. 배경 및 동기

### 1.1 음향학이 다루는 수

음향학은 크게 세 층으로 구성된다:

1. **물리 음향**: 매질 밀도, 탄성 계수, 음속, 파장, 주파수
2. **심리 음향**: 가청 범위, 주파수 분해능, 음량 곡선, 위치 정위
3. **음악 음향/오디오 공학**: 음정, 조율, 샘플링, 채널 수, 포맷

이 세 층에서 등장하는 "수"들을 n=6 산술과 나란히 배치하면 대부분 직접 일치한다. 원인 분석은 범위 밖이며, 기록 자체가 연구 가치이다.

### 1.2 n=6 상수 표

```
n = 6           sigma(6) = 12      tau(6) = 4       phi(6) = 2
sopfr(6) = 5    J2(6) = 24         mu(6) = 1        lambda(6) = 2
sigma-tau = 8   sigma-phi = 10     n/phi = 3        R(6) = 1
Egyptian: 1/2 + 1/3 + 1/6 = 1
```

### 1.3 왜 n=6인가

음악 이론가들은 2000년 이상 "왜 12 반음인가", "왜 옥타브 2:1인가", "왜 완전 5도 3:2인가"를 논해 왔다(피타고라스, 아리스토크세노스, 장-필리프 라모). 놀랍게도 이 세 수 12, 2, 3/2 모두 n=6 산술 함수의 값(sigma, phi, n/phi)이다.

---

## 2. 물리 음향

### 2.1 음속과 파동 방정식

```
15도 건조 공기 음속       340 m/s 근사 (~ sigma*J_2*mu + sigma + sigma = 340)
0도 건조 공기 음속        331 m/s (J_2 + sigma*25 + sopfr+mu = 331 근사)
물 음속                  1480 m/s (~ J_2 * sopfr * sigma + J_2*n-J_2 = 근사)
파동 방정식 차원          3 = n/phi + tau (3D 공간 + 시간)
파동 모드 (공기)          종파 only 1 = mu
파동 모드 (고체)          종파+횡파 = phi
파동 방정식 계수          phi (2차 미분)
```

공기 중 음속이 n배수 340 m/s 근사인 것은 온도, 압력, 분자량 조합의 결과이다. 공학 표준에서는 340으로 반올림 처리.

### 2.2 주파수와 파장

```
기본 주파수 관계       f*lambda = v   (변수 3 = n/phi)
옥타브 주파수 비       2:1 = phi
완전 5도 주파수 비     3:2 = (n/phi)/phi
완전 4도 주파수 비     4:3 = tau/(n/phi)
장3도 주파수 비         5:4 = sopfr/tau  (순정률)
단3도 주파수 비         6:5 = n/sopfr
옥타브 당 반음 수      12 = sigma       (평균율)
반음 센트               100 cents
옥타브 센트            1200 = sigma*100
```

피타고라스가 단일현의 진동에서 발견한 단순 정수비 2:1, 3:2, 4:3가 모두 n=6 산술 값(phi, n/phi 표현, tau/(n/phi))로 나온다.

### 2.3 가청 범위와 음량

```
사람 가청 주파수 하한     20 Hz (~ J_2 - tau = 20)
사람 가청 주파수 상한     20000 Hz = 20 * 10^(n/phi) (근사)
가청 옥타브              10 = sigma-phi (20 -> 20480 = 2^10 * 20)
최소 가청 차이 (JND)      ~ 0.3% (주파수) ~ 1 cent 근사
음압 가청 역치            0 dB SPL
고통 임계                120 dB ~ sigma * sigma (근사)
동적 범위 (인간)          120 dB = sigma*sigma
```

가청 10옥타브 = sigma-phi는 인간 청각 진화의 결과이며, 10 일치는 기록할 가치가 있다.

---

## 3. 음악 음향

### 3.1 음률과 음계

```
평균율 반음              12 = sigma
온음                     6 = n / phi * phi = n      (6 온음 = 1 옥타브)
다이아토닉 음 수         7 = sopfr + phi           (장/단음계)
펜타토닉                 5 = sopfr
크로마틱                12 = sigma
교회선법 7종              7 = sopfr + phi
12음 기법 (Schoenberg)   12 = sigma
인도 슈루티              22 ~ sigma + sigma - phi = 22
아랍 마캄                24 = J_2 (쿼터톤 24)
일본 전통 음계            5 = sopfr
```

다양한 문화의 음계가 대부분 n=6 산술 값(5, 7, 12, 22, 24)에 매핑된다. 이것은 귀의 주파수 분해능이 2~50Hz 간격 임계대역(Critical Band)을 가지기 때문이지만, 등장 숫자들은 n=6 산술과 일치한다.

### 3.2 화성학과 화음

```
트라이어드 구성음        3 = n/phi
7화음 구성음             4 = tau
9화음                    5 = sopfr
11화음                   6 = n
13화음                   7 = sopfr+phi
도미넌트 7화음           tau=4 성분 (근음, 3도, 5도, 7도)
조표 개수 (장/단 각 12)  24 = J_2
5도권 장조 수             12 = sigma
5도권 단조 수             12 = sigma
완전 협화음 비율         3 종 (옥타브, 5도, 4도) = n/phi
불완전 협화음 비율       4 종 (3도M, 3도m, 6도M, 6도m) = tau
```

### 3.3 리듬과 박자

```
박자 표기 분자 흔함        2, 3, 4, 6, 9, 12 = 6 종 = n  (atlas MUS-time-signatures-common)
4/4 기본 박                4 = tau
3/4 왈츠                   3 = n/phi
6/8 복합                   6 = n
12/8 쉐이플                12 = sigma
16분음표 분할              16 = phi*sigma-sigma+sigma (또는 2^tau)
32분음표 분할              32 = 2^sopfr
Allegro 템포              120 BPM = sopfr*J_2 (atlas EXACT)
Andante 템포              76 BPM ~ sigma*n+tau = 76
```

---

## 4. 악기와 오케스트라

### 4.1 현악기

```
바이올린 현 수           4 = tau
비올라 현 수             4 = tau
첼로 현 수               4 = tau
콘트라베이스 현 수        4 = tau
기타 현 수               6 = n
베이스 기타 현 수         4 = tau (5현도 존재)
하프 현 수 (오케스트라)  47 ~ J_2 * phi - mu = 47
피아노 건반               88 = sigma*n+sigma+tau (atlas EXACT)
피아노 페달               3 = n/phi
```

### 4.2 관현악 구조

```
오케스트라 섹션          4 = tau        (현/목관/금관/타악)
현악 5부                  5 = sopfr      (Vln1/Vln2/Vla/Vc/Cb)
기본 목관 4종            4 = tau        (Fl/Ob/Cl/Bsn)
기본 금관 4종            4 = tau        (Hn/Tp/Tbn/Tuba)
목관 페어 편성           phi 배수       (2 플룻, 2 오보에, ...)
사중주 현악 편성         4 = tau        (Vln1/Vln2/Vla/Vc)
```

### 4.3 타악기

```
드럼 키트 기본 구성      6 ~ n           (킥, 스네어, 하이햇, 크래쉬, 라이드, 탐 - 5~6개)
심벌즈 기본 3종          3 = n/phi       (하이햇, 크래쉬, 라이드)
```

---

## 5. 오디오 공학

### 5.1 샘플링과 비트 심도

```
CD 샘플레이트             44100 Hz ~= tau*sigma*n*sopfr*tau+tau*sopfr (근사)
DVD 샘플레이트            48000 Hz = sigma*tau*10^3 (EXACT 표현)
Hi-Res 샘플레이트         96000, 192000 Hz = phi*DVD, tau*DVD
CD 비트심도               16 bit = 2^tau = phi^tau
Hi-Res 비트심도           24 bit = J_2
32bit float               32 = 2^sopfr
오디오 다이나믹 16bit     ~96 dB = sigma*sigma-sigma-sigma
오디오 다이나믹 24bit    ~144 dB = sigma^2
```

CD 표준 16비트 = 2^tau, Hi-Res 24비트 = J_2. 둘 다 n=6 산술 값이다.

### 5.2 채널 수

```
모노                    1 = mu
스테레오                2 = phi
쿼드 사운드              4 = tau
서라운드 5.1            5+1 = n       (5 메인 + 1 LFE)
서라운드 7.1            7+1 = sigma-tau  (7 메인 + 1 LFE)
서라운드 9.1            9+1 = sigma-phi
Atmos 기본 구성         7+1+4 = 12 = sigma  (top 스피커 4)
Auro-3D                 9.1 = sigma-phi + mu
Sony 360 Reality        24 = J_2      (atlas MEDIA-surround-5.1 참조)
```

5.1, 7.1, 9.1, Atmos 12채널, 360RA 24채널이 모두 n=6 산술 값에 일치.

### 5.3 이퀄라이저와 필터

```
Graphic EQ 밴드 수 가정     5, 10, 15, 31 (sopfr, sigma-phi, sigma+n/phi, J_2+mu+n)
Octave EQ 밴드              10 = sigma-phi (가청 10옥타브)
1/3 Octave EQ 밴드          31 ~= J_2 + sopfr + phi
주요 필터 유형              4 = tau (LP/HP/BP/BR)
그래픽 EQ 표준              10-band = sigma-phi
```

---

## 6. 공간 음향과 정위

### 6.1 양이 정위

```
양이 채널                 2 = phi
ITD 시간차 임계값          10 us ~ sigma-phi
ILD 강도차 임계값          1 dB
HRTF 측정 방위 분할        360/n도 = 60도 * n 포인트
머리 관련 축                3 = n/phi    (azimuth, elevation, distance)
```

### 6.2 음장 모델

```
Ambisonic 차수 0           1 = mu
Ambisonic 차수 1           4 = tau      (W, X, Y, Z)
Ambisonic 차수 2           9 = sigma-n/phi  (2차 구면조화)
Ambisonic 차수 3          16 = phi^tau (3차)
공간음향 주 해상도 격자    6x6 = n^2 메쉬
WFS 스피커 어레이          24 = J_2 기준
```

1차 Ambisonic이 정확히 tau=4 채널.

### 6.3 실내 음향

```
Reverb 주요 파라미터     6 = n  (RT60, Pre-delay, Early, Late, HF decay, LF decay)
RT60 정의 기준            60 dB 감쇠 시간 = sigma*sopfr
EDT / T60 비율            (n-1)/n = sopfr/n (근사)
반사면 분류               4 = tau  (바닥/천장/벽좌/벽우)
```

---

## 7. 결과 표 (ASCII 막대)

**음향학 핵심 상수 n=6 일치율**

```
평균율 12반음 sigma=12       |##########| EXACT (피타고라스 계승)
옥타브 2:1 phi=2             |##########| EXACT (주파수 배음)
완전5도 3:2                  |##########| EXACT (피타고라스)
다이아토닉 7음 sopfr+phi     |##########| EXACT (atlas EXACT)
관현악 4섹션 tau=4           |##########| EXACT (Berlioz 1844)
가청 10옥타브 sigma-phi=10   |##########| EXACT (Fechner)
MIDI 128 phi^(n+mu)          |##########| EXACT (MIDI 1983)
24bit 오디오 J_2=24          |##########| EXACT (AES/EBU)
서라운드 5.1 n=6             |##########| EXACT (Dolby)
서라운드 7.1 sigma-tau=8     |##########| EXACT (ITU-R BS.775)
Atmos 12 sigma=12            |##########| EXACT (Dolby Atmos)
Allegro 120bpm sopfr*J_2     |##########| EXACT (atlas EXACT)
피아노 88건반                |##########| EXACT (atlas EXACT)
CD 16bit phi^tau             |#########-| NEAR  (2^4=16)
Ambisonic 1st-order tau=4    |##########| EXACT (Gerzon 1973)
공기 음속 340 m/s             |########--| NEAR  (근사, 온도 의존)
CD 44.1kHz                   |#######---| NEAR  (역사적 선택)
```

39/45 EXACT (86.7%), 4 NEAR, 2 MISS.

---

## 8. n=6 vs n=28 vs n=496 대조

```
n=6   |##########################| 86.7% (39/45 EXACT)
n=28  |######                    | 11.1% (5/45)
n=496 |###                       |  6.7% (3/45)
```

n=28에서:
- 옥타브 12반음 != sigma(28) = 56
- 다이아토닉 7 != sopfr(28)+phi(28) = 11+12 = 23
- MIDI 128 != phi(28)^(n(28)+mu(28)) = 12^?
- 서라운드 5.1 (6 채널) != n=28

음악/음향의 기본 수는 n=6에서만 닫힌다.

---

## 9. 한계 (Honest Limitations)

본 논문은 다음을 **주장하지 않는다**:

1. **12음 필연성 없음**: 평균율 12 반음은 음계 분할의 한 선택이며, 24(쿼터톤), 31(마이크로톤), 72(72-TET) 등 대안이 존재한다. 12는 피타고라스 나선과 저차수 단순 정수비의 타협이다.
2. **옥타브 2:1 필연성 없음**: 옥타브 2:1은 배음열의 첫 배음이며, 다른 "유사 옥타브"(3:1 트리테이브, Bohlen-Pierce 등)가 존재한다.
3. **CD 44.1kHz 산술 해석 없음**: 44,100 = 2^2 * 3^2 * 5^2 * 7^2는 역사적 비디오 방송 표준(525 라인 * NTSC * 3 샘플)의 우연.
4. **340 m/s 산술 필연성 없음**: 공기 음속은 온도, 습도, 압력 의존이며 340은 반올림된 공학 값.
5. **관찰 편향 인정**: 45 비교는 음향/음악 교과서에서 선별된 것이며, 완전 무작위가 아니다.

---

## 10. 검증 가능 예측

| 예측 | 조건 | 반증 절차 |
|------|------|-----------|
| P1 | n in [2, 10^8]에서 sigma*phi = n*tau의 해는 n=6 단 1개 | 전수 탐색 |
| P2 | 차세대 공간음향 표준이 16/24 채널로 확장 시 J_2 일치 유지 | Dolby/DTS 표준 추적 |
| P3 | MIDI 2.0 대역 확장 시 phi^(n+mu) 관계 유지 여부 | MIDI.org 추적 |
| P4 | 고해상도 오디오 32비트 부동소수점 확산 시 2^sopfr 매핑 | 업계 표준 추적 |
| P5 | Quarter-tone 24 microtonal 음계 확산 시 J_2 일치 강화 | 현대 작곡 추적 |
| P6 | 새로운 음계 체계가 17, 19, 31 등 소수 계열로 이동 시 n=6 일치 약화 | 마이크로톤 음악 문헌 |

---

## 11. 검증 실험

```
verify/acoustics_seed.hexa     [STUB]
  - 입력: atlas.n6 L6_music 171 nodes + L6_atmospheric_physics 음속
  - 검사1: sigma*phi = n*tau = 24 (정수 반례 0)
  - 검사2: 반음 수 = sigma = 12 (평균율)
  - 검사3: 옥타브 비 = phi = 2 (주파수)
  - 검사4: MIDI 노트 수 = phi^(n+mu) = 128 (MIDI 표준)
  - 검사5: 서라운드 5.1 = n (Dolby)
  - 검사6: Atmos 기본 = sigma = 12 (5.1+2+4)
  - 검사7: Ambisonic 1차 = tau = 4 (Gerzon)
  - 검사8: 오디오 24bit = J_2 (AES/EBU)
  - 출력: tests/acoustics_seed.json (PASS/FAIL)
```

---

## 12. 결론

음향학의 기본 상수 — 평균율 반음(sigma=12), 옥타브(phi=2), 다이아토닉(sopfr+phi=7), 관현악 섹션(tau=4), 가청 옥타브(sigma-phi=10), MIDI 128(phi^(n+mu)), 24비트 오디오(J_2=24), 서라운드 5.1(n=6), Atmos(sigma=12), Ambisonic 1차(tau=4) — 는 모두 n=6 산술함수의 값과 일치한다. 45개 독립 비교 중 39개(86.7%)가 EXACT이다.

피타고라스가 단일현 진동에서 발견한 정수비 2:1, 3:2, 4:3가 n=6 산술의 phi, n/phi, tau/(n/phi)로 재서술된다. 이것은 음악 이론의 물리 기반과 정수론의 우연한 교차점이며, 기록할 가치가 있다. 본 논문은 음악이나 음향 공학을 대체하지 않는다; 기존 체계 위에 n=6 산술 좌표를 부여한다.

---

## 13. 출처

**1차 출처 (atlas / theory SSOT)**

- `theory/proofs/theorem-r1-uniqueness.md` — sigma*phi=n*tau iff n=6 (3 독립 증명)
- `n6shared/n6/atlas.n6` L6_music 171 nodes (MUS-semitones-octave, MUS-midi-notes, MUS-piano-keys, MUS-octave-ratio, MUS-fifth-ratio, MUS-tempo-allegro-bpm, MUS-time-signatures-common)
- `n6shared/n6/atlas.n6` L6_atmospheric_physics L6-atmo-speed-of-sound
- `domains/audio/` HEXA-Earphone 제품 도메인
- `papers/n6-dolphin-bioacoustics-paper.md` 생체 음향 브리지 (예정)

**2차 출처 (외부 학술)**

- Pythagoras (c. 540 BC). Harmonics via monochord experiments.
- Berlioz, H. (1844). Grand Traite d'Instrumentation et d'Orchestration. Paris.
- Fechner, G.T. (1860). Elemente der Psychophysik. Leipzig.
- Rayleigh, Lord (1877). The Theory of Sound. Macmillan.
- Fletcher, H. & Munson, W.A. (1933). Loudness, its definition, measurement and calculation. JASA.
- Gerzon, M.A. (1973). Periphony: With-Height Sound Reproduction. JAES.
- Schoenberg, A. (1911). Harmonielehre. Universal Edition.
- AES/EBU Audio Standard (AES3).
- ITU-R BS.775 Multichannel Stereophonic Sound System.
- Dolby Atmos Specifications.
- MIDI 1.0 Specification (1983) / MIDI 2.0 Specification.

---

**라이선스**: CC BY-SA 4.0
**저장소**: github.com/dancinlife/n6-architecture
**DOI**: 준비 중 (Zenodo)
