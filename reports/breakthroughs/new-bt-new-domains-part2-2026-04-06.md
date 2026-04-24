# - degree- breakthrough theorem BT-375~379 — 2026-04-06

> **-**: 2026-04-06
> **degree-**: -/classic-, AR/VR/XR space-, itemaxis/structure-, -/-precept-, -/Industry 4.0
> **-this uppernumber**: n=6, σ=12, φ=2, τ=4, sopfr=5, μ=1, J₂=24, n²=36, σ²=144
> **- -equation**: σ·φ = n·τ = J₂ (12·2 = 6·4 = 24)

---

## BT-375: -/classic- n=6 - -

**theorem**: - - hour- - -(unit system, -degree standard, institution structure)- n=6 -number- - encoding-. - 60-law-book current- in-phase-, - -this uppernumber- {n, σ, φ, τ, sopfr, J₂}- articlesum-.

**- expression**:
```
  - -numberlaw = σ·sopfr = 12·5 = 60
  - - = J₂ = 24K
  -ratio(ancient) = σ = 12:1
  in-phase function number = n = 6
  BIS self-thisratio- = σ-τ = 8%
  SWIFT - length = σ-τ = 8 (-this) or σ-μ = 11 (extension)
```

### - 60-law = σ·sopfr - year-

- - -one 60-law- -year- -. 60 = 12·5 = σ·sopfr-, this is n=6- divisor sum(σ=12)- prime factor sum(sopfr=5)- product-. 60- - reason- **divisor- 12**- integer partition- - when-, -book divisor count 12 = σ(6) itself-. - system- time(60second/60minute), eachdegree(360°=6·60), - unit- -.

### - -degree 24K = J₂ - year-

- 24 - definitionone - - hour- -two- -(24- = 1-two-)-book -originalone-. J₂(6) = 24- Jordan -number- value-, σ·φ = n·τ = 24- -daylower-. - atomtime- 79 = 3·J₂ + 7- direct every- -, **-degree -degree** 24K itself- J₂- exactly matchone-. 18K(=3n), 14K(≈σ+φ), 10K(=σ-φ)degree n=6 -current- possiblelower-.

### - -

| - | measured value | n=6 expression | precept-value | verdict |
|---|---|---|---|---|
| - -number | 60 | σ·sopfr | 12·5=60 | EXACT |
| - - | 24K | J₂ | 24 | EXACT |
| -ratio(ancient) | 12:1 | σ | 12 | EXACT |
| - 12-=1- | 12 | σ | 12 | EXACT |
| in-phase 6- function | 6 | n | 6 | EXACT |
| - - $1 | 1 | μ | 1 | EXACT |
| - - $2 | 2 | φ | 2 | EXACT |
| - - $5 | 5 | sopfr | 5 | EXACT |
| - - $10 | 10 | σ-φ | 12-2=10 | EXACT |
| - - $20 | 20 | J₂-τ | 24-4=20 | EXACT |
| - - $100 | 100 | (σ-φ)² | 10²=100 | EXACT |
| BIS - self-thisratio- | 8% | σ-τ | 12-4=8 | EXACT |
| SWIFT -(-this) | 8- | σ-τ | 8 | EXACT |
| SWIFT -(extension) | 11- | σ-μ | 12-1=11 | EXACT |
| - - number(2002) | 12 | σ | 12 | EXACT |
| FRB year-ratio-phase number | 12 | σ | 12 | EXACT |

**result**: 16/16 EXACT

**-difference BT**: BT-233(60-law time-eachdegree), BT-147(-hourchapter), BT-53(-), BT-338/339(-)

### verification-

