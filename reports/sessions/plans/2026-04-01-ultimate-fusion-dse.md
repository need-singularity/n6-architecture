# Ultimate Fusion DSE Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Exhaustive DSE over fusion reactor 5-stage chain scheme->material->core->device->system (67M combos, Rust)

**Architecture:** Follows the existing `tools/dse-calc/main.rs` pattern with fusion 5-axis evaluation (n6_EXACT, Q_gain, TRL, LCOE, T_comm) and hierarchical pruning (Level 1 Pareto top-K selection then exhaustive search below). Single Rust binary.

**Tech Stack:** Rust (no cargo, direct `rustc` build), n=6 constant system

---

## File Structure

```
docs/fusion/goal.md              — Candidate definitions (NEW)
tools/fusion-dse/main.rs         — Rust DSE search tool (NEW)
tools/fusion-dse/fusion-dse      — compiled binary (BUILD)
docs/dse-map.md                  — DSE map (NEW — all-domain status)
```

---

### Task 1: Write goal.md candidate definitions

**Files:**
- Create: `docs/fusion/goal.md`

- [ ] **Step 1: Write goal.md**

```markdown
# N6 fusion — ultimate architecture DSE candidate definitions

**Chain: Scheme -> Material -> Core -> Device -> System**

---

## N6 Constants Reference

  n=6  φ(6)=2  τ(6)=4  σ(6)=12  sopfr(6)=5
  μ(6)=1  J₂(6)=24  R(6)=1  λ(6)=2
  σ-τ=8  σ-φ=10  σ-μ=11  σ·τ=48  n/φ=3
  Egyptian: 1/2 + 1/3 + 1/6 = 1

---

## Level 1 — Scheme [6 kinds]

| ID | Scheme | Q upper | TRL | LCOE_est ($/MWh) | n6 core link |
|----|------|-------|-----|-----------------|-------------|
| S1 | Tokamak | 10+ | 7 | 60 | PF=6=n, CS=6=n, a=2=φ, A≈3=n/φ, Q=10=sopfr×φ |
| S2 | Stellarator | 5+ | 5 | 80 | W7-X periods=5=sopfr, coils complex |
| S3 | ICF (Laser) | 1.5+ | 4 | 200 | NIF 192=φ·σ(σ-τ) beams |
| S4 | FRC (Field-Reversed) | 2+ | 3 | 100 | compact, TAE C-2W |
| S5 | Mirror | 1+ | 3 | 150 | simple geometry |
| S6 | Z-pinch | 0.1+ | 2 | 300 | Zap Energy pulsed |

## Level 2 — Material [48 combos = 4x4x3]

### Superconductor [4 kinds]
| ID | Superconductor | Tc(K) | B_max(T) | Cost rank | n6 link |
|----|---------|-------|---------|---------|--------|
| SC1 | LTS-NbTi | 9 | 10 | 1 | Tc~sigma-n/phi? |
| SC2 | LTS-Nb3Sn | 18 | 24 | 2 | B_max=J2=24 |
| SC3 | HTS-REBCO | 92 | 45 | 4 | current SPARC/ARC choice |
| SC4 | HTS-BSCCO | 108 | 35 | 3 | Bi-2223 |

### Blanket [4 kinds]
| ID | Blanket | TBR | Coolant | Cost rank | n6 link |
|----|--------|-----|--------|---------|--------|
| BL1 | Li-ceramic | 1.05 | He | 2 | Li-6 breeding=n |
| BL2 | PbLi-eutectic | 1.15 | PbLi self | 3 | eutectic 460C |
| BL3 | FLiBe-molten | 1.10 | FLiBe | 4 | 2LiF-BeF₂ |
| BL4 | He-cooled-pebble | 1.08 | He | 2 | HCPB ITER TBM |

### Structural [3 kinds]
| ID | Structural | Radiation limit (dpa) | Operating temp (C) | Cost rank |
|----|--------|-------------|------------|---------|
| ST1 | RAFM-steel | 80 | 550 | 1 |
| ST2 | V-alloy | 150 | 700 | 3 |
| ST3 | SiC-SiC | 200 | 1000 | 4 |

## Level 3 — Core [48 combos = 4x3x4]

### Heating [4 kinds]
| ID | Heating | Freq/energy | Efficiency | n6 link |
|----|------|-------------|------|--------|
| H1 | NBI | 120keV=sigma*10 | 40% | KSTAR 8MW=sigma-tau |
| H2 | ICRH | 40-80MHz | 60% | KSTAR 6MW=n |
| H3 | ECRH | 170GHz | 70% | KSTAR 1MW=mu |
| H4 | LHCD | 5GHz=sopfr | 65% | current-drive specialty |

### Confinement [3 kinds]
| ID | Confinement | B_T range (T) | Cost |
|----|------|-----------|------|
| C1 | SC-coil (superconducting) | 5-20 | high |
| C2 | Normal-Cu | 2-8 | low |
| C3 | Permanent+SC hybrid | 3-12 | medium |

### Fuel [4 kinds]
| ID | Fuel | Q_value (MeV) | Cross section (keV) | n6 link |
|----|------|-------------|---------------|--------|
| F1 | D-T | 17.6 | 10-100 | D=phi, T=n/phi, sum=sopfr |
| F2 | D-D | 3.65 | 100+ | D=phi, sum=tau |
| F3 | D-He3 | 18.3 | 200+ | aneutronic |
| F4 | p-B11 | 8.7 | 500+ | B=11=sigma-mu |

## Level 4 — Device [180 combos = 4x5x3x3]

### Coil configuration [4 kinds]
| ID | Coils | n6 expression |
|----|--------|--------|
| TF1 | 6 | n |
| TF2 | 12 | σ |
| TF3 | 16 | 2^τ |
| TF4 | 18 | σ+n |

### Geometry (Aspect Ratio A) [5 kinds]
| ID | A | n6 expression | Example device |
|----|---|--------|----------|
| A1 | 2.5 | sopfr/phi | compact |
| A2 | 3.0 | n/phi | ARC/SPARC |
| A3 | 3.1 | ITER actual | ITER |
| A4 | 4.0 | tau | mid-size |
| A5 | 5.0 | sopfr | W7-X |

### Magnetic field B_T [3 kinds]
| ID | B_T (T) | n6 expression |
|----|--------|--------|
| B1 | 5 | sopfr |
| B2 | 12 | sigma |
| B3 | 20 | J2-tau |

### Q target [3 kinds]
| ID | Q | n6 expression | Meaning |
|----|---|--------|------|
| Q1 | 2 | phi | breakeven |
| Q2 | 10 | sopfr*phi | ITER target |
| Q3 | 1000 | infinity (ignition) | self-sustained |

## Level 5 — System [27 combos = 3x3x3]

### Power conversion [3 kinds]
| ID | Power | Efficiency | Maturity |
|----|------|------|--------|
| PW1 | Rankine (steam) | 33%=1/(n/phi) | high |
| PW2 | Brayton (gas) | 45% | medium |
| PW3 | Direct-conversion | 60% | low |

### TBR strategy [3 kinds]
| ID | TBR method | TBR value | Cost |
|----|---------|--------|------|
| TR1 | Li6-ceramic-breeder | 1.05 | medium |
| TR2 | PbLi-self-cooled | 1.15 | high |
| TR3 | Dual-coolant-DCLL | 1.20=sigma/(sigma-phi) | high |

### Grid [3 kinds]
| ID | Grid | Freq/voltage | n6 link |
|----|--------|-----------|--------|
| G1 | AC-50Hz | 50Hz=sopfr*(sigma-phi) | BT-62 |
| G2 | AC-60Hz | 60Hz=sigma*sopfr | BT-62 |
| G3 | HVDC | +-500kV | BT-68 |

---

## Total combination count

  6 × (4×4×3) × (4×3×4) × (4×5×3×3) × (3×3×3)
= 6 × 48 × 48 × 180 × 27
= 67,184,640 combinations

-> Rust DSE required (>10K threshold)

## 5 evaluation axes

| Axis | Description | Range | Weight |
|----|------|------|--------|
| n6_EXACT | Discrete param n=6 match ratio | 0-100% | 35% |
| Q_gain | Energy gain | 0-1000 | 25% |
| TRL | Tech readiness | 1-9 | 20% |
| LCOE | Levelized cost ($/MWh) | 300->10 (inverse) | 12% |
| T_comm | Commercialization time (earlier better) | 2050->2030 | 8% |
```

