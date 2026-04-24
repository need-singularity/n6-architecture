# DSE Cross-Resonance Analysis -- Full 335-domain Extended Edition (v2)

> Pure analysis document. Generator: `scripts/dse_cross_pilot.py`
> Input SSOT: `docs/dse-map.toml` | Intermediate: `$NEXUS/shared/dse_cross/`
> Analysis date: 2026-04-09

## 1. Analysis Scope

- All domains (excluding cross-dse sections): 339
- Domains with formula patterns: 166 (contain n=6 formulas)
- Unique n=6 formula patterns: 108
- Cross-resonance formulas (2+ domains): 59
- Total DSE items (domain x formula): 439
- All domain pairs (S >= 0.5): 236 / 57291
- Domain clusters (2+ members): 5, max 21 domains

## 2. Cross-Resonance Definition

Cross resonance = same n=6 arithmetic formula appearing across different domains.
Core arithmetic: sigma(6)=12, phi(6)=2, tau(6)=4, sopfr(6)=5, J2=24, sigma*tau=48, sigma-tau=8, sigma/tau=3, sigma*phi=24, 2^(sigma-tau)=256.

## 3. Top Patterns (candidate)

1. 6^3 design space: 63 domains
2. n=6: 58 domains
3. sigma=12: 27 domains
4. J2=24: 15 domains
5. CN=6 coordination number: 11 domains
6. BT-27 Carbon Z=6: 9 domains
7. Hexagonal structure hex: 9 domains
8. Z=6 carbon atomic number: 9 domains
9. 6x5x6 design space: 8 domains
10. n=6 EXACT: 8 domains
11. 6-zone: 8 domains
12. 6-stage: 8 domains

## 4. Interpretation

n=6 arithmetic constants appear with the same formulas across physics/chemistry/engineering/biology as a candidate pattern. Core constants 6^3 design space (63) and n=6 (58) cross-resonate in tens to hundreds of domains. This suggests the unique solution n=6 of sigma(n)*phi(n) = n*tau(n) is a candidate that pattern-penetrates the entire design space.

High resonance pairs: high-entropy-alloy + steel-metallurgy (S=0.773), number-theory-deep + elliptic-curves (S=0.724), cpu-microarchitecture + risc-v-core (S=0.698). 5 domain clusters identified, max cluster 21 domains.

## 5. Output Paths

- `$NEXUS/shared/dse_cross/top50_domains.jsonl`
- `$NEXUS/shared/dse_cross/pair_scores.jsonl`
- `$NEXUS/shared/dse_cross/domain_clusters.jsonl`