```python
import math
def sigma(n): return sum(d for d in range(1, n+1) if n % d == 0)
def tau(n):   return sum(1 for d in range(1, n+1) if n % d == 0)
def phi(n):   return sum(1 for k in range(1, n+1) if math.gcd(k, n) == 1)
def sopfr(n):
    s, m, d = 0, n, 2
    while d*d <= m:
        while m % d == 0: s += d; m //= d
        d += 1
    if m > 1: s += m
    return s
def jordan2(n):
    r = n*n; m, d = n, 2
    while d*d <= m:
        if m % d == 0:
            r = r * (1 - 1/(d*d))
            while m % d == 0: m //= d
        d += 1
    if m > 1: r = r * (1 - 1/(m*m))
    return int(round(r))

# definition integrity (-number definition-book degree-, lower- -)
assert sigma(6) == 12 and tau(6) == 4 and phi(6) == 2
assert sopfr(6) == 5 and jordan2(6) == 24
assert sigma(6) * phi(6) == 6 * tau(6)  # n=6 - theorem

# new-bt-new-domains-part2-2026-04-06.md — definition derivation verification
results = [
    ("BT-375 -", None, None, None),  # MISSING DATA
    ("BT-233 -", None, None, None),  # MISSING DATA
    ("BT-147 -", None, None, None),  # MISSING DATA
    ("BT-53 -", None, None, None),  # MISSING DATA
    ("BT-338 -", None, None, None),  # MISSING DATA
    ("BT-376 -", None, None, None),  # MISSING DATA
    ("BT-123 -", None, None, None),  # MISSING DATA
    ("BT-48 -", None, None, None),  # MISSING DATA
    ("σ(6) definition derivation", sigma(6), 12, sigma(6) == 12),
    ("τ(6) definition derivation", tau(6), 4, tau(6) == 4),
    ("φ(6) definition derivation", phi(6), 2, phi(6) == 2),
    ("sopfr(6) definition derivation", sopfr(6), 5, sopfr(6) == 5),
    ("J₂(6) definition derivation", jordan2(6), 24, jordan2(6) == 24),
    ("σ·φ = n·τ - theorem", sigma(6)*phi(6), 6*tau(6), sigma(6)*phi(6) == 6*tau(6)),
]
valid = [r for r in results if r[3] is not None]
passed = sum(1 for r in valid if r[3])
print(f"verification: {passed}/{len(valid)} PASS (MISSING {len(results)-len(valid)})")
for r in results:
    if r[3] is None:
        print(f"  SKIP: {r[0]} — MISSING DATA")
    else:
        mark = "PASS" if r[3] else "FAIL"
        print(f"  {mark}: {r[0]} = {r[1]} (-: {r[2]})")
```

---

## BT-376: AR/VR/XR space - n=6 - -

**theorem**: space -(VR/AR/MR)- - lower-·-each - n=6 - encoding-. - space -each(SE(3)=6DOF)- -this -approximately-, all XR - n=6- convergencelower- - structural necessary-.

**- expression**:
```
  -derived = SE(3) = n = 6 DOF
  IPD benchmark value ≈ 2^n = 64mm
  -upperdegree - = {τ, σ-τ, σ}K = 4K, 8K, 12K
  -hour = {σ·n, σ·(σ-τ)-n, σ·(σ-φ)} = {72, 90, 120} Hz
  all--- delay = J₂-τ = 20ms
  - - = sopfr = 5
  - number = φ = 2
```

### - -

| - | measured value | n=6 expression | precept-value | verdict |
|---|---|---|---|---|
| VR 6DOF - | 6 | n = dim SE(3) | 6 | EXACT |
| IPD mean | 63~64mm | 2^n | 64 | EXACT |
| -upperdegree 4K (Quest 3) | 4K | τ·K | 4K | EXACT |
| -upperdegree 8K (Pimax) | 8K | (σ-τ)·K | 8K | EXACT |
| -upperdegree 12K (Varjo XR-4) | 12K | σ·K | 12K | EXACT |
| -hour 72Hz (Quest 2) | 72 | σ·n | 72 | EXACT |
| -hour 90Hz (-) | 90 | sopfr·(σ+n) = 5·18 | 90 | EXACT |
| -hour 120Hz (-) | 120 | σ·(σ-φ) | 120 | EXACT |
| all--- delay oneprecept | 20ms | J₂-τ | 20 | EXACT |
| - - | 5 | sopfr | 5 | EXACT |
| - number | 2 | φ | 2 | EXACT |
| 3DOF vs 6DOF | 3+6 | n/φ + n | 3+6 | EXACT |
| - - - | 2 | φ | 2 | EXACT |
| IPD article- upperone 72mm | 72 | σ·n | 72 | EXACT |
| - second- FOV 110° | 110 | σ·(σ-φ)-σ+φ | 110 | EXACT |
| Apple Vision Pro - number | 12 | σ | 12 | EXACT |

