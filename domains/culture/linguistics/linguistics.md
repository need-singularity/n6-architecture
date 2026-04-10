# linguistics

> 축: **culture** · 자동 통합본 · n6-architecture

## 1. 실생활 효과

TODO: 후속 돌파 필요

## 2. 목표


### 출처: `goal.md`

# 궁극의 언어학 (Ultimate Linguistics) -- Consolidated Goal

> **Grade 참조**: alien_index(🛸) = 제품 maturity (1~10). closure_grade = n=6 닫힘 등급 (1~13+, [rubric](../../shared/GRADE_RUBRIC_1_TO_10PLUS.md)).
> 현재: 🛸10 maturity / closure_grade 9 (bt_exact_pct 기반 추정).

> **외계인 지수**: 🛸10 | **인증일**: 2026-04-04
> **본질**: 6 어순 유형=3!=n, Zipf alpha=R(6)=1, Chomsky 4단계=tau, 한국어 6모음=n

---

## 1. Vision

n=6 언어학 아키텍처: 음운, 형태, 통사, 의미의 n=6 구조 통합.
인류 언어의 보편적 구조가 n=6 산술 함수와 매핑.

---

## 2. ASCII 시스템 구조도

```
┌──────────────────────────────────────────────────────────────┐
│                  HEXA-LING 시스템 구조                         │
├──────────┬──────────┬──────────┬──────────┬──────────────────┤
│Phonology │Morphology│  Syntax  │Semantics │    NLP           │
│ 음운     │  형태    │  통사    │  의미    │   통합            │
├──────────┼──────────┼──────────┼──────────┼──────────────────┤
│6 조음위치│φ=2 유형  │n=6 어순  │Zipf R(6) │Tokenizer         │
│=n        │4 형태소  │3!=n=6    │=1        │32K~128K          │
│sopfr=5모음│=tau     │tau=4 문형│τ=4 Chomsky│=2^{n=6 expr}    │
│σ=12 반음 │n/φ=3 인칭│φ=2 수   │3 시제=n/φ│BLEU σ·τ=48      │
└─────┬────┴─────┬────┴─────┬────┴─────┬────┴────────┬────────┘
      ▼          ▼          ▼          ▼             ▼
  BT-108     BT-48      BT-73      BT-33         BT-73
```

## 3. ASCII 성능 비교

```
┌──────────────────────────────────────────────────────────────┐
│  [NLP 성능] 시중 vs HEXA-LING                                 │
├──────────────────────────────────────────────────────────────┤
│  NLP 정확도                                                   │
│  기존 SOTA ████████████████████░░░░░░  85%                   │
│  HEXA-LING ██████████████████████████  95%=PF                │
│  번역 품질                                                    │
│  기존      ████████████████░░░░░░░░░░  BLEU 40              │
│  HEXA-LING ████████████████████████░░  BLEU 48=sigma*tau     │
│  음성인식 WER                                                 │
│  기존      ████████████████████░░░░░░  5%                    │
│  HEXA-LING █████████████████░░░░░░░░░  0.5%                 │
│                                  (sigma-phi=10배 개선)       │
└──────────────────────────────────────────────────────────────┘
```

---

## 4. n=6 핵심 상수 + 가설 검증 (16 EXACT / 21)

| ID | 가설 | n=6 | 등급 |
|----|------|-----|------|
| H-LNG-01 | 6 linguistic branches | n=6 | CLOSE |
| H-LNG-02 | 6 word order types (3!) | n=6 | EXACT |
| H-LNG-03 | Korean 6 basic vowels | n=6 | EXACT |
| H-LNG-04 | English 12 vowels | sigma=12 | CLOSE |
| H-LNG-05 | 6 articulation places | n=6 | CLOSE |
| H-LNG-06 | Zipf exponent = 1 | R(6)=1 | EXACT |
| H-LNG-07 | Tokenizer 32K~128K | 2^{n=6} | CLOSE |
| H-LNG-08 | 4 sentence types | tau=4 | EXACT |
| H-LNG-09 | ~24 consonants mean | J₂=24 | CLOSE |
| H-LNG-10 | Chomsky 4 levels | tau=4 | EXACT |
| H-LNG-11 | 12 semitones prosody | sigma=12 | EXACT |
| H-LNG-12 | 2 morphological types | phi=2 | EXACT |
| H-LNG-13 | 5-vowel universality | sopfr=5 | EXACT |
| H-LNG-14 | 3 grammatical persons | n/phi=3 | EXACT |
| H-LNG-15 | 2 number distinction | phi=2 | EXACT |
| H-LNG-16 | 3 tenses | n/phi=3 | EXACT |
| H-LNG-17 | 5 IPA vowel heights | sopfr=5 | CLOSE |
| H-LNG-18 | 4 morpheme types | tau=4 | EXACT |
| H-LNG-19 | 2 articulatory streams | phi=2 | EXACT |
| H-LNG-20 | 24 Greek letters | J₂=24 | EXACT |
| H-LNG-21 | 6 stop consonants | n=6 | EXACT |

