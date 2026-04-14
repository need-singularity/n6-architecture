# products.json 링크 무결성 감사 리포트

- 생성일: 2026-04-11
- 원본: `$NEXUS/shared/n6/docs/products.json`
- 기준 경로: `$N6_ARCH/` (R1 감사 예외 .md)
- 특수 규칙: `nexus/shared/n6/scripts/...` 접두사는 `$NEXUS/shared/n6/scripts/` 로 해석

## 총괄

- 총 섹션: 34
- 총 제품: 173
- 총 검사 항목: 416
- PASS: 3
- MISS: 413
- 완성도: 0.7%

### 해석자(resolver) 분포

| 해석자 | 개수 | 비고 |
|---|---:|---|
| N6A | 416 | `$N6_ARCH/<path>` 기준 |
| NEXUS_SCRIPTS | 0 | `nexus/shared/n6/scripts/` 접두사 |
| ABS | 0 | 절대경로 |
| EMPTY | 0 | 빈 문자열 |

## 섹션별 집계 (MISS 내림차순)

| 섹션 id | 제목 | 제품수 | 검사항목 | PASS | MISS | 완성도 |
|---|---|---:|---:|---:|---:|---:|
| frontier | 최전선/대발견 | 39 | 110 | 0 | 110 | 0.0% |
| tech-industry | 기술/산업 | 22 | 50 | 3 | 47 | 6.0% |
| chip | 칩/반도체 | 5 | 24 | 0 | 24 | 0.0% |
| physics | 물리/수학 | 5 | 18 | 0 | 18 | 0.0% |
| environment | 환경보호 | 6 | 17 | 0 | 17 | 0.0% |
| software | 소프트웨어/인프라 | 5 | 17 | 0 | 17 | 0.0% |
| audio | 오디오 | 7 | 16 | 0 | 16 | 0.0% |
| civilization | 문명/인문 | 7 | 15 | 0 | 15 | 0.0% |
| life-culture | 생활/문화 | 9 | 15 | 0 | 15 | 0.0% |
| materials | 물질합성 | 6 | 15 | 0 | 15 | 0.0% |
| energy | 에너지 | 5 | 14 | 0 | 14 | 0.0% |
| fusion | 핵융합 | 5 | 13 | 0 | 13 | 0.0% |
| ai | AI/ML | 9 | 12 | 0 | 12 | 0.0% |
| play | 유희 | 2 | 9 | 0 | 9 | 0.0% |
| robotics | 로봇 | 2 | 8 | 0 | 8 | 0.0% |
| aerospace | 우주항공 | 1 | 7 | 0 | 7 | 0.0% |
| safety | 안전 | 2 | 7 | 0 | 7 | 0.0% |
| cognitive-social | 인지/사회 | 6 | 6 | 0 | 6 | 0.0% |
| display | 디스플레이 | 2 | 6 | 0 | 6 | 0.0% |
| hiv-treatment | HIV 치료 | 1 | 6 | 0 | 6 | 0.0% |
| virology | 바이러스학 | 4 | 6 | 0 | 6 | 0.0% |
| marketing | 마케팅 | 4 | 4 | 0 | 4 | 0.0% |
| natural-science | 자연과학 | 4 | 4 | 0 | 4 | 0.0% |
| digital-medical | 디지털/의료기기 | 3 | 3 | 0 | 3 | 0.0% |
| hygiene | 위생 | 2 | 2 | 0 | 2 | 0.0% |
| manufacturing-quality | 제조 품질관리 | 1 | 2 | 0 | 2 | 0.0% |
| mobility | 이동/수송 | 2 | 2 | 0 | 2 | 0.0% |
| sf | UFO/비행접시 | 1 | 2 | 0 | 2 | 0.0% |
| horology | 시계학 | 1 | 1 | 0 | 1 | 0.0% |
| keyboard | 키보드 | 1 | 1 | 0 | 1 | 0.0% |
| mouse | 마우스 | 1 | 1 | 0 | 1 | 0.0% |
| network | 네트워크 | 1 | 1 | 0 | 1 | 0.0% |
| quantum-computer | 양자컴퓨터 | 1 | 1 | 0 | 1 | 0.0% |
| tattoo-removal | 타투 제거 | 1 | 1 | 0 | 1 | 0.0% |

## ASCII 막대 차트 (섹션별 완성도)

```
섹션 id                    완성도  [              막대                ]  PASS/TOTAL
------------------------------------------------------------------------------------------
frontier                     0.0%  [....................................]    0/110
tech-industry                6.0%  [##..................................]    3/ 50
chip                         0.0%  [....................................]    0/ 24
physics                      0.0%  [....................................]    0/ 18
environment                  0.0%  [....................................]    0/ 17
software                     0.0%  [....................................]    0/ 17
audio                        0.0%  [....................................]    0/ 16
civilization                 0.0%  [....................................]    0/ 15
life-culture                 0.0%  [....................................]    0/ 15
materials                    0.0%  [....................................]    0/ 15
energy                       0.0%  [....................................]    0/ 14
fusion                       0.0%  [....................................]    0/ 13
ai                           0.0%  [....................................]    0/ 12
play                         0.0%  [....................................]    0/  9
robotics                     0.0%  [....................................]    0/  8
aerospace                    0.0%  [....................................]    0/  7
safety                       0.0%  [....................................]    0/  7
cognitive-social             0.0%  [....................................]    0/  6
display                      0.0%  [....................................]    0/  6
hiv-treatment                0.0%  [....................................]    0/  6
virology                     0.0%  [....................................]    0/  6
marketing                    0.0%  [....................................]    0/  4
natural-science              0.0%  [....................................]    0/  4
digital-medical              0.0%  [....................................]    0/  3
hygiene                      0.0%  [....................................]    0/  2
manufacturing-quality        0.0%  [....................................]    0/  2
mobility                     0.0%  [....................................]    0/  2
sf                           0.0%  [....................................]    0/  2
horology                     0.0%  [....................................]    0/  1
keyboard                     0.0%  [....................................]    0/  1
mouse                        0.0%  [....................................]    0/  1
network                      0.0%  [....................................]    0/  1
quantum-computer             0.0%  [....................................]    0/  1
tattoo-removal               0.0%  [....................................]    0/  1
------------------------------------------------------------------------------------------
TOTAL                        0.7%  [....................................]    3/416
```

## 섹션별 MISS 상세