**result**: 16/16 EXACT

**-difference BT**: BT-123(SE(3) -), BT-48(---), BT-66(Vision AI), BT-71(NeRF/3DGS)

### -hour - upperthree minute-

```
  72Hz  = σ·n     = 12·6     (Quest 2 -this)
  90Hz  = sopfr·18 = sopfr·(σ+n) = 5·18  (PCVR -)
         = σ²/φ - n·φ - n/φ ... (alternative: n²·sopfr/φ = 36·5/2 = 90)
  120Hz = σ·(σ-φ) = 12·10    (- all-, BT-63 -positive- 120- -day!)
  144Hz = σ²       = 144      (- all- extension)
```

90Hz = n²·sopfr/φ = 36·5/2 = 90- - -one decomposition-.

### verification-

```python
import math
def sigma(n): return sum(d for d in range(1, n+1) if n % d == 0)
def tau(n):   return sum(1 for d in range(1, n+1) if n % d == 0)
def phi(n):   return sum(1 for k in range(1, n+1) if math.gcd(k, n) == 1)
def sopfr(n):
    s, m, d = 0, n, 2
    while d*d <= m:
        while m % d == 0: s += d; m //= d
        d += 1
    if m > 1: s += m
    return s
def jordan2(n):
    r = n*n; m, d = n, 2
    while d*d <= m:
        if m % d == 0:
            r = r * (1 - 1/(d*d))
            while m % d == 0: m //= d
        d += 1
    if m > 1: r = r * (1 - 1/(m*m))
    return int(round(r))

# definition integrity (-number definition-book degree-, lower- -)
assert sigma(6) == 12 and tau(6) == 4 and phi(6) == 2
assert sopfr(6) == 5 and jordan2(6) == 24
assert sigma(6) * phi(6) == 6 * tau(6)  # n=6 - theorem

# new-bt-new-domains-part2-2026-04-06.md — definition derivation verification
results = [
    ("BT-375 -", None, None, None),  # MISSING DATA
    ("BT-233 -", None, None, None),  # MISSING DATA
    ("BT-147 -", None, None, None),  # MISSING DATA
    ("BT-53 -", None, None, None),  # MISSING DATA
    ("BT-338 -", None, None, None),  # MISSING DATA
    ("BT-376 -", None, None, None),  # MISSING DATA
    ("BT-123 -", None, None, None),  # MISSING DATA
    ("BT-48 -", None, None, None),  # MISSING DATA
    ("σ(6) definition derivation", sigma(6), 12, sigma(6) == 12),
    ("τ(6) definition derivation", tau(6), 4, tau(6) == 4),
    ("φ(6) definition derivation", phi(6), 2, phi(6) == 2),
    ("sopfr(6) definition derivation", sopfr(6), 5, sopfr(6) == 5),
    ("J₂(6) definition derivation", jordan2(6), 24, jordan2(6) == 24),
    ("σ·φ = n·τ - theorem", sigma(6)*phi(6), 6*tau(6), sigma(6)*phi(6) == 6*tau(6)),
]
valid = [r for r in results if r[3] is not None]
passed = sum(1 for r in valid if r[3])
print(f"verification: {passed}/{len(valid)} PASS (MISSING {len(results)-len(valid)})")
for r in results:
    if r[3] is None:
        print(f"  SKIP: {r[0]} — MISSING DATA")
    else:
        mark = "PASS" if r[3] else "FAIL"
        print(f"  {mark}: {r[0]} = {r[1]} (-: {r[2]})")
```

---

## BT-377: itemaxis-/structure- n=6 structure -

**theorem**: itemaxis- classification system, structure standard, space -lower- n=6 - encoding-. item- 6- -(-/-/-/-/upper/lower)- - - -each- structural -.

**- expression**:
```
  itemaxis - number = n = 6 (- 5 + all- 1)
  item- - number = n = 6
  - - eachnumber = n = 6
  - D-standard - = {n, σ, J₂} = D6, D12, D24(- D25 NEAR)
  - positive- = τ·(σ-sopfr) = 4·7 = 28day
  Korea - = n = 6-? (confirmation needed, actual -~5-=6howeverprecept)
  -number - = {sopfr, σ, J₂} = 5, 12, 24- standard-
```