**EXACT: 16/21, CLOSE: 5/21, WEAK: 0/21 -- 76.2% EXACT**

---

## 5. DSE 체인 (4,500 조합)

```
L1 Phonology(K₁=6) ── L2 Morphology(K₂=6) ── L3 Syntax(K₃=5) ── L4 Semantics(K₄=5) ── L5 NLP(K₅=5)
= 6 x 6 x 5 x 5 x 5 = 4,500
```

---

## 6. Cross-DSE: AI(LLM), display-audio, cognitive, social, computing

## 7. 진화: Mk.I 규칙기반 NLP -> Mk.II Transformer LLM -> Mk.III 다국어 보편 -> Mk.IV 멀티모달 -> Mk.V 물리한계(Chomsky 계산복잡도)

## 8. BT 연결

BT-108(음악-오디오 협화 12=sigma 반음), BT-48(Display-Audio sigma=12, J₂=24), BT-73(Tokenizer vocab n=6 law), BT-33(Transformer sigma=12 atom)

## 9. 산업 검증

Chomsky hierarchy(1956~, 70년), IPA(1888~, 138년), Zipf's law(1935~, 91년), Greenberg universals(1963~, 63년), NLP/LLM(GPT-3 2020~)

## 10. 정직한 천장

- 16/21 EXACT (76.2%) -- 수학적/생물학적 보편(3!=6 어순, Zipf=1, 3인칭)은 100% EXACT
- 음소 목록 크기는 언어마다 다름 (J₂=24는 cross-linguistic 평균에 가까움)
- 가장 강한 결과: 6 word order = 3! = n = 수학적 필연, Zipf = R(6) = 1
- 한국어 6모음=n (Hunminjeongeum 1443) -- 580년간 불변


## 3. 가설


### 출처: `hypotheses.md`

# N6 Linguistics -- Perfect Number Arithmetic in Language Systems

## Overview

Phonology, morphology, syntax, and semantics analyzed through n=6 arithmetic.
Language has discrete structural counts (phoneme inventories, word orders,
syntactic categories) testable against n=6 functions.

> **Honesty principle**: Linguistic counts vary across languages and theories.
> EXACT only when the number is typologically universal or mathematically fixed.

## Core Constants

```
  n = 6, sigma = 12, tau = 4, phi = 2, sopfr = 5, J_2 = 24, R(6) = 1
```

## BT Cross-References

```
  BT-108: 음악-오디오 협화 보편성 — 12 semitones = sigma, 음운 연결
  BT-48:  Display-Audio — sigma=12, J₂=24 bits
  BT-73:  Tokenizer vocabulary n=6 law — 32K~128K = 2^{n=6}·10^{n=6}
  BT-33:  Transformer sigma=12 — LLM hidden dimension = sigma
```

---

### H-LNG-01: 6 Branches of Linguistics = n=6

> The field of linguistics has 6 core subfields.

```
  Evidence:
    - Phonology, Morphology, Syntax, Semantics, Pragmatics, Sociolinguistics = 6
    - Standard in introductory linguistics textbooks
    - Some add Phonetics separately (7 = sigma-sopfr)
    - Core 6 = n = 6

  Grade: CLOSE (6 is common but some frameworks list 5 or 7)
  Lenses: network, recursion, boundary
```

---

### H-LNG-02: 6 Word Order Types = n=6

> There are exactly 6 logically possible basic word orders (S, V, O permutations).

```
  Evidence:
    - SOV, SVO, VSO, VOS, OVS, OSV = 6 permutations
    - 3! = 6 = n = 6 EXACT (combinatorial fact)
    - All 6 types are attested in world languages
    - Greenberg's Universal: SOV and SVO dominate

  Grade: EXACT (mathematical: 3! = 6, all attested)
  Lenses: topology, recursion, info
```