### frontier — 최전선/대발견 (MISS 110/110)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| HEXA-NEURO 뇌-기계 인터페이스 | link[문서] | `docs/neuro/goal.md` | `$N6_ARCH/docs/neuro/goal.md` | N6A |
| HEXA-NEURO 뇌-기계 인터페이스 | link[논문] | `docs/paper/n6-biology-medical-paper.md` | `$N6_ARCH/docs/paper/n6-biology-medical-paper.md` | N6A |
| HEXA-NEURO 뇌-기계 인터페이스 | link[논문] | `docs/paper/n6-hexa-neuro-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-neuro-paper.md` | N6A |
| HEXA-NEURO 뇌-기계 인터페이스 | verify_script | `docs/neuro/verify_alien10.py` | `$N6_ARCH/docs/neuro/verify_alien10.py` | N6A |
| HEXA-GRAV 중력파 검출/통신 | link[문서] | `docs/gravity-wave/goal.md` | `$N6_ARCH/docs/gravity-wave/goal.md` | N6A |
| HEXA-GRAV 중력파 검출/통신 | link[논문] | `docs/paper/n6-hexa-grav-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-grav-paper.md` | N6A |
| HEXA-GRAV 중력파 검출/통신 | verify_script | `docs/gravity-wave/verify_alien10.py` | `$N6_ARCH/docs/gravity-wave/verify_alien10.py` | N6A |
| HEXA-CLOAK 투명망토/스텔스 | link[문서] | `docs/cloak/goal.md` | `$N6_ARCH/docs/cloak/goal.md` | N6A |
| HEXA-CLOAK 투명망토/스텔스 | link[논문] | `docs/paper/n6-hexa-cloak-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-cloak-paper.md` | N6A |
| HEXA-CLOAK 투명망토/스텔스 | verify_script | `docs/cloak/verify_alien10.py` | `$N6_ARCH/docs/cloak/verify_alien10.py` | N6A |
| HEXA-DEFENSE 지구방어 시스템 | link[문서] | `docs/earth-defense/goal.md` | `$N6_ARCH/docs/earth-defense/goal.md` | N6A |
| HEXA-DEFENSE 지구방어 시스템 | link[논문] | `docs/paper/n6-hexa-defense-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-defense-paper.md` | N6A |
| HEXA-DEFENSE 지구방어 시스템 | verify_script | `docs/earth-defense/verify_alien10.py` | `$N6_ARCH/docs/earth-defense/verify_alien10.py` | N6A |
| HEXA-TELEPORT 양자얽힘 통신망 | link[문서] | `docs/quantum-network/goal.md` | `$N6_ARCH/docs/quantum-network/goal.md` | N6A |
| HEXA-TELEPORT 양자얽힘 통신망 | link[논문] | `docs/paper/n6-hexa-teleport-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-teleport-paper.md` | N6A |
| HEXA-TELEPORT 양자얽힘 통신망 | verify_script | `docs/quantum-network/verify_alien10.py` | `$N6_ARCH/docs/quantum-network/verify_alien10.py` | N6A |
| HEXA-HOVER 개인 호버보드 | link[문서] | `docs/hover/goal.md` | `$N6_ARCH/docs/hover/goal.md` | N6A |
| HEXA-HOVER 개인 호버보드 | link[논문] | `docs/paper/n6-hexa-hover-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-hover-paper.md` | N6A |
| HEXA-HOVER 개인 호버보드 | verify_script | `docs/hover/verify_alien10.py` | `$N6_ARCH/docs/hover/verify_alien10.py` | N6A |
| HEXA-MRAM 초전도 비휘발 메모리 | link[문서] | `docs/sc-memory/goal.md` | `$N6_ARCH/docs/sc-memory/goal.md` | N6A |
| HEXA-MRAM 초전도 비휘발 메모리 | link[논문] | `docs/paper/n6-hexa-mram-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-mram-paper.md` | N6A |
| HEXA-MRAM 초전도 비휘발 메모리 | verify_script | `docs/sc-memory/verify_alien10.py` | `$N6_ARCH/docs/sc-memory/verify_alien10.py` | N6A |
| HEXA-SEABED 대륙간 해저 송전 | link[문서] | `docs/seabed-grid/goal.md` | `$N6_ARCH/docs/seabed-grid/goal.md` | N6A |
| HEXA-SEABED 대륙간 해저 송전 | link[논문] | `docs/paper/n6-seabed-grid-paper.md` | `$N6_ARCH/docs/paper/n6-seabed-grid-paper.md` | N6A |
| HEXA-SEABED 대륙간 해저 송전 | verify_script | `docs/seabed-grid/verify_alien10.py` | `$N6_ARCH/docs/seabed-grid/verify_alien10.py` | N6A |
| HEXA-ACCEL 소형 입자가속기 | link[문서] | `docs/mini-accelerator/goal.md` | `$N6_ARCH/docs/mini-accelerator/goal.md` | N6A |
| HEXA-ACCEL 소형 입자가속기 | link[논문] | `docs/paper/n6-hexa-accel-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-accel-paper.md` | N6A |
| HEXA-ACCEL 소형 입자가속기 | verify_script | `docs/mini-accelerator/verify_alien10.py` | `$N6_ARCH/docs/mini-accelerator/verify_alien10.py` | N6A |
| HEXA-WEATHER 대기 전자기 제어 | link[문서] | `docs/weather-control/goal.md` | `$N6_ARCH/docs/weather-control/goal.md` | N6A |
| HEXA-WEATHER 대기 전자기 제어 | link[논문] | `docs/paper/n6-hexa-weather-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-weather-paper.md` | N6A |
| HEXA-WEATHER 대기 전자기 제어 | verify_script | `docs/weather-control/verify_alien10.py` | `$N6_ARCH/docs/weather-control/verify_alien10.py` | N6A |
| HEXA-MIND 의식 업로드 | link[문서] | `docs/mind-upload/goal.md` | `$N6_ARCH/docs/mind-upload/goal.md` | N6A |
| HEXA-MIND 의식 업로드 | link[논문] | `docs/paper/n6-hexa-mind-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-mind-paper.md` | N6A |
| HEXA-MIND 의식 업로드 | verify_script | `docs/mind-upload/verify_alien10.py` | `$N6_ARCH/docs/mind-upload/verify_alien10.py` | N6A |
| HEXA-TELEPATHY 뇌-뇌 직접통신 | link[문서] | `docs/telepathy/goal.md` | `$N6_ARCH/docs/telepathy/goal.md` | N6A |
| HEXA-TELEPATHY 뇌-뇌 직접통신 | link[논문] | `docs/paper/n6-hexa-telepathy-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-telepathy-paper.md` | N6A |
| HEXA-TELEPATHY 뇌-뇌 직접통신 | verify_script | `docs/telepathy/verify_alien10.py` | `$N6_ARCH/docs/telepathy/verify_alien10.py` | N6A |
| HEXA-HOLO 홀로그래픽 디스플레이 | link[문서] | `docs/holography/goal.md` | `$N6_ARCH/docs/holography/goal.md` | N6A |
| HEXA-HOLO 홀로그래픽 디스플레이 | link[논문] | `docs/paper/n6-hexa-holo-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-holo-paper.md` | N6A |
| HEXA-HOLO 홀로그래픽 디스플레이 | verify_script | `docs/holography/verify_alien10.py` | `$N6_ARCH/docs/holography/verify_alien10.py` | N6A |
| HEXA-DREAM 꿈 기록/재생 | link[문서] | `docs/dream-recorder/goal.md` | `$N6_ARCH/docs/dream-recorder/goal.md` | N6A |
| HEXA-DREAM 꿈 기록/재생 | link[논문] | `docs/paper/n6-hexa-dream-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-dream-paper.md` | N6A |
| HEXA-DREAM 꿈 기록/재생 | verify_script | `docs/dream-recorder/verify_alien10.py` | `$N6_ARCH/docs/dream-recorder/verify_alien10.py` | N6A |
| HEXA-SKYWAY 공중 고속도로망 | link[문서] | `docs/skyway/goal.md` | `$N6_ARCH/docs/skyway/goal.md` | N6A |
| HEXA-SKYWAY 공중 고속도로망 | link[논문] | `docs/paper/n6-hexa-skyway-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-skyway-paper.md` | N6A |
| HEXA-SKYWAY 공중 고속도로망 | verify_script | `docs/skyway/verify_alien10.py` | `$N6_ARCH/docs/skyway/verify_alien10.py` | N6A |
| HEXA-TSUNAMI 해일 방지기 | link[문서] | `docs/tsunami-shield/goal.md` | `$N6_ARCH/docs/tsunami-shield/goal.md` | N6A |
| HEXA-TSUNAMI 해일 방지기 | link[논문] | `docs/paper/n6-hexa-tsunami-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-tsunami-paper.md` | N6A |
| HEXA-TSUNAMI 해일 방지기 | verify_script | `docs/tsunami-shield/verify_alien10.py` | `$N6_ARCH/docs/tsunami-shield/verify_alien10.py` | N6A |
| HEXA-ANTIMATTER 반물질 공장 | link[문서] | `docs/antimatter-factory/goal.md` | `$N6_ARCH/docs/antimatter-factory/goal.md` | N6A |
| HEXA-ANTIMATTER 반물질 공장 | link[논문] | `docs/paper/n6-antimatter-factory-paper.md` | `$N6_ARCH/docs/paper/n6-antimatter-factory-paper.md` | N6A |
| HEXA-ANTIMATTER 반물질 공장 | verify_script | `docs/antimatter-factory/verify_alien10.py` | `$N6_ARCH/docs/antimatter-factory/verify_alien10.py` | N6A |
| HEXA-COSMIC 초기우주 관측망 | link[문서] | `docs/cosmic-observatory/goal.md` | `$N6_ARCH/docs/cosmic-observatory/goal.md` | N6A |
| HEXA-COSMIC 초기우주 관측망 | link[논문] | `docs/paper/n6-hexa-cosmic-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-cosmic-paper.md` | N6A |
| HEXA-COSMIC 초기우주 관측망 | verify_script | `docs/cosmic-observatory/verify_alien10.py` | `$N6_ARCH/docs/cosmic-observatory/verify_alien10.py` | N6A |
| HEXA-DESAL 초전도 담수화 | link[문서] | `docs/desalination/goal.md` | `$N6_ARCH/docs/desalination/goal.md` | N6A |
| HEXA-DESAL 초전도 담수화 | link[논문] | `docs/paper/n6-desal-paper.md` | `$N6_ARCH/docs/paper/n6-desal-paper.md` | N6A |
| HEXA-DESAL 초전도 담수화 | verify_script | `docs/desalination/verify_alien10.py` | `$N6_ARCH/docs/desalination/verify_alien10.py` | N6A |
| HEXA-ORACLE 양자 예측기 | link[문서] | `docs/quantum-oracle/goal.md` | `$N6_ARCH/docs/quantum-oracle/goal.md` | N6A |
| HEXA-ORACLE 양자 예측기 | link[논문] | `docs/paper/n6-hexa-oracle-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-oracle-paper.md` | N6A |
| HEXA-ORACLE 양자 예측기 | verify_script | `docs/quantum-oracle/verify_alien10.py` | `$N6_ARCH/docs/quantum-oracle/verify_alien10.py` | N6A |
| HEXA-ONE 통합 웨어러블 | link[문서] | `docs/hexa-one/goal.md` | `$N6_ARCH/docs/hexa-one/goal.md` | N6A |
| HEXA-ONE 통합 웨어러블 | link[논문] | `docs/paper/n6-hexa-one-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-one-paper.md` | N6A |
| HEXA-ONE 통합 웨어러블 | verify_script | `docs/hexa-one/verify_alien10.py` | `$N6_ARCH/docs/hexa-one/verify_alien10.py` | N6A |
| HEXA-GLASS AI 안경 | link[문서] | `docs/hexa-glass/goal.md` | `$N6_ARCH/docs/hexa-glass/goal.md` | N6A |
| HEXA-GLASS AI 안경 | link[논문] | `docs/paper/n6-hexa-glass-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-glass-paper.md` | N6A |
| HEXA-GLASS AI 안경 | verify_script | `docs/hexa-glass/verify_alien10.py` | `$N6_ARCH/docs/hexa-glass/verify_alien10.py` | N6A |
| HEXA-EAR AI 이어폰 | link[문서] | `docs/hexa-ear/goal.md` | `$N6_ARCH/docs/hexa-ear/goal.md` | N6A |
| HEXA-EAR AI 이어폰 | verify_script | `docs/hexa-ear/verify_alien10.py` | `$N6_ARCH/docs/hexa-ear/verify_alien10.py` | N6A |
| HEXA-EXO AI 외골격 | link[문서] | `docs/hexa-exo/goal.md` | `$N6_ARCH/docs/hexa-exo/goal.md` | N6A |
| HEXA-EXO AI 외골격 | link[논문] | `docs/paper/n6-hexa-exo-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-exo-paper.md` | N6A |
| HEXA-EXO AI 외골격 | verify_script | `docs/hexa-exo/verify_alien10.py` | `$N6_ARCH/docs/hexa-exo/verify_alien10.py` | N6A |
| HEXA-LIMB AI 의수/의족 | link[문서] | `docs/hexa-limb/goal.md` | `$N6_ARCH/docs/hexa-limb/goal.md` | N6A |
| HEXA-LIMB AI 의수/의족 | link[논문] | `docs/paper/n6-hexa-limb-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-limb-paper.md` | N6A |
| HEXA-LIMB AI 의수/의족 | verify_script | `docs/hexa-limb/verify_alien10.py` | `$N6_ARCH/docs/hexa-limb/verify_alien10.py` | N6A |
| HEXA-SKIN 전자 피부 | link[문서] | `docs/hexa-skin/goal.md` | `$N6_ARCH/docs/hexa-skin/goal.md` | N6A |
| HEXA-SKIN 전자 피부 | link[논문] | `docs/paper/n6-hexa-skin-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-skin-paper.md` | N6A |
| HEXA-SKIN 전자 피부 | verify_script | `docs/hexa-skin/verify_alien10.py` | `$N6_ARCH/docs/hexa-skin/verify_alien10.py` | N6A |
| HEXA-FABRIC AI 의류 | link[문서] | `docs/hexa-fabric/goal.md` | `$N6_ARCH/docs/hexa-fabric/goal.md` | N6A |
| HEXA-FABRIC AI 의류 | link[논문] | `docs/paper/n6-hexa-fabric-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-fabric-paper.md` | N6A |
| HEXA-FABRIC AI 의류 | verify_script | `docs/hexa-fabric/verify_alien10.py` | `$N6_ARCH/docs/hexa-fabric/verify_alien10.py` | N6A |
| HEXA-OLFACT 디지털 후각 | link[문서] | `docs/hexa-olfact/goal.md` | `$N6_ARCH/docs/hexa-olfact/goal.md` | N6A |
| HEXA-OLFACT 디지털 후각 | link[논문] | `docs/paper/n6-hexa-olfact-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-olfact-paper.md` | N6A |
| HEXA-OLFACT 디지털 후각 | verify_script | `docs/hexa-olfact/verify_alien10.py` | `$N6_ARCH/docs/hexa-olfact/verify_alien10.py` | N6A |
| HEXA-DREAM 꿈 인터페이스 | link[문서] | `docs/hexa-dream/goal.md` | `$N6_ARCH/docs/hexa-dream/goal.md` | N6A |
| HEXA-DREAM 꿈 인터페이스 | verify_script | `docs/hexa-dream/verify_alien10.py` | `$N6_ARCH/docs/hexa-dream/verify_alien10.py` | N6A |
| HEXA-EMPATH 감정 공유 | link[문서] | `docs/hexa-empath/goal.md` | `$N6_ARCH/docs/hexa-empath/goal.md` | N6A |
| HEXA-EMPATH 감정 공유 | link[논문] | `docs/paper/n6-hexa-empath-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-empath-paper.md` | N6A |
| HEXA-EMPATH 감정 공유 | verify_script | `docs/hexa-empath/verify_alien10.py` | `$N6_ARCH/docs/hexa-empath/verify_alien10.py` | N6A |
| 바이러스학 n=6 캡시드-팬데믹 아키텍처 | link[목표] | `docs/virology/goal.md` | `$N6_ARCH/docs/virology/goal.md` | N6A |
| 바이러스학 n=6 캡시드-팬데믹 아키텍처 | verify_script | `docs/virology/verify_alien10.py` | `$N6_ARCH/docs/virology/verify_alien10.py` | N6A |
| 곤충학 n=6 Hexapoda 완전 생물학 | link[목표] | `docs/entomology/goal.md` | `$N6_ARCH/docs/entomology/goal.md` | N6A |
| 곤충학 n=6 Hexapoda 완전 생물학 | link[논문] | `docs/paper/n6-entomology-paper.md` | `$N6_ARCH/docs/paper/n6-entomology-paper.md` | N6A |
| 곤충학 n=6 Hexapoda 완전 생물학 | verify_script | `docs/entomology/verify_alien10.py` | `$N6_ARCH/docs/entomology/verify_alien10.py` | N6A |
| 균류학 n=6 포자-발효 아키텍처 | link[가설] | `docs/mycology/hypotheses.md` | `$N6_ARCH/docs/mycology/hypotheses.md` | N6A |
| 균류학 n=6 포자-발효 아키텍처 | verify_script | `docs/mycology/verify_alien10.py` | `$N6_ARCH/docs/mycology/verify_alien10.py` | N6A |
| 광업/광물학 n=6 경도-결정 아키텍처 | link[가설] | `docs/mining/hypotheses.md` | `$N6_ARCH/docs/mining/hypotheses.md` | N6A |
| 광업/광물학 n=6 경도-결정 아키텍처 | verify_script | `docs/mining/verify_alien10.py` | `$N6_ARCH/docs/mining/verify_alien10.py` | N6A |
| 수의학 n=6 동물해부 보편성 | link[가설] | `docs/veterinary/hypotheses.md` | `$N6_ARCH/docs/veterinary/hypotheses.md` | N6A |
| 수의학 n=6 동물해부 보편성 | verify_script | `docs/veterinary/verify_alien10.py` | `$N6_ARCH/docs/veterinary/verify_alien10.py` | N6A |
| 원예학 n=6 식물 성장 아키텍처 | link[가설] | `docs/horticulture/hypotheses.md` | `$N6_ARCH/docs/horticulture/hypotheses.md` | N6A |
| 원예학 n=6 식물 성장 아키텍처 | verify_script | `docs/horticulture/verify_alien10.py` | `$N6_ARCH/docs/horticulture/verify_alien10.py` | N6A |
| HEXA-SIM 우주 시뮬레이션 | link[문서] | `docs/simulation-theory/goal.md` | `$N6_ARCH/docs/simulation-theory/goal.md` | N6A |
| HEXA-SIM 우주 시뮬레이션 | link[논문] | `docs/paper/n6-hexa-sim-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-sim-paper.md` | N6A |
| HEXA-SIM 우주 시뮬레이션 | verify_script | `docs/simulation-theory/verify_alien10.py` | `$N6_ARCH/docs/simulation-theory/verify_alien10.py` | N6A |
| 크로스 도메인 메가 브릿지 | link[문서] | `docs/new-bt-dimensional-unfolding-2026-04-06.md` | `$N6_ARCH/docs/new-bt-dimensional-unfolding-2026-04-06.md` | N6A |
| 크로스 도메인 메가 브릿지 | verify_script | `docs/cross-domain-mega/verify_alien10.py` | `$N6_ARCH/docs/cross-domain-mega/verify_alien10.py` | N6A |
| HEXA-NANOBOT 치료 나노봇 | link[문서] | `docs/therapeutic-nanobot/goal.md` | `$N6_ARCH/docs/therapeutic-nanobot/goal.md` | N6A |
| HEXA-NANOBOT 치료 나노봇 | link[논문] | `docs/paper/n6-therapeutic-nanobot-paper.md` | `$N6_ARCH/docs/paper/n6-therapeutic-nanobot-paper.md` | N6A |
| HEXA-NANOBOT 치료 나노봇 | verify_script | `docs/therapeutic-nanobot/verify_alien10.py` | `$N6_ARCH/docs/therapeutic-nanobot/verify_alien10.py` | N6A |

