2026-04-12
# Chip L1~L10 integrated comparison

> Source: empirical basis from 12 files `domains/compute/chip-design/hexa-*.md`
> Principle: honest verification (report as-is, no exaggeration)
> Chip-design file total line count: 14,190

---

## 1. Overall comparison

```
+------+--------------------+------------------------+---------------+----------+----------+---------+
| stage| architecture        | gates/neurons           | clock/speed    | power     | n=6 map   | ceiling  |
+------+--------------------+------------------------+---------------+----------+----------+---------+
| L1   | HEXA-1-DIGITAL     | 12x12=144 MAC          | 5nm ~2GHz      | Egyptian  | sigma^2   | 2nm     |
|      | digital SoC        | (sigma^2)              | tau=4 pipe     | 1/2+1/3   | =144 MAC  | GAAFET  |
|      |                    |                        | line           | +1/6=1    |           |         |
+------+--------------------+------------------------+---------------+----------+----------+---------+
| L2   | HEXA-2-PIM         | sigma=12 layers x      | HBM inside     | Egyptian  | sigma lyr | CXL 3.0 |
|      | in-memory compute  | 8 PIM/layer = 6144 MAC | ~48 TB/s       | 48W split | x8 PIM    | PIM     |
+------+--------------------+------------------------+---------------+----------+----------+---------+
| L3   | HEXA-3D-STACK      | n=6 layer TSV stack    | TSV ~96TB/s    | Egyptian  | n=6 lyr   | molecular|
|      | 3D stack           | + micro-fluid cooling  | vertical links | 3D split  |           | self-asmb|
+------+--------------------+------------------------+---------------+----------+----------+---------+
| L4   | HEXA-PHOTONIC      | n=6 wavelength WDM     | ~576 Tbps      | photonic  | n wavlen  | all-opt  |
|      | photonic interconn | sigma=12 channel route | optical tx     | split     | sigma ch  |          |
+------+--------------------+------------------------+---------------+----------+----------+---------+
| L5   | HEXA-WAFER         | n^2=36 die tiles       | ~2 PB/s        | Egyptian  | n^2=36    | quantum- |
|      | wafer-scale        | sigma=12 NoC/tile      | wafer-wide     | cooling    | dies      | wafer   |
+------+--------------------+------------------------+---------------+----------+----------+---------+
| L6   | HEXA-SUPERCOND     | 6-JJ SFQ gate          | ~300 GHz       | cryo       | n=6 JJ    | RT      |
|      | superconducting SFQ| sigma=12 JJ/gate       | SFQ clock      | 4.2 K      | sigma JJ  | supercond|
+------+--------------------+------------------------+---------------+----------+----------+---------+
| L7   | HEXA-QUANTUM-HYB   | 6 qubit hexagonal      | surface code   | cryo       | n qubit   | topolog. |
|      | quantum-classic hy | sigma=12 coupling      | d=6 err corr   | 15 mK      | sigma coup| qubit   |
+------+--------------------+------------------------+---------------+----------+----------+---------+
| L8   | HEXA-TOPO-ANYON    | n=6 anyon braid group  | tau=4 braid    | cryo       | n anyon   | non-abel|
|      | topological anyon  | sigma=12 topological Q | depth          | 2 mK       | tau depth | univ QC |
+------+--------------------+------------------------+---------------+----------+----------+---------+
| L9a  | HEXA-FIELD-EFFECT  | n=6 field lattice      | field prop     | cryo       | n lattice | univ    |
|      | field-effect comp. | sigma=12 field mode cp | tau=4 depth    | 2 mK       | sigma coup| field   |
+------+--------------------+------------------------+---------------+----------+----------+---------+
| L9b  | HEXA-PHOTON-TOPO   | sigma=12 optical modes | photon fusion  | 300K/2mK   | sigma mode| 6-wavelen|
|      | photon-topo fusion | phi=2 polariz qubit    | 6-wavelen WDM  | hetero     | phi polar | photon   |
+------+--------------------+------------------------+---------------+----------+----------+---------+
| L9c  | HEXA-NEUROMORPHIC  | n=6 neuron fanout      | spike timing   | room/      | n fanout  | bio      |
|      | neuromorphic       | sigma=12 synapse bits  | tau=4 timing   | 28nm       | sigma bits| efficient|
+------+--------------------+------------------------+---------------+----------+----------+---------+
| L10  | HEXA-DNA-MOLECULAR | 6-base-pair codon unit | molecular rxn  | room       | n bp      | in-vivo |
|      | DNA/molecular comp | sigma=12 reaction well | tau=4 gate     | biochem    | sigma well| molecular|
+------+--------------------+------------------------+---------------+----------+----------+---------+
```