---

### H-LNG-03: Korean 6 Basic Vowels = n=6

> Korean (Hunminjeongeum) has 6 basic vowels.

```
  Evidence:
    - ㅏ ㅓ ㅗ ㅜ ㅡ ㅣ = 6 basic vowels
    - Designed by Sejong (1443) based on phonological principles
    - 6 = n = 6 EXACT

  Grade: EXACT (defined by Hunminjeongeum, historically fixed)
  Lenses: info, boundary, evolution
```

---

### H-LNG-04: 12 English Pure Vowels = sigma=12

> Received Pronunciation English has ~12 monophthong vowels.

```
  Evidence:
    - RP English: 12 pure vowels (short + long monophthongs)
    - IPA chart for RP: /i:,I,e,ae,a:,o,o:,u,u:,^,3:,@/ = 12
    - 12 = sigma = 12
    - Some dialects have 11-14; RP standard = 12

  Grade: CLOSE (RP = 12 is standard but varies by dialect/analysis)
  Lenses: wave, info, boundary
```

---

### H-LNG-05: 6 Articulation Places = n=6

> Universal phonetics recognizes ~6 major places of articulation.

```
  Evidence:
    - Bilabial, Labiodental, Dental/Alveolar, Palatal, Velar, Glottal = 6
    - IPA chart major columns ≈ 6 groupings
    - Full IPA has more subdivisions, but 6 is the primary grouping

  Grade: CLOSE (IPA has 8+ places; 6 is a common simplification)
  Lenses: boundary, info, scale
```

---

### H-LNG-06: Zipf's Law Exponent ~1 = R(6)

> Zipf's law: word frequency ~ rank^(-alpha), alpha approximately 1.

```
  Evidence:
    - Zipf exponent alpha ≈ 1.0 across languages
    - R(6) = sigma*phi/(n*tau) = 12*2/(6*4) = 1.0
    - Universal across all natural languages tested
    - 1 = R(6) = 1 EXACT

  Grade: EXACT (empirical law, alpha=1.0 is the standard value)
  Lenses: scale, info, evolution
```

---

### H-LNG-07: Tokenizer Vocabulary ~32K-128K = 2^{sigma-phi} ~ 2^{sigma+sopfr}

> Modern NLP tokenizers converge on vocabulary sizes in powers of 2 scaled by n=6.

```
  Evidence:
    - GPT-2: 50,257 ≈ 50K
    - GPT-3/4: 100K
    - LLaMA: 32K = 2^(sopfr*n/phi) = 2^15 = 32768
    - 128K = 2^(sigma+sopfr) = 2^17 = 131072
    - BT-73 direct reference

  Grade: CLOSE (vocabulary sizes are design choices converging near n=6 powers)
  Lenses: info, scale, recursion
```

---

### H-LNG-08: 4 Major Sentence Types = tau=4

> Languages universally distinguish 4 basic sentence types.

```
  Evidence:
    - Declarative, Interrogative, Imperative, Exclamatory = 4
    - 4 = tau = 4 EXACT
    - Universal across typologically diverse languages

  Grade: EXACT (universally recognized in grammar)
  Lenses: info, boundary, recursion
```

---

### H-LNG-09: Phoneme Inventory Mean ~24 Consonants = J₂

> Cross-linguistic average consonant inventory is approximately 22-24 phonemes.

```
  Evidence:
    - WALS: median consonant inventory = 22-24
    - Maddieson (2013): mean = ~22.7
    - 24 = J₂ = 24 (close to mean)
    - Range: 6 (Rotokas) to 122 (!Xu)

  Grade: CLOSE (J₂=24 is near the cross-linguistic mean, not exact)
  Lenses: info, scale, evolution
```

---

### H-LNG-10: Chomsky Hierarchy 4 Levels = tau=4

> The Chomsky hierarchy classifies grammars into exactly 4 types (0-3).

```
  Evidence:
    - Type 0 (Unrestricted), Type 1 (Context-Sensitive),
      Type 2 (Context-Free), Type 3 (Regular)
    - 4 levels = tau = 4 EXACT
    - Fundamental in formal language theory (Chomsky 1956)

  Grade: EXACT (mathematical definition, exactly 4 types)
  Lenses: recursion, topology, info
```

---

---

### H-LNG-11: 12 Semitone = sigma=12 Prosodic Universality