### tech-industry — 기술/산업 (MISS 47/50)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| 반도체 패키징 n=6 적층 래더 | link[문서] | `docs/advanced-packaging/goal.md` | `$N6_ARCH/docs/advanced-packaging/goal.md` | N6A |
| 반도체 패키징 n=6 적층 래더 | link[논문] | `docs/paper/n6-advanced-packaging-paper.md` | `$N6_ARCH/docs/paper/n6-advanced-packaging-paper.md` | N6A |
| 반도체 패키징 n=6 적층 래더 | link[논문] | `docs/paper/n6-ecology-agriculture-food-paper.md` | `$N6_ARCH/docs/paper/n6-ecology-agriculture-food-paper.md` | N6A |
| 반도체 패키징 n=6 적층 래더 | link[논문] | `docs/paper/n6-manufacturing-quality-paper.md` | `$N6_ARCH/docs/paper/n6-manufacturing-quality-paper.md` | N6A |
| AR/VR/XR 공간컴퓨팅 n=6 센서 | link[문서] | `docs/ar-vr-xr/goal.md` | `$N6_ARCH/docs/ar-vr-xr/goal.md` | N6A |
| AR/VR/XR 공간컴퓨팅 n=6 센서 | link[논문] | `docs/paper/n6-ar-vr-xr-paper.md` | `$N6_ARCH/docs/paper/n6-ar-vr-xr-paper.md` | N6A |
| 디지털 트윈 n=6 동기화 | link[문서] | `docs/digital-twin/goal.md` | `$N6_ARCH/docs/digital-twin/goal.md` | N6A |
| 디지털 트윈 n=6 동기화 | link[논문] | `docs/paper/n6-cognitive-social-psychology-paper.md` | `$N6_ARCH/docs/paper/n6-cognitive-social-psychology-paper.md` | N6A |
| 디지털 트윈 n=6 동기화 | link[논문] | `docs/paper/n6-digital-twin-paper.md` | `$N6_ARCH/docs/paper/n6-digital-twin-paper.md` | N6A |
| 건축/구조공학 n=6 하중 보편성 | link[문서] | `docs/construction-structural/goal.md` | `$N6_ARCH/docs/construction-structural/goal.md` | N6A |
| 건축/구조공학 n=6 하중 보편성 | link[논문] | `docs/paper/n6-construction-structural-paper.md` | `$N6_ARCH/docs/paper/n6-construction-structural-paper.md` | N6A |
| 건축/구조공학 n=6 하중 보편성 | verify_script | `docs/construction-structural/verify_alien10_hex.py` | `$N6_ARCH/docs/construction-structural/verify_alien10_hex.py` | N6A |
| 지하공간/터널 n=6 굴착 구조 | link[문서] | `docs/underground-tunnel/goal.md` | `$N6_ARCH/docs/underground-tunnel/goal.md` | N6A |
| 지하공간/터널 n=6 굴착 구조 | link[논문] | `docs/paper/n6-underground-tunnel-paper.md` | `$N6_ARCH/docs/paper/n6-underground-tunnel-paper.md` | N6A |
| 전자상거래/핀테크 n=6 결제 보안 | link[문서] | `docs/ecommerce-fintech/goal.md` | `$N6_ARCH/docs/ecommerce-fintech/goal.md` | N6A |
| 전자상거래/핀테크 n=6 결제 보안 | link[논문] | `docs/paper/n6-ecommerce-fintech-paper.md` | `$N6_ARCH/docs/paper/n6-ecommerce-fintech-paper.md` | N6A |
| 전자상거래/핀테크 n=6 결제 보안 | link[논문] | `docs/paper/n6-economics-finance-paper.md` | `$N6_ARCH/docs/paper/n6-economics-finance-paper.md` | N6A |
| 나일론 6/6,6 폴리아미드 | link[문서] | `docs/nylon/goal.md` | `$N6_ARCH/docs/nylon/goal.md` | N6A |
| 나일론 6/6,6 폴리아미드 | verify_script | `calc/kolon_n6_breakthrough.py` | `$N6_ARCH/calc/kolon_n6_breakthrough.py` | N6A |
| 아라미드 (Heracron) | link[문서] | `docs/aramid/goal.md` | `$N6_ARCH/docs/aramid/goal.md` | N6A |
| 아라미드 (Heracron) | verify_script | `calc/kolon_n6_breakthrough.py` | `$N6_ARCH/calc/kolon_n6_breakthrough.py` | N6A |
| 타이어코드 | link[문서] | `docs/tire-cord/goal.md` | `$N6_ARCH/docs/tire-cord/goal.md` | N6A |
| 타이어코드 | verify_script | `calc/kolon_n6_breakthrough.py` | `$N6_ARCH/calc/kolon_n6_breakthrough.py` | N6A |
| 에폭시/페놀 수지 | link[문서] | `docs/epoxy/goal.md` | `$N6_ARCH/docs/epoxy/goal.md` | N6A |
| 에폭시/페놀 수지 | verify_script | `calc/kolon_n6_breakthrough.py` | `$N6_ARCH/calc/kolon_n6_breakthrough.py` | N6A |
| PET 광학필름 | link[문서] | `docs/pet-film/goal.md` | `$N6_ARCH/docs/pet-film/goal.md` | N6A |
| PET 광학필름 | verify_script | `calc/kolon_n6_breakthrough.py` | `$N6_ARCH/calc/kolon_n6_breakthrough.py` | N6A |
| 에어백 | link[문서] | `docs/airbag/goal.md` | `$N6_ARCH/docs/airbag/goal.md` | N6A |
| 에어백 | verify_script | `calc/kolon_n6_breakthrough.py` | `$N6_ARCH/calc/kolon_n6_breakthrough.py` | N6A |
| 수처리 멤브레인 | link[문서] | `docs/water-treatment/goal.md` | `$N6_ARCH/docs/water-treatment/goal.md` | N6A |
| 수처리 멤브레인 | verify_script | `calc/kolon_n6_phase2.py` | `$N6_ARCH/calc/kolon_n6_phase2.py` | N6A |
| PEMFC 수소 연료전지 | link[문서] | `docs/pemfc/goal.md` | `$N6_ARCH/docs/pemfc/goal.md` | N6A |
| PEMFC 수소 연료전지 | verify_script | `calc/kolon_n6_phase2.py` | `$N6_ARCH/calc/kolon_n6_phase2.py` | N6A |
| 건설 콘크리트 | link[문서] | `docs/concrete/goal.md` | `$N6_ARCH/docs/concrete/goal.md` | N6A |
| 건설 콘크리트 | verify_script | `calc/kolon_n6_phase2.py` | `$N6_ARCH/calc/kolon_n6_phase2.py` | N6A |
| 바이오 약물전달/제약 | link[문서] | `docs/bio-pharma/goal.md` | `$N6_ARCH/docs/bio-pharma/goal.md` | N6A |
| 바이오 약물전달/제약 | verify_script | `calc/kolon_n6_phase2.py` | `$N6_ARCH/calc/kolon_n6_phase2.py` | N6A |
| HVAC 냉난방 n=6 COP 최적화 | link[검증] | `docs/hvac-system/verify_alien10.py` | `$N6_ARCH/docs/hvac-system/verify_alien10.py` | N6A |
| HVAC 냉난방 n=6 COP 최적화 | verify_script | `docs/hvac-system/verify_alien10.py` | `$N6_ARCH/docs/hvac-system/verify_alien10.py` | N6A |
| 내진설계 n=6 DOF 보편성 | link[검증] | `docs/earthquake-engineering/verify_alien10.py` | `$N6_ARCH/docs/earthquake-engineering/verify_alien10.py` | N6A |
| 내진설계 n=6 DOF 보편성 | verify_script | `docs/earthquake-engineering/verify_alien10.py` | `$N6_ARCH/docs/earthquake-engineering/verify_alien10.py` | N6A |
| 콘크리트+탄소포집 n=6 광물화 | link[검증] | `docs/concrete-technology/verify_alien10.py` | `$N6_ARCH/docs/concrete-technology/verify_alien10.py` | N6A |
| 콘크리트+탄소포집 n=6 광물화 | verify_script | `docs/concrete-technology/verify_alien10.py` | `$N6_ARCH/docs/concrete-technology/verify_alien10.py` | N6A |
| 스마트시티 통합 n=6 도시 시스템 | link[검증] | `docs/smart-city/verify_alien10.py` | `$N6_ARCH/docs/smart-city/verify_alien10.py` | N6A |
| 스마트시티 통합 n=6 도시 시스템 | verify_script | `docs/smart-city/verify_alien10.py` | `$N6_ARCH/docs/smart-city/verify_alien10.py` | N6A |
| 토목/구조역학 kissing number 사슬 | link[검증] | `docs/civil-engineering/verify_alien10.py` | `$N6_ARCH/docs/civil-engineering/verify_alien10.py` | N6A |
| 토목/구조역학 kissing number 사슬 | verify_script | `docs/civil-engineering/verify_alien10.py` | `$N6_ARCH/docs/civil-engineering/verify_alien10.py` | N6A |

