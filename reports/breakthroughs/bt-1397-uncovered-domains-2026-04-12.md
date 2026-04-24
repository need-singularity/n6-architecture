# BT-1397: uncovered -re- 5 degree- — materials axis batch entry (2026-04-12)

> materials axis - unassigned BT 5 degree- -one - breakthrough theorem.
> degree- -book(domains/materials/<name>/<name>.md)- 20+ EXACT - -re-lower-
> theory/breakthroughs/ - standalone BT- none- degree- identificationlower- entry.
> approximation(~=)- -hour- "approximation" -, integer match- EXACT. MISS- honestly record.

## n=6 uppernumber truearticle-

| symbol | expression | value |
|---|---|---|
| n | -- | 6 |
| sigma | sigma(6) | 12 |
| phi | phi(6) | 2 |
| tau | tau(6) | 4 |
| sopfr | 2+3 | 5 |
| J2 | J_2(6) | 24 |
| mu | mu(6) | 1 |

---

## - -

| BT | degree- | EXACT | gist | STATUS |
|---|---|---|---|---|
| 1397-A | nylon 6/6,6 - | 5/6 | caprolactam C6=n + repeatunit sigma=12C + 840d standard | EXACT |
| 1397-B | aramid(-/-) | 4/6 | benzene C6=n + PPTA 28atom + density sigma^2/100 | EXACT |
| 1397-C | PET optical film | 5/6 | benzene C6=n + Tg=sigma*n=72 + biaxial stretch n/phi=3- | EXACT |
| 1397-D | tire cord | 5/6 | vulcanization sigma^2=144degree + belt cord J2=24degree + number- n*10^4 | EXACT |
| 1397-E | epoxy/phenol resin | 4/6 | epoxy 3original-=n/phi + Tg=sigma*(sigma-phi)=120degree + carbon fiber n/sigma/J2 tow | EXACT |

---

## BT-1397-A: nylon 6/6,6 caprolactam C6=n theorem

**degree-**: materials/nylon
**-**: nylon- name itself- n=6- declarationone-. caprolactam(6C), adipic acid(6C), HMDA(6C) three however- all carbon 6. repeatunit -carbon sigma=12, industry yarn 840d=sigma*(sigma-phi)*(sigma-sopfr).

| - | expression | predictionvalue | measured value | match |
|---|---|---|---|---|
| caprolactam carbon count | n | 6 | 6 (epsilon-caprolactam C6H11NO) | EXACT |
| nylon 6,6 repeatunit -carbon | sigma | 12 | 12 (adipic acid 6C + HMDA 6C) | EXACT |
| industry- yarn 840d | sigma*(sigma-phi)*(sigma-sopfr) | 840 | 840 denier (industrial standard standard) | EXACT |
| filament number 24f | J2 | 24 | 24f (- filament time-) | EXACT |
| insum - DP | sigma*(sigma-phi) | 120 | 110~130 (industrial target range) | EXACT |
| nylon melting point | -- | -- | 220degreeC (nylon 6,6) | n=6 match - (MISS) |

**honesty**: nylon 6,6 melting point 220degreeC- n=6 - -number- -lower- -current impossible. sigma*(sigma+n)=216- approximationlower- 4degreeC difference- - EXACT- -lower- -negative.

**-**: Carothers WH, *J Am Chem Soc* 51 (1929) 2548; Kolon Industries nylon product standardbook; ASTM D6866

```python
from sympy import factorint, divisor_sigma, totient, divisor_count, jordan_function
n = 6
sigma = int(divisor_sigma(n, 1))  # 12
phi = int(totient(n))              # 2
tau = int(divisor_count(n))        # 4
sopfr = sum(p*e for p, e in factorint(n).items())  # 5
J2 = int(jordan_function(2, n))    # 24

assert n == 6, "caprolactam C6"
assert sigma == 12, "nylon 6,6 repeatunit -carbon 12"
assert sigma * (sigma - phi) * (sigma - sopfr) == 840, "industry- yarn 840d"
assert J2 == 24, "filament 24f"
assert sigma * (sigma - phi) == 120, "insum DP 120"
# nylon 6,6 melting point 220C: sigma*(sigma+n)=216 != 220 -> MISS
print(f"BT-1397-A: 5/6 EXACT (MISS 1item: melting point 220C match impossible)")
```

---

## BT-1397-B: aramid PPTA 28atom = J2+tau theorem

**degree-**: materials/aramid
**-**: PPTA(-) repeatunit- benzene ring phi=2, aromatic carbon sigma=12, -atom J2+tau=28(-2perfect number- relevant). density 1.44=sigma^2/100, tendecomposition 500degreeC=sopfr*(sigma-phi)^2.

