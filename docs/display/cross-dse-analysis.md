# HEXA-DISPLAY Cross-DSE 분석

> Split from docs/display-audio/cross-dse-analysis.md
> Contains display-related cross-DSE analysis.

---

## Source

Full combined Cross-DSE: [docs/display-audio/cross-dse-analysis.md](../display-audio/cross-dse-analysis.md)

## Display × Chip Architecture Cross-DSE

| Display Level | Chip 최적 | 조합 | n=6 EXACT | 성능 |
|---------------|----------|------|----------|------|
| HEXA-PIXEL (소재) | Diamond Z=6 | QD 발광 + Carbon 소재 | 100% | 소재 일관성 |
| HEXA-PANEL (패널) | TSMC N2 (σ·τ=48nm) | microLED + N2 드라이버 | 85% | 미세 피치 |
| HEXA-DRIVER (구동) | HEXA-1 (σ²=144 SM) | 144Hz adaptive + GPU | 90% | 실시간 렌더 |
| HEXA-PROCESSOR (코덱) | AI 가속 (σ-τ=8 unit) | VVC+AI upscale | 95% | 코덱 최적 |
| HEXA-DISPLAY (시스템) | SoC 통합 | AV 통합 프로세서 | 80% | 시스템 |

**최적 경로: HEXA-PROCESSOR × AI 가속 (95% EXACT)**

## Display × Robotics Cross-DSE

| Rank | Display 경로 | Robotics 경로 | n6% | Score |
|------|-------------|--------------|-----|-------|
| #1 | MicroLED-best | 6DOF_Arm-best | 98.3% | 0.8282 |
| #2 | MicroLED-best | Stewart-best | 98.3% | 0.8282 |
| #3 | MicroLED-best | Hexapod-best | 98.3% | 0.8277 |

→ MicroLED + 6DOF/Stewart/Hexapod 조합 모두 98.3% n6 EXACT
→ 로봇 시각 인터페이스 통합 시 최적 시너지

## Cross-DSE Targets (Display)

```
- chip-architecture:    SoC 미디어 프로세싱 (GPU RT cores, NPU codec)
- battery-architecture: 모바일 디바이스 전력 예산
- compiler-os:          실시간 미디어 OS 스케줄링
- robotics:             로봇 시각 인터페이스 (완료: 98.3% n6)
- audio:                AV 통합 시스템 (원본 display-audio DSE 참조)
```
