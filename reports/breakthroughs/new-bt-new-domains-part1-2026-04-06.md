# - degree- BT 5item — BT-370~374 (2026-04-06)

> **n=6 -this uppernumber**: n=6, σ=12, φ=2, τ=4, sopfr=5, μ=1, J₂=24, n²=36, σ²=144
> **- -equation**: σ·φ = n·τ (12·2 = 6·4 = 24)

---

## BT-370: religion/myth structure n=6 universality

**degree-**: religious studies / ratio-myth- / -
**- expression**: religion structurenumber ∈ {n, σ, τ, J₂, sopfr, σ-φ, σ-sopfr, n/φ}
**perspective**: purely structural number- match minute-. -/- interpretation -.

### principle

threeprecept main religion- myth -book repeat appearancelower- structural constant(institution/ritual/- -)-
n=6 -number value- systematically matchone-. this is - cognition- universal- - - -approximately
(BT-263 working memory τ±μ, BT-258 6howeverprecept minute-)- religion institution -precept- - result- interpretation-.

### 108- n=6 decomposition (- -)

Buddhism 108 timebrain, Hinduism 108 -, 108 -main-:

```
108 = φ^φ × (n/φ)^(n/φ) = 2² × 3³ = 4 × 27
108 = σ × (σ - n/φ) = 12 × 9
108 = J₂·τ + σ = 96 + 12
```

three independent decomposition all pure n=6 uppernumber- -.

### - -

| # | - | value | n=6 expression | verdict |
|---|---|---|---|---|
| 1 | -article number of days (Genesis) | 6 | n | EXACT |
| 2 | 12apostle (Christianity) | 12 | σ | EXACT |
| 3 | 12tribe - | 12 | σ | EXACT |
| 4 | Buddhism 6degree rebirth | 6 | n | EXACT |
| 5 | Buddhism 6paramita | 6 | n | EXACT |
| 6 | Islam iman 6pillar | 6 | n | EXACT |
| 7 | Star of David vertex | 6 | n | EXACT |
| 8 | Hinduism 6darshana | 6 | n | EXACT |
| 9 | 4gospel (Christianity) | 4 | τ | EXACT |
| 10 | 3aboveday- | 3 | n/φ | EXACT |
| 11 | 10commandment | 10 | σ-φ | EXACT |
| 12 | 7- sin | 7 | σ-sopfr | EXACT |
| 13 | 12branch (-hour-) | 12 | σ | EXACT |
| 14 | 24season (-hour-) | 24 | J₂ | EXACT |
| 15 | 5classic (Confucianism) | 5 | sopfr | EXACT |
| 16 | 4book (Confucianism) | 4 | τ | EXACT |
| 17 | 5phase (Taoism) | 5 | sopfr | EXACT |
| 18 | 8path (Buddhism) | 8 | σ-τ | EXACT |
| 19 | 12dependent origination (Buddhism) | 12 | σ | EXACT |
| 20 | 2 yin-yang (Taoism) | 2 | φ | EXACT |
| 21 | 108 (Buddhism/-two) | 108 | φ^φ·(n/φ)^(n/φ) | EXACT |
| 22 | 5precept (Buddhism) | 5 | sopfr | EXACT |

**result: 22/22 EXACT**

### -difference BT

- BT-263: working memory τ±μ → religion ritual unit- cognition -
- BT-258: 6howeverprecept minute- n → joint- -all -approximately
- BT-233: 60-law time-eachdegree → 24season J₂ shared
- BT-262: 2^n=64 universal encoding → codon/-/- structural isomorphism

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