### chip — 칩/반도체 (MISS 24/24)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| HEXA 칩 7단 | link[문서] | `docs/chip-architecture/goal.md` | `$N6_ARCH/docs/chip-architecture/goal.md` | N6A |
| HEXA 칩 7단 | link[논문] | `docs/paper/n6-dram-paper.md` | `$N6_ARCH/docs/paper/n6-dram-paper.md` | N6A |
| HEXA 칩 7단 | link[논문] | `docs/paper/n6-exynos-paper.md` | `$N6_ARCH/docs/paper/n6-exynos-paper.md` | N6A |
| HEXA 칩 7단 | link[논문] | `docs/paper/n6-hexa-3d-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-3d-paper.md` | N6A |
| HEXA 칩 7단 | link[논문] | `docs/paper/n6-hexa-photon-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-photon-paper.md` | N6A |
| HEXA 칩 7단 | link[논문] | `docs/paper/n6-hexa-pim-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-pim-paper.md` | N6A |
| HEXA 칩 7단 | link[논문] | `docs/paper/n6-hexa-wafer-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-wafer-paper.md` | N6A |
| HEXA 칩 7단 | link[논문] | `docs/paper/n6-performance-chip-paper.md` | `$N6_ARCH/docs/paper/n6-performance-chip-paper.md` | N6A |
| HEXA 칩 7단 | link[논문] | `docs/paper/n6-unified-soc-paper.md` | `$N6_ARCH/docs/paper/n6-unified-soc-paper.md` | N6A |
| HEXA 칩 7단 | link[논문] | `docs/paper/n6-vnand-paper.md` | `$N6_ARCH/docs/paper/n6-vnand-paper.md` | N6A |
| HEXA 칩 7단 | verify_script | `docs/chip-architecture/verify_alien10.py` | `$N6_ARCH/docs/chip-architecture/verify_alien10.py` | N6A |
| ANIMA-SOC | link[문서] | `docs/chip-architecture/ultimate-consciousness-soc.md` | `$N6_ARCH/docs/chip-architecture/ultimate-consciousness-soc.md` | N6A |
| ANIMA-SOC | link[논문] | `docs/paper/n6-anima-soc-paper.md` | `$N6_ARCH/docs/paper/n6-anima-soc-paper.md` | N6A |
| ANIMA-SOC | link[논문] | `docs/paper/n6-consciousness-soc-paper.md` | `$N6_ARCH/docs/paper/n6-consciousness-soc-paper.md` | N6A |
| ANIMA-SOC | verify_script | `docs/chip-architecture/verify_alien10.py` | `$N6_ARCH/docs/chip-architecture/verify_alien10.py` | N6A |
| HEXA-TOPO | link[문서] | `docs/chip-architecture/hexa-topological-performance-chip.md` | `$N6_ARCH/docs/chip-architecture/hexa-topological-performance-chip.md` | N6A |
| HEXA-TOPO | link[논문] | `docs/paper/n6-hexa-topo-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-topo-paper.md` | N6A |
| HEXA-TOPO | verify_script | `docs/chip-architecture/verify_alien10.py` | `$N6_ARCH/docs/chip-architecture/verify_alien10.py` | N6A |
| HEXA-ASIC | link[문서] | `docs/chip-architecture/hexa-asic-skywater.md` | `$N6_ARCH/docs/chip-architecture/hexa-asic-skywater.md` | N6A |
| HEXA-ASIC | link[논문] | `docs/paper/n6-hexa-asic-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-asic-paper.md` | N6A |
| HEXA-ASIC | verify_script | `docs/chip-architecture/verify_alien10.py` | `$N6_ARCH/docs/chip-architecture/verify_alien10.py` | N6A |
| 천장확인 | link[문서] | `docs/chip-architecture/full-verification-matrix.md` | `$N6_ARCH/docs/chip-architecture/full-verification-matrix.md` | N6A |
| 천장확인 | verify_script | `docs/chip-architecture/verify_alien10.py` | `$N6_ARCH/docs/chip-architecture/verify_alien10.py` | N6A |
| (섹션 공통) | section.verify_scripts | `docs/chip-architecture/verify_alien10.py` | `$N6_ARCH/docs/chip-architecture/verify_alien10.py` | N6A |

### physics — 물리/수학 (MISS 18/18)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| 궁극의 초전도체 | link[문서] | `docs/superconductor/goal.md` | `$N6_ARCH/docs/superconductor/goal.md` | N6A |
| 궁극의 초전도체 | link[논문] | `docs/paper/n6-superconductor-paper.md` | `$N6_ARCH/docs/paper/n6-superconductor-paper.md` | N6A |
| 궁극의 초전도체 | verify_script | `docs/pure-mathematics/verify_alien10.py` | `$N6_ARCH/docs/pure-mathematics/verify_alien10.py` | N6A |
| 궁극의 순수수학 | link[문서] | `docs/pure-mathematics/goal.md` | `$N6_ARCH/docs/pure-mathematics/goal.md` | N6A |
| 궁극의 순수수학 | link[논문] | `docs/paper/n6-pure-mathematics-paper.md` | `$N6_ARCH/docs/paper/n6-pure-mathematics-paper.md` | N6A |
| 궁극의 순수수학 | verify_script | `docs/pure-mathematics/verify_alien10.py` | `$N6_ARCH/docs/pure-mathematics/verify_alien10.py` | N6A |
| 궁극의 우주론/입자 | link[문서] | `docs/cosmology-particle/goal.md` | `$N6_ARCH/docs/cosmology-particle/goal.md` | N6A |
| 궁극의 우주론/입자 | link[논문] | `docs/paper/n6-classical-mechanics-accelerator-paper.md` | `$N6_ARCH/docs/paper/n6-classical-mechanics-accelerator-paper.md` | N6A |
| 궁극의 우주론/입자 | link[논문] | `docs/paper/n6-particle-cosmology-paper.md` | `$N6_ARCH/docs/paper/n6-particle-cosmology-paper.md` | N6A |
| 궁극의 우주론/입자 | link[논문] | `docs/paper/n6-thermodynamics-paper.md` | `$N6_ARCH/docs/paper/n6-thermodynamics-paper.md` | N6A |
| 궁극의 우주론/입자 | verify_script | `docs/pure-mathematics/verify_alien10.py` | `$N6_ARCH/docs/pure-mathematics/verify_alien10.py` | N6A |
| 궁극의 상온 초전도체 | link[문서] | `docs/room-temp-sc/goal.md` | `$N6_ARCH/docs/room-temp-sc/goal.md` | N6A |
| 궁극의 상온 초전도체 | link[논문] | `docs/paper/n6-hexa-super-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-super-paper.md` | N6A |
| 궁극의 상온 초전도체 | link[논문] | `docs/paper/n6-quantum-computing-paper.md` | `$N6_ARCH/docs/paper/n6-quantum-computing-paper.md` | N6A |
| 궁극의 상온 초전도체 | verify_script | `docs/room-temp-sc/verify_alien10.py` | `$N6_ARCH/docs/room-temp-sc/verify_alien10.py` | N6A |
| 차원펼침 돌파 — 텐서/mod3/로그 | link[문서] | `docs/new-bt-dimensional-unfolding-2026-04-06.md` | `$N6_ARCH/docs/new-bt-dimensional-unfolding-2026-04-06.md` | N6A |
| (섹션 공통) | section.verify_scripts | `docs/pure-mathematics/verify_alien10.py` | `$N6_ARCH/docs/pure-mathematics/verify_alien10.py` | N6A |
| (섹션 공통) | section.verify_scripts | `docs/cosmology-particle/verify_alien10.py` | `$N6_ARCH/docs/cosmology-particle/verify_alien10.py` | N6A |