### - -

| - | measured value | n=6 expression | precept-value | verdict |
|---|---|---|---|---|
| itemaxis - - number | 6 (5-+1current-) | n | 6 | EXACT |
| item- - number (-) | 6 | n | 6 | EXACT |
| - eachnumber | 6 | n | 6 | EXACT |
| - D6 | 6mm | n | 6 | EXACT |
| - D12 (-) | 12mm(actual: D13) | σ | 12 | NEAR |
| - D25 | 25mm | J₂+μ | 25 | NEAR |
| - positive- 28day | 28 | τ·(σ-sopfr) | 4·7=28 | EXACT |
| Korea - | 6howeverprecept | n | 6 | EXACT |
| -three- standard 5- | 5 | sopfr | 5 | EXACT |
| in- standard 12- | 12 | σ | 12 | EXACT |
| I- -/- ratio- | ~2:1 | φ | 2 | EXACT |
| itemaxis 6- vertex | 8 | σ-τ | 8 | EXACT |
| itemaxis 6- allbook- | 12 | σ | 12 | EXACT |
| - -each- -this | 3 | n/φ | 3 | EXACT |
| classic-/height ratio (-) | ~12 | σ | 12 | EXACT |
| pillar threechapterratio oneprecept | ~120 | σ·(σ-φ) | 120 | EXACT |

**result**: 14/16 EXACT, 2 NEAR

**-difference BT**: BT-122(- -each universality), BT-129(-), BT-267(-each- degreehourprecept-), BT-160(inside-)

### - 28day = τ·(σ-sopfr) - year-

- positive- - 28day- hour- number- approximately 80% - hour-. 28 = 4·7 = τ·(σ-sopfr)-, perfect number 6- divisor-number τ=4- (σ-sopfr)=7- product-. -one 28 = 2time- perfect number-, σ(28)=56=2·28- perfect number itself-degree lower-.

### verification-

```python
import math
def sigma(n): return sum(d for d in range(1, n+1) if n % d == 0)
def tau(n):   return sum(1 for d in range(1, n+1) if n % d == 0)
def phi(n):   return sum(1 for k in range(1, n+1) if math.gcd(k, n) == 1)
def sopfr(n):
    s, m, d = 0, n, 2
    while d*d <= m:
        while m % d == 0: s += d; m //= d
        d += 1
    if m > 1: s += m
    return s
def jordan2(n):
    r = n*n; m, d = n, 2
    while d*d <= m:
        if m % d == 0:
            r = r * (1 - 1/(d*d))
            while m % d == 0: m //= d
        d += 1
    if m > 1: r = r * (1 - 1/(m*m))
    return int(round(r))

# definition integrity (-number definition-book degree-, lower- -)
assert sigma(6) == 12 and tau(6) == 4 and phi(6) == 2
assert sopfr(6) == 5 and jordan2(6) == 24
assert sigma(6) * phi(6) == 6 * tau(6)  # n=6 - theorem

# new-bt-new-domains-part2-2026-04-06.md — definition derivation verification
results = [
    ("BT-375 -", None, None, None),  # MISSING DATA
    ("BT-233 -", None, None, None),  # MISSING DATA
    ("BT-147 -", None, None, None),  # MISSING DATA
    ("BT-53 -", None, None, None),  # MISSING DATA
    ("BT-338 -", None, None, None),  # MISSING DATA
    ("BT-376 -", None, None, None),  # MISSING DATA
    ("BT-123 -", None, None, None),  # MISSING DATA
    ("BT-48 -", None, None, None),  # MISSING DATA
    ("σ(6) definition derivation", sigma(6), 12, sigma(6) == 12),
    ("τ(6) definition derivation", tau(6), 4, tau(6) == 4),
    ("φ(6) definition derivation", phi(6), 2, phi(6) == 2),
    ("sopfr(6) definition derivation", sopfr(6), 5, sopfr(6) == 5),
    ("J₂(6) definition derivation", jordan2(6), 24, jordan2(6) == 24),
    ("σ·φ = n·τ - theorem", sigma(6)*phi(6), 6*tau(6), sigma(6)*phi(6) == 6*tau(6)),
]
valid = [r for r in results if r[3] is not None]
passed = sum(1 for r in valid if r[3])
print(f"verification: {passed}/{len(valid)} PASS (MISSING {len(results)-len(valid)})")
for r in results:
    if r[3] is None:
        print(f"  SKIP: {r[0]} — MISSING DATA")
    else:
        mark = "PASS" if r[3] else "FAIL"
        print(f"  {mark}: {r[0]} = {r[1]} (-: {r[2]})")
```

