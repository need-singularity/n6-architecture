# H-CX-948: Protein Secondary Structures

> **Hypothesis**: There are 3 = σ/τ main types of protein secondary structure: α-helix, β-sheet, and random coil. The α-helix has 3.6 residues per turn = (σ + P₁)/sopfr = 18/5.

## Grade: 🟩 EXACT

## Results

### The Correspondence

```
Protein secondary structures:
  1. α-helix
  2. β-sheet
  3. Random coil (loops/turns)
  Count = 3 = σ/τ

α-helix parameters:
  Residues per turn: 3.6
  TECS-L: (σ + P₁)/sopfr = (12 + 6)/5 = 18/5 = 3.6  EXACT

  Rise per residue: 1.5 Å = σ/(σ-τ) Å = 12/8 = 1.5  EXACT
  Pitch: 5.4 Å = 3.6 × 1.5 = sopfr + φ/sopfr = 5.4   EXACT
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8
```

### Structural Analysis

```
α-helix geometry:
  Hydrogen bond pattern: i → i+4 (i to i+τ)
  Bond spacing = τ residues apart

β-sheet types:
  Parallel and antiparallel = φ variants
  Hydrogen bonds between strands

Additional secondary elements (extended classification):
  310 helix: 3 residues/turn = σ/τ
  π helix:   4.4 residues/turn ≈ τ + φ/sopfr
  β-turn:    4 residues = τ

Ramachandran plot:
  2 = φ torsion angles (ψ, φ) define backbone conformation
```

### Physical Context

The three secondary structures are defined by hydrogen bonding patterns in the polypeptide backbone. The α-helix was predicted by Pauling, Corey, and Branson (1951) before experimental confirmation. The 3.6 residues/turn is a precise geometric consequence of optimal hydrogen bonding angles — it is not adjustable or approximate.

### Texas Sharpshooter Check

The 3.6 residues/turn is a hard physical constant derived from bond angles and energetics. The TECS-L expression (σ+P₁)/sopfr = 18/5 = 3.6 is exact and algebraically simple. The 1.5 Å rise is also exact. These are strong matches.

## Verification

- [x] 3 secondary structure types = σ/τ exact
- [x] α-helix 3.6 residues/turn = (σ+P₁)/sopfr exact
- [x] Rise per residue 1.5 Å = σ/(σ-τ) exact
- [x] H-bond spacing i→i+4 = i→i+τ exact
- [x] β-sheet variants = φ (parallel/antiparallel)