### environment — 환경보호 (MISS 17/17)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| 궁극의 환경보호 8단 | link[문서] | `docs/environmental-protection/` | `$N6_ARCH/docs/environmental-protection/` | N6A |
| 궁극의 환경보호 8단 | link[논문] | `docs/paper/n6-environment-thermal-paper.md` | `$N6_ARCH/docs/paper/n6-environment-thermal-paper.md` | N6A |
| 궁극의 환경보호 8단 | verify_script | `docs/environmental-protection/verify_alien10.py` | `$N6_ARCH/docs/environmental-protection/verify_alien10.py` | N6A |
| HEXA-MICROPLASTICS | link[문서] | `docs/environmental-protection/microplastics-solution.md` | `$N6_ARCH/docs/environmental-protection/microplastics-solution.md` | N6A |
| HEXA-MICROPLASTICS | link[논문] | `docs/paper/n6-microplastics-paper.md` | `$N6_ARCH/docs/paper/n6-microplastics-paper.md` | N6A |
| HEXA-MICROPLASTICS | verify_script | `docs/environmental-protection/verify_alien10.py` | `$N6_ARCH/docs/environmental-protection/verify_alien10.py` | N6A |
| 궁극의 탄소포집 8단 | link[문서] | `docs/carbon-capture/goal.md` | `$N6_ARCH/docs/carbon-capture/goal.md` | N6A |
| 궁극의 탄소포집 8단 | link[논문] | `docs/paper/n6-carbon-capture-paper.md` | `$N6_ARCH/docs/paper/n6-carbon-capture-paper.md` | N6A |
| 궁극의 탄소포집 8단 | verify_script | `docs/environmental-protection/verify_alien10.py` | `$N6_ARCH/docs/environmental-protection/verify_alien10.py` | N6A |
| 진화 Mk.I~V | link[문서] | `docs/environmental-protection/evolution/` | `$N6_ARCH/docs/environmental-protection/evolution/` | N6A |
| 진화 Mk.I~V | verify_script | `docs/environmental-protection/verify_alien10.py` | `$N6_ARCH/docs/environmental-protection/verify_alien10.py` | N6A |
| 예측 + 검증 | link[문서] | `docs/environmental-protection/testable-predictions-2030.md` | `$N6_ARCH/docs/environmental-protection/testable-predictions-2030.md` | N6A |
| 예측 + 검증 | verify_script | `docs/environmental-protection/verify_alien10.py` | `$N6_ARCH/docs/environmental-protection/verify_alien10.py` | N6A |
| 궁극의 재활용 — HEXA-RECYCLE | link[설계] | `docs/recycling/goal.md` | `$N6_ARCH/docs/recycling/goal.md` | N6A |
| 궁극의 재활용 — HEXA-RECYCLE | link[논문] | `docs/paper/n6-hexa-recycle-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-recycle-paper.md` | N6A |
| (섹션 공통) | section.verify_scripts | `docs/environmental-protection/verify_alien10.py` | `$N6_ARCH/docs/environmental-protection/verify_alien10.py` | N6A |
| (섹션 공통) | section.verify_scripts | `docs/carbon-capture/verify_alien10.py` | `$N6_ARCH/docs/carbon-capture/verify_alien10.py` | N6A |

### software — 소프트웨어/인프라 (MISS 17/17)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| 궁극의 프로그래밍언어 | link[문서] | `docs/programming-language/goal.md` | `$N6_ARCH/docs/programming-language/goal.md` | N6A |
| 궁극의 프로그래밍언어 | link[논문] | `docs/paper/n6-hexa-proglang-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-proglang-paper.md` | N6A |
| 궁극의 프로그래밍언어 | link[논문] | `docs/paper/n6-software-crypto-paper.md` | `$N6_ARCH/docs/paper/n6-software-crypto-paper.md` | N6A |
| 궁극의 프로그래밍언어 | verify_script | `docs/programming-language/verify_alien10.py` | `$N6_ARCH/docs/programming-language/verify_alien10.py` | N6A |
| 천장확인 | link[문서] | `docs/software-design/full-verification-matrix.md` | `$N6_ARCH/docs/software-design/full-verification-matrix.md` | N6A |
| 천장확인 | verify_script | `docs/programming-language/verify_alien10.py` | `$N6_ARCH/docs/programming-language/verify_alien10.py` | N6A |
| 궁극의 macOS | link[문서] | `docs/hexa-macos/goal.md` | `$N6_ARCH/docs/hexa-macos/goal.md` | N6A |
| 궁극의 macOS | link[논문] | `docs/paper/n6-hexa-macos-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-macos-paper.md` | N6A |
| 궁극의 macOS | verify_script | `experiments/verify_hexa_macos.py` | `$N6_ARCH/experiments/verify_hexa_macos.py` | N6A |
| 궁극의 iOS | link[문서] | `docs/hexa-ios/goal.md` | `$N6_ARCH/docs/hexa-ios/goal.md` | N6A |
| 궁극의 iOS | link[논문] | `docs/paper/n6-hexa-ios-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-ios-paper.md` | N6A |
| 궁극의 iOS | verify_script | `experiments/verify_hexa_ios.py` | `$N6_ARCH/experiments/verify_hexa_ios.py` | N6A |
| 궁극의 네트워크 프로토콜 | link[문서] | `docs/network-protocol/goal.md` | `$N6_ARCH/docs/network-protocol/goal.md` | N6A |
| 궁극의 네트워크 프로토콜 | link[논문] | `docs/paper/n6-hexa-netproto-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-netproto-paper.md` | N6A |
| (섹션 공통) | section.verify_scripts | `docs/programming-language/verify_alien10.py` | `$N6_ARCH/docs/programming-language/verify_alien10.py` | N6A |
| (섹션 공통) | section.verify_scripts | `docs/software-design/verify_alien10.py` | `$N6_ARCH/docs/software-design/verify_alien10.py` | N6A |
| (섹션 공통) | section.verify_scripts | `verify_hexa_macos.py` | `$N6_ARCH/verify_hexa_macos.py` | N6A |

### audio — 오디오 (MISS 16/16)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| 궁극의 오디오 7단 | link[문서] | `docs/audio/goal.md` | `$N6_ARCH/docs/audio/goal.md` | N6A |
| 궁극의 오디오 7단 | link[논문] | `docs/paper/n6-isocell-comms-paper.md` | `$N6_ARCH/docs/paper/n6-isocell-comms-paper.md` | N6A |
| 궁극의 오디오 7단 | link[논문] | `docs/paper/n6-telecom-linguistics-paper.md` | `$N6_ARCH/docs/paper/n6-telecom-linguistics-paper.md` | N6A |
| 궁극의 오디오 7단 | verify_script | `docs/audio/verify_alien10.py` | `$N6_ARCH/docs/audio/verify_alien10.py` | N6A |
| 천장확인 | link[문서] | `docs/audio/full-verification-matrix.md` | `$N6_ARCH/docs/audio/full-verification-matrix.md` | N6A |
| 천장확인 | verify_script | `docs/audio/verify_alien10.py` | `$N6_ARCH/docs/audio/verify_alien10.py` | N6A |
| HEXA-SPEAK (AI 음성출력 Non-TTS) | link[문서] | `docs/hexa-speak/goal.md` | `$N6_ARCH/docs/hexa-speak/goal.md` | N6A |
| HEXA-SPEAK (AI 음성출력 Non-TTS) | link[논문] | `docs/paper/n6-hexa-speak-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-speak-paper.md` | N6A |
| HEXA-SPEAK (AI 음성출력 Non-TTS) | verify_script | `docs/hexa-speak/verify_alien10.py` | `$N6_ARCH/docs/hexa-speak/verify_alien10.py` | N6A |
| HEXA-EAR Ultimate | link[문서] | `docs/audio/hexa-ear-ultimate.md` | `$N6_ARCH/docs/audio/hexa-ear-ultimate.md` | N6A |
| HEXA-EAR Ultimate | link[논문] | `docs/paper/n6-hexa-ear-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-ear-paper.md` | N6A |
| HEXA-BONE 골전도 이어폰 | link[문서] | `docs/audio/hexa-bone-ultimate.md` | `$N6_ARCH/docs/audio/hexa-bone-ultimate.md` | N6A |
| HEXA-EAR-CELL 이어폰 배터리 | link[문서] | `docs/audio/hexa-ear-cell.md` | `$N6_ARCH/docs/audio/hexa-ear-cell.md` | N6A |
| HEXA-SPEAKER 궁극 스피커 | link[문서] | `docs/audio/hexa-speaker-ultimate.md` | `$N6_ARCH/docs/audio/hexa-speaker-ultimate.md` | N6A |
| (섹션 공통) | section.verify_scripts | `docs/audio/verify_alien10.py` | `$N6_ARCH/docs/audio/verify_alien10.py` | N6A |
| (섹션 공통) | section.verify_scripts | `docs/hexa-speak/verify_alien10.py` | `$N6_ARCH/docs/hexa-speak/verify_alien10.py` | N6A |

### civilization — 문명/인문 (MISS 15/15)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| 종교/신화 n=6 보편 구조 | link[목표] | `docs/religion/goal.md` | `$N6_ARCH/docs/religion/goal.md` | N6A |
| 종교/신화 n=6 보편 구조 | link[논문] | `docs/paper/n6-religion-mythology-paper.md` | `$N6_ARCH/docs/paper/n6-religion-mythology-paper.md` | N6A |
| 법학/사법 n=6 정의 아키텍처 | link[목표] | `docs/jurisprudence/goal.md` | `$N6_ARCH/docs/jurisprudence/goal.md` | N6A |
| 법학/사법 n=6 정의 아키텍처 | link[논문] | `docs/paper/n6-jurisprudence-paper.md` | `$N6_ARCH/docs/paper/n6-jurisprudence-paper.md` | N6A |
| 한글/문자체계 n=6 인코딩 | link[목표] | `docs/writing-systems/goal.md` | `$N6_ARCH/docs/writing-systems/goal.md` | N6A |
| 한글/문자체계 n=6 인코딩 | link[논문] | `docs/paper/n6-writing-systems-paper.md` | `$N6_ARCH/docs/paper/n6-writing-systems-paper.md` | N6A |
| 고고학/문명사 n=6 기원 | link[목표] | `docs/archaeology/goal.md` | `$N6_ARCH/docs/archaeology/goal.md` | N6A |
| 고고학/문명사 n=6 기원 | link[논문] | `docs/paper/n6-archaeology-paper.md` | `$N6_ARCH/docs/paper/n6-archaeology-paper.md` | N6A |
| 화폐/경제사 n=6 통화 래더 | link[목표] | `docs/monetary-history/goal.md` | `$N6_ARCH/docs/monetary-history/goal.md` | N6A |
| 화폐/경제사 n=6 통화 래더 | link[논문] | `docs/paper/n6-monetary-history-paper.md` | `$N6_ARCH/docs/paper/n6-monetary-history-paper.md` | N6A |
| 무용/안무 n=6 공간 기하학 | link[목표] | `docs/dance-choreography/goal.md` | `$N6_ARCH/docs/dance-choreography/goal.md` | N6A |
| 무용/안무 n=6 공간 기하학 | link[논문] | `docs/paper/n6-dance-choreography-paper.md` | `$N6_ARCH/docs/paper/n6-dance-choreography-paper.md` | N6A |
| 시계학/호롤로지 n=6 시간 아키텍처 | link[가설] | `docs/horology/hypotheses.md` | `$N6_ARCH/docs/horology/hypotheses.md` | N6A |
| 시계학/호롤로지 n=6 시간 아키텍처 | link[논문] | `docs/paper/n6-calendar-time-geography-paper.md` | `$N6_ARCH/docs/paper/n6-calendar-time-geography-paper.md` | N6A |
| 시계학/호롤로지 n=6 시간 아키텍처 | link[논문] | `docs/paper/n6-horology-paper.md` | `$N6_ARCH/docs/paper/n6-horology-paper.md` | N6A |