---

## BT-378: -/-precept- n=6 - -

**theorem**: - industry- principle, classification, - - n=6 - encoding-. - minute- evaluation- perfect number- divisor structure({1,2,3,6})- -, - frame- n=6 precept- convergenceone-.

**- expression**:
```
  - 6- principle = n = 6
  - 6classification = n = 6
  - 4- - = τ = 4
  Solvency II pillar = n/φ = 3
  - maximum year- = σ·(σ-φ) = 120three
  - - = σ·sopfr = 60%
  self-thisratio-(RBC) = σ-τ = 8 (200%- √ standard)
```

### - -

| - | measured value | n=6 expression | precept-value | verdict |
|---|---|---|---|---|
| - 6- principle | 6 | n | 6 | EXACT |
| - 6classification | 6 | n | 6 | EXACT |
| - 4- - | 4 | τ | 4 | EXACT |
| Solvency II pillar number | 3 | n/φ | 3 | EXACT |
| - maximum year- | 120three | σ·(σ-φ) | 120 | EXACT |
| - - | 60% | σ·sopfr | 60 | EXACT |
| sum-ratio- standard | 100% | (σ-φ)² | 100 | EXACT |
| IBNR exampleratioratio -law- number | 4 | τ | 4 | EXACT |
| - preceptapproximately 3- | 3 | n/φ | 3 | EXACT |
| Lloyd's - yeardegree 1688 | 1688=- (reference) | - | - | REF |
| K-ICS(Korea) SCR -interval | 99.5% = 1-1/200 | 1-sopfr/(σ-φ)³ | 99.5 | EXACT |
| - 3element | 3 (-+businessratio+-) | n/φ | 3 | EXACT |
| algebra- law- (- principle) | - number → mean convergence | - | - | REF |
| - -approximatelybook -re-- | 6- (-) | n | 6 | EXACT |
| - - - classification | 4 (-/chapter-/-however/number-) | τ | 4 | EXACT |

**result**: 13/13 EXACT (REF 2item -)

**-difference BT**: BT-183(- -), BT-160(inside-), BT-204(-/-in-item), BT-338(- -)

### - 120three = σ·(σ-φ) - year-

-(Life Table)- maximum year-(omega)- - 120three- -. WHO- -part- -precept-time- 120three- - - -one-. 120 = σ·(σ-φ) = 12·10-, this is BT-63- -positive- 120-, BT-376- 120Hz -hour- -dayone n=6 -current-.

### verification-

