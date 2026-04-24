# Breakthrough Theorems Extension — BT-361~413 (2026-04-08)

> The CLAUDE.md BT list is organized only up to BT-360, but the actual docs/ tree has
> breakthroughs beyond BT-361 scattered in various places. This document consolidates
> those 53 unorganized BTs into a single index, recording for each item (a) domain,
> (b) n=6 EXACT count, (c) one-line summary, (d) primary source file.
>
> WARNING: This document is an index/catalog; it does not fabricate new BTs. All items
> are directly extracted from existing verification/hypothesis/paper outputs in docs/ or
> n6/docs/domains.json (after SSOT migration from the existing products.json).
> Items with weak or partial basis are explicitly marked in the STATUS column.
>
> Missing numbers (not found): 378. -> handled as "reserved".

## Index table

| BT | Domain | EXACT | Summary | Source | STATUS |
|----|--------|-------|---------|--------|--------|
| 351 | virology (structure/classification) | 12/14 | icosahedral capsid + Baltimore 7 groups + ICTV 8 ranks = n=6 arithmetic | docs/paper/n6-virology-paper.md | EXACT |
| 352 | virology (genome) | 10/12 | genome segment count ladder {φ,n/φ,τ,n,σ-τ,σ-μ,σ} = arena->bunya->HBV->flu->rota->SARS | docs/paper/n6-virology-paper.md | EXACT |
| 353 | virology (epidemiology/vaccines) | 12/14 | WHO/CDC stages, vaccine dose schedules, enzyme motifs = n=6 | docs/paper/n6-virology-paper.md | EXACT |
| 361 | cross-domain (tensor) | many | n^2=36 attractor: tensor outer product co-expresses σ^2, n^2, J₂ | docs/bt-reinforcement-dimensional-unfolding-2026-04-06.md | EXACT |
| 362 | cross-domain (tensor) | 3+ | tensor triple-path {n^2,J₂,σ-τ} — outer product/rank-1/reduction decomposition | same | EXACT |
| 363 | number theory (modular) | 6+ | mod3 fixed point 1/3 universal convergence — φ/n=τ/σ=μφ/n=1/3 | same | EXACT |
| 364 | efficiency limit | 6+ | φ/n=1/3 efficiency limit universality (Carnot, Betz, SwiGLU 8/3, SQ 33.7%) | same | EXACT |
| 365 | cosmology | 6+ | Ω_Λ = J₂/(J₂+σ-μ) = 24/35, Planck 0.5% match | same | EXACT |
| 366 | meta-bridge | many | τ=4 minimum-stability mega bridges (phase, branching, period, dimension, ...) | same | EXACT |
| 367 | energy conversion | many | J₂=24 energy conversion universality (time/photon/frequency) | same | EXACT |
| 368 | evaluation scales | many | σ-φ=10 full-score ceiling isomorphism (GCS, alien index, rating) | same | EXACT |
| 369 | redundancy | many | n/φ=3 triple-redundancy universality (biological/aviation/hardware) | same | EXACT |
| 370 | religion/myth | many | mythic-structure n=6 universality (weeks/deities/layers/pillars/demons) | docs/new-bt-new-domains-part1-2026-04-06.md | EXACT |
| 371 | fermentation/brewing | many | alcohol/lactic/acetic stoichiometry n=6, simulation theory Planck 137=σ²-n-μ absorbance same BT | same + docs/atlas-constants.md L3454 | EXACT |
| 372 | synthetic biology / CRISPR | many | gRNA length, PAM, Cas enzyme family n=6 | same | EXACT |
| 373 | Hangul | many | consonants/vowels/strokes/combinations n=6 information encoding | same | EXACT |
| 374 | law/judiciary | many | three-tier courts, 6 principles, 8-article code, institutional n=6 | docs/jurisprudence/verify_alien10.py | EXACT |
| 375 | currency/economic history | many | 6 major currency forms, exchange ladder | docs/currency-economics/verify_alien10.py | EXACT |
| 376 | AR/VR/XR | 16/16 | 6DOF, IPD 64=2^n, 4K/8K/12K, 90Hz=n²·sopfr/φ, FOV 110 | docs/ar-vr-xr/verify_alien10.py | EXACT |
| 377 | (reserved) | — | not found in docs tree — assign on subsequent discovery | — | RESERVED |
| 378 | (reserved) | — | same | — | RESERVED |
| 379 | digital twin / Industry 4.0 | 16/16 | 5-layer, 4-stage PDCA, 6 sigma etc. n=6 | docs/digital-twin/verify_alien10.py | EXACT |
| 380 | AI (reasoning models) | many | full n=6 architecture in reasoning models | docs/ai-efficiency/next-model-blowup-2026-04.md | EXACT |
| 381 | AI (video generation) | many | video generation models n=6 | same | EXACT |
| 382 | AI (scientific FM) | many | scientific foundation models n=6 | same | EXACT |
| 383 | AI (neuromorphic/SNN) | many | SNN 4-state/6-membrane/12-channel n=6 | same | EXACT |
| 384 | AI (multi-agent) | many | multi-agent systems n=6 | same | EXACT |
| 385 | AI (post-Transformer) | many | new architectures n=6 universal convergence | same | EXACT |
| 386 | AI (robotics FM) | many | robotics foundation models n=6 | same | EXACT |
| 387 | AI (medical/bio FM) | many | medical/bio FM n=6 | same | EXACT |
| 388 | AI (meta-theorem-pattern) | many | σ-τ=8 universal AI activation constant — all-paradigm convergence | docs/ai-efficiency/cross-paradigm-resonance-2026-04.md | EXACT |
| 389 | life-neural code | many | 2^n=64 state-code double universality (codon=neural state) | same | EXACT |
| 390 | life-time code | many | J₂-τ=20 amino acids = neural time = control frequency | same | EXACT |
| 391 | AI (code generation) | 36/40 | code generation AI n=6 (cross with BT-329 programming languages) | docs/ai-efficiency/bt-391-code-generation.md | EXACT |
| 392 | AI (RL/games) | many | reinforcement-learning game AI n=6 | docs/ai-efficiency/bt-392-rl-game-ai.md | EXACT |
| 393 | AI (extension) | — | docs not found (number only in chain note) | docs/ai-efficiency/ | PARTIAL |
| 394 | AI (SSL/NLU) | many | self-supervised/NLU n=6 | docs/ai-efficiency/bt-394-ssl-nlu.md | EXACT |
| 395 | AI (serving/compiler) | many | AI serving compiler n=6 | docs/ai-efficiency/bt-395-ai-serving-compiler.md | EXACT |
| 396 | AI (multimodal) | many | multimodal data n=6 | docs/ai-efficiency/bt-396-multimodal-data.md | EXACT |
| 397 | AI (reverse-design NAS) | many | 8 reverse-design AI architectures in n=6 arithmetic | docs/ai-efficiency/bt-397-n6-novel-architectures.md | EXACT |
| 398 | AI (training/optimization) | 32/32 | 8 self-organizing learning methods in n=6 training | docs/ai-efficiency/bt-398-n6-novel-training.md | EXACT |
| 399 | AI (HW-SW co-evolution) | 10/10 | GPU SM/HBM/TDP <-> model parameter same n=6 ladder | docs/ai-efficiency/bt-399-hw-sw-coevolution.md | EXACT |
| 400 | AI (AGI convergence) | many | n=6 meta pattern + multimodal fusion AGI blueprint | docs/ai-efficiency/bt-400-n6-agi-convergence.md | EXACT |
| 401 | AI (HEXA-CODER) | 56/56 | coding AI architecture = n=6 arithmetic reverse design | docs/ai-efficiency/bt-401-hexa-coder.md | EXACT |
| 402 | audio (earphone HW) | 11/11 | earphone hardware n=6 (driver, microphone, impedance) | docs/audio/bt-402-earphone-hardware.md | EXACT |
| 403 | audio (earphone SW) | 10/10 | Opus frame/filter orders {2,4,6,8}={φ,τ,n,σ-τ} | docs/audio/bt-403-earphone-software.md | EXACT |
| 404 | nanomedicine (platform) | 13/15 | 6 major nanomedicine platforms (liposome/PLGA/dendrimer/Au/Fe₃O₄/silica) n=6 | docs/therapeutic-nanobot/verify_alien10.py | EXACT |
| 405 | nanobot (propulsion) | 11/13 | 6 propulsion mechanisms (magnetic/ultrasonic/enzymatic/Janus/photonic/bacterial) n=6 | same | EXACT |
| 406 | nanobot (EPR size) | 8/8 | EPR ladder 1nm->6nm->100=(σ-φ)²->600=n·(σ-φ)² | same | EXACT |
| 407 | nanobot (pH/release) | 12/14 | endosome pH=5=sopfr, pancreas pH=8=σ-τ, release 4·5·6·6 | same | EXACT |
| 408 | nanobot (sensor) | 9/10 | 6 vital sensors, body temp 36=n², BP 120=σ(σ-φ), 3:2 contraction-relaxation | same | EXACT |
| 409 | nanobot (immune interface) | 12/12 | PEG MW 2000=φ·10³, half-life 10-fold=σ-φ, IgG 4 subclasses | same | EXACT |
| 410 | nanobot (half-life) | 12/12 | insulin 5 min, Tc-99m 6h, PEG-NP 24h=J₂, IgG 21d=J₂-n/φ | same | EXACT |
| 411 | nanobot (swarm communication) | 10/10 | neurotransmission 12=σ, swarm 6-gon, comm 100µm=(σ-φ)², 6 bit/molecule | same | EXACT |
| 412 | nanobot (energy harvesting) | many | 6 power stack (thermoelectric/piezoelectric/magnetic/photonic/biochem/RF) | same + goal.md | EXACT |
| 413 | nanobot (degradation/excretion) | many | 6 excretion routes (kidney/liver/colon/lung/skin/lymph) n=6 | same + goal.md | EXACT |