- [ ] **Step 2: Commit**

```bash
git add docs/fusion/goal.md
git commit -m "feat: ultimate fusion DSE candidate definitions (goal.md)

5-stage chain: scheme(6)->material(48)->core(48)->device(180)->system(27)
Total 67,184,640 combos, 5-axis evaluation (n6_EXACT/Q/TRL/LCOE/T_comm)

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```

---

### Task 2: Rust DSE search tool — constants + Level 1-2 structs

**Files:**
- Create: `tools/fusion-dse/main.rs`

- [ ] **Step 1: Create file — n=6 constants + Level 1 Scheme + Level 2 Material structs**

```rust
/// N6 Fusion DSE — ultimate fusion exhaustive combinatorial search
///
/// scheme(6) x material(48) x core(48) x device(180) x system(27)
/// = 67,184,640 combinations exhaustive search
///
/// Evaluation: n6_EXACT + Q_gain + TRL + LCOE + T_comm (5 axes)
/// Output: Pareto frontier + optimal path + ASCII diagram
///
/// Build: ~/.cargo/bin/rustc tools/fusion-dse/main.rs -o tools/fusion-dse/fusion-dse

// ─── n=6 Constants ───
const N: u64 = 6;
const PHI: u64 = 2;
const TAU: u64 = 4;
const SIGMA: u64 = 12;
const SOPFR: u64 = 5;
const MU: u64 = 1;
const J2: u64 = 24;

// n=6 base set for EXACT matching
const N6_BASE: &[u64] = &[1, 2, 3, 4, 5, 6, 12, 24];

fn is_n6_base(v: u64) -> bool {
    N6_BASE.contains(&v)
}

fn is_n6_derived(v: u64) -> bool {
    if is_n6_base(v) { return true; }
    for &a in N6_BASE {
        for &b in N6_BASE {
            if a + b == v || a * b == v { return true; }
            if a > b && a - b == v { return true; }
            if b > 0 && a % b == 0 && a / b == v { return true; }
        }
    }
    false
}

// ═══════════════════════════════════════════
// Level 1: Scheme
// ═══════════════════════════════════════════
#[derive(Clone, Copy, Debug)]
struct Scheme {
    name: &'static str,
    q_upper: u64,       // Q upper bound
    trl: u64,           // TRL 1-9
    lcoe_est: u64,      // expected LCOE $/MWh
    t_comm: u64,        // expected commercialization year
    pf_coils: u64,      // PF coil count (tokamak family)
    cs_modules: u64,    // CS module count
    n6_bonus: u64,      // unique n6 match bonus
}

const SCHEMES: &[Scheme] = &[
    Scheme { name: "Tokamak",      q_upper: 10,  trl: 7, lcoe_est: 60,  t_comm: 2035, pf_coils: 6, cs_modules: 6, n6_bonus: 5 },
    Scheme { name: "Stellarator",  q_upper: 5,   trl: 5, lcoe_est: 80,  t_comm: 2040, pf_coils: 0, cs_modules: 0, n6_bonus: 2 },
    Scheme { name: "ICF_Laser",    q_upper: 2,   trl: 4, lcoe_est: 200, t_comm: 2045, pf_coils: 0, cs_modules: 0, n6_bonus: 1 },
    Scheme { name: "FRC",          q_upper: 2,   trl: 3, lcoe_est: 100, t_comm: 2042, pf_coils: 0, cs_modules: 0, n6_bonus: 0 },
    Scheme { name: "Mirror",       q_upper: 1,   trl: 3, lcoe_est: 150, t_comm: 2045, pf_coils: 0, cs_modules: 0, n6_bonus: 0 },
    Scheme { name: "Z_Pinch",      q_upper: 1,   trl: 2, lcoe_est: 300, t_comm: 2050, pf_coils: 0, cs_modules: 0, n6_bonus: 0 },
];

// ═══════════════════════════════════════════
// Level 2: Material = SC x Blanket x Structural
// ═══════════════════════════════════════════
#[derive(Clone, Copy, Debug)]
struct Superconductor {
    name: &'static str,
    tc_k: u64,          // critical temperature K
    b_max_t: u64,       // max magnetic field T
    cost_rank: u64,     // 1-5
}

const SUPERCONDUCTORS: &[Superconductor] = &[
    Superconductor { name: "NbTi",   tc_k: 9,   b_max_t: 10,  cost_rank: 1 },
    Superconductor { name: "Nb3Sn",  tc_k: 18,  b_max_t: 24,  cost_rank: 2 },
    Superconductor { name: "REBCO",  tc_k: 92,  b_max_t: 45,  cost_rank: 4 },
    Superconductor { name: "BSCCO",  tc_k: 108, b_max_t: 35,  cost_rank: 3 },
];

#[derive(Clone, Copy, Debug)]
struct Blanket {
    name: &'static str,
    tbr: u64,           // TBR x 100 (int encoded: 105=1.05)
    coolant_types: u64, // coolant type count
    max_temp_c: u64,    // max operating temp C
    cost_rank: u64,
}

const BLANKETS: &[Blanket] = &[
    Blanket { name: "Li_ceramic",  tbr: 105, coolant_types: 1, max_temp_c: 550,  cost_rank: 2 },
    Blanket { name: "PbLi_eut",    tbr: 115, coolant_types: 2, max_temp_c: 700,  cost_rank: 3 },
    Blanket { name: "FLiBe_molt",  tbr: 110, coolant_types: 1, max_temp_c: 800,  cost_rank: 4 },
    Blanket { name: "He_pebble",   tbr: 108, coolant_types: 1, max_temp_c: 500,  cost_rank: 2 },
];

#[derive(Clone, Copy, Debug)]
struct Structural {
    name: &'static str,
    dpa_limit: u64,     // radiation limit dpa
    max_temp_c: u64,
    cost_rank: u64,
}

const STRUCTURALS: &[Structural] = &[
    Structural { name: "RAFM",   dpa_limit: 80,  max_temp_c: 550,  cost_rank: 1 },
    Structural { name: "V_alloy", dpa_limit: 150, max_temp_c: 700,  cost_rank: 3 },
    Structural { name: "SiC_SiC", dpa_limit: 200, max_temp_c: 1000, cost_rank: 4 },
];
```

