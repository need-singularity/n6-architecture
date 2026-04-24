# MISS Link Audit Report

**Date**: 2026-04-12
**Scope**: papers/_products.json all links + papers/_papers.json ghost papers
**Author**: ROI #11 + ROI #15 integrated audit

---

## 1. Audit Summary

| Item | Count |
|------|------|
| _products.json total links | 248 |
| _products.json MISS links | 6 (other files; paper MISS 0) |
| _papers.json registered papers | 96 |
| Pre-audit ghost papers | 10 |
| Post-audit ghost papers | 0 (all resolved) |

---

## 2. Ghost Paper Resolution (ROI #11)

### 2.1 10 Resolved Papers (chunk_c, human body / medical / biology)

| # | File | Lines | EXACT | Field |
|---|--------|------|-------|------|
| 1 | n6-hexa-neuro-paper.md | 237 | 15/15 (100%) | neuromorphic chip / BCI |
| 2 | n6-hexa-mind-paper.md | 255 | 36/40 (90%) | cognitive science / psychology |
| 3 | n6-hexa-telepathy-paper.md | 192 | 10/10 (100%) | brain-brain communication / BBI |
| 4 | n6-hexa-dream-paper.md | 181 | 20/20 (100%) | sleep science / lucid dreaming |
| 5 | n6-hexa-skin-paper.md | 179 | 20/20 (100%) | electronic skin / haptic |
| 6 | n6-hexa-exo-paper.md | 189 | 20/20 (100%) | AI exoskeleton / SE(3) |
| 7 | n6-hexa-limb-paper.md | 197 | 24/24 (100%) | AI prosthetic limb |
| 8 | n6-hexa-olfact-paper.md | 188 | 20/20 (100%) | digital olfaction / electronic nose |
| 9 | n6-entomology-paper.md | 226 | 23/23 (100%) | entomology / Hexapoda |
| 10 | n6-dolphin-bioacoustics-paper.md | 222 | 18/18 (100%) | dolphin bioacoustics |

Total: 2,066 lines, average EXACT ratio 97%

### 2.2 Elements Included in Each Paper

All papers include:
- Title, abstract, keywords (Korean in source)
- Foundation (n=6 core identities)
- Domain (core constant table)
- Performance comparison (ASCII graph)
- Verifiable predictions (TP)
- Limitations and MISS disclosure
- n=6 connection summary
- Cross-domain connections
- Appendix A: Python-embedded verification code (N62 compliant)
- References

---

## 3. _products.json MISS Link Audit (ROI #15)

### 3.1 Current 6 MISS Links (all non-paper files)

| # | Path | Label | Section | Product |
|---|------|------|------|------|
| 1 | experiments/experiment_full_n6_pipeline.py | other | ai | Full N6 Pipeline |
| 2 | domains/energy/hvac-system/verify.hexa | other | tech-industry | HVAC heating/cooling |
| 3 | domains/infra/earthquake-engineering/verify.hexa | other | tech-industry | Earthquake design |
| 4 | domains/materials/concrete-technology/verify.hexa | other | tech-industry | Concrete + carbon capture |
| 5 | domains/infra/smart-city/verify.hexa | other | tech-industry | Smart city |
| 6 | domains/infra/civil-engineering/verify.hexa | other | tech-industry | Civil / structural mechanics |

### 3.2 MISS Type Analysis

- Paper MISS: 0 (all paper MISS resolved via the 10 ghost papers)
- Calculator MISS: 0
- hexa verification code MISS: 5 (verify.hexa not generated)
- Experiment code MISS: 1 (.py file -- N62 rule forbids .py, path itself needs review)

### 3.3 Resolution Recommendations

| Priority | MISS Item | Resolution |
|----------|-----------|-----------|
| High | experiment_full_n6_pipeline.py | Convert .py -> .hexa or delete path (N62 forbids .py) |
| Medium | hvac-system/verify.hexa | Create verify.hexa stub in domain folder |
| Medium | earthquake-engineering/verify.hexa | Create verify.hexa stub in domain folder |
| Medium | concrete-technology/verify.hexa | Create verify.hexa stub in domain folder |
| Medium | smart-city/verify.hexa | Create verify.hexa stub in domain folder |
| Medium | civil-engineering/verify.hexa | Create verify.hexa stub in domain folder |

---

## 4. Pre/Post Audit Comparison

```
  Before Audit:
  |-- ghost papers: 10 (all of chunk_c)
  |-- _products.json MISS: 6
  +-- total MISS: 16 (ghost 10 + links 6)

  After Audit:
  |-- ghost papers: 0 (all resolved)
  |-- _products.json MISS: 6 (non-paper files)
  +-- total MISS: 6 (paper MISS 0 + other 6)

  Resolution rate: 10/16 = 62.5%
```

---

## 5. Registry Update Notes

- _papers.json: ghost_resolved metadata added
- _registry.json: papers_chunk_c ghost_status updated

---

## 6. Next Steps

1. Of the remaining 6 MISS, the 5 verify.hexa items will be a separate ROI to generate hexa stubs
2. experiment_full_n6_pipeline.py violates N62 rule -- change path to .hexa or delete
3. Confirm all 96 papers ready for Zenodo/OSF publication