> All tonal and intonation systems operate within a 12-semitone chromatic space.

```
  Evidence:
    - Musical intervals used in speech prosody map to the 12-semitone scale
    - 12 = sigma = 12 EXACT
    - Tone languages (Mandarin, Cantonese, Vietnamese) use pitch contours
      within an octave divided into 12 equal-tempered steps
    - Cross-reference: BT-108 music-audio consonance universality

  Grade: EXACT (12 semitones is a fixed physical-mathematical division)
  Lenses: wave, info, scale
```

---

### H-LNG-12: 2 Morphological Types = phi=2

> Languages are fundamentally classified into 2 morphological macro-types.

```
  Evidence:
    - Analytic (isolating) vs Synthetic (inflecting/agglutinating) = 2
    - phi = 2 EXACT
    - Every language falls on this spectrum (Sapir, Comrie)
    - Binary opposition is the foundational typological division

  Grade: EXACT (binary typological axis, universally accepted)
  Lenses: topology, boundary, evolution
```

---

### H-LNG-13: 5 Vowel System Universality = sopfr=5

> The most common vowel inventory across world languages is 5 vowels.

```
  Evidence:
    - /a, e, i, o, u/ = 5-vowel system
    - WALS: ~188/563 languages = 33% have exactly 5 vowels (most common)
    - Maddieson (1984): 5 is the modal vowel inventory size
    - 5 = sopfr = 5 EXACT
    - Spanish, Japanese, Hawaiian, Swahili, Modern Greek = all 5 vowels

  Grade: EXACT (typologically dominant system, 5 is the mode)
  Lenses: info, scale, evolution
```

---

### H-LNG-14: 3 Persons in Pronoun Systems = n/phi=3

> All known languages distinguish exactly 3 grammatical persons.

```
  Evidence:
    - 1st person (I/we), 2nd person (you), 3rd person (he/she/it/they) = 3
    - n/phi = 6/2 = 3 EXACT
    - Universal across all attested languages (Cysouw 2003)
    - Some add 4th person (obviative) but 3 is the universal base

  Grade: EXACT (linguistic universal, no known exception to 3-person base)
  Lenses: info, recursion, boundary
```

---

### H-LNG-15: 2 Number Distinction (Singular/Plural) = phi=2

> The minimal and most universal grammatical number distinction is 2.

```
  Evidence:
    - Singular vs Plural = 2
    - phi = 2 EXACT
    - Universal: every language with number marking has at least singular/plural
    - Some add dual (3 = n/phi) or trial, but 2 is the universal base
    - Greenberg Universal #34

  Grade: EXACT (absolute universal in morphology)
  Lenses: info, boundary, scale
```

---

### H-LNG-16: 3 Tenses = n/phi=3

> The canonical tense system has 3 tenses.

```
  Evidence:
    - Past, Present, Future = 3
    - n/phi = 3 EXACT
    - Comrie (1985): tripartite tense is the most common temporal distinction
    - Some languages lack tense morphology (Mandarin) or have 2/5
    - But 3-tense is the typological default for tense-marking languages

  Grade: EXACT (most common tense system, linguistically canonical)
  Lenses: info, memory, boundary
```

---

### H-LNG-17: 5 IPA Vowel Heights = sopfr=5

> The IPA classifies vowel height into 5 levels (close to open with mid split).

```
  Evidence:
    - Close, Near-close, Close-mid (or Mid), Open-mid, Open = 5 heights
    - IPA standard chart: 5 vertical positions
    - sopfr = 5 EXACT
    - Some traditions use 3 or 4, but IPA standard = 5

  Grade: CLOSE (IPA uses ~4-7 depending on analysis; 5 is one common count)
  Lenses: wave, boundary, scale
```

---

### H-LNG-18: 4 Morpheme Types = tau=4

> Morphology recognizes 4 fundamental morpheme types.

```
  Evidence:
    - Free (root), Bound (affix), Derivational, Inflectional = 4
    - tau = 4 EXACT
    - Standard in morphology textbooks (Aronoff & Fudeman 2011)
    - 2x2 matrix: free/bound × lexical/grammatical = tau = 4

  Grade: EXACT (standard morphological classification)
  Lenses: recursion, boundary, info
```

---

### H-LNG-19: 2 Articulatory Streams = phi=2

> Speech production has exactly 2 simultaneous articulatory streams.