- [ ] **Step 2: Commit (partial — Level 3-5 added next)**

```bash
git add tools/fusion-dse/main.rs
git commit -m "feat(fusion-dse): n6 constants + Level 1-2 structs

Scheme(6) + Superconductor(4) x Blanket(4) x Structural(3)

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```

---

### Task 3: Rust DSE — Level 3-5 structs

**Files:**
- Modify: `tools/fusion-dse/main.rs`

- [ ] **Step 1: Append Level 3 Core structs (after Structural)**

```rust
// ═══════════════════════════════════════════
// Level 3: Core = Heating x Confinement x Fuel
// ═══════════════════════════════════════════
#[derive(Clone, Copy, Debug)]
struct Heating {
    name: &'static str,
    freq_or_energy: u64,  // frequency (MHz) or energy (keV)
    efficiency_pct: u64,
    typical_mw: u64,        // KSTAR baseline MW
}

const HEATINGS: &[Heating] = &[
    Heating { name: "NBI",  freq_or_energy: 120, efficiency_pct: 40, typical_mw: 8 },
    Heating { name: "ICRH", freq_or_energy: 60,  efficiency_pct: 60, typical_mw: 6 },
    Heating { name: "ECRH", freq_or_energy: 170, efficiency_pct: 70, typical_mw: 1 },
    Heating { name: "LHCD", freq_or_energy: 5,   efficiency_pct: 65, typical_mw: 3 },
];

#[derive(Clone, Copy, Debug)]
struct Confinement {
    name: &'static str,
    b_range_min: u64,   // T
    b_range_max: u64,   // T
    cost_rank: u64,
}

const CONFINEMENTS: &[Confinement] = &[
    Confinement { name: "SC_coil",     b_range_min: 5,  b_range_max: 20, cost_rank: 4 },
    Confinement { name: "Normal_Cu",   b_range_min: 2,  b_range_max: 8,  cost_rank: 1 },
    Confinement { name: "Perm_Hybrid", b_range_min: 3,  b_range_max: 12, cost_rank: 2 },
];

#[derive(Clone, Copy, Debug)]
struct Fuel {
    name: &'static str,
    q_value_10x: u64,       // Q-value x 10 (MeV, int-encoded)
    cross_section_kev: u64,  // optimal reaction temperature keV
    neutron_fraction_pct: u64, // neutron energy fraction %
    d_mass: u64,             // sum of reactant mass numbers
}

const FUELS: &[Fuel] = &[
    Fuel { name: "D_T",    q_value_10x: 176, cross_section_kev: 10,  neutron_fraction_pct: 80, d_mass: 5 },
    Fuel { name: "D_D",    q_value_10x: 37,  cross_section_kev: 100, neutron_fraction_pct: 33, d_mass: 4 },
    Fuel { name: "D_He3",  q_value_10x: 183, cross_section_kev: 200, neutron_fraction_pct: 5,  d_mass: 5 },
    Fuel { name: "p_B11",  q_value_10x: 87,  cross_section_kev: 500, neutron_fraction_pct: 0,  d_mass: 12 },
];
```

