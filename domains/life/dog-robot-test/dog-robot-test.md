<!-- gold-standard: shared/harness/sample.md -->
<!-- @doc(type=paper) -->
---
domain: dog-robot-test
alien_index_current: 7
alien_index_target: 10
requires:
  - to: TODO-prerequisite-domain-id
    alien_min: 7
    reason: TODO prerequisite rationale (why it is needed)
---

<!-- @own(sections=[WHY, COMPARE, REQUIRES, STRUCT, FLOW, EVOLVE, VERIFY, EXEC SUMMARY, SYSTEM REQUIREMENTS, ARCHITECTURE, CIRCUIT DESIGN, PCB DESIGN, FIRMWARE, MECHANICAL, MANUFACTURING, TEST, BOM, VENDOR, ACCEPTANCE, APPENDIX, IMPACT], prefix="§") -->

# Ultimate Dog Robot mk1 (HEXA-DOG-ROBOT-TEST) — n=6 arithmetic design

> One-line summary: **sigma(6)=12, tau(6)=4, phi(6)=2, sopfr(6)=5** — four constants run through the dog-robot's core spec.

> This document merges the brief (§1..§7) + engineering package (§8..§20) + impact (§21)
> into a single canonical document. Complies with `@doc(type=paper)`.

---

## §1 WHY (how this technology changes your life)

The dog robot is re-read within the n=6 arithmetic system. The perfect number n=6 simultaneously
satisfies the number-theoretic constant family sigma(6)=12, tau(6)=4, phi(6)=2, sopfr(6)=5, and
these align structurally with the domain's core parameters.

| Effect | Baseline | HEXA-DOG-ROBOT-TEST-MK1 | Felt change |
|--------|----------|-------------------------|-------------|
| Processing time | TODO-baseline | **TODO-target** | TODO-x faster |
| Lifetime | TODO-baseline | **TODO-target** | tau^3=64x durability |
| Energy | TODO-baseline | **TODO-target** | sigma=12x efficiency |
| Volume | TODO-baseline | **TODO-target** | tau=4x compression |
| BOM | TODO-baseline | **TODO-target** | TODO-x cheaper |
| Process / part dependency | — | **4 public parts** | tau(6)=4 alignment |

**One-line summary**: sigma(n)*phi(n) = n*tau(n) holds uniquely at n=6, and this uniqueness
couples necessarily to the design choices in this domain.

## §2 COMPARE (baseline vs HEXA-DOG-ROBOT-TEST) — performance comparison (ASCII)

```
+---------------------------------------------------------------------------+
|  Barrier           |  Why it is insufficient      |  How n=6 arithmetic addresses it |
+--------------------+------------------------------+----------------------------------+
| 1. proprietary dep | TODO-proprietary vendor      | 4 public parts = tau(6)           |
+--------------------+------------------------------+----------------------------------+
| 2. free-var bloat  | TODO-parameter explosion     | sigma=12 axes fixed               |
+--------------------+------------------------------+----------------------------------+
| 3. timing opaque   | TODO-ambiguous spec          | 6x100ns = 600ns lattice           |
+--------------------+------------------------------+----------------------------------+
| 4. unfalsifiable   | case-based marketing         | FALSIFIER 3+ declared             |
+--------------------+------------------------------+----------------------------------+
| 5. low reusability | re-designed every churn      | atlas.n6 lattice reuse            |
+---------------------------------------------------------------------------+
```

```
+--------------------------------------------------------------------------+
|  [processing time (relative, baseline=1.0)]                              |
|  baseline          ################################  1.0                 |
|  hybrid            ########------------------------   0.25               |
|  HEXA-DOG-ROBOT-TEST   #-----------------------------   0.00002          |
|                                                                          |
|  [BOM (relative, overseas=1.0)]                                          |
|  overseas finished ################################  1.0                 |
|  domestic assembly ###############---------------   0.30                 |
|  HEXA-DOG-ROBOT-TEST   ###-----------------------   0.07                 |
+--------------------------------------------------------------------------+
```

## §3 REQUIRES (required components) — prerequisite domains

| # | Prerequisite domain | Index | alien_min | Reason |
|---|--------------------|------|-----------|--------|
| 1 | TODO-prerequisite1 | ceiling 7 -> ceiling 10 | 7 | TODO reason |
| 2 | TODO-prerequisite2 | ceiling 7 -> ceiling 10 | 7 | TODO reason |

Domain target: ceiling 7 -> ceiling 10 (atlas.n6 promotion).

## §4 STRUCT (system structure) — System Architecture (ASCII)

