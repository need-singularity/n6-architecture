# H-CX-570: Equal Temperament — 12 Semitones = σ(6), Octave Ratio = φ(6)

> **Hypothesis**: The 12-tone equal temperament system uses σ(6)=12 semitones per octave, where an octave is a frequency ratio of φ(6)=2.

## Grade: 🟩 CONFIRMED (exact; cultural universal)

## Results

```
Semitones per octave: 12 = σ(6)
Octave frequency ratio: 2 = φ(6)
Semitone ratio: 2^(1/12) = φ^(1/σ)
Perfect fifth: 7 semitones → 2^(7/12) ≈ 3/2 = (σ/τ)/φ
Major third: 4 semitones = τ(6) semitones
Minor third: 3 semitones = σ/τ semitones
Tritone: 6 semitones = P₁ (divides octave exactly in half)
```

### Why 12?

12 = σ(6) is chosen because it gives the best rational approximations to the harmonics:
- 2^(7/12) = 1.4983 ≈ 3/2 (perfect fifth, 0.11% error)
- 2^(4/12) = 1.2599 ≈ 5/4 (major third, 0.79% error)

### Pythagorean Comma

```
(3/2)^12 / 2^7 = 3^12/2^19 = 531441/524288 = 1.01364...

The comma arises because (σ/τ)^σ ≠ 2^(something exact).
The 12-semitone system distributes this comma equally.
```

### Circle of Fifths

12 = σ perfect fifths cycle through all notes. This works because gcd(7,12) = 1, i.e., 7 and σ are coprime. And 7 = M₃ (Mersenne prime).

## Status

- [x] 12 = σ semitones
- [x] 2 = φ octave ratio
- [x] Perfect fifth at 7 = M₃ semitones
- [x] Tritone = P₁ = 6 semitones