- [ ] **Step 2: Append Level 4 Device structs**

```rust
// ═══════════════════════════════════════════
// Level 4: Device = TF_coils x Geometry x B_field x Q_target
// ═══════════════════════════════════════════
#[derive(Clone, Copy, Debug)]
struct TfConfig {
    name: &'static str,
    coil_count: u64,
}

const TF_CONFIGS: &[TfConfig] = &[
    TfConfig { name: "TF6",  coil_count: 6 },
    TfConfig { name: "TF12", coil_count: 12 },
    TfConfig { name: "TF16", coil_count: 16 },
    TfConfig { name: "TF18", coil_count: 18 },
];

#[derive(Clone, Copy, Debug)]
struct Geometry {
    name: &'static str,
    aspect_ratio_10x: u64,  // A x 10 (int-encoded: 30=3.0)
}

const GEOMETRIES: &[Geometry] = &[
    Geometry { name: "A2.5", aspect_ratio_10x: 25 },
    Geometry { name: "A3.0", aspect_ratio_10x: 30 },
    Geometry { name: "A3.1", aspect_ratio_10x: 31 },
    Geometry { name: "A4.0", aspect_ratio_10x: 40 },
    Geometry { name: "A5.0", aspect_ratio_10x: 50 },
];

#[derive(Clone, Copy, Debug)]
struct BField {
    name: &'static str,
    bt_t: u64,     // toroidal field (T)
}

const BFIELDS: &[BField] = &[
    BField { name: "B5",  bt_t: 5 },
    BField { name: "B12", bt_t: 12 },
    BField { name: "B20", bt_t: 20 },
];

#[derive(Clone, Copy, Debug)]
struct QTarget {
    name: &'static str,
    q_value: u64,
}

const QTARGETS: &[QTarget] = &[
    QTarget { name: "Q2",    q_value: 2 },
    QTarget { name: "Q10",   q_value: 10 },
    QTarget { name: "Q_ign", q_value: 1000 },
];
```

- [ ] **Step 3: Append Level 5 System structs**

```rust
// ═══════════════════════════════════════════
// Level 5: System = Power x TBR x Grid
// ═══════════════════════════════════════════
#[derive(Clone, Copy, Debug)]
struct PowerConversion {
    name: &'static str,
    efficiency_pct: u64,
    maturity: u64,       // 1-5
}

const POWER_CONVERSIONS: &[PowerConversion] = &[
    PowerConversion { name: "Rankine",   efficiency_pct: 33, maturity: 5 },
    PowerConversion { name: "Brayton",   efficiency_pct: 45, maturity: 3 },
    PowerConversion { name: "Direct_CV", efficiency_pct: 60, maturity: 1 },
];

#[derive(Clone, Copy, Debug)]
struct TbrStrategy {
    name: &'static str,
    tbr_100x: u64,       // TBR x 100
    cost_rank: u64,
}

const TBR_STRATEGIES: &[TbrStrategy] = &[
    TbrStrategy { name: "Li6_ceramic", tbr_100x: 105, cost_rank: 2 },
    TbrStrategy { name: "PbLi_self",   tbr_100x: 115, cost_rank: 3 },
    TbrStrategy { name: "DCLL_dual",   tbr_100x: 120, cost_rank: 4 },
];

#[derive(Clone, Copy, Debug)]
struct Grid {
    name: &'static str,
    freq_hz: u64,        // 0 for DC
    voltage_kv: u64,
}

const GRIDS: &[Grid] = &[
    Grid { name: "AC_50Hz", freq_hz: 50,  voltage_kv: 500 },
    Grid { name: "AC_60Hz", freq_hz: 60,  voltage_kv: 500 },
    Grid { name: "HVDC",    freq_hz: 0,   voltage_kv: 800 },
];
```

- [ ] **Step 4: Commit**

```bash
git add tools/fusion-dse/main.rs
git commit -m "feat(fusion-dse): Level 3-5 structs (Core/Device/System)

Heating(4) x Confinement(3) x Fuel(4) + TF(4) x Geom(5) x B(3) x Q(3) + Power(3) x TBR(3) x Grid(3)

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```

---

### Task 4: Rust DSE — n6_EXACT evaluation + compatibility check

**Files:**
- Modify: `tools/fusion-dse/main.rs`

- [ ] **Step 1: Write n6 EXACT computation**

Reorganize the `count_n6_exact` pattern from existing `dse-calc/main.rs:129-197` for fusion.