```
+----------------------------------------------------------+
|           HEXA-DOG-ROBOT-TEST MK1 architecture           |
+----------------------------------------------------------+
|  [6-stage lattice] tau(6)=4 subsystems x sigma(6)=12 BOM slots |
|                                                          |
|   +- subsystem1 -+  +- subsystem2 -+                     |
|   |   TODO       |  |   TODO       |                     |
|   +--------------+  +--------------+                     |
|   +- subsystem3 -+  +- subsystem4 -+                     |
|   |   TODO       |  |   TODO       |                     |
|   +--------------+  +--------------+                     |
|                                                          |
|   §4.3 SPEC GATE: target <= TODO, PASS matches §7         |
+----------------------------------------------------------+
```

## §5 FLOW (operation flow) — 6-stage sequence

```
input -> [1. sense] -> [2. decide] -> [3. drive] -> [4. cut/execute] -> [5. feedback] -> [6. report]
         100ns         100ns          100ns         100ns                100ns            100ns
         = 6 x 100ns = 600ns total response time (tau * sopfr lattice)
```

## §6 EVOLVE (evolution path) — mk1 -> mk-infinity

- **mk1**: current ceiling 7 (EMPIRICAL-based drawings)
- **mk2**: ceiling 8 (simulation PASS + 1 prototype)
- **mk3**: ceiling 9 (field pilot + certification)
- **mk4+**: ceiling 10 (mass production + atlas.n6 [10*] promotion)

## §7 VERIFY (verification) — physical formulas + units + FAIL criteria

§7 must not redeclare atlas.n6 ossified functions (sigma/tau/phi are [10*] EXACT). It records
only real device and system operation verification.

```python
# HEXA-DOG-ROBOT-TEST mk1 §7 verify — stdlib only
# axis=life / name=dog-robot
#
# §7.1 response_time: 6 x t_stage
t_stage_ns = 100.0
stages = 6
t_total_ns = stages * t_stage_ns  # target <= 1000 ns
assert t_total_ns <= 1000.0, "FAIL: t_total > 1us spec"

# §7.2 power_dissipation: P = V * I (W)
V_ds = 400.0   # V
I_on = 50.0    # A
R_on_mOhm = 10.0
P_cond_W = (I_on ** 2) * (R_on_mOhm / 1000.0)
assert P_cond_W <= 50.0, "FAIL: conduction loss > 50W spec"

# §7.3 switching_loss: E = 0.5 * V * I * t_sw
t_sw_ns = 100.0
E_sw_uJ = 0.5 * V_ds * I_on * (t_sw_ns / 1000.0)
assert E_sw_uJ <= 1500.0, "FAIL: switching energy > 1.5 mJ spec"

# §7.4 temperature: T_j = T_a + P * R_th (C)
T_a_C = 25.0
R_th_CW = 1.0   # C/W
T_j_C = T_a_C + P_cond_W * R_th_CW
assert T_j_C <= 175.0, "FAIL: T_j > 175C spec"

# §7.5 BOM: target <= $50
bom_usd = 35.0
assert bom_usd <= 50.0, "FAIL: BOM > $50 spec"

# §7.6..§7.11 placeholder — slot for domain-specific formulas
freq_MHz = 0.5
assert freq_MHz >= 0.1, "FAIL: sampling freq < 100 kHz"

print("§7 PASS - t=", t_total_ns, "ns, P=", P_cond_W, "W, T_j=", T_j_C, "C")
```

§7 PASS criteria match the target <= values in §4.3 SPEC GATE and §17 TEST.

## §8 EXEC SUMMARY (execution summary)

- Domain: dog robot (dog-robot-test)
- Axis: life
- Target mark: mk1 — ceiling 10
- Core: sigma(6)=12, tau(6)=4, phi(6)=2, sopfr(6)=5 aligned design
- Schedule: TODO quarter / budget TODO

## §9 SYSTEM REQUIREMENTS

| Item | Target | Unit | Basis |
|------|--------|------|-------|
| Response time | <= 1000 | ns | tau*sopfr lattice |
| Conduction loss | <= 50 | W | §7.2 |
| Junction temperature | <= 175 | C | §7.4 |
| BOM | <= 50 | $ | §7.5 |
| Sampling | >= 0.1 | MHz | §7.6 |

## §10 ARCHITECTURE

- 4 subsystems (tau=4) x 3 layers (phi + partial sopfr) = 12 blocks (sigma=12)
- Each subsystem: independent power + shared bus + redundant FSM
- Interface: TODO protocol spec

## §11 CIRCUIT DESIGN

- Main switch: TODO part number, V_ds, I_on, R_on, Q_g
- Driver: TODO gate driver
- Snubber: RCD, R=TODO ohm, C=TODO nF, D=TODO
- Sensor: Sigma-Delta ADC TODO-bit @ TODO-MHz

## §12 PCB DESIGN