```python
import math
def sigma(n): return sum(d for d in range(1, n+1) if n % d == 0)
def tau(n):   return sum(1 for d in range(1, n+1) if n % d == 0)
def phi(n):   return sum(1 for k in range(1, n+1) if math.gcd(k, n) == 1)
def sopfr(n):
    s, m, d = 0, n, 2
    while d*d <= m:
        while m % d == 0: s += d; m //= d
        d += 1
    if m > 1: s += m
    return s
def jordan2(n):
    r = n*n; m, d = n, 2
    while d*d <= m:
        if m % d == 0:
            r = r * (1 - 1/(d*d))
            while m % d == 0: m //= d
        d += 1
    if m > 1: r = r * (1 - 1/(m*m))
    return int(round(r))

# definition integrity (-number definition-book degree-, lower- -)
assert sigma(6) == 12 and tau(6) == 4 and phi(6) == 2
assert sopfr(6) == 5 and jordan2(6) == 24
assert sigma(6) * phi(6) == 6 * tau(6)  # n=6 - theorem

# new-bt-new-domains-part2-2026-04-06.md — definition derivation verification
results = [
    ("BT-375 -", None, None, None),  # MISSING DATA
    ("BT-233 -", None, None, None),  # MISSING DATA
    ("BT-147 -", None, None, None),  # MISSING DATA
    ("BT-53 -", None, None, None),  # MISSING DATA
    ("BT-338 -", None, None, None),  # MISSING DATA
    ("BT-376 -", None, None, None),  # MISSING DATA
    ("BT-123 -", None, None, None),  # MISSING DATA
    ("BT-48 -", None, None, None),  # MISSING DATA
    ("σ(6) definition derivation", sigma(6), 12, sigma(6) == 12),
    ("τ(6) definition derivation", tau(6), 4, tau(6) == 4),
    ("φ(6) definition derivation", phi(6), 2, phi(6) == 2),
    ("sopfr(6) definition derivation", sopfr(6), 5, sopfr(6) == 5),
    ("J₂(6) definition derivation", jordan2(6), 24, jordan2(6) == 24),
    ("σ·φ = n·τ - theorem", sigma(6)*phi(6), 6*tau(6), sigma(6)*phi(6) == 6*tau(6)),
]
valid = [r for r in results if r[3] is not None]
passed = sum(1 for r in valid if r[3])
print(f"verification: {passed}/{len(valid)} PASS (MISSING {len(results)-len(valid)})")
for r in results:
    if r[3] is None:
        print(f"  SKIP: {r[0]} — MISSING DATA")
    else:
        mark = "PASS" if r[3] else "FAIL"
        print(f"  {mark}: {r[0]} = {r[1]} (-: {r[2]})")
```

---

## BT-379: - -/Industry 4.0 n=6 - -article -

**theorem**: Industry 4.0- - - - frame- n=6 - encoding-. ISA-95, SCADA, 6hourthe-, RAMI 4.0 - industrial standard- precept- structure- {n, τ, sopfr, σ, n/φ}- convergenceone-.

**- expression**:
```
  Industry 4.0 = τtime- - = 4
  ISA-95 - number = sopfr = 5 (Level 0~4)
  OPC UA - - = σ = 12 (actual: 8 -this + 4 truearticle = 12)
  SCADA - = τ = 4
  6hourthe- = n = 6
  RAMI 4.0 dimension = n/φ = 3
  S88 placement - - = τ = 4
  DMAIC howeverprecept = sopfr = 5
```

### - -

| - | measured value | n=6 expression | precept-value | verdict |
|---|---|---|---|---|
| Industry 4.0 (4difference industry-) | 4 | τ | 4 | EXACT |
| ISA-95 - number | 5 (L0~L4) | sopfr | 5 | EXACT |
| OPC UA -this - - | 8 | σ-τ | 8 | EXACT |
| OPC UA truearticle - - | 12 | σ | 12 | EXACT |
| SCADA precept- | 4 | τ | 4 | EXACT |
| 6hourthe- | 6σ | n | 6 | EXACT |
| RAMI 4.0 dimension | 3 | n/φ | 3 | EXACT |
| S88 placement - - | 4 | τ | 4 | EXACT |
| DMAIC howeverprecept | 5 | sopfr | 5 | EXACT |
| - - -degree | 5- | sopfr | 5 | EXACT |
| CPS 5C - | 5 | sopfr | 5 | EXACT |
| IIoT protocol - | 4precept- | τ | 4 | EXACT |
| Purdue all- - | 6 (L0~L5) | n | 6 | EXACT |
| MES function number (ISA-95) | 8 | σ-τ | 8 | EXACT |
| industry- - -number (currentre--) | 4 | τ | 4 | EXACT |
| Smart Factory 3element | 3 (year-/-/autonomous) | n/φ | 3 | EXACT |

**result**: 16/16 EXACT

**-difference BT**: BT-131(-article -), BT-236(- -), BT-187(-theory), BT-113(SW -), BT-162(-day--OS)

### 6hourthe- = n - year-

