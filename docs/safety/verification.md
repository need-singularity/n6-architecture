# N6 Safety Architecture — Verification Matrix

> 각 가설의 출처, 검증 방법, 등급을 독립 검증.

## Core Hypotheses Verification (H-SF-01 ~ H-SF-30)

| ID | 가설 | n=6 수식 | 출처 | Grade |
|----|------|----------|------|-------|
| H-SF-01 | 화재 삼각형 3요소 | n/φ=3 | 연소화학 기본 | **EXACT** |
| H-SF-02 | 소방 분류 6등급 | n=6 | NFPA/KFS | **EXACT** |
| H-SF-03 | 배터리 열폭주 6단계 | n=6 | 논문/NREL | **CLOSE** |
| H-SF-04 | NFPA 704 4구역 | τ=4 | NFPA 704 | **EXACT** |
| H-SF-05 | SIL 4등급 | τ=4 | IEC 61508 | **EXACT** |
| H-SF-06 | 화재감지 6원리 | n=6 | 소방 실무 | **CLOSE** |
| H-SF-07 | 센서퓨전 12채널 | σ=12 | DC 모니터링 | **CLOSE** |
| H-SF-08 | LEL 경보 10% | σ-φ=10 | IEC 60079 | **EXACT** |
| H-SF-09 | 아크플래시 4등급 | τ=4 | NFPA 70E | **EXACT** |
| H-SF-10 | DC 안전전압 24V | J₂=24 | IEC 60364 | **EXACT** |
| H-SF-11 | 심층방호 6계층 | n=6 | IAEA/LOPA | **EXACT** |
| H-SF-12 | TMR 3다중화 | n/φ=3 | 항공/원자력 | **EXACT** |
| H-SF-13 | 소화약제 6종 | n=6 | 소방 표준 | **CLOSE** |
| H-SF-14 | 스프링클러 6등급 | n=6 | NFPA 13 | **EXACT** |
| H-SF-15 | 비상대응 6단계 | n=6 | FEMA/ISO | **CLOSE** |
| H-SF-16 | 방사선차폐 6소재 | n=6 | 핵공학 | **CLOSE** |
| H-SF-17 | 토카막 안전 6계통 | n=6 | ITER | **CLOSE** |
| H-SF-18 | 퀜치감지 0.1초 | 1/(σ-φ) | ITER/LHC | **EXACT** |
| H-SF-19 | GHS 그림문자 9종 | σ-n/φ=9 | UN GHS | **EXACT** |
| H-SF-20 | HAZOP 가이드워드 9종 | σ-n/φ=9 | IEC 61882 | **CLOSE** |
| H-SF-21 | 교토 온실가스 6종 | n=6 | BT-118 | **EXACT** |
| H-SF-22 | LOPA IPL 6계층 | n=6 | 화학공장 | **EXACT** |
| H-SF-23 | DC 소화 6구역 | n=6 | DC 설계 | **CLOSE** |
| H-SF-24 | DC 전압 체인 | BT-60 | BT-60 | **EXACT** |
| H-SF-25 | GFCI 30mA | sopfr·n=30 | IEC/NFPA | **EXACT** |
| H-SF-26 | 로봇 안전 4구역 | τ=4 | ISO 10218 | **EXACT** |
| H-SF-27 | 인체 부위 6그룹 | n=6 | ISO/TS 15066 | **CLOSE** |
| H-SF-28 | 비상정지 4카테고리 | τ=4 | IEC 60204 | **EXACT** |
| H-SF-29 | MMI 12등급 | σ=12 | USGS | **EXACT** |
| H-SF-30 | 보퍼트 0~12 | σ=12 | WMO | **EXACT** |

## Grade Summary

| Grade | Count | % |
|-------|-------|---|
| EXACT | 20 | 66.7% |
| CLOSE | 10 | 33.3% |
| WEAK | 0 | 0% |
| FAIL | 0 | 0% |
| **Total** | **30** | **100%** |

## Cross-Verification Notes

- H-SF-05 (SIL=τ=4)와 H-SFX-19 (DAL=sopfr=5): 같은 기능안전이지만 IEC vs RTCA 체계에서 서로 다른 n=6 상수 사용
- H-SF-11 (DiD=n=6)과 H-SFX-03 (Swiss cheese n=6): 독립 프레임워크에서 동일 값 수렴
- H-SF-25 (GFCI=30=sopfr·n)와 H-SF-10 (SELV=24=J₂): 전기안전 상수쌍이 n=6 체계 내 정합
- H-SFX-04 (Heinrich 300=sopfr·n·(σ-φ)): 가장 예상 밖의 EXACT — 독립 검증 우선순위 높음