```rust
// ═══════════════════════════════════════════
// N6 EXACT computation — fusion axis 1 of 5
// ═══════════════════════════════════════════
fn count_n6_exact(
    scheme: &Scheme, sc: &Superconductor, bl: &Blanket, st: &Structural,
    heat: &Heating, conf: &Confinement, fuel: &Fuel,
    tf: &TfConfig, geom: &Geometry, bf: &BField, qt: &QTarget,
    pwr: &PowerConversion, tbr: &TbrStrategy, grid: &Grid,
) -> (u64, u64) {
    let mut exact = 0u64;
    let mut total = 0u64;

    // --- Level 1: scheme ---
    // PF coils
    if scheme.pf_coils > 0 {
        total += 1;
        if scheme.pf_coils == N { exact += 1; }
    }
    // CS modules
    if scheme.cs_modules > 0 {
        total += 1;
        if scheme.cs_modules == N { exact += 1; }
    }

    // --- Level 2: material ---
    // SC B_max
    total += 1;
    if is_n6_base(sc.b_max_t) || is_n6_derived(sc.b_max_t) { exact += 1; }
    // Blanket coolant types
    total += 1;
    if is_n6_base(bl.coolant_types) { exact += 1; }

    // --- Level 3: core ---
    // Heating MW (KSTAR)
    total += 1;
    if is_n6_base(heat.typical_mw) { exact += 1; }
    // Fuel mass sum
    total += 1;
    if is_n6_base(fuel.d_mass) { exact += 1; }
    // Fuel neutron fraction: 80% = 4/5 = tau/sopfr
    total += 1;
    if fuel.neutron_fraction_pct == 80 { exact += 1; } // tau/sopfr

    // --- Level 4: device ---
    // TF coil count
    total += 1;
    if is_n6_base(tf.coil_count) { exact += 1; }
    // Aspect ratio x10: 30=n/phi*10
    total += 1;
    if geom.aspect_ratio_10x == (N * 10 / PHI) { exact += 1; } // 30
    // B_T
    total += 1;
    if bf.bt_t == SIGMA { exact += 1; } // 12T = sigma
    // Q target
    total += 1;
    if qt.q_value == SOPFR * PHI { exact += 1; } // 10 = sopfr*phi

    // --- Level 5: system ---
    // Power conversion efficiency: 33% ~ 1/3 = 1/(n/phi)
    total += 1;
    if pwr.efficiency_pct == 33 { exact += 1; }
    // TBR: 1.20 = sigma/(sigma-phi)
    total += 1;
    if tbr.tbr_100x == 120 { exact += 1; }
    // Grid freq: 60Hz = sigma*sopfr
    total += 1;
    if grid.freq_hz == SIGMA * SOPFR / MU { exact += 1; } // 60
    // Grid freq: 50Hz = sopfr*(sigma-phi)
    if grid.freq_hz == SOPFR * (SIGMA - PHI) { exact += 1; } // 50 (disjoint — both 60 and 50 can match)

    (exact, total)
}
```

- [ ] **Step 2: Compatibility check (remove non-physical combos)**

```rust
// ═══════════════════════════════════════════
// Compatibility filter — remove non-physical combos
// ═══════════════════════════════════════════
fn is_compatible(
    scheme: &Scheme, sc: &Superconductor, _bl: &Blanket, st: &Structural,
    _heat: &Heating, conf: &Confinement, fuel: &Fuel,
    _tf: &TfConfig, _geom: &Geometry, bf: &BField, _qt: &QTarget,
    _pwr: &PowerConversion, _tbr: &TbrStrategy, _grid: &Grid,
) -> bool {
    // SC B_max must support requested B_T
    if bf.bt_t > sc.b_max_t { return false; }

    // Normal Cu coils can't reach >8T
    if conf.name == "Normal_Cu" && bf.bt_t > 8 { return false; }

    // p-B11 needs >300keV → only SC coil with B≥12T is plausible
    if fuel.name == "p_B11" && bf.bt_t < 12 { return false; }

    // ICF/Z-pinch don't use TF coils in same way, but we allow all combos
    // (pruning happens via lower Q/TRL scores)

    // Structural temp must handle blanket temp
    // (simplified: SiC handles everything, RAFM limited)

    true
}
```

- [ ] **Step 3: Commit**

```bash
git add tools/fusion-dse/main.rs
git commit -m "feat(fusion-dse): n6_EXACT evaluation + compatibility filter

14 discrete-param match + remove non-physical combos (B_max, coil type, etc.)

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```

---

### Task 5: Rust DSE — 5-axis scoring + Pareto + result struct

**Files:**
- Modify: `tools/fusion-dse/main.rs`

- [ ] **Step 1: Result struct + 5-axis scoring function**

```rust
// ═══════════════════════════════════════════
// Result struct
// ═══════════════════════════════════════════
#[derive(Clone)]
struct DseResult {
    scheme: &'static str,
    sc: &'static str,
    blanket: &'static str,
    structural: &'static str,
    heating: &'static str,
    confinement: &'static str,
    fuel: &'static str,
    tf_coils: u64,
    aspect: &'static str,
    bt: u64,
    q_target: u64,
    power_conv: &'static str,
    tbr_strat: &'static str,
    grid: &'static str,
    // scores
    n6_exact: u64,
    n6_total: u64,
    n6_pct: f64,
    q_score: f64,
    trl_score: f64,
    lcoe_score: f64,
    tcomm_score: f64,
    pareto_score: f64,
}

// ═══════════════════════════════════════════
// 5-axis score computation
// ═══════════════════════════════════════════
fn compute_scores(
    scheme: &Scheme, fuel: &Fuel, qt: &QTarget, pwr: &PowerConversion,
    sc: &Superconductor, bl: &Blanket, st: &Structural, conf: &Confinement,
    n6_exact: u64, n6_total: u64,
) -> (f64, f64, f64, f64, f64, f64) {
    // axis 1: n6_EXACT (0-100)
    let n6_pct = if n6_total > 0 { (n6_exact as f64) / (n6_total as f64) * 100.0 } else { 0.0 };

    // axis 2: Q_gain (0-100)
    // Q = min(scheme.q_upper, qt.q_value) scaled by fuel difficulty
    let q_achievable = scheme.q_upper.min(qt.q_value) as f64;
    let fuel_penalty = fuel.cross_section_kev as f64 / 10.0; // D-T=1, D-He3=20, p-B11=50
    let q_score = (q_achievable / fuel_penalty * 10.0).min(100.0);

    // axis 3: TRL (0-100)
    // scheme TRL + maturity bonuses
    let mat_bonus = if sc.cost_rank <= 2 { 5 } else { 0 }; // mature SC
    let conv_bonus = pwr.maturity as u64 * 2;
    let trl_score = ((scheme.trl * 10 + mat_bonus + conv_bonus) as f64).min(100.0);

    // axis 4: LCOE (0-100, lower is better -> invert)
    // LCOE affected by: scheme base + fuel cost + SC cost + conversion efficiency
    let lcoe_raw = scheme.lcoe_est as f64
        * (fuel.cross_section_kev as f64 / 10.0).sqrt()  // harder fuel = more expensive
        * (1.0 + sc.cost_rank as f64 * 0.1)              // expensive SC
        / (pwr.efficiency_pct as f64 / 33.0);             // better conversion = lower LCOE
    let lcoe_score = (100.0 - lcoe_raw / 5.0).max(0.0).min(100.0);

    // axis 5: T_comm (0-100, earlier is better)
    let tcomm_score = ((2055 - scheme.t_comm) as f64 * 5.0).max(0.0).min(100.0);

    // Pareto composite: 35% n6 + 25% Q + 20% TRL + 12% LCOE + 8% T_comm
    let pareto = n6_pct * 0.35 + q_score * 0.25 + trl_score * 0.20
        + lcoe_score * 0.12 + tcomm_score * 0.08;

    (n6_pct, q_score, trl_score, lcoe_score, tcomm_score, pareto)
}
```