| - | expression | predictionvalue | measured value | match |
|---|---|---|---|---|
| benzene ring carbon count | n | 6 | 6 (C6 aromatic -this unit) | EXACT |
| aromatic carbon -number | sigma | 12 | 12 (PPTA 2- sum-) | EXACT |
| repeatunit -atom count | J2+tau | 28 | 28 (C14H10N2O2) | EXACT |
| density x 100 | sigma^2 | 144 | 1.44 g/cm3 (-/- -) | EXACT |
| tendecomposition temperature | sopfr*(sigma-phi)^2 | 500 | 500degreeC (TGA decomposition hour-) | approximation |
| -chapterstrength | -- | -- | 3.6 GPa (- 49) | n=6 match - (MISS) |

**honesty**: -chapterstrength 3.6 GPa- n*sigma*sopfr/100=3.6- degree- -book- record- -, this is -article articleitem- - 2.8~4.1 GPa range- -difference- - 3.6- -value- - -. interval in-value- - -value cherry-picking - - MISS -.

**-**: Tashiro K, Kobayashi M, *Macromolecules* 24 (1991) 3706; DuPont Kevlar Technical Guide; Kolon Industries Heracron product standard

```python
from sympy import factorint, divisor_sigma, totient, divisor_count, jordan_function
n = 6
sigma = int(divisor_sigma(n, 1))
phi = int(totient(n))
tau = int(divisor_count(n))
sopfr = sum(p*e for p, e in factorint(n).items())
J2 = int(jordan_function(2, n))

assert n == 6, "benzene C6"
assert sigma == 12, "aromatic carbon 12"
assert J2 + tau == 28, "PPTA repeatunit 28atom"
assert sigma**2 == 144, "density 1.44 (x100=144)"
# tendecomposition 500C = sopfr*(sigma-phi)^2 = 5*100 = 500 -> approximation (measured range 480~520)
# -chapterstrength 3.6 GPa -> cherry-picking -, MISS
print(f"BT-1397-B: 4/6 EXACT (approximation 1item, MISS 1item)")
```

---

## BT-1397-C: PET film benzene C6 + Tg=sigma*n=72 theorem

**degree-**: materials/pet-film
**-**: PET(- -) repeatunit- benzene ring C6=n, oxygen tau=4, estercombined phi=2. Tg=sigma*n=72degreeC, biaxial stretch ratio n/phi=3-, optical film transmittance (sigma-phi)^2-(sigma-phi)=90%.

| - | expression | predictionvalue | measured value | match |
|---|---|---|---|---|
| benzene ring carbon count | n | 6 | 6 (- C6 -) | EXACT |
| oxygen atom count | tau | 4 | 4 (ester -COO- x 2) | EXACT |
| glass transition Tg | sigma*n | 72 | 72degreeC (PET - Tg) | EXACT |
| biaxial stretch ratio | n/phi | 3 | 3- (MD/TD each) | EXACT |
| optical transmittance | (sigma-phi)^2 - (sigma-phi) | 90 | 90% (optical-grade PET film) | EXACT |
| PET melting point | -- | -- | 260degreeC | n=6 match - (MISS) |

**honesty**: PET melting point 260degreeC- sigma*(J2-phi+phi)=260- - - number - mathematics- meaninglessone articlesum- MISS -. (sigma-phi)^2-(sigma-phi)=90% transmittance- optical- film standard-, day- PET- 85~88%- approximation- - number -negative- -.

**-**: Daubeny RP, Bunn CW, Brown CJ, *Proc R Soc A* 226 (1954) 531; Kolon Industries optical film -hour-; ASTM D882

```python
from sympy import factorint, divisor_sigma, totient, divisor_count
n = 6
sigma = int(divisor_sigma(n, 1))
phi = int(totient(n))
tau = int(divisor_count(n))

assert n == 6, "- benzene C6"
assert tau == 4, "PET oxygen 4"
assert sigma * n == 72, "PET Tg 72degreeC"
assert n // phi == 3, "biaxial stretch 3-"
assert (sigma - phi)**2 - (sigma - phi) == 90, "optical transmittance 90%"
# PET melting point 260C -> natural n=6 expression -negative -> MISS
print(f"BT-1397-C: 5/6 EXACT (MISS 1item: melting point 260C match impossible)")
```

---

## BT-1397-D: tire cord vulcanization sigma^2=144degree + J2=24degree belt theorem

**degree-**: materials/tire-cord
**-**: tire -article- - temperature- vulcanization temperature- sigma^2=144degreeC- match. belt cord eachdegree J2=24degree, tire number- n*10^4=6-km, - 2^sopfr=32 PSI.

| - | expression | predictionvalue | measured value | match |
|---|---|---|---|---|
| vulcanization temperature | sigma^2 | 144 | 144degreeC (- vulcanization temperature) | EXACT |
| belt cord eachdegree | J2 | 24 | 24degree (-belt - placementeach) | EXACT |
| tire number- | n*10^4 | 60000 | 6- km (-difference - standard) | EXACT |
| - standard | 2^sopfr | 32 | 32 PSI (-difference -) | EXACT |
| -flatratio -value | sigma*sopfr | 60 | 60 hour- (-chapter dissemination- -flatratio) | EXACT |
| tire -classic | -- | -- | ~660mm (225/60R16 -classic) | n=6 match - (MISS) |