---

## 2. Verification-metric comparison

```
+------+--------------------+----------+--------+-----------+------------+----------+---------+
| stage| name               | Rating   | EXACT  | DSE combos| no-go thms  | lens cons | op temp |
+------+--------------------+----------+--------+-----------+------------+----------+---------+
| L1   | HEXA-1-DIGITAL     | 7/10     | 24/24  | 248,832   | 6          | 11/22    | RT      |
| L2   | HEXA-2-PIM         | 8/10     | 26/26  | 1,327,104 | 6          | 13/22    | RT      |
| L3   | HEXA-3D-STACK      | 9/10     | 42/42  | 7,962,624 | 12         | 14/22    | RT      |
| L4   | HEXA-PHOTONIC      | 9/10     | 48/48  | 5,971,968 | 12         | 14/22    | RT      |
| L5   | HEXA-WAFER         | 9/10     | 54/54  | 10,077,696| 12         | 14/22    | RT      |
| L6   | HEXA-SUPERCOND     | 8/10     | 60/60  | 5,308,416 | 12         | 14/22    | 4.2 K   |
| L7   | HEXA-QUANTUM-HYB   | 7/10     | 66/66  | 5,308,416 | 12         | 12/22    | 15 mK   |
| L8   | HEXA-TOPO-ANYON    | 6/10     | 72/72  | 5,308,416 | 12         | 10/22    | 2 mK    |
| L9a  | HEXA-FIELD-EFFECT  | 5/10     | 78/78  | 5,308,416 | 12         | 8/22     | 2 mK    |
| L9b  | HEXA-PHOTON-TOPO   | 7/10     | 78/78  | 7,464,960 | 14         | 12/22    | 300K/2mK|
| L9c  | HEXA-NEUROMORPHIC  | 7/10     | 78/78  | 7,464,960 | 12         | 10/22    | RT      |
| L10  | HEXA-DNA-MOLECULAR | 4/10     | 78/84  | 3,359,232 | 12         | 8/22     | RT      |
+------+--------------------+----------+--------+-----------+------------+----------+---------+
| tot  |                    |          |704/710 |64,710,840 | 134        |          |         |
+------+--------------------+----------+--------+-----------+------------+----------+---------+
```

---

## 3. ASCII performance comparison charts

### 3-1. Rating (product maturity)

```
L1  DIGITAL      |#######...         7/10  (5nm mass-productionable)
L2  PIM          |########..         8/10  (Samsung/Hynix 1st gen shipped)
L3  3D-STACK     |#########.         9/10  (TSMC CoWoS commercial)
L4  PHOTONIC     |#########.         9/10  (Intel research stage)
L5  WAFER        |#########.         9/10  (Cerebras commercial)
L6  SUPERCOND    |########..         8/10  (IARPA SFQ research)
L7  QUANTUM-HYB  |#######...         7/10  (IBM/Google 100+ qubits)
L8  TOPO-ANYON   |######....         6/10  (Microsoft early research)
L9a FIELD-EFF    |#####.....         5/10  (theoretical stage)
L9b PHOTON-TOPO  |#######...         7/10  (Xanadu/PsiQuantum research)
L9c NEUROMORPHIC |#######...         7/10  (Intel Loihi gen 2)
L10 DNA-MOLECUL  |####......         4/10  (early university research)
                  0    2    4    6    8   10
```

### 3-2. EXACT verification rate

```
L1  DIGITAL      |########################  24/24   100%
L2  PIM          |########################  26/26   100%
L3  3D-STACK     |########################  42/42   100%
L4  PHOTONIC     |########################  48/48   100%
L5  WAFER        |########################  54/54   100%
L6  SUPERCOND    |########################  60/60   100%
L7  QUANTUM-HYB  |########################  66/66   100%
L8  TOPO-ANYON   |########################  72/72   100%
L9a FIELD-EFF    |########################  78/78   100%
L9b PHOTON-TOPO  |########################  78/78   100%
L9c NEUROMORPHIC |########################  78/78   100%
L10 DNA-MOLECUL  |#######################.  78/84   92.8%
                  0%       25%      50%      75%     100%
```