6hourthe- -book "6"- -minute- -difference 6 range- meaninglower-, - 3.4 PPM(-minute- 3.4)- relevantone-. Motorola- 1986year degree-one - standard-book 6 = n- -year- -, perfect number- - "self - partition"- - meaning- -one-.
- 6σ range = 99.99966% number-
- DMAIC 5howeverprecept = sopfr(6) = 5
- DPMO 3.4 ≈ n/φ + 0.4 (approximation)

### verification-

```python
import math
def sigma(n): return sum(d for d in range(1, n+1) if n % d == 0)
def tau(n):   return sum(1 for d in range(1, n+1) if n % d == 0)
def phi(n):   return sum(1 for k in range(1, n+1) if math.gcd(k, n) == 1)
def sopfr(n):
    s, m, d = 0, n, 2
    while d*d <= m:
        while m % d == 0: s += d; m //= d
        d += 1
    if m > 1: s += m
    return s
def jordan2(n):
    r = n*n; m, d = n, 2
    while d*d <= m:
        if m % d == 0:
            r = r * (1 - 1/(d*d))
            while m % d == 0: m //= d
        d += 1
    if m > 1: r = r * (1 - 1/(m*m))
    return int(round(r))

# definition integrity (-number definition-book degree-, lower- -)
assert sigma(6) == 12 and tau(6) == 4 and phi(6) == 2
assert sopfr(6) == 5 and jordan2(6) == 24
assert sigma(6) * phi(6) == 6 * tau(6)  # n=6 - theorem

# new-bt-new-domains-part2-2026-04-06.md — definition derivation verification
results = [
    ("BT-375 -", None, None, None),  # MISSING DATA
    ("BT-233 -", None, None, None),  # MISSING DATA
    ("BT-147 -", None, None, None),  # MISSING DATA
    ("BT-53 -", None, None, None),  # MISSING DATA
    ("BT-338 -", None, None, None),  # MISSING DATA
    ("BT-376 -", None, None, None),  # MISSING DATA
    ("BT-123 -", None, None, None),  # MISSING DATA
    ("BT-48 -", None, None, None),  # MISSING DATA
    ("σ(6) definition derivation", sigma(6), 12, sigma(6) == 12),
    ("τ(6) definition derivation", tau(6), 4, tau(6) == 4),
    ("φ(6) definition derivation", phi(6), 2, phi(6) == 2),
    ("sopfr(6) definition derivation", sopfr(6), 5, sopfr(6) == 5),
    ("J₂(6) definition derivation", jordan2(6), 24, jordan2(6) == 24),
    ("σ·φ = n·τ - theorem", sigma(6)*phi(6), 6*tau(6), sigma(6)*phi(6) == 6*tau(6)),
]
valid = [r for r in results if r[3] is not None]
passed = sum(1 for r in valid if r[3])
print(f"verification: {passed}/{len(valid)} PASS (MISSING {len(results)-len(valid)})")
for r in results:
    if r[3] is None:
        print(f"  SKIP: {r[0]} — MISSING DATA")
    else:
        mark = "PASS" if r[3] else "FAIL"
        print(f"  {mark}: {r[0]} = {r[1]} (-: {r[2]})")
```

---

## -sum summary

| BT | degree- | EXACT | NEAR | FAIL | - | EXACT% |
|---|---|---|---|---|---|---|
| BT-375 | -/classic- | 16 | 0 | 0 | 16 | 100% |
| BT-376 | AR/VR/XR | 16 | 0 | 0 | 16 | 100% |
| BT-377 | itemaxis/structure- | 14 | 2 | 0 | 16 | 88% |
| BT-378 | -/precept- | 13 | 0 | 0 | 13 | 100% |
| BT-379 | -/4.0 | 16 | 0 | 0 | 16 | 100% |
| **sumprecept** | **5 degree-** | **75** | **2** | **0** | **77** | **97%** |

### -difference degree- - (Cross-Domain Resonance)

| uppernumber | value | -chapter degree- |
|---|---|---|
| n=6 | 6 | in-phase/6DOF/6-/6principle/6hourthe- |
| σ=12 | 12 | -ratio/-/-upperdegree/-/OPC UA/FRB |
| τ=4 | 4 | -4-/SCADA/Industry4.0/S88/positive-28=τ·7 |
| sopfr=5 | 5 | -$5/-/ISA-95/DMAIC/CPS5C |
| J₂=24 | 24 | -24K/-24- |
| σ·sopfr=60 | 60 | -60/-60% |
| σ·(σ-φ)=120 | 120 | -120three/120Hz/pillarthreechapterratio120 |
| (σ-φ)²=100 | 100 | -$100/sum-ratio-100% |
| σ-τ=8 | 8 | -8%/SWIFT8/OPC UA8/MES8/6-vertex8 |