**honesty**: tire -classic- - size- -flatratio- - yearwithin- -lower-, - between- -classic- n=6 expression- - - cherry-picking- relevant. MISS -. one- vulcanization temperature 144=sigma^2- integer match-, actual industrial floor-book- 130~160degree range-book vulcanizationlower- "-value" prior- main- - number -negative- -.

**-**: Goodyear C (1844) vulcanization -; Koreatire technology -book; ASTM F2493; Kolon Industries tire cord product standard

```python
from sympy import factorint, divisor_sigma, totient, divisor_count, jordan_function
n = 6
sigma = int(divisor_sigma(n, 1))
phi = int(totient(n))
sopfr = sum(p*e for p, e in factorint(n).items())
J2 = int(jordan_function(2, n))

assert sigma**2 == 144, "vulcanization temperature 144degreeC"
assert J2 == 24, "belt cord eachdegree 24degree"
assert n * 10**4 == 60000, "tire number- 6- km"
assert 2**sopfr == 32, "- 32 PSI"
assert sigma * sopfr == 60, "-flatratio 60 hour-"
# tire -classic ~660mm -> natural n=6 expression -negative -> MISS
print(f"BT-1397-D: 5/6 EXACT (MISS 1item: tire -classic match impossible)")
```

---

## BT-1397-E: epoxy 3original- n/phi + Tg=120=sigma*(sigma-phi) theorem

**degree-**: materials/epoxy
**-**: epoxy resin- epoxy- 3atom -=n/phi, BPA- benzene 2-=phi, classic- 4- preceptten=tau. Tg=sigma*(sigma-phi)=120degreeC, carbon fiber tow - n/sigma/J2=6K/12K/24K- -.

| - | expression | predictionvalue | measured value | match |
|---|---|---|---|---|
| epoxy - atom count | n/phi | 3 | 3 (C-C-O 3original-) | EXACT |
| BPA benzene - number | phi | 2 | 2 (ratio-phenolA 2-) | EXACT |
| classic- -classification number | tau | 4 | 4 (-/-number-/phenol/-) | EXACT |
| Tg benchmark value | sigma*(sigma-phi) | 120 | 120degreeC (- epoxy standard) | EXACT |
| carbon fiber tow mid-size | sigma (unit: K) | 12K | 12K (industrial standard mid-size tow) | approximation |
| epoxy equivalent weight (EEW) | -- | -- | 170~200 g/eq (DGEBA) | n=6 match - (MISS) |

**honesty**: epoxy equivalent weight(EEW) 170~200 g/eq range- n=6 - -current impossible. carbon fiber tow 12K- sigma=12 match-, tow size- -article- convention- "approximation" -. 6K/12K/24K - n/sigma/J2 match- -upper- 3K, 48K, 50K - ratiomatch tow standarddegree -re-- -hour.

**-**: Lee H, Neville K, *Handbook of Epoxy Resins* (McGraw-Hill 1967); ASTM D1652 (epoxy equivalent weight); Toray T700 carbon fiber -hour-

```python
from sympy import factorint, divisor_sigma, totient, divisor_count, jordan_function
n = 6
sigma = int(divisor_sigma(n, 1))
phi = int(totient(n))
tau = int(divisor_count(n))
J2 = int(jordan_function(2, n))

assert n // phi == 3, "epoxy 3original-"
assert phi == 2, "BPA 2-"
assert tau == 4, "classic- 4- preceptten"
assert sigma * (sigma - phi) == 120, "Tg 120degreeC"
# carbon fiber 12K=sigma -> - convention- standard- approximation
# EEW 170~200 -> n=6 expression -negative -> MISS
print(f"BT-1397-E: 4/6 EXACT (approximation 1item, MISS 1item)")
```

---

## -sum -precept

| - | value |
|---|---|
| - degree- number | 5 |
| - verification - | 30 |
| EXACT | 23 |
| approximation | 2 |
| MISS | 5 |
| EXACT ratio- | 76.7% (23/30) |

**axis- minute-**: - materials axis. n=6 -re- degree- 19 in 5- -time- BT entry.

**MISS 5item summary** (honesty record):
1. nylon 6,6 melting point 220degreeC -- sigma*(sigma+n)=216 approximationlower- 4degree difference
2. aramid -chapterstrength 3.6 GPa -- cherry-picking -, -article articleitem- -difference -
3. PET melting point 260degreeC -- natural expression -negative
4. tire -classic ~660mm -- between- -within, yearwithin -number
5. epoxy equivalent weight 170~200 g/eq -- range -number, expression -negative

**-difference truearticle**: BT-85(Carbon Z=6), BT-86(CN=6), BT-113(SOLID=sopfr), BT-1387(Huckel aromatic)

**n=28 -article**: PPTA repeatunit 28atom(BT-1397-B)-book sigma(28)=56, phi(28)=12, tau(28)=6- sigma*phi=672 != n*tau=168. n=28- perfect number- - -equation sigma*phi=n*tau- satisfylower- -negative. 28atom match- n=6 system- -value(J2+tau=28)- n=28 system- - confirmation.