### life-culture — 생활/문화 (MISS 15/15)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| 발효/양조 n=6 완전수 화학양론 | link[문서] | `docs/fermentation/goal.md` | `$N6_ARCH/docs/fermentation/goal.md` | N6A |
| 발효/양조 n=6 완전수 화학양론 | link[논문] | `docs/paper/n6-fermentation-paper.md` | `$N6_ARCH/docs/paper/n6-fermentation-paper.md` | N6A |
| 와인/소믈리에 n=6 테이스팅 | link[문서] | `docs/wine-enology/goal.md` | `$N6_ARCH/docs/wine-enology/goal.md` | N6A |
| 와인/소믈리에 n=6 테이스팅 | link[논문] | `docs/paper/n6-wine-enology-paper.md` | `$N6_ARCH/docs/paper/n6-wine-enology-paper.md` | N6A |
| 패션/섬유 n=6 직조 구조 | link[문서] | `docs/fashion-textile/goal.md` | `$N6_ARCH/docs/fashion-textile/goal.md` | N6A |
| 패션/섬유 n=6 직조 구조 | link[논문] | `docs/paper/n6-fashion-textile-paper.md` | `$N6_ARCH/docs/paper/n6-fashion-textile-paper.md` | N6A |
| 수산/양식 n=6 해양 생태 | link[문서] | `docs/aquaculture/goal.md` | `$N6_ARCH/docs/aquaculture/goal.md` | N6A |
| 수산/양식 n=6 해양 생태 | link[논문] | `docs/paper/n6-aquaculture-paper.md` | `$N6_ARCH/docs/paper/n6-aquaculture-paper.md` | N6A |
| 보험/보험계리 n=6 리스크 구조 | link[문서] | `docs/insurance/goal.md` | `$N6_ARCH/docs/insurance/goal.md` | N6A |
| 보험/보험계리 n=6 리스크 구조 | link[논문] | `docs/paper/n6-insurance-paper.md` | `$N6_ARCH/docs/paper/n6-insurance-paper.md` | N6A |
| 돌고래 n=6 생물음향 아키텍처 | link[가설] | `docs/dolphin/hypotheses.md` | `$N6_ARCH/docs/dolphin/hypotheses.md` | N6A |
| 돌고래 n=6 생물음향 아키텍처 | link[논문] | `docs/paper/n6-dolphin-bioacoustics-paper.md` | `$N6_ARCH/docs/paper/n6-dolphin-bioacoustics-paper.md` | N6A |
| 커피과학 n=6 추출 아키텍처 | link[가설] | `docs/coffee-science/hypotheses.md` | `$N6_ARCH/docs/coffee-science/hypotheses.md` | N6A |
| 향수/향료 n=6 피라미드 구조 | link[가설] | `docs/perfumery/hypotheses.md` | `$N6_ARCH/docs/perfumery/hypotheses.md` | N6A |
| 도자기/세라믹 n=6 소성 래더 | link[가설] | `docs/ceramics/hypotheses.md` | `$N6_ARCH/docs/ceramics/hypotheses.md` | N6A |

### materials — 물질합성 (MISS 15/15)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| 궁극의 물질합성 8단 | link[문서] | `docs/material-synthesis/goal.md` | `$N6_ARCH/docs/material-synthesis/goal.md` | N6A |
| 궁극의 물질합성 8단 | link[논문] | `docs/paper/n6-crystallography-materials-paper.md` | `$N6_ARCH/docs/paper/n6-crystallography-materials-paper.md` | N6A |
| 궁극의 물질합성 8단 | link[논문] | `docs/paper/n6-material-synthesis-paper.md` | `$N6_ARCH/docs/paper/n6-material-synthesis-paper.md` | N6A |
| 궁극의 물질합성 8단 | verify_script | `docs/material-synthesis/verify_alien10.py` | `$N6_ARCH/docs/material-synthesis/verify_alien10.py` | N6A |
| BT-85~88 + BT-128~135 | link[문서] | `docs/material-synthesis/breakthrough-theorems.md` | `$N6_ARCH/docs/material-synthesis/breakthrough-theorems.md` | N6A |
| BT-85~88 + BT-128~135 | verify_script | `docs/material-synthesis/verify_alien10.py` | `$N6_ARCH/docs/material-synthesis/verify_alien10.py` | N6A |
| 가설 36/36 100%EXACT | link[문서] | `docs/material-synthesis/hypotheses.md` | `$N6_ARCH/docs/material-synthesis/hypotheses.md` | N6A |
| 가설 36/36 100%EXACT | verify_script | `docs/material-synthesis/verify_alien10.py` | `$N6_ARCH/docs/material-synthesis/verify_alien10.py` | N6A |
| 산업검증 20소재+12금속 | link[문서] | `docs/material-synthesis/industrial-validation.md` | `$N6_ARCH/docs/material-synthesis/industrial-validation.md` | N6A |
| 산업검증 20소재+12금속 | verify_script | `docs/material-synthesis/verify_alien10.py` | `$N6_ARCH/docs/material-synthesis/verify_alien10.py` | N6A |
| 실험검증 + TP 28/28 | link[문서] | `docs/material-synthesis/experimental-verification.md` | `$N6_ARCH/docs/material-synthesis/experimental-verification.md` | N6A |
| 실험검증 + TP 28/28 | verify_script | `docs/material-synthesis/verify_alien10.py` | `$N6_ARCH/docs/material-synthesis/verify_alien10.py` | N6A |
| 물리한계 증명 | link[문서] | `docs/material-synthesis/physical-limit-proof.md` | `$N6_ARCH/docs/material-synthesis/physical-limit-proof.md` | N6A |
| 물리한계 증명 | verify_script | `docs/material-synthesis/verify_alien10.py` | `$N6_ARCH/docs/material-synthesis/verify_alien10.py` | N6A |
| (섹션 공통) | section.verify_scripts | `docs/material-synthesis/verify_alien10.py` | `$N6_ARCH/docs/material-synthesis/verify_alien10.py` | N6A |

### energy — 에너지 (MISS 14/14)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| 궁극의 배터리 8단 | link[문서] | `docs/superpowers/specs/2026-04-01-hexa-battery-design.md` | `$N6_ARCH/docs/superpowers/specs/2026-04-01-hexa-battery-design.md` | N6A |
| 궁극의 배터리 8단 | link[논문] | `docs/paper/n6-battery-energy-paper.md` | `$N6_ARCH/docs/paper/n6-battery-energy-paper.md` | N6A |
| 궁극의 배터리 8단 | verify_script | `docs/battery-architecture/verify_alien10.py` | `$N6_ARCH/docs/battery-architecture/verify_alien10.py` | N6A |
| 궁극의 태양전지 | link[문서] | `docs/solar-architecture/goal.md` | `$N6_ARCH/docs/solar-architecture/goal.md` | N6A |
| 궁극의 태양전지 | verify_script | `docs/battery-architecture/verify_alien10.py` | `$N6_ARCH/docs/battery-architecture/verify_alien10.py` | N6A |
| 궁극의 에너지 통합 | link[문서] | `docs/energy-architecture/goal.md` | `$N6_ARCH/docs/energy-architecture/goal.md` | N6A |
| 궁극의 에너지 통합 | link[논문] | `docs/paper/n6-energy-efficiency-paper.md` | `$N6_ARCH/docs/paper/n6-energy-efficiency-paper.md` | N6A |
| 궁극의 에너지 통합 | verify_script | `docs/battery-architecture/verify_alien10.py` | `$N6_ARCH/docs/battery-architecture/verify_alien10.py` | N6A |
| 궁극의 데이터센터 원자로 | link[문서] | `docs/smr-datacenter/goal.md` | `$N6_ARCH/docs/smr-datacenter/goal.md` | N6A |
| 궁극의 데이터센터 원자로 | link[논문] | `docs/paper/n6-datacenter-reactor-paper.md` | `$N6_ARCH/docs/paper/n6-datacenter-reactor-paper.md` | N6A |
| HEXA-AUTO 자동차배터리 | link[문서] | `docs/battery-architecture/hexa-auto-battery.md` | `$N6_ARCH/docs/battery-architecture/hexa-auto-battery.md` | N6A |
| (섹션 공통) | section.verify_scripts | `docs/battery-architecture/verify_alien10.py` | `$N6_ARCH/docs/battery-architecture/verify_alien10.py` | N6A |
| (섹션 공통) | section.verify_scripts | `docs/solar-architecture/verify_alien10.py` | `$N6_ARCH/docs/solar-architecture/verify_alien10.py` | N6A |
| (섹션 공통) | section.verify_scripts | `docs/energy-architecture/verify_alien10.py` | `$N6_ARCH/docs/energy-architecture/verify_alien10.py` | N6A |

### fusion — 핵융합 (MISS 13/13)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| 궁극의 핵융합 발전소 | link[문서] | `docs/superpowers/specs/2026-04-02-ultimate-fusion-powerplant-design.md` | `$N6_ARCH/docs/superpowers/specs/2026-04-02-ultimate-fusion-powerplant-design.md` | N6A |
| 궁극의 핵융합 발전소 | link[논문] | `docs/paper/n6-fusion-powerplant-paper.md` | `$N6_ARCH/docs/paper/n6-fusion-powerplant-paper.md` | N6A |
| 궁극의 핵융합 발전소 | link[논문] | `docs/paper/n6-plasma-fusion-deep-paper.md` | `$N6_ARCH/docs/paper/n6-plasma-fusion-deep-paper.md` | N6A |
| 궁극의 핵융합 발전소 | verify_script | `docs/fusion/verify_alien10.py` | `$N6_ARCH/docs/fusion/verify_alien10.py` | N6A |
| KSTAR-N6 | link[문서] | `docs/superpowers/specs/2026-04-02-kstar-n6-tokamak-design.md` | `$N6_ARCH/docs/superpowers/specs/2026-04-02-kstar-n6-tokamak-design.md` | N6A |
| KSTAR-N6 | verify_script | `docs/fusion/verify_alien10.py` | `$N6_ARCH/docs/fusion/verify_alien10.py` | N6A |
| 진화 Mk.I~V | link[문서] | `docs/fusion/evolution/mk-1-first-light.md` | `$N6_ARCH/docs/fusion/evolution/mk-1-first-light.md` | N6A |
| 진화 Mk.I~V | verify_script | `docs/fusion/verify_alien10.py` | `$N6_ARCH/docs/fusion/verify_alien10.py` | N6A |
| 발견 + 예측 + 가설v5 | link[문서] | `docs/fusion/alien-level-discoveries.md` | `$N6_ARCH/docs/fusion/alien-level-discoveries.md` | N6A |
| 발견 + 예측 + 가설v5 | verify_script | `docs/fusion/verify_alien10.py` | `$N6_ARCH/docs/fusion/verify_alien10.py` | N6A |
| 천장확인 | link[문서] | `docs/fusion/physical-limit-proof.md` | `$N6_ARCH/docs/fusion/physical-limit-proof.md` | N6A |
| 천장확인 | verify_script | `docs/fusion/verify_alien10.py` | `$N6_ARCH/docs/fusion/verify_alien10.py` | N6A |
| (섹션 공통) | section.verify_scripts | `docs/fusion/verify_alien10.py` | `$N6_ARCH/docs/fusion/verify_alien10.py` | N6A |