### 3-3. DSE exploration space (combination count)

```
L1  DIGITAL      |##                                   248K
L2  PIM          |######                               1.3M
L3  3D-STACK     |####################                 8.0M
L4  PHOTONIC     |###############                      6.0M
L5  WAFER        |#########################            10.1M  <-- max
L6  SUPERCOND    |#############                        5.3M
L7  QUANTUM-HYB  |#############                        5.3M
L8  TOPO-ANYON   |#############                        5.3M
L9a FIELD-EFF    |#############                        5.3M
L9b PHOTON-TOPO  |##################                   7.5M
L9c NEUROMORPHIC |##################                   7.5M
L10 DNA-MOLECUL  |########                             3.4M
                  0M      2M      4M      6M      8M     10M
```

### 3-4. Lens consensus (max 22)

```
L1  DIGITAL      |###########...........   11/22
L2  PIM          |#############.........   13/22
L3  3D-STACK     |##############........   14/22
L4  PHOTONIC     |##############........   14/22
L5  WAFER        |##############........   14/22
L6  SUPERCOND    |##############........   14/22
L7  QUANTUM-HYB  |############..........   12/22
L8  TOPO-ANYON   |##########............   10/22
L9a FIELD-EFF    |########..............    8/22
L9b PHOTON-TOPO  |############..........   12/22
L9c NEUROMORPHIC |##########............   10/22
L10 DNA-MOLECUL  |########..............    8/22
                  0    4    8   12   16   20  22
```

---

## 4. Evolution path (Mk.I -> Mk.V ceiling)

| Stage | Mk.I (current tech) | Mk.V (physical ceiling) |
|------|-------------------|-------------------|
| L1 | 5nm CMOS synthesizable | 2nm GAAFET limit |
| L2 | HBM3-PIM-based | CXL 3.0 + PIM integration |
| L3 | 6-layer TSV basic stack | molecular self-assembly limit |
| L4 | electro-optical hybrid | all-optical limit |
| L5 | 36 die tiles baseline | quantum-wafer limit |
| L6 | Nb SFQ 4.2K | room-temp superconducting computation limit |
| L7 | Transmon 6Q 15mK | topological-qubit quantum-advantage limit |
| L8 | Majorana 6-leg T-junction 20mK | non-abelian anyon universal QC limit |
| L9a | topological field-effect transistor 6-channel 20mK | universal field-effect computation limit |
| L9b | cryo photon-topology interface single module | 6-wavelength photonic-quantum universal computation limit |
| L9c | 6-fanout neuron core CMOS 28nm | biological efficiency limit approach |
| L10 | strand-displacement 4-gate reactor room temp | in-vivo molecular computer physical limit |

---

## 5. n=6 structural-through constants

Common n=6 arithmetic structures across all 12 design files:

```
n = 6             basic unit across all levels (MAC size, layers, wavelengths, qubits, dies, neuron fanout, codons)
sigma(6) = 12     wiring/channel/coupling/synaptic bits/reaction wells
tau(6) = 4        pipeline/stage/depth/timing
phi(6) = 2        polarization/channel/pair/polarization
Egyptian split    1/2 + 1/3 + 1/6 = 1 (power/cooling budget partition)
R(6) = 1          sigma*phi / (n*tau) = 24/24 = 1 (reversibility condition)
J2(6) = 24        accumulator width
sopfr(6) = 5      conservation-law / pipeline
```

---

## 6. Grand totals

| Item | Measured |
|------|-----:|
| Chip-design file count | 12 .md + 1 comparison |
| Total lines | 14,190 |
| Total BT EXACT | 704/710 (99.2%) |
| Total DSE combinations | 64,710,840 |
| Total no-go theorems | 134 |
| Average lens consensus | 11.8/22 |

> L1~L9: 100% EXACT across all stages. Only L10 is at 92.8% (6 TIGHT cases).
> End of comparison.