- [ ] **Step 2: Commit**

```bash
git add tools/fusion-dse/main.rs
git commit -m "feat(fusion-dse): 5-axis scoring + DseResult struct

n6_EXACT(35%) + Q_gain(25%) + TRL(20%) + LCOE(12%) + T_comm(8%)

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```

---

### Task 6: Rust DSE — main() exhaustive loop + output

**Files:**
- Modify: `tools/fusion-dse/main.rs`

- [ ] **Step 1: main() — header print + exhaustive loop**

```rust
fn main() {
    let total_theoretical = SCHEMES.len() * SUPERCONDUCTORS.len() * BLANKETS.len()
        * STRUCTURALS.len() * HEATINGS.len() * CONFINEMENTS.len() * FUELS.len()
        * TF_CONFIGS.len() * GEOMETRIES.len() * BFIELDS.len() * QTARGETS.len()
        * POWER_CONVERSIONS.len() * TBR_STRATEGIES.len() * GRIDS.len();

    println!("═══════════════════════════════════════════════════════════════════");
    println!("  N6 FUSION DSE — ultimate fusion exhaustive search");
    println!("  scheme x material x core x device x system");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();
    println!("  Level 1 scheme:  {} ({} kinds)", SCHEMES.iter().map(|s| s.name).collect::<Vec<_>>().join(", "), SCHEMES.len());
    println!("  Level 2 mat:     SC({}) x BL({}) x ST({}) = {}",
        SUPERCONDUCTORS.len(), BLANKETS.len(), STRUCTURALS.len(),
        SUPERCONDUCTORS.len() * BLANKETS.len() * STRUCTURALS.len());
    println!("  Level 3 core:    H({}) x C({}) x F({}) = {}",
        HEATINGS.len(), CONFINEMENTS.len(), FUELS.len(),
        HEATINGS.len() * CONFINEMENTS.len() * FUELS.len());
    println!("  Level 4 device:  TF({}) x G({}) x B({}) x Q({}) = {}",
        TF_CONFIGS.len(), GEOMETRIES.len(), BFIELDS.len(), QTARGETS.len(),
        TF_CONFIGS.len() * GEOMETRIES.len() * BFIELDS.len() * QTARGETS.len());
    println!("  Level 5 system:  P({}) x TBR({}) x Grid({}) = {}",
        POWER_CONVERSIONS.len(), TBR_STRATEGIES.len(), GRIDS.len(),
        POWER_CONVERSIONS.len() * TBR_STRATEGIES.len() * GRIDS.len());
    println!();
    println!("  Theoretical combos: {}", total_theoretical);

    let mut results: Vec<DseResult> = Vec::new();
    let mut pruned = 0u64;

    for scheme in SCHEMES {
        for sc in SUPERCONDUCTORS {
            for bl in BLANKETS {
                for st in STRUCTURALS {
                    for heat in HEATINGS {
                        for conf in CONFINEMENTS {
                            for fuel in FUELS {
                                for tf in TF_CONFIGS {
                                    for geom in GEOMETRIES {
                                        for bf in BFIELDS {
                                            for qt in QTARGETS {
                                                for pwr in POWER_CONVERSIONS {
                                                    for tbr in TBR_STRATEGIES {
                                                        for grid in GRIDS {
                                                            if !is_compatible(scheme, sc, bl, st, heat, conf, fuel, tf, geom, bf, qt, pwr, tbr, grid) {
                                                                pruned += 1;
                                                                continue;
                                                            }
                                                            let (n6_exact, n6_total) = count_n6_exact(scheme, sc, bl, st, heat, conf, fuel, tf, geom, bf, qt, pwr, tbr, grid);
                                                            let (n6_pct, q_score, trl_score, lcoe_score, tcomm_score, pareto) =
                                                                compute_scores(scheme, fuel, qt, pwr, sc, bl, st, conf, n6_exact, n6_total);

                                                            results.push(DseResult {
                                                                scheme: scheme.name,
                                                                sc: sc.name,
                                                                blanket: bl.name,
                                                                structural: st.name,
                                                                heating: heat.name,
                                                                confinement: conf.name,
                                                                fuel: fuel.name,
                                                                tf_coils: tf.coil_count,
                                                                aspect: geom.name,
                                                                bt: bf.bt_t,
                                                                q_target: qt.q_value,
                                                                power_conv: pwr.name,
                                                                tbr_strat: tbr.name,
                                                                grid: grid.name,
                                                                n6_exact, n6_total, n6_pct,
                                                                q_score, trl_score, lcoe_score, tcomm_score,
                                                                pareto_score: pareto,
                                                            });
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    results.sort_by(|a, b| b.pareto_score.partial_cmp(&a.pareto_score).unwrap());

    let explored = results.len();
    println!("  Compatible combos: {} (pruned: {})", explored, pruned);
    println!();
```