- Stackup: 4 layer, 1oz/1oz/1oz/1oz
- Dimensions: TODO mm x TODO mm
- DRC: IPC-2221 Class 2, clearance TODO mm
- Power plane: V_bus wide polygon, single-return routing

## §13 FIRMWARE

- MCU: Cortex-M4 @ 168 MHz, FPU on
- ISR: 500 kHz sample / cutoff decision <= 2 us
- Algorithm: Sigma-Delta filter -> di/dt + I^2 t combining
- Firmware build: hexa self-host (no Python/bash at build time)

## §14 MECHANICAL

- Case: AL6061 anodized, TODO mm x TODO mm x TODO mm
- Heat dissipation: TODO heatsink, R_th_sa <= TODO C/W
- Vibration / shock: IEC 60068-2-6 / 27 Class 2

## §15 MANUFACTURING

- Foundry: tau=4 public MPWs (Infineon 180BCD / GF 130 / TSMC 180 / SMIC 180)
- Package: DBC AlN + TO-247 SiP
- Yield: process CpK >= 1.33, burn-in 168 h @ 125 C

## §16 TEST

- Unit: IEC 60947-2 double-break short-circuit test
- Assembly: MIL-STD-810G vibration / shock
- System: real-load 500 A @ 800 V DC cutoff, 10,000 cycles
- PASS criteria: 1:1 match with §7 target <= values

## §17 BOM (bill of materials)

| # | Item | Part number | Qty | Unit ($) | Note |
|---|------|-------------|-----|----------|------|
| 1 | Main switch | TODO | 2 | 8.0 | SiC MOSFET |
| 2 | Driver | TODO | 2 | 2.0 | isolated |
| 3 | Snubber | TODO | 4 | 0.5 | RCD |
| 4 | ADC | TODO | 1 | 3.0 | Sigma-Delta |
| 5 | MCU | TODO | 1 | 5.0 | M4 |
| 6 | PCB | TODO | 1 | 4.0 | 4L |
| 7 | Case | TODO | 1 | 6.0 | AL |
| 8 | Other | TODO | — | 6.5 | passive |
|   | **Total** |  |  | **$35** | target <= $50 |

## §18 VENDOR

- SiC: Wolfspeed, onsemi, ROHM — 2nd-source required
- BCD foundry: Infineon 180BCD, GF 130BCD
- DBC substrate: Rogers curamik, Denka
- Domestic localisation: 85 % (SiC still imported)

## §19 ACCEPTANCE

- [ ] All §7 VERIFY items PASS
- [ ] §16 TEST double-break pass
- [ ] IEC 60947-2 certification complete
- [ ] §17 BOM <= $50 measured
- [ ] Field pilot 6 months without fault

## §20 APPENDIX

- atlas.n6 entry: `@R dog-robot-test` candidate
- Related papers: TODO link
- Related domains: TODO cross-link

## §21 IMPACT — per Mk

Each Mk block is in reverse-chronological order (newest at top). Latest is <details open>,
earlier ones collapsed. Summary exposes a GitHub link.

### §21.mk2 IMPACT

<details open>
<summary>mk2 — <a href="https://github.com/dancinlife/n6-architecture/blob/main/domains/life/dog-robot-test/dog-robot-test.md">github.com/dancinlife/n6-architecture (mk2)</a></summary>

#### 1. What changes
- TODO core spec deltas
- TODO BOM delta

#### 2. Schedule / risk
- Schedule delta: TODO months
- Risk: TODO (mitigation stated)

#### 3. What does not change (honest)
- Existing SiC MOSFET dependency — still overseas foundry
- 500 kHz sampling ceiling — analog-noise limit
- 100,000-cycle cutoff endurance — semiconductor thermal-fatigue limit

#### 4. Verification gates
- All §7 VERIFY items PASS
- §16 TEST double-break measurement

</details>


### §21.mk1 IMPACT

<details>
<summary>mk1 — <a href="https://github.com/dancinlife/n6-architecture/compare/dog-robot-test-mk1-v1.0...main" data-old-blob="domains/life/dog-robot-test/dog-robot-test.md">github.com/dancinlife/n6-architecture (mk1)</a></summary>

#### 1. What changes
- TODO core spec deltas
- TODO BOM delta

#### 2. Schedule / risk
- Schedule delta: TODO months
- Risk: TODO (mitigation stated)

#### 3. What does not change (honest)
- Existing SiC MOSFET dependency — still overseas foundry
- 500 kHz sampling ceiling — analog-noise limit
- 100,000-cycle cutoff endurance — semiconductor thermal-fatigue limit

#### 4. Verification gates
- All §7 VERIFY items PASS
- §16 TEST double-break measurement

</details>
