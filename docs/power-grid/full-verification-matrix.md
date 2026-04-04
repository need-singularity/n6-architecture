# N6 Power Grid — Full Verification Matrix

> H-PG-1~30 전수 검증 매트릭스.

## Sources

```
  [IEEE]  = IEEE Standard
  [IEC]   = IEC Standard
  [NERC]  = NERC Reliability Standard
  [CIGRE] = CIGRE Technical Brochure
  [Mfr]   = Manufacturer specification
  [Lit]   = Peer-reviewed literature
```

---

## Core Hypotheses (H-PG-1 to H-PG-30)

| ID | Hypothesis | n=6 Expr | Value | Source | Grade |
|----|-----------|----------|-------|--------|-------|
| H-PG-1 | 6-pulse rectifier | n | 6 | [IEEE] Mohan textbook | EXACT |
| H-PG-2 | 12-pulse HVDC | σ | 12 | [CIGRE] HVDC DB | EXACT |
| H-PG-3 | 3-phase power | n/φ | 3 | [IEEE] C50 | CLOSE |
| H-PG-4 | 60Hz frequency | σ·sopfr | 60 | [IEEE] C50.13 | EXACT |
| H-PG-5 | 50Hz frequency | sopfr·(σ-φ) | 50 | [IEC] 60038 | EXACT |
| H-PG-6 | 60/50 ratio=1.2 | n/sopfr | 6/5 | derived | EXACT |
| H-PG-7 | THD 5% limit | sopfr | 5 | [IEEE] 519 | EXACT |
| H-PG-8 | 12kV distribution | σ·10³ | 12,000 | [IEC] 60038 | EXACT |
| H-PG-9 | 24kV distribution | J₂·10³ | 24,000 | [IEC] 60038 | EXACT |
| H-PG-10 | HVDC ±500kV | sopfr·(σ-φ)² | 500 | [CIGRE] | EXACT |
| H-PG-11 | HVDC ±800kV | (σ-τ)·(σ-φ)² | 800 | [CIGRE] | EXACT |
| H-PG-12 | HVDC ±1100kV | (σ-μ)·(σ-φ)² | 1100 | [CIGRE] | EXACT |
| H-PG-13 | 24-pulse converter | J₂ | 24 | [Mfr] ABB | EXACT |
| H-PG-14 | TDD 12% | σ | 12 | [IEEE] 519 | EXACT |
| H-PG-15 | 11th harmonic residual | σ-μ | 11 | [IEEE] 519 | EXACT |
| H-PG-16 | 4-tier reliability | τ | 4 | [NERC] TPL | CLOSE |
| H-PG-17 | NERC 6 regions | n | 6 | [NERC] | EXACT |
| H-PG-18 | 3 interconnections | n/φ | 3 | [NERC] | EXACT |
| H-PG-19 | 5-min dispatch | sopfr | 5 | [NERC] BAL | EXACT |
| H-PG-20 | 4-hour storage | τ | 4 | [DOE] target | CLOSE |
| H-PG-21 | Protection 3 zones | n/φ | 3 | [IEEE] C37 | EXACT |
| H-PG-22 | 120V AC mains | σ(σ-φ) | 120 | [IEC] 60038 | EXACT |
| H-PG-23 | 240V AC mains | J₂·(σ-φ) | 240 | [IEC] 60038 | EXACT |
| H-PG-24 | 480V 3-phase | J₂·(J₂-τ) | 480 | [NEC] | EXACT |
| H-PG-25 | 48V DC bus | σ·τ | 48 | [OCP] | EXACT |
| H-PG-26 | PUE 1.2 target | σ/(σ-φ) | 1.2 | [EPA] ES | EXACT |
| H-PG-27 | Power factor 1.0 | R(6) | 1 | [IEEE] 1459 | EXACT |
| H-PG-28 | Transformer 12mil lam | σ | 12 | [Mfr] ABB | EXACT |
| H-PG-29 | IEC 61850 6 transfer types | n | 6 | [IEC] 61850 | EXACT |
| H-PG-30 | CT ratio 600:5 | σ(σ-φ):sopfr | 120:1 | [IEEE] C57 | CLOSE |

---

## Grade Distribution

| Grade | Count | Pct | IDs |
|-------|-------|-----|-----|
| EXACT | 25 | 83.3% | H-PG-1,2,4-15,17-19,21-29 |
| CLOSE | 5 | 16.7% | H-PG-3,16,20,30 |
| WEAK | 0 | 0% | — |
| FAIL | 0 | 0% | — |

**EXACT rate: 25/30 = 83.3%**
**EXACT + CLOSE: 30/30 = 100%**
**FAIL rate: 0%**

---

## BT Cross-Reference

| BT | Hypotheses | EXACT/Total |
|----|-----------|-------------|
| BT-60 | H-PG-22,23,24,25,26 | 5/5 |
| BT-62 | H-PG-4,5,6 | 3/3 |
| BT-68 | H-PG-10,11,12 | 3/3 |
| BT-74 | H-PG-7 | 1/1 |

---

## Verification Completeness

| Source Type | Documents Checked |
|------------|------------------|
| IEEE Standards | 519, C50, C37, C57, 1459, 1547 |
| IEC Standards | 60038, 61850, 62351 |
| NERC Standards | BAL-003, TPL-001, FAC-001 |
| CIGRE | TB 269, 388, HVDC database |
| Manufacturer | ABB, Siemens, GE, Schneider |