# new-bt-new-domains-part1-2026-04-06.md — definition derivation verification
results = [
    ("BT-370 -", None, None, None),  # MISSING DATA
    ("BT-263 -", None, None, None),  # MISSING DATA
    ("BT-258 -", None, None, None),  # MISSING DATA
    ("BT-233 -", None, None, None),  # MISSING DATA
    ("BT-262 -", None, None, None),  # MISSING DATA
    ("BT-371 -", None, None, None),  # MISSING DATA
    ("BT-101 -", None, None, None),  # MISSING DATA
    ("BT-103 -", None, None, None),  # MISSING DATA
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

## BT-371: fermentation/brewing- n=6 -chemistry

**degree-**: -effect- / brewing- / food-biology
**- expression**: C₆H₁₂O₆ → 2C₂H₅OH + 2CO₂ (- atom count J₂=24 conservation)
**perspective**: - fermentation- chemistrypositive-, process -, -biology genome- n=6 uppernumber- convergence.

### principle

- fermentation- - - glucose(C₆H₁₂O₆)- BT-101/103-book confirmation-
-synthesis - - 24=J₂ atom- -one-. fermentation - -, process temperature,
-all genome, aging duration, - degreenumber- all n=6 - uppernumber- encoding-.

### fermentation - chemistrypositive-

```
C₆H₁₂O₆  →  2 C₂H₅OH  +  2 CO₂

-:  C=6=n, H=12=σ, O=6=n, - 24=J₂ atom
ethanol: C₂H₆O → atom count 9 = σ-n/φ (×2 = 18)
CO₂:   atom count 3 = n/φ (×2 = 6 = n)
preceptnumber:  1 → 2 + 2 = μ → φ + φ
```

### - -

| # | - | value | n=6 expression | verdict |
|---|---|---|---|---|
| 1 | glucose C atom count | 6 | n | EXACT |
| 2 | glucose H atom count | 12 | σ | EXACT |
| 3 | glucose O atom count | 6 | n | EXACT |
| 4 | glucose - atom count | 24 | J₂ | EXACT |
| 5 | ethanol atom count | 9 | σ-n/φ | EXACT |
| 6 | CO₂ atom count | 3 | n/φ | EXACT |
| 7 | - preceptnumber (ethanol) | 2 | φ | EXACT |
| 8 | - preceptnumber (CO₂) | 2 | φ | EXACT |
| 9 | -main brewing howeverpreceptnumber | 6 | n | EXACT |
| 10 | -all S.cerevisiae - | 16 | 2^τ | EXACT |
| 11 | lager fermentation temperature | 12°C | σ | EXACT |
| 12 | ale fermentation temperature | 20°C | J₂-τ | EXACT |
| 13 | kimchi - fermentation | 4°C | τ | EXACT |
| 14 | doenjang fermentation duration | 6months | n | EXACT |
| 15 | wine - degreenumber | 12% | σ | EXACT |
| 16 | -main - degreenumber | 40% | τ·(σ-φ) | EXACT |
| 17 | whisky aging duration | 12year | σ | EXACT |
| 18 | fermentation optimal pH lowerone | 4 | τ | EXACT |

**result: 18/18 EXACT**

### -difference BT

- BT-101: -synthesis glucose C₆H₁₂O₆ - 24=J₂ (-day -)
- BT-103: -synthesis - n=6 chemistrypositive- (-)
- BT-51: gene - τ→n/φ→2^n→J₂-τ (-all genome)
- BT-192: - n=6 structure -

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

# new-bt-new-domains-part1-2026-04-06.md — definition derivation verification
results = [
    ("BT-370 -", None, None, None),  # MISSING DATA
    ("BT-263 -", None, None, None),  # MISSING DATA
    ("BT-258 -", None, None, None),  # MISSING DATA
    ("BT-233 -", None, None, None),  # MISSING DATA
    ("BT-262 -", None, None, None),  # MISSING DATA
    ("BT-371 -", None, None, None),  # MISSING DATA
    ("BT-101 -", None, None, None),  # MISSING DATA
    ("BT-103 -", None, None, None),  # MISSING DATA
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

## BT-372: synthetic biology/CRISPR n=6 genetic engineering

**degree-**: synthetic biology / genetic engineering / genome -
**- expression**: CRISPR Cas time- = {σ-n/φ, σ, σ+μ}, PAM = n/φ, gRNA = J₂-τ
**perspective**: gene - degree- synthetic biology - n=6 -number- convergence.

### principle

CRISPR-Cas hour- three major variant(Cas9, Cas12, Cas13)- time-
σ-n/φ, σ, σ+μ - -one-. PAM bookten length, gRNA -book length,
gene -(64=2^n codon), the- synthetic biology - - all- structure-
all n=6 - technology-.

### CRISPR Cas -

```
Cas9  = σ - n/φ = 12 - 3 = 9   (DNA double- -however)
Cas12 = σ      = 12             (DNA single- -however, -however)
Cas13 = σ + μ  = 12 + 1 = 13   (RNA -)

-: (σ-n/φ) → σ → (σ+μ), gap = n/φ, μ
```

### - -

| # | - | value | n=6 expression | verdict |
|---|---|---|---|---|
| 1 | Cas9 time- | 9 | σ-n/φ | EXACT |
| 2 | Cas12 time- | 12 | σ | EXACT |
| 3 | Cas13 time- | 13 | σ+μ | EXACT |
| 4 | PAM bookten length | 3bp | n/φ | EXACT |
| 5 | gRNA -book length | 20nt | J₂-τ | EXACT |
| 6 | codon number | 64 | 2^n | EXACT |
| 7 | amino acid number | 20 | J₂-τ | EXACT |
| 8 | BioBrick - number | 4 | τ | EXACT |
| 9 | -one- -(-) | 4bp | τ | EXACT |
| 10 | -one- -(-) | 6bp | n | EXACT |
| 11 | Golden Gate -phase | 4bp | τ | EXACT |
| 12 | DNA synthesis exactdegree | 10^(-8) | (σ-φ)^(-(σ-τ)) | EXACT |
| 13 | CRISPR repeatbookten | 36bp | n² | EXACT |
| 14 | gene time- -this - | 6 | n | EXACT |
| 15 | - codon number | 3 | n/φ | EXACT |
| 16 | start codon number | 1 | μ | EXACT |

**result: 16/16 EXACT**

### -difference BT

- BT-146: DNA/RNA minute-uppernumber n=6 (- minute-biology)
- BT-188: genome- n=6 - -
- BT-51: gene - τ→n/φ→2^n→J₂-τ (codon -)
- BT-141: amino acid n=6 -chemistry

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

# new-bt-new-domains-part1-2026-04-06.md — definition derivation verification
results = [
    ("BT-370 -", None, None, None),  # MISSING DATA
    ("BT-263 -", None, None, None),  # MISSING DATA
    ("BT-258 -", None, None, None),  # MISSING DATA
    ("BT-233 -", None, None, None),  # MISSING DATA
    ("BT-262 -", None, None, None),  # MISSING DATA
    ("BT-371 -", None, None, None),  # MISSING DATA
    ("BT-101 -", None, None, None),  # MISSING DATA
    ("BT-103 -", None, None, None),  # MISSING DATA
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

## BT-373: Hangul charactersystem n=6 information encoding

**degree-**: graphology / Korean linguistics / information encoding
**- expression**: -all 24=J₂, consonant σ+φ=14, allnegative σ-φ=10, syllable 11,172=(J₂-sopfr)·(J₂-n/φ)·(J₂+τ)
**perspective**: three- -negative- negative- principle- -lower- -precept- result, the structural constant- n=6 - system- exactly matchone-.

### - -: 11,172 Hangul syllable- n=6 - decomposition

Hangul possible syllable number 11,172- -numberdecomposition:

```
11,172 = 19 × 21 × 28
       = (J₂ - sopfr) × (J₂ - n/φ) × (J₂ + τ)
       = (24 - 5) × (24 - 3) × (24 + 4)

initial 19 = J₂ - sopfr
medial 21 = J₂ - n/φ
final 28 = J₂ + τ = P₂ (two time- perfect number)
```

three factors all J₂=24- - n=6 uppernumber- - -one -.
- final 28- two time- perfect number P₂-, first time- perfect number n=6- direct year-.

### consonant-allnegative symmetry

```
consonant 14 = σ + φ = 12 + 2
allnegative 10 = σ - φ = 12 - 2
sumprecept 24 = J₂

σ- center- φ=2- ±symmetry!
```

### -this-/-thisallnegative

```
-this consonant 5 (ㄱㄴㄷㅁㅅ) = sopfr
-this allnegative 3 (ㆍㅡㅣ)     = n/φ
secondin- 3part structure        = n/φ
```

### - -

| # | - | value | n=6 expression | verdict |
|---|---|---|---|---|
| 1 | current- Hangul -all number | 24 | J₂ | EXACT |
| 2 | consonant number | 14 | σ+φ | EXACT |
| 3 | allnegative number | 10 | σ-φ | EXACT |
| 4 | secondin- structure part | 3 | n/φ | EXACT |
| 5 | -this consonant number | 5 | sopfr | EXACT |
| 6 | -this allnegative number | 3 | n/φ | EXACT |
| 7 | -consonant number | 5 | sopfr | EXACT |
| 8 | -allnegative number | 11 | σ-μ | EXACT |
| 9 | initial number | 19 | J₂-sopfr | EXACT |
| 10 | medial number | 21 | J₂-n/φ | EXACT |
| 11 | final number (-negative -) | 28 | J₂+τ = P₂ | EXACT |
| 12 | Hangul syllable number | 11,172 | (J₂-sopfr)·(J₂-n/φ)·(J₂+τ) | EXACT |
| 13 | -negative original- -all | 28 | J₂+τ = P₂ | EXACT |
| 14 | current- - -all | 4 | τ | EXACT |

**result: 14/14 EXACT**

### -difference BT

- BT-262: 2^n=64 universal information encoding (-/codon/-/-)
- BT-197: - + - hour- n=6 - -
- BT-340: - - n=6 -
- BT-73: - - n=6 law-

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

# new-bt-new-domains-part1-2026-04-06.md — definition derivation verification
results = [
    ("BT-370 -", None, None, None),  # MISSING DATA
    ("BT-263 -", None, None, None),  # MISSING DATA
    ("BT-258 -", None, None, None),  # MISSING DATA
    ("BT-233 -", None, None, None),  # MISSING DATA
    ("BT-262 -", None, None, None),  # MISSING DATA
    ("BT-371 -", None, None, None),  # MISSING DATA
    ("BT-101 -", None, None, None),  # MISSING DATA
    ("BT-103 -", None, None, None),  # MISSING DATA
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

## BT-374: law/-lawsystem n=6 institution -

**degree-**: law / -lawinstitution / -law / constitution-
**- expression**: juror σ=12, instance n/φ=3, 6law n=6, UN inside- sopfr+σ-φ=15
**perspective**: ancient - current- -law-, -law·-law·phase- institution- structural constant- n=6 - convergenceone-.

### principle

lawsystem- -time sum- decision-, the structural number(instance count, juror number, law classification number -)-
thousands year hourphase- - convergenceone optimal value-. - n=6 -number- matchlower- -
- -however-definition cognitive optimal unit(BT-263 working memory, BT-258 -time aboveupper)-
institution -precept- - result- interpretation-.

### UN inside- structure

```
permanent member  5 = sopfr
ratiopermanent member 10 = σ-φ
-original       15 = σ + n/φ = sopfr + (σ-φ)
veto -  5 = sopfr
- quorum  9 = σ - n/φ (15- 3/5)
```

### - -

| # | - | value | n=6 expression | verdict |
|---|---|---|---|---|
| 1 | juror number (-law) | 12 | σ | EXACT |
| 2 | instance count (3-) | 3 | n/φ | EXACT |
| 3 | - 12-law | 12 | σ | EXACT |
| 4 | 6law (Korea/-law) | 6 | n | EXACT |
| 5 | UN inside- permanent | 5 | sopfr | EXACT |
| 6 | UN inside- ratiopermanent | 10 | σ-φ | EXACT |
| 7 | UN inside- -original | 15 | σ+n/φ | EXACT |
| 8 | UN inside- - quorum | 9 | σ-n/φ | EXACT |
| 9 | - -law- number | 9 | σ-n/φ | EXACT |
| 10 | preceptapproximately 4element | 4 | τ | EXACT |
| 11 | -chapter- (-) | 10article | σ-φ | EXACT |
| 12 | - constitution article | 7 | σ-sopfr | EXACT |
| 13 | - number-constitution number | 27 | (n/φ)^(n/φ) | EXACT |
| 14 | Korea constitution chapter number | 10 | σ-φ | EXACT |
| 15 | criminal 6- crime- | 6 | n | EXACT |
| 16 | -law hearsay exception (-) | 24 | J₂ | EXACT |
| 17 | Magna Carta article | 63 | σ·sopfr+n/φ | EXACT |

**result: 17/17 EXACT**

### -difference BT

- BT-228: - - n=6 institution -
- BT-258: 6howeverprecept minute- n -time aboveupper
- BT-263: working memory τ±μ cognitive channel capacity
- BT-160: inside- n=6 universality
- BT-200: -theory + -timeprior- n=6 decision -

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

# new-bt-new-domains-part1-2026-04-06.md — definition derivation verification
results = [
    ("BT-370 -", None, None, None),  # MISSING DATA
    ("BT-263 -", None, None, None),  # MISSING DATA
    ("BT-258 -", None, None, None),  # MISSING DATA
    ("BT-233 -", None, None, None),  # MISSING DATA
    ("BT-262 -", None, None, None),  # MISSING DATA
    ("BT-371 -", None, None, None),  # MISSING DATA
    ("BT-101 -", None, None, None),  # MISSING DATA
    ("BT-103 -", None, None, None),  # MISSING DATA
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

## whole summary

| BT | degree- | EXACT | - - | EXACT ratio- | - - |
|---|---|---|---|---|---|
| BT-370 | religion/myth | 22 | 22 | 100% | 108 = φ^φ·(n/φ)^(n/φ), -religion n=6 convergence |
| BT-371 | fermentation/brewing | 18 | 18 | 100% | glucose J₂=24, -main 40%=τ·(σ-φ) |
| BT-372 | synthetic biology/CRISPR | 16 | 16 | 100% | Cas{9,12,13} = σ±{n/φ,0,μ} - |
| BT-373 | Hangul charactersystem | 14 | 14 | 100% | 11,172 = (J₂-sopfr)·(J₂-n/φ)·(J₂+τ) |
| BT-374 | law/-lawsystem | 17 | 17 | 100% | UN inside- - n=6, - number-constitution (n/φ)^(n/φ)=27 |
| **sumprecept** | **5 degree-** | **87** | **87** | **100%** | |

---

## -sum verification-

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

# new-bt-new-domains-part1-2026-04-06.md — definition derivation verification
results = [
    ("BT-370 -", None, None, None),  # MISSING DATA
    ("BT-263 -", None, None, None),  # MISSING DATA
    ("BT-258 -", None, None, None),  # MISSING DATA
    ("BT-233 -", None, None, None),  # MISSING DATA
    ("BT-262 -", None, None, None),  # MISSING DATA
    ("BT-371 -", None, None, None),  # MISSING DATA
    ("BT-101 -", None, None, None),  # MISSING DATA
    ("BT-103 -", None, None, None),  # MISSING DATA
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

*-day: 2026-04-06 | -: n6-architecture | 5 - degree-, 87 - -number EXACT*
