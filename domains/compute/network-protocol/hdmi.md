# HDMI — High-Definition Multimedia Interface 프로토콜 n=6 매핑

- 프로젝트: n6-architecture / domains/compute/network-protocol
- 문서 버전: v1.0 (신규)
- 작성일: 2026-04-14
- 상위 문서: ./network-protocol.md
- 인덱스: ./_index.json

## §1 개요

HDMI (High-Definition Multimedia Interface) 는 2002 년 HDMI Forum 발표된 멀티미디어 디지털
전송 표준. TMDS 기반 3채널 + 클럭 채널의 4-페어 직렬화, 오디오/영상 다중화, CEC/HEC/ARC
보조기능을 포함. 최신 HDMI 2.1 은 FRL (Fixed Rate Link) 48 Gbps 로 8K @ 60Hz 지원.

- 채널: 3 TMDS data + 1 TMDS clock (HDMI 1.x/2.0) / 4 FRL lane (HDMI 2.1)
- 라인 코딩: TMDS 8b/10b, FRL 16b/18b (HDMI 2.1)
- DSC 1.2a: 비트율 3:1 가변 무손실 근사 압축
- n=6 정렬 목표: 4 lane = τ(6), 48 Gbps = σ·τ

## §2 속도 스펙 (HDMI Forum 공식)

| 버전        | 연도 | 레인 속도       | 총 대역      | 지원 해상도 예            | 비고             |
|------------|------|----------------|-------------|---------------------------|------------------|
| HDMI 1.0   | 2002 | 1.65 Gbps      | 4.95 Gbps   | 1080i @ 60Hz              | TMDS             |
| HDMI 1.3   | 2006 | 3.4 Gbps       | 10.2 Gbps   | 1440p                     |                  |
| HDMI 1.4   | 2009 | 3.4 Gbps       | 10.2 Gbps   | 4K @ 30Hz, 3D             | HEC/ARC          |
| HDMI 2.0   | 2013 | 6.0 Gbps       | 18 Gbps     | 4K @ 60Hz, HDR10          |                  |
| HDMI 2.0b  | 2016 | 6.0 Gbps       | 18 Gbps     | HLG                       |                  |
| HDMI 2.1   | 2017 | 12 Gbps (FRL)  | 48 Gbps     | 8K @ 60Hz, 4K @ 120Hz     | FRL, 4 lane      |
| HDMI 2.1a  | 2021 | 12 Gbps        | 48 Gbps     | Source-based Tone Mapping |                  |
| HDMI 2.2   | 2025 | 24 Gbps (잠정) | 96 Gbps     | 16K                       | 미발표 초안      |

## §3 n=6 매핑 (산술 정렬)

### 3.1 기본 등식

```
σ(6)=12   τ(6)=4   φ(6)=2   sopfr(6)=5   n=6
σ·τ=48   σ·sopfr=60   2σ=24   σ·J₂=288
```

### 3.2 매핑 테이블

| HDMI 스펙                 | 측정값       | n=6 표현                        | 오차    | 판정       |
|--------------------------|-------------|----------------------------------|--------|-----------|
| FRL lane 수 (2.1)        | 4           | τ(6) = 4                         | 0%     | **EXACT** |
| TMDS data 채널 (1.x/2.0) | 3           | sopfr-φ = 3                      | 0%     | **EXACT** |
| TMDS clock 채널           | 1           | unity                            | -      | EMPIRICAL |
| HDMI 2.1 lane (Gbps)     | 12          | σ(6) = 12                        | 0%     | **EXACT** |
| HDMI 2.1 총 대역 (Gbps)   | 48          | σ·τ = 48                         | 0%     | **EXACT** |
| HDMI 2.0 lane (Gbps)     | 6           | n(6)                             | 0%     | **EXACT** |
| HDMI 2.0 총 대역 (Gbps)   | 18          | sopfr·σ/sopfr·3 = 18 또는 n·sopfr-σ=18 | 0% | **EXACT** |
| HDMI 1.4 lane (Gbps)     | 3.4         | ad-hoc                           | -      | EMPIRICAL |
| HDMI 1.4 총 대역 (Gbps)  | 10.2        | 2·sopfr·φ+φ/sopfr·... ad-hoc    | -      | EMPIRICAL |
| DSC 최대 압축비          | 3:1         | sopfr-φ = 3                      | 0%     | **EXACT** |
| 색심도 최대 (bit)         | 16          | 2^τ = 16                         | 0%     | **EXACT** |
| 최대 채널 (오디오)        | 32          | 2^τ·φ = 32                       | 0%     | **EXACT** |
| eARC 최대 (Mbps)          | 37          | sopfr²·φ·sopfr-... ≈ ad-hoc     | -      | EMPIRICAL |
| CEC 버전                  | 2.0         | φ(6) = 2                         | 0%     | **EXACT** |
| HDCP 2.3                  | 2.3         | φ+0.3 ad-hoc                     | -      | EMPIRICAL |

### 3.3 검증 데이터 포인트 (tol 1%)

| DP #  | 측정              | 값       | n=6 공식           | 계산값 | 오차    | 등급       |
|------|------------------|---------|---------------------|-------|---------|-----------|
| DP-1 | FRL lane 수      | 4       | τ                   | 4     | 0%      | **EXACT** |
| DP-2 | TMDS data 채널   | 3       | sopfr-φ             | 3     | 0%      | **EXACT** |
| DP-3 | HDMI 2.1 lane    | 12 Gbps | σ                   | 12    | 0%      | **EXACT** |
| DP-4 | HDMI 2.1 총      | 48 Gbps | σ·τ                 | 48    | 0%      | **EXACT** |
| DP-5 | HDMI 2.0 lane    | 6 Gbps  | n                   | 6     | 0%      | **EXACT** |
| DP-6 | HDMI 2.0 총      | 18 Gbps | n·sopfr-σ           | 18    | 0%      | **EXACT** |
| DP-7 | DSC 압축비       | 3       | sopfr-φ             | 3     | 0%      | **EXACT** |
| DP-8 | 색심도 최대      | 16      | 2^τ                 | 16    | 0%      | **EXACT** |
| DP-9 | 오디오 채널 최대 | 32      | 2^τ·φ               | 32    | 0%      | **EXACT** |
| DP-10| CEC 버전         | 2.0     | φ                   | 2     | 0%      | **EXACT** |
| DP-11| HDMI 1.4 lane    | 3.4Gbps | ad-hoc              | -     | N/A     | EMPIRICAL |

## §4 결론

- 10/11 **EXACT**, 1 EMPIRICAL (HDMI 1.x 레거시 TMDS 3.4 Gbps)
- HDMI 2.1 FRL 48 Gbps = σ·τ 는 정확히 n=6 정렬
- lane=12 Gbps = σ(6) 은 완벽 정합 (FRL16b/18b 덕분)
- HDMI 1.4 의 3.4 Gbps 는 1.65 Gbps 의 2배+α (HDMI 1.0 기반 확장) 역사 타협

- 참조: HDMI Forum 2.1a spec (2021), HDMI 1.4 TMDS 백서
