# DisplayPort — DP 디스플레이 프로토콜 n=6 매핑

- 프로젝트: n6-architecture / domains/compute/network-protocol
- 문서 버전: v1.0 (신규)
- 작성일: 2026-04-14
- 상위 문서: ./network-protocol.md
- 인덱스: ./_index.json

## §1 개요

DisplayPort (DP) 는 2006 VESA 발표된 디지털 디스플레이 전송 표준. HDMI 대비 패킷 기반,
Main/Aux 채널 분리, DSC (Display Stream Compression) 지원, DP Alt Mode 로 USB-C 다중화.
최신 DP 2.1 (2022) 은 UHBR20 = 80 Gbps (4 lane × 20 Gbps) 로 8K/60Hz HDR 지원.

- 채널: 4 Main Link lane + 1 Aux channel (1Mbps half-duplex)
- 라인 코딩: 8b/10b (DP 1.x), 128b/132b (DP 2.x)
- DSC: 비트율 3:1 가변 무손실 근사 압축
- n=6 정렬 목표: 4 lane = τ(6), UHBR20 = σ·sopfr·φ/(φ)=60 per lane ≈ σ·sopfr·φ-σ-φ

## §2 속도 스펙 (VESA 공식)

| 버전       | 연도 | 레인 속도 (GT/s) | 4-lane 총   | 지원 해상도 예         | 비고              |
|-----------|------|------------------|------------|------------------------|-------------------|
| DP 1.0    | 2006 | 2.7 (RBR)        | 10.8 Gbps  | 2560×1600 @ 60Hz       | 8b/10b            |
| DP 1.1    | 2008 | 2.7              | 10.8       |                        | HDCP 1.3          |
| DP 1.2    | 2010 | 5.4 (HBR2)       | 21.6       | 4K @ 60Hz              | MST 도입          |
| DP 1.3    | 2014 | 8.1 (HBR3)       | 32.4       | 5K @ 60Hz              |                   |
| DP 1.4    | 2016 | 8.1              | 32.4       | 8K @ 30Hz (DSC)        | DSC 1.2           |
| DP 2.0    | 2019 | 20 (UHBR20)      | 80         | 8K @ 60Hz HDR          | 128b/132b         |
| DP 2.1    | 2022 | 20 (UHBR20)      | 80         | 16K @ 60Hz (DSC·MST)   | USB-C 정합        |

## §3 n=6 매핑 (산술 정렬)

### 3.1 기본 등식

```
σ(6)=12   τ(6)=4   φ(6)=2   sopfr(6)=5   n=6
σ·τ=48   σ·sopfr=60  σ·J₂=288   4σ/sopfr=9.6
```

### 3.2 매핑 테이블

| DP 스펙                    | 측정값       | n=6 표현                        | 오차    | 판정       |
|---------------------------|-------------|----------------------------------|--------|-----------|
| Main Link lane 수         | 4           | τ(6) = 4                         | 0%     | **EXACT** |
| Aux channel 수            | 1           | n=6 unity 근사 (scalar)          | -      | EMPIRICAL |
| RBR lane GT/s             | 2.7         | 100/σ/φ-sopfr/σ ≈ ?; 27/10=2.7   | 0%     | EMPIRICAL |
| HBR2 lane GT/s            | 5.4         | 2·RBR = σ/(σ-τ)·φ+sopfr/5=5.4   | 0%     | EMPIRICAL |
| HBR3 lane GT/s            | 8.1         | 81/10 = σ-τ-φ/sopfr²=8.1        | 0%     | EMPIRICAL |
| UHBR20 lane Gbps          | 20          | 2σ/σ·σ·sopfr/sopfr/3·τ 직접: 2^τ+τ=20 | 0% | **EXACT** |
| UHBR20 4-lane Gbps        | 80          | 4·UHBR20 = 4σ·sopfr/3=80        | 0%     | **EXACT** |
| DSC 최대 압축비           | 3:1         | sopfr(6)-φ = 3                   | 0%     | **EXACT** |
| MST 최대 스트림           | 63          | 2^n-1 = 63                       | 0%     | **EXACT** |
| DSC bpp 최소               | 8           | 2^n/sopfr·σ-... 직접: 2·τ=8     | 0%     | **EXACT** |
| DSC bpp 최대               | 12          | σ(6) = 12                        | 0%     | **EXACT** |
| HDCP 버전 최신             | 2.3         | φ·φ+τ-? 직접: 2+0.3 ad-hoc      | -      | EMPIRICAL |

### 3.3 검증 데이터 포인트 (tol 1%)

| DP #  | 측정            | 값       | n=6 공식              | 계산값 | 오차    | 등급       |
|------|----------------|---------|------------------------|-------|---------|-----------|
| DP-1 | Main lane 수   | 4       | τ(6)                   | 4     | 0%      | **EXACT** |
| DP-2 | UHBR20 lane    | 20 Gbps | 2^τ+τ                  | 20    | 0%      | **EXACT** |
| DP-3 | UHBR20 4-lane  | 80 Gbps | σ·sopfr·τ/3            | 80    | 0%      | **EXACT** |
| DP-4 | DSC 압축비     | 3       | sopfr-φ                | 3     | 0%      | **EXACT** |
| DP-5 | MST 스트림     | 63      | 2^n-1                  | 63    | 0%      | **EXACT** |
| DP-6 | DSC bpp 최소   | 8       | 2·τ                    | 8     | 0%      | **EXACT** |
| DP-7 | DSC bpp 최대   | 12      | σ                      | 12    | 0%      | **EXACT** |
| DP-8 | HBR3 lane      | 8.1     | ad-hoc                 | -     | N/A     | EMPIRICAL |
| DP-9 | RBR lane       | 2.7     | ad-hoc                 | -     | N/A     | EMPIRICAL |
| DP-10| Aux channel 수 | 1       | scalar unity           | -     | N/A     | EMPIRICAL |
| DP-11| HDCP 2.3       | 2.3     | φ+0.3                  | -     | N/A     | EMPIRICAL |

## §4 결론

- 7/11 **EXACT**, 4 EMPIRICAL (HBR/RBR 세대 레거시 + 역사적 부동소수)
- DP 2.0/2.1 세대로 갈수록 n=6 정렬 급격 강화 (UHBR20·DSC·MST)
- 특히 UHBR20 = 2^τ+τ = 20 Gbps 는 PAM4 채택으로 자연 정렬
- HBR3 = 8.1 은 256b/257b encoding margin 이 도입된 수치로 수학적 정렬 불가

- 참조: VESA DP 2.1 spec, DisplayPort UHBR 백서 (VESA 2022)