### ai — AI/ML (MISS 12/12)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| 66 Techniques | link[문서] | `docs/ai-efficiency/techniques-complete.md` | `$N6_ARCH/docs/ai-efficiency/techniques-complete.md` | N6A |
| Full N6 Pipeline | link[문서] | `experiments/experiment_full_n6_pipeline.py` | `$N6_ARCH/experiments/experiment_full_n6_pipeline.py` | N6A |
| Full N6 Pipeline | link[논문] | `docs/paper/n6-causal-chain-paper.md` | `$N6_ARCH/docs/paper/n6-causal-chain-paper.md` | N6A |
| Full N6 Pipeline | link[논문] | `docs/paper/n6-reality-map-paper.md` | `$N6_ARCH/docs/paper/n6-reality-map-paper.md` | N6A |
| Full N6 Pipeline | link[논문] | `docs/paper/n6-rtsc-12-products-evolution-paper.md` | `$N6_ARCH/docs/paper/n6-rtsc-12-products-evolution-paper.md` | N6A |
| N6 Inevitability Engine | link[문서] | `docs/superpowers/specs/2026-03-28-n6-inevitability-engine-design.md` | `$N6_ARCH/docs/superpowers/specs/2026-03-28-n6-inevitability-engine-design.md` | N6A |
| AI Energy Savings Guide | link[문서] | `docs/ai-energy-savings-guide.md` | `$N6_ARCH/docs/ai-energy-savings-guide.md` | N6A |
| Chip Architecture Guide | link[문서] | `docs/chip-architecture-guide.md` | `$N6_ARCH/docs/chip-architecture-guide.md` | N6A |
| 천장확인 | link[문서] | `docs/ai-efficiency/full-verification-matrix.md` | `$N6_ARCH/docs/ai-efficiency/full-verification-matrix.md` | N6A |
| Next-Gen AI 8-Paradigm Blowup | link[문서] | `docs/ai-efficiency/next-model-blowup-2026-04.md` | `$N6_ARCH/docs/ai-efficiency/next-model-blowup-2026-04.md` | N6A |
| AI 6-Domain Sweep | link[코드생성] | `docs/ai-efficiency/bt-391-code-generation.md` | `$N6_ARCH/docs/ai-efficiency/bt-391-code-generation.md` | N6A |
| N6 Reverse-Engineering Suite | link[신규모델] | `docs/ai-efficiency/bt-397-n6-novel-architectures.md` | `$N6_ARCH/docs/ai-efficiency/bt-397-n6-novel-architectures.md` | N6A |

### play — 유희 (MISS 9/9)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| 궁극의 펀카 | link[문서] | `docs/fun-car/goal.md` | `$N6_ARCH/docs/fun-car/goal.md` | N6A |
| 궁극의 펀카 | link[논문] | `docs/paper/n6-fun-car-paper.md` | `$N6_ARCH/docs/paper/n6-fun-car-paper.md` | N6A |
| 궁극의 펀카 | link[논문] | `docs/paper/n6-games-sports-paper.md` | `$N6_ARCH/docs/paper/n6-games-sports-paper.md` | N6A |
| 궁극의 펀카 | verify_script | `docs/fun-car/verify_alien10.py` | `$N6_ARCH/docs/fun-car/verify_alien10.py` | N6A |
| 궁극의 바이크 | link[문서] | `docs/motorcycle/goal.md` | `$N6_ARCH/docs/motorcycle/goal.md` | N6A |
| 궁극의 바이크 | link[논문] | `docs/paper/n6-motorcycle-paper.md` | `$N6_ARCH/docs/paper/n6-motorcycle-paper.md` | N6A |
| 궁극의 바이크 | verify_script | `docs/motorcycle/verify_alien10.py` | `$N6_ARCH/docs/motorcycle/verify_alien10.py` | N6A |
| (섹션 공통) | section.verify_scripts | `docs/fun-car/verify_alien10.py` | `$N6_ARCH/docs/fun-car/verify_alien10.py` | N6A |
| (섹션 공통) | section.verify_scripts | `docs/motorcycle/verify_alien10.py` | `$N6_ARCH/docs/motorcycle/verify_alien10.py` | N6A |

### robotics — 로봇 (MISS 8/8)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| 궁극의 로봇 8단 | link[문서] | `docs/robotics/goal.md` | `$N6_ARCH/docs/robotics/goal.md` | N6A |
| 궁극의 로봇 8단 | link[논문] | `docs/paper/n6-autonomous-driving-paper.md` | `$N6_ARCH/docs/paper/n6-autonomous-driving-paper.md` | N6A |
| 궁극의 로봇 8단 | link[논문] | `docs/paper/n6-control-automation-paper.md` | `$N6_ARCH/docs/paper/n6-control-automation-paper.md` | N6A |
| 궁극의 로봇 8단 | link[논문] | `docs/paper/n6-robotics-transport-paper.md` | `$N6_ARCH/docs/paper/n6-robotics-transport-paper.md` | N6A |
| 궁극의 로봇 8단 | verify_script | `docs/robotics/verify_alien10.py` | `$N6_ARCH/docs/robotics/verify_alien10.py` | N6A |
| 천장확인 | link[문서] | `docs/robotics/full-verification-matrix.md` | `$N6_ARCH/docs/robotics/full-verification-matrix.md` | N6A |
| 천장확인 | verify_script | `docs/robotics/verify_alien10.py` | `$N6_ARCH/docs/robotics/verify_alien10.py` | N6A |
| (섹션 공통) | section.verify_scripts | `docs/robotics/verify_alien10.py` | `$N6_ARCH/docs/robotics/verify_alien10.py` | N6A |

### aerospace — 우주항공 (MISS 7/7)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| 궁극의 재사용 발사체 (HEXA-STARSHIP) | link[문서] | `docs/hexa-starship/goal.md` | `$N6_ARCH/docs/hexa-starship/goal.md` | N6A |
| 궁극의 재사용 발사체 (HEXA-STARSHIP) | link[논문] | `docs/paper/n6-aerospace-transport-paper.md` | `$N6_ARCH/docs/paper/n6-aerospace-transport-paper.md` | N6A |
| 궁극의 재사용 발사체 (HEXA-STARSHIP) | link[논문] | `docs/paper/n6-hexa-starship-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-starship-paper.md` | N6A |
| 궁극의 재사용 발사체 (HEXA-STARSHIP) | link[논문] | `docs/paper/n6-space-systems-paper.md` | `$N6_ARCH/docs/paper/n6-space-systems-paper.md` | N6A |
| 궁극의 재사용 발사체 (HEXA-STARSHIP) | verify_script | `docs/hexa-starship/verify_hexa_starship.py` | `$N6_ARCH/docs/hexa-starship/verify_hexa_starship.py` | N6A |
| (섹션 공통) | section.verify_scripts | `docs/hexa-starship/verify_hexa_starship.py` | `$N6_ARCH/docs/hexa-starship/verify_hexa_starship.py` | N6A |
| (섹션 공통) | section.verify_scripts | `docs/hexa-starship/verify_subsystems.py` | `$N6_ARCH/docs/hexa-starship/verify_subsystems.py` | N6A |

### safety — 안전 (MISS 7/7)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| 궁극의 안전 8단 | link[문서] | `docs/safety/goal.md` | `$N6_ARCH/docs/safety/goal.md` | N6A |
| 궁극의 안전 8단 | link[논문] | `docs/paper/n6-governance-safety-urban-paper.md` | `$N6_ARCH/docs/paper/n6-governance-safety-urban-paper.md` | N6A |
| 궁극의 안전 8단 | link[논문] | `docs/paper/n6-ultimate-safety-paper.md` | `$N6_ARCH/docs/paper/n6-ultimate-safety-paper.md` | N6A |
| 궁극의 안전 8단 | verify_script | `docs/safety/verify_alien10.py` | `$N6_ARCH/docs/safety/verify_alien10.py` | N6A |
| 가설 30+극한 20 | link[문서] | `docs/safety/hypotheses.md` | `$N6_ARCH/docs/safety/hypotheses.md` | N6A |
| 가설 30+극한 20 | verify_script | `docs/safety/verify_alien10.py` | `$N6_ARCH/docs/safety/verify_alien10.py` | N6A |
| (섹션 공통) | section.verify_scripts | `docs/safety/verify_alien10.py` | `$N6_ARCH/docs/safety/verify_alien10.py` | N6A |

### cognitive-social — 인지/사회 (MISS 6/6)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| HEXA-COGNI n=6 인지 아키텍처 | link[문서] | `docs/cognitive-architecture/goal.md` | `$N6_ARCH/docs/cognitive-architecture/goal.md` | N6A |
| HEXA-CONSCIOUSNESS 의식 프로세서 | link[논문] | `docs/paper/n6-consciousness-chip-paper.md` | `$N6_ARCH/docs/paper/n6-consciousness-chip-paper.md` | N6A |
| HEXA-SOCIAL n=6 사회 아키텍처 | link[문서] | `docs/social-architecture/goal.md` | `$N6_ARCH/docs/social-architecture/goal.md` | N6A |
| HEXA-TEMPORAL n=6 시간 아키텍처 | link[문서] | `docs/temporal-architecture/goal.md` | `$N6_ARCH/docs/temporal-architecture/goal.md` | N6A |
| HEXA-LING n=6 언어 아키텍처 | link[문서] | `docs/linguistics/goal.md` | `$N6_ARCH/docs/linguistics/goal.md` | N6A |
| HEXA-ECON n=6 경제학 | link[문서] | `docs/economics/goal.md` | `$N6_ARCH/docs/economics/goal.md` | N6A |