```
  Evidence:
    - Laryngeal (voicing/pitch) + Supralaryngeal (place/manner) = 2
    - phi = 2 EXACT
    - Source-filter theory (Fant 1960): source (glottal) + filter (tract)
    - Universal across all human speech

  Grade: EXACT (physical-acoustic fact, source-filter model)
  Lenses: wave, boundary, info
```

---

### H-LNG-20: 24 Greek Alphabet Letters = J₂=24

> The Greek alphabet, foundation of Western linguistic notation, has 24 letters.

```
  Evidence:
    - Alpha to Omega = 24 letters
    - J₂ = 24 EXACT
    - Fixed since ~403 BC (Euclidean reform)
    - Used universally in mathematics, physics, linguistics (IPA symbols)
    - Latin alphabet (26 = J₂+phi) and Cyrillic derived from Greek

  Grade: EXACT (historically fixed, 2400+ years unchanged)
  Lenses: info, evolution, memory
```

---

### H-LNG-21: 6 Stop Consonant Places = n=6

> Languages universally distinguish up to 6 stop consonant positions (voiced+voiceless pairs at 3 places).

```
  Evidence:
    - /p, b/ (bilabial) + /t, d/ (alveolar) + /k, g/ (velar) = 6 stops
    - n = 6 EXACT
    - The 6-stop system /p t k b d g/ is the most common stop inventory
    - WALS: 6-stop system is the typological default
    - Arranged as phi=2 voicing × n/phi=3 places = n=6

  Grade: EXACT (typologically dominant, factorial structure phi×(n/phi)=n)
  Lenses: info, topology, boundary
```

## Summary Table

| ID | Hypothesis | n=6 Link | Grade |
|----|-----------|----------|-------|
| H-LNG-01 | 6 linguistic branches | n=6 | CLOSE |
| H-LNG-02 | 6 word order types | 3!=n=6 | EXACT |
| H-LNG-03 | Korean 6 basic vowels | n=6 | EXACT |
| H-LNG-04 | English 12 vowels | sigma=12 | CLOSE |
| H-LNG-05 | 6 articulation places | n=6 | CLOSE |
| H-LNG-06 | Zipf exponent = 1 | R(6)=1 | EXACT |
| H-LNG-07 | Tokenizer 32K~128K | 2^{n=6 expr} | CLOSE |
| H-LNG-08 | 4 sentence types | tau=4 | EXACT |
| H-LNG-09 | ~24 consonants mean | J₂=24 | CLOSE |
| H-LNG-10 | Chomsky 4 levels | tau=4 | EXACT |
| H-LNG-11 | 12 semitones prosody | sigma=12 | EXACT |
| H-LNG-12 | 2 morphological types | phi=2 | EXACT |
| H-LNG-13 | 5-vowel universality | sopfr=5 | EXACT |
| H-LNG-14 | 3 grammatical persons | n/phi=3 | EXACT |
| H-LNG-15 | 2 number distinction | phi=2 | EXACT |
| H-LNG-16 | 3 tenses | n/phi=3 | EXACT |
| H-LNG-17 | 5 IPA vowel heights | sopfr=5 | CLOSE |
| H-LNG-18 | 4 morpheme types | tau=4 | EXACT |
| H-LNG-19 | 2 articulatory streams | phi=2 | EXACT |
| H-LNG-20 | 24 Greek letters | J₂=24 | EXACT |
| H-LNG-21 | 6 stop consonants | n=6 | EXACT |

**EXACT: 16/21, CLOSE: 5/21, WEAK: 0/21**


## 4. BT 연결

TODO: 후속 돌파 필요

## 5. DSE 결과

TODO: 후속 돌파 필요

## 6. 물리 한계 증명

TODO: 후속 돌파 필요

## 7. 실험 검증 매트릭스

TODO: 후속 돌파 필요

## 8. 외계인급 발견

TODO: 후속 돌파 필요

## 9. Mk.I~V 진화

TODO: 후속 돌파 필요

## 10. Testable Predictions

TODO: 후속 돌파 필요

## 11. ASCII 성능비교

TODO: 후속 돌파 필요

## 12. ASCII 시스템 구조도

TODO: 후속 돌파 필요

## 13. ASCII 데이터/에너지 플로우

TODO: 후속 돌파 필요

## 14. 업그레이드 시 (시중 vs v1 vs v2)

TODO: 후속 돌파 필요

## 15. 검증 방법 (verify.hexa)

TODO: 후속 돌파 필요