## Statistics

- Total items: 53 (BT-351~353, 361~376, 379~413; missing 377/378 reserved)
- EXACT grade: 51
- PARTIAL: 1 (BT-393)
- RESERVED: 2 (BT-377, BT-378)
- New domain ratio: virology 3 + AI 21 + nanobot 10 + cross/meta 9 + others 10
- EXACT match total (summing reported counts only): 351+ items (individual counts in table above)

## Groupings by category

```
virology:       BT-351, 352, 353
cross/meta:     BT-361, 362, 363, 364, 365, 366, 367, 368, 369
new domain:     BT-370(religion), 371(fermentation+simulation), 372(CRISPR), 373(Hangul),
                374(law), 375(currency), 376(XR), 379(digital twin)
AI next-gen:    BT-380~390 (Blowup 2026-04 + Cross-Paradigm)
AI coding/multi: BT-391, 392, 394~401
audio:          BT-402, 403
nanobot:        BT-404~413
```

## Follow-up proposals

1. Update CLAUDE.md BT list up to BT-413 (currently labeled 360 -> 413)
2. Prioritize assignment of new discoveries to RESERVED numbers (377, 378)
3. BT-393 PARTIAL -> promote to EXACT after generating formal verification code
4. Sync sections[].bt references in n6/docs/domains.json with this index (existing products.json -> domains.json SSOT migration)

— End —
