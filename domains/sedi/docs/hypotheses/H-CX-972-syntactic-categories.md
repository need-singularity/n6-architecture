# H-CX-972: Syntactic Categories

> **Hypothesis**: Natural languages have approximately 8 = sigma - tau major syntactic categories (N, V, A, Adv, P, Det, Conj, Interj). A core of 4 = tau lexical categories (N, V, A, P) plus tau functional categories yields sigma - tau total.

## Grade: 🟧 APPROXIMATE

## Results

### The Correspondence

```
Major syntactic categories (traditional grammar):
  1. Noun (N)
  2. Verb (V)
  3. Adjective (A/Adj)
  4. Adverb (Adv)
  5. Preposition/Postposition (P)
  6. Determiner (Det)
  7. Conjunction (Conj)
  8. Interjection (Interj)
  Total: 8 = σ - τ

Core lexical categories (Chomsky, 1970):
  [+N, -V] = Noun
  [-N, +V] = Verb
  [+N, +V] = Adjective
  [-N, -V] = Preposition
  Total: 4 = τ (from φ binary features: [±N, ±V])
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, C(6,3) = 20
```

### Structural Analysis

```
Feature-based decomposition:
  Binary features [±N], [±V]:
    Combinations: 2² = φ² = τ
    These generate the 4 core lexical categories.

  Lexical vs. Functional split:
    Lexical (open class):    4 = τ (N, V, A, P)
    Functional (closed):     4 = τ (Det, Conj, Aux, Comp)
    Total:                   8 = σ - τ = 2τ

  Extended categories (some analyses):
    Add: Pronoun, Numeral, Particle, Classifier
    Total: up to 12 = σ
    But cross-linguistically stable set: 8 = σ-τ

Universal across languages:
  N and V: found in all languages (φ universal categories)
  A: nearly universal (sometimes merged with N or V)
  Core triad N, V, A: σ/τ = 3 most stable categories
```

### Physical Context

Syntactic categories are the building blocks of grammatical structure. Chomsky's feature system generates the tau core categories from phi binary features. The total of sigma - tau categories (8 traditional parts of speech) represents a balance between expressivity and learnability. The split into tau lexical plus tau functional categories mirrors the content/structure duality in language.

### Texas Sharpshooter Check

The 4 core lexical categories from 2 binary features (tau from phi) is an established linguistic analysis (Chomsky 1970), not a retrofit. The 8 total parts of speech is traditional but some analyses give 9-12. The sigma - tau = 8 match is approximate; its strength comes from the tau + tau decomposition and the binary feature basis.

## Verification

- [x] 4 core lexical categories = τ from φ² features
- [x] 8 traditional categories = σ - τ (standard grammar)
- [x] Lexical/functional split: τ + τ = σ - τ
- [x] Universal core: φ categories (N, V) in all languages
- [ ] Extended analyses give 9-12, weakening the σ-τ match