### display — 디스플레이 (MISS 6/6)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| 궁극의 디스플레이 8단 | link[문서] | `docs/display/goal.md` | `$N6_ARCH/docs/display/goal.md` | N6A |
| 궁극의 디스플레이 8단 | link[논문] | `docs/paper/n6-display-8stack-paper.md` | `$N6_ARCH/docs/paper/n6-display-8stack-paper.md` | N6A |
| 궁극의 디스플레이 8단 | verify_script | `docs/display/verify_alien10.py` | `$N6_ARCH/docs/display/verify_alien10.py` | N6A |
| 천장확인 | link[문서] | `docs/display/full-verification-matrix.md` | `$N6_ARCH/docs/display/full-verification-matrix.md` | N6A |
| 천장확인 | verify_script | `docs/display/verify_alien10.py` | `$N6_ARCH/docs/display/verify_alien10.py` | N6A |
| (섹션 공통) | section.verify_scripts | `docs/display/verify_alien10.py` | `$N6_ARCH/docs/display/verify_alien10.py` | N6A |

### hiv-treatment — HIV 치료 (MISS 6/6)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| HEXA-HIV 6축 완전 치료 체인 | link[논문] | `docs/paper/n6-hiv-paper.md` | `$N6_ARCH/docs/paper/n6-hiv-paper.md` | N6A |
| HEXA-HIV 6축 완전 치료 체인 | link[Mk.I] | `docs/hiv-treatment/evolution/mk-1-basic.md` | `$N6_ARCH/docs/hiv-treatment/evolution/mk-1-basic.md` | N6A |
| HEXA-HIV 6축 완전 치료 체인 | link[Mk.II] | `docs/hiv-treatment/evolution/mk-2-short.md` | `$N6_ARCH/docs/hiv-treatment/evolution/mk-2-short.md` | N6A |
| HEXA-HIV 6축 완전 치료 체인 | link[Mk.III] | `docs/hiv-treatment/evolution/mk-3-mid.md` | `$N6_ARCH/docs/hiv-treatment/evolution/mk-3-mid.md` | N6A |
| HEXA-HIV 6축 완전 치료 체인 | link[Mk.IV] | `docs/hiv-treatment/evolution/mk-4-long.md` | `$N6_ARCH/docs/hiv-treatment/evolution/mk-4-long.md` | N6A |
| HEXA-HIV 6축 완전 치료 체인 | link[Mk.V] | `docs/hiv-treatment/evolution/mk-5-ultimate.md` | `$N6_ARCH/docs/hiv-treatment/evolution/mk-5-ultimate.md` | N6A |

### virology — 바이러스학 (MISS 6/6)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| 바이러스 구조-분류 완전 n=6 맵 | link[문서] | `docs/virology/goal.md` | `$N6_ARCH/docs/virology/goal.md` | N6A |
| 바이러스 구조-분류 완전 n=6 맵 | link[논문] | `docs/paper/n6-virology-paper.md` | `$N6_ARCH/docs/paper/n6-virology-paper.md` | N6A |
| 바이러스 구조-분류 완전 n=6 맵 | verify_script | `docs/virology/verify_alien10.py` | `$N6_ARCH/docs/virology/verify_alien10.py` | N6A |
| 바이러스 게놈 분절-유전자 n=6 래더 | link[문서] | `docs/virology/goal.md` | `$N6_ARCH/docs/virology/goal.md` | N6A |
| 바이러스 역학-백신-효소 n=6 완전 폐쇄 | link[문서] | `docs/virology/goal.md` | `$N6_ARCH/docs/virology/goal.md` | N6A |
| 진화 Mk.I~V | link[Mk.I] | `docs/virology/evolution/mk-1-current.md` | `$N6_ARCH/docs/virology/evolution/mk-1-current.md` | N6A |

### marketing — 마케팅 (MISS 4/4)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| HEXA-MKT 마케팅 불변의 법칙 | link[문서] | `docs/marketing/goal.md` | `$N6_ARCH/docs/marketing/goal.md` | N6A |
| HEXA-NEXUS 서비스 플랫폼 | link[문서] | `docs/nexus-service/goal.md` | `$N6_ARCH/docs/nexus-service/goal.md` | N6A |
| HEXA-ANIMA 감성 서비스 | link[문서] | `docs/anima-service/goal.md` | `$N6_ARCH/docs/anima-service/goal.md` | N6A |
| HEXA-UNIFIED 완전인지 플랫폼 | link[문서] | `docs/unified-service/goal.md` | `$N6_ARCH/docs/unified-service/goal.md` | N6A |

### natural-science — 자연과학 (MISS 4/4)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| HEXA-BIO n=6 생명 아키텍처 | link[문서] | `docs/biology/goal.md` | `$N6_ARCH/docs/biology/goal.md` | N6A |
| HEXA-AGRI n=6 농업과학 | link[문서] | `docs/agriculture/goal.md` | `$N6_ARCH/docs/agriculture/goal.md` | N6A |
| HEXA-FOOD n=6 식품과학 | link[문서] | `docs/food-science/goal.md` | `$N6_ARCH/docs/food-science/goal.md` | N6A |
| HEXA-OCEAN n=6 해양과학 | link[문서] | `docs/oceanography/goal.md` | `$N6_ARCH/docs/oceanography/goal.md` | N6A |

### digital-medical — 디지털/의료기기 (MISS 3/3)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| HEXA-BROWSER 특이점 브라우저 | link[문서] | `docs/browser/goal.md` | `$N6_ARCH/docs/browser/goal.md` | N6A |
| HEXA-MED n=6 의료기기 | link[문서] | `docs/medical-device/goal.md` | `$N6_ARCH/docs/medical-device/goal.md` | N6A |
| HEXA-AESTHETIC n=6 성형외과 | link[문서] | `docs/cosmetic-surgery/goal.md` | `$N6_ARCH/docs/cosmetic-surgery/goal.md` | N6A |

### hygiene — 위생 (MISS 2/2)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| 남성청결제 n=6 피부과학 아키텍처 | link[돌파] | `docs/mens-intimate-cleanser/breakthrough.md` | `$N6_ARCH/docs/mens-intimate-cleanser/breakthrough.md` | N6A |
| 여성청결제 n=6 질내생태계 아키텍처 | link[돌파] | `docs/womens-intimate-cleanser/breakthrough.md` | `$N6_ARCH/docs/womens-intimate-cleanser/breakthrough.md` | N6A |

### manufacturing-quality — 제조 품질관리 (MISS 2/2)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| HEXA-QC 제조 품질관리 아키텍처 | link[목표] | `docs/manufacturing-quality/goal.md` | `$N6_ARCH/docs/manufacturing-quality/goal.md` | N6A |
| HEXA-QC 제조 품질관리 아키텍처 | link[논문] | `docs/paper/n6-manufacturing-quality-paper.md` | `$N6_ARCH/docs/paper/n6-manufacturing-quality-paper.md` | N6A |

### mobility — 이동/수송 (MISS 2/2)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| HEXA-DRIVE n=6 자율주행 | link[문서] | `docs/autonomous-driving/goal.md` | `$N6_ARCH/docs/autonomous-driving/goal.md` | N6A |
| HEXA-WING n=6 항공공학 | link[문서] | `docs/aviation/goal.md` | `$N6_ARCH/docs/aviation/goal.md` | N6A |

### sf — UFO/비행접시 (MISS 2/2)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| 궁극의 UFO 비행접시 (HEXA-UFO) | link[문서] | `docs/sf/goal.md` | `$N6_ARCH/docs/sf/goal.md` | N6A |
| 궁극의 UFO 비행접시 (HEXA-UFO) | link[논문] | `docs/paper/n6-hexa-ufo-paper.md` | `$N6_ARCH/docs/paper/n6-hexa-ufo-paper.md` | N6A |

### horology — 시계학 (MISS 1/1)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| 시계학 n=6 시간 산술 아키텍처 | link[목표] | `docs/horology/goal.md` | `$N6_ARCH/docs/horology/goal.md` | N6A |

### keyboard — 키보드 (MISS 1/1)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| 키보드 n=6 인체공학 아키텍처 | link[목표] | `docs/keyboard/goal.md` | `$N6_ARCH/docs/keyboard/goal.md` | N6A |

### mouse — 마우스 (MISS 1/1)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| HEXA-MOUSE n=6 인체공학 마우스 | link[목표] | `docs/mouse/goal.md` | `$N6_ARCH/docs/mouse/goal.md` | N6A |

### network — 네트워크 (MISS 1/1)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| HEXA-NET 네트워크 아키텍처 | link[목표] | `docs/network/goal.md` | `$N6_ARCH/docs/network/goal.md` | N6A |

### quantum-computer — 양자컴퓨터 (MISS 1/1)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| HEXA-QUANTUM 양자컴퓨터 아키텍처 | link[목표] | `docs/quantum-computer/goal.md` | `$N6_ARCH/docs/quantum-computer/goal.md` | N6A |

### tattoo-removal — 타투 제거 (MISS 1/1)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 | 해석자 |
|---|---|---|---|---|
| 타투 제거 n=6 면역학적 아키텍처 | link[목표] | `docs/tattoo-removal/goal.md` | `$N6_ARCH/docs/tattoo-removal/goal.md` | N6A |

## PASS 상세 (참조)

### tech-industry — 기술/산업 (PASS 3/50)

| 제품 | 종류 | 선언 경로 | 해석 절대경로 |
|---|---|---|---|
| 합성생물학 n=6 이중 완전수 | link[도메인] | `domains/life/synbio/synbio.md` | `$N6_ARCH/domains/life/synbio/synbio.md` |
| 합성생물학 n=6 이중 완전수 | link[goal] | `domains/life/synbio/goal.md` | `$N6_ARCH/domains/life/synbio/goal.md` |
| 합성생물학 n=6 이중 완전수 | link[논문] | `papers/n6-synthetic-biology-paper.md` | `$N6_ARCH/papers/n6-synthetic-biology-paper.md` |

## 판정 주석 (정직한 검증 원칙)

- 본 감사는 products.json 선언값만 입력으로 삼고, 파일 시스템 존재 여부만 측정했다.
- 자기참조 금지 — 제품 내부 field 간 상호 확인이 아니라 실제 파일 존재를 검증했다.
- 실측 경로는 그대로 출력했다 (경로 추측/보정 없음).
- 관찰: 선언된 상대경로는 대부분 `docs/...` 접두사이나, `$N6_ARCH/docs/` 에는 해당 계층이 존재하지 않는다.
- `domains/...`, `papers/...` 접두사 중 3건만이 n6-architecture 트리에 실재한다 (tech-industry:합성생물학).
- `nexus/shared/n6/scripts/` 접두사를 가진 verify_script는 0건이었다.
- 드리프트 원인 가설: products.json이 `nexus/shared/n6/` 내부 문서 트리를 기준으로 작성되었으나, n6-architecture 리포 내부에는 해당 docs 디렉터리가 분리/미싱크 상태.
- 본 리포트는 수정 권고가 아닌 현재 상태 측정치이며, products.json은 건드리지 않았다.

## 산출 메타

- 리포트 경로: `$N6_ARCH/reports/audits/products-link-audit-2026-04-11.md`
- 감사 대상 해시성 원본: 154868 bytes
- 기록 범위: 섹션 34개 / 제품 173개 / 링크·스크립트 416건
