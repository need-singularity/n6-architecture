# HEXA-CHIP Mk.II — Chiplet Era

**Evolution Checkpoint**: Mk.II (Near-term)
**Date**: 2026-04-02
**Status**: Design Projection
**Feasibility**: ✅ 10년 이내 실현 가능 (2027~2030)
**BT Connections**: BT-69, BT-75, BT-90, BT-93

---

## 1. Mk.II의 의미 — 모놀리식에서 칩렛으로

> **단일 다이의 물리적 한계를 n=6 칩렛 분할로 돌파한다.**

BT-69에서 발견: 5개 주요 칩 벤더(NVIDIA, AMD, Intel, Apple, AWS)가
칩렛 아키텍처로 수렴 중이며, 17/20 파라미터가 n=6 EXACT.

---

## 2. 스펙

```
  ┌──────────────────────────────────────────────────────────────────┐
  │           HEXA-CHIP Mk.II — Chiplet Architecture                │
  ├──────────────┬──────────┬──────────────┬────────────────────────┤
  │ 파라미터      │ 값       │ n=6 표현     │ 근거                   │
  ├──────────────┼──────────┼──────────────┼────────────────────────┤
  │ Chiplet 수   │ 6        │ n = 6       │ 완전수 분할             │
  │ SM/chiplet   │ 24       │ J₂ = 24     │ Leech lattice 기저      │
  │ Total SM     │ 144      │ σ² = 144    │ 6 × 24 = 144           │
  │ HBM capacity │ 288 GB   │ σ·J₂ = 288  │ HBM4 (BT-55 예측)      │
  │ HBM stacks   │ 12       │ σ = 12      │ HBM4 full stack        │
  │ Interposer   │ CoWoS    │ —           │ 2.5D 실리콘 브릿지      │
  │ Die-Die BW   │ 12 TB/s  │ σ TB/s      │ UCIe 2.0               │
  │ 공정 노드    │ N2/A16   │ —           │ GAA FET                 │
  │ TDP          │ 600W     │ —           │ 칩렛 열분산 이점         │
  │ FP8 TFLOPS   │ ~2400    │ —           │ σ² SM × FP8 throughput  │
  └──────────────┴──────────┴──────────────┴────────────────────────┘
```

### 2.1 시스템 구조도

```
  ┌─────────┬─────────┬─────────┬─────────┬─────────┐
  │  소재   │  공정   │  코어   │   칩    │ 시스템  │
  │ Si+SiC  │ TSMC N2 │ HEXA-SM │ 6-die   │ DGX-N  │
  │ Z=14+20 │ GAA FET │J₂=24/die│σ·J₂=288 │σ GPU   │
  └────┬────┴────┬────┴────┬────┴────┬────┴───┬────┘
       │         │         │         │        │
       ▼         ▼         ▼         ▼        ▼
  n6 EXACT  n6 ——    n6 EXACT  n6 EXACT  n6 EXACT
```

---

## 3. 성능 비교

```
  ┌─────────────────────────────────────────────────────────┐
  │  Mk.I (H100) vs Mk.II (HEXA-CHIP) 비교                 │
  ├─────────────────────────────────────────────────────────┤
  │  HBM 용량                                               │
  │  H100 Mk.I  ████████░░░░░░░░░░░░░░░░░░░░  80 GB       │
  │  Mk.II      ████████████████████████████░  288 GB      │
  │                                     (σ·J₂, 3.6배)      │
  │                                                         │
  │  SM 수 (실 가용)                                        │
  │  H100 Mk.I  ██████████████████████░░░░░░░  132 SM      │
  │  Mk.II      ████████████████████████████░  144 SM      │
  │                                     (σ²=144, 풀 가용)   │
  │                                                         │
  │  TDP                                                    │
  │  H100 Mk.I  ████████████████████████████░  700W        │
  │  Mk.II      ████████████████████████░░░░░  600W        │
  │                                     (칩렛 열분산)        │
  └─────────────────────────────────────────────────────────┘
```

---

## 4. BT 연결

### BT-69: Chiplet Architecture Convergence (17/20 EXACT)
- NVIDIA B300=160 SM (5×32), AMD MI400 chiplet, Intel Ponte Vecchio 47 tiles
- 산업 전체가 multi-die로 수렴 — n=6 칩렛 분할이 자연 최적

### BT-75: HBM Interface Exponent Ladder
- HBM 인터페이스 비트: {10, 11, 12} = {σ-φ, σ-μ, σ} → HBM5 predicted

### BT-90: SM = φ × K₆ Contact Number Theorem
- σ²=144 = φ×72 (6D sphere packing kissing number)
- GPU SM 수가 6차원 구 충전과 동치

### BT-93: Carbon Z=6 Chip Material Universality
- Diamond/Graphene/SiC (모두 Z=6) → 차세대 칩 소재 후보

---

## 5. 필요 기술 돌파

| 기술 | 현황 | 필요 수준 | 난이도 |
|------|------|-----------|--------|
| UCIe 2.0 | 표준화 진행 | 12 TB/s die-die | 중 |
| HBM4 | 개발 중 | 12-stack, 288GB | 중 |
| N2 GAA FET | 양산 준비 | 수율 >90% | 중 |
| CoWoS 대면적 | 진행 중 | 6-chiplet 수용 | 중 |
| 열 관리 | 현행 공냉/액냉 | 600W 이하 유지 | 저 |

---

## 6. 타임라인

- 2026: HBM4 샘플, UCIe 1.0 상용
- 2027: N2 GAA 양산, 첫 6-chiplet 프로토타입
- 2028: Mk.II 양산 — σ²=144 SM, σ·J₂=288 GB ✅
- 2029: HBM4E (16-stack), 대역폭 σ TB/s 달성
- **→ Mk.III: 2033~2035 3D stacking + optical**