- [ ] **Step 2: TOP-20 Pareto table output**

```rust
    // TOP-20
    println!("═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!("  TOP 20 PARETO FRONTIER");
    println!("═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!();
    println!("  {:>3} │ {:>11} │ {:>6} {:>10} {:>6} │ {:>4} {:>4} {:>5} │ {:>3} {:>4} {:>3} {:>4} │ {:>7} {:>4} {:>5} │ {:>5} │ {:>5} │ {:>5} │ {:>5} │ {:>5} │ {:>6}",
        "#", "Scheme", "SC", "Blanket", "Struct",
        "Heat", "Conf", "Fuel",
        "TF", "A", "B", "Q",
        "PwrConv", "TBR", "Grid",
        "n6%", "Q_sc", "TRL", "LCOE", "Tcom", "Pareto");
    println!("  ────┼─────────────┼─────────────────────────┼────────────────────┼──────────────────┼────────────────────┼───────┼───────┼───────┼───────┼───────┼────────");

    for (i, r) in results.iter().take(20).enumerate() {
        println!("  {:>3} │ {:>11} │ {:>6} {:>10} {:>6} │ {:>4} {:>4} {:>5} │ {:>3} {:>4} {:>3} {:>4} │ {:>7} {:>4} {:>5} │ {:>4.1}% │ {:>5.1} │ {:>5.1} │ {:>5.1} │ {:>5.1} │ {:>6.2}",
            i + 1, r.scheme, r.sc, r.blanket, r.structural,
            r.heating, r.confinement, r.fuel,
            r.tf_coils, r.aspect, r.bt, r.q_target,
            r.power_conv, r.tbr_strat, r.grid,
            r.n6_pct, r.q_score, r.trl_score, r.lcoe_score, r.tcomm_score,
            r.pareto_score);
    }
```

- [ ] **Step 3: Statistics + optimal path ASCII diagram output**

```rust
    // Statistics
    println!();
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  STATISTICS");
    println!("═══════════════════════════════════════════════════════════════════");

    let max_n6 = results.iter().map(|r| r.n6_pct).fold(0.0f64, f64::max);
    let avg_n6 = results.iter().map(|r| r.n6_pct).sum::<f64>() / explored as f64;
    let above80 = results.iter().filter(|r| r.n6_pct >= 80.0).count();
    let above60 = results.iter().filter(|r| r.n6_pct >= 60.0).count();

    println!("  Total theoretical: {}", total_theoretical);
    println!("  Explored (compatible): {}", explored);
    println!("  Pruned (incompatible): {}", pruned);
    println!("  Max n6 EXACT:  {:.1}%", max_n6);
    println!("  Avg n6 EXACT:  {:.1}%", avg_n6);
    println!("  ≥80% EXACT:    {} ({:.2}%)", above80, above80 as f64 / explored as f64 * 100.0);
    println!("  ≥60% EXACT:    {} ({:.2}%)", above60, above60 as f64 / explored as f64 * 100.0);

    // Best per axis
    println!();
    println!("  BEST BY AXIS:");
    let best_n6_r = results.iter().max_by(|a, b| a.n6_pct.partial_cmp(&b.n6_pct).unwrap()).unwrap();
    let best_q_r = results.iter().max_by(|a, b| a.q_score.partial_cmp(&b.q_score).unwrap()).unwrap();
    let best_trl_r = results.iter().max_by(|a, b| a.trl_score.partial_cmp(&b.trl_score).unwrap()).unwrap();
    println!("    Best n6:  {} / {} / {} (n6={:.1}%)", best_n6_r.scheme, best_n6_r.fuel, best_n6_r.sc, best_n6_r.n6_pct);
    println!("    Best Q:   {} / {} / {} (Q={:.1})", best_q_r.scheme, best_q_r.fuel, best_q_r.sc, best_q_r.q_score);
    println!("    Best TRL: {} / {} / {} (TRL={:.1})", best_trl_r.scheme, best_trl_r.fuel, best_trl_r.sc, best_trl_r.trl_score);

    // Optimal path ASCII
    let best = &results[0];
    println!();
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  OPTIMAL PATH — ultimate fusion optimal path");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();
    println!("  ┌─────────────┐    ┌──────────────────────┐    ┌─────────────────────┐");
    println!("  │  Level 1    │    │  Level 2             │    │  Level 3            │");
    println!("  │  scheme     │───▶│  material            │───▶│  core               │");
    println!("  │  {}  │    │  SC: {}          │    │  H: {}          │", pad(best.scheme, 9), pad(best.sc, 12), pad(best.heating, 13));
    println!("  │             │    │  BL: {}          │    │  C: {}          │", pad(best.blanket, 12), pad(best.confinement, 13));
    println!("  │             │    │  ST: {}          │    │  F: {}          │", pad(best.structural, 12), pad(best.fuel, 13));
    println!("  └─────────────┘    └──────────────────────┘    └─────────┬───────────┘");
    println!("                                                           │");
    println!("                                                           ▼");
    println!("  ┌──────────────────────────────┐    ┌──────────────────────────────┐");
    println!("  │  Level 5                     │    │  Level 4                     │");
    println!("  │  system                      │◀───│  device                      │");
    println!("  │  PW: {}              │    │  TF: {} coils              │", pad(best.power_conv, 14), pad(&best.tf_coils.to_string(), 12));
    println!("  │  TBR: {}             │    │  A:  {}                    │", pad(best.tbr_strat, 13), pad(best.aspect, 12));
    println!("  │  Grid: {}            │    │  B:  {}T                   │", pad(best.grid, 12), pad(&best.bt.to_string(), 11));
    println!("  │                              │    │  Q:  {}                    │", pad(&best.q_target.to_string(), 12));
    println!("  └──────────────────────────────┘    └──────────────────────────────┘");
    println!();
    println!("  Pareto Score: {:.2}", best.pareto_score);
    println!("  n6 EXACT: {}/{} = {:.1}%", best.n6_exact, best.n6_total, best.n6_pct);
    println!("  Q: {:.1} | TRL: {:.1} | LCOE: {:.1} | T_comm: {:.1}",
        best.q_score, best.trl_score, best.lcoe_score, best.tcomm_score);

    // Scheme breakdown
    println!();
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  SCHEME BREAKDOWN (best per scheme)");
    println!("═══════════════════════════════════════════════════════════════════");
    for scheme_name in &["Tokamak", "Stellarator", "ICF_Laser", "FRC", "Mirror", "Z_Pinch"] {
        if let Some(best_in_scheme) = results.iter().filter(|r| r.scheme == *scheme_name).max_by(|a, b| a.pareto_score.partial_cmp(&b.pareto_score).unwrap()) {
            println!("  {:>12}: n6={:.1}% Q={:.1} TRL={:.1} LCOE={:.1} Pareto={:.2} | {}/{}/{}/TF{}/B{}T/Q{}",
                scheme_name, best_in_scheme.n6_pct, best_in_scheme.q_score, best_in_scheme.trl_score,
                best_in_scheme.lcoe_score, best_in_scheme.pareto_score,
                best_in_scheme.sc, best_in_scheme.fuel, best_in_scheme.heating,
                best_in_scheme.tf_coils, best_in_scheme.bt, best_in_scheme.q_target);
        }
    }

    println!();
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  FUSION DSE done — {} combos searched, optimal path derived", explored);
    println!("═══════════════════════════════════════════════════════════════════");
}

fn pad(s: &str, width: usize) -> String {
    if s.len() >= width { s.to_string() } else { format!("{}{}", s, " ".repeat(width - s.len())) }
}
```