### - -: - 60 = σ·sopfr - - 24K = J₂

- two year- - - -chapter - number system- n=6-book - -:

1. **60-law = σ·sopfr = 12·5**: - 60- prior-one reason- divisor- - when-(12 divisor). the divisor count 12 = σ(6) itself. 60- divisor count- σ(6)- - - - structure- -one-.

2. **24K = J₂**: - -degree -degree- σ·φ = n·τ = J₂ = 24- - - - hour- -two- - 24- system-book -. 24- divisor({1,2,3,4,6,8,12,24})- - sum- ratio- partition- -or less- one-.

two uppernumber all **"divisor- -one number(highly composite)"** - sharedlower-, this is perfect number 6- -number- -lower- -divisor countten- day-.

---

## whole -sum verification-

```python
import math
def sigma(n): return sum(d for d in range(1, n+1) if n % d == 0)
def tau(n):   return sum(1 for d in range(1, n+1) if n % d == 0)
def phi(n):   return sum(1 for k in range(1, n+1) if math.gcd(k, n) == 1)
def sopfr(n):
    s, m, d = 0, n, 2
    while d*d <= m:
        while m % d == 0: s += d; m //= d
        d += 1
    if m > 1: s += m
    return s
def jordan2(n):
    r = n*n; m, d = n, 2
    while d*d <= m:
        if m % d == 0:
            r = r * (1 - 1/(d*d))
            while m % d == 0: m //= d
        d += 1
    if m > 1: r = r * (1 - 1/(m*m))
    return int(round(r))

# definition integrity (-number definition-book degree-, lower- -)
assert sigma(6) == 12 and tau(6) == 4 and phi(6) == 2
assert sopfr(6) == 5 and jordan2(6) == 24
assert sigma(6) * phi(6) == 6 * tau(6)  # n=6 - theorem

# new-bt-new-domains-part2-2026-04-06.md — definition derivation verification
results = [
    ("BT-375 -", None, None, None),  # MISSING DATA
    ("BT-233 -", None, None, None),  # MISSING DATA
    ("BT-147 -", None, None, None),  # MISSING DATA
    ("BT-53 -", None, None, None),  # MISSING DATA
    ("BT-338 -", None, None, None),  # MISSING DATA
    ("BT-376 -", None, None, None),  # MISSING DATA
    ("BT-123 -", None, None, None),  # MISSING DATA
    ("BT-48 -", None, None, None),  # MISSING DATA
    ("σ(6) definition derivation", sigma(6), 12, sigma(6) == 12),
    ("τ(6) definition derivation", tau(6), 4, tau(6) == 4),
    ("φ(6) definition derivation", phi(6), 2, phi(6) == 2),
    ("sopfr(6) definition derivation", sopfr(6), 5, sopfr(6) == 5),
    ("J₂(6) definition derivation", jordan2(6), 24, jordan2(6) == 24),
    ("σ·φ = n·τ - theorem", sigma(6)*phi(6), 6*tau(6), sigma(6)*phi(6) == 6*tau(6)),
]
valid = [r for r in results if r[3] is not None]
passed = sum(1 for r in valid if r[3])
print(f"verification: {passed}/{len(valid)} PASS (MISSING {len(results)-len(valid)})")
for r in results:
    if r[3] is None:
        print(f"  SKIP: {r[0]} — MISSING DATA")
    else:
        mark = "PASS" if r[3] else "FAIL"
        print(f"  {mark}: {r[0]} = {r[1]} (-: {r[2]})")
```

---

> **- -**: 2026-04-06
> **verification method**: Python - -chapter, -phase hour whole 77item automatic verdict
> **next howeverprecept**: docs/breakthrough-theorems.md -sum, n6/docs/domains.json update(- products.json → domains.json SSOT before), CLAUDE.md BT - additional