- [ ] **Step 4: Commit**

```bash
git add tools/fusion-dse/main.rs
git commit -m "feat(fusion-dse): main() exhaustive loop + TOP-20 + ASCII optimal path

67M theoretical combos, exhaustive search after compatibility pruning
Pareto frontier + best per scheme + optimal path diagram

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```

---

### Task 7: Build + run + check

**Files:**
- Build: `tools/fusion-dse/fusion-dse`

- [ ] **Step 1: Create directory + build**

```bash
mkdir -p tools/fusion-dse
~/.cargo/bin/rustc tools/fusion-dse/main.rs -o tools/fusion-dse/fusion-dse -O
```

Expected: compilation success, no warnings (or minimal unused warnings)

- [ ] **Step 2: Run (background)**

```bash
tools/fusion-dse/fusion-dse
```

Expected:
- theoretical combo count printed (~67,184,640)
- compatible combo count < theoretical (pruning applied)
- TOP-20 Pareto table
- optimal path ASCII diagram
- per-scheme best analysis

- [ ] **Step 3: Check results**

Items to confirm:
1. Does Tokamak + D-T + REBCO + 12T + Q10 rank in the top group?
2. Is the highest-n6_EXACT combo physically reasonable?
3. Among per-scheme optima, is Tokamak first by Pareto?

- [ ] **Step 4: Commit binary (optional)**

```bash
git add tools/fusion-dse/main.rs
git commit -m "feat: ultimate fusion DSE search tool done + run check

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```

---

### Task 8: dse-map.md DSE map create + update

**Files:**
- Create: `docs/dse-map.md`

- [ ] **Step 1: Write DSE map file**

Write based on run results. Template below:

```markdown
# N6 DSE map — all-domain Design Space Exploration status

---

## DSE status

| # | Domain | Chain | Combos | Search | Best n6% | Best path | Cross-DSE |
|---|--------|------|---------|------|---------|----------|-----------|
| 1 | Chip architecture | material x process x core x chip x system | 3,000 | done | XX% | Diamond+TSMC_N2+HEXA-P+... | - |
| 2 | Battery | material x process x core x chip x system | TBD | in progress | - | - | - |
| 3 | **Fusion** | **scheme x material x core x device x system** | **67M** | **done** | **XX%** | **Tokamak+REBCO+...** | available |

## Cross-DSE candidates

| Combo | Domain A | Domain B | Intersection | Status |
|------|---------|---------|--------|------|
| chip x fusion | chip-arch | fusion | power control/BMS | not started |
| battery x fusion | battery | fusion | energy storage + generation | not started |
```

Fill XX% with actual numbers from the run result.

- [ ] **Step 2: Commit**

```bash
git add docs/dse-map.md
git commit -m "feat: DSE map (dse-map.md) — chip/battery/fusion status + Cross-DSE candidates

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```

---

## Task Dependency Graph

```
  Task 1 (goal.md)
    │
    ▼
  Task 2 (L1-L2 structs) ──▶ Task 3 (L3-L5 structs) ──▶ Task 4 (n6_EXACT + compat)
                                                              │
                                                              ▼
                                                         Task 5 (scoring + Pareto)
                                                              │
                                                              ▼
                                                         Task 6 (main loop + output)
                                                              │
                                                              ▼
                                                         Task 7 (build + run + verify)
                                                              │
                                                              ▼
                                                         Task 8 (dse-map.md)
```

Tasks 1 is independent. Tasks 2-7 are sequential (single file build). Task 8 depends on Task 7 output.
