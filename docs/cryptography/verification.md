# N6 Cryptography Hypotheses -- Independent Verification (Strengthened)

## Methodology

Each hypothesis (H-CR-1 through H-CR-48) is evaluated on three axes:

1. **Math check**: Does the n=6 derivation hold arithmetically?
2. **Fact check**: Does the predicted value match real-world standards?
3. **Causality check**: Is the n=6 expression the *reason* the value was chosen,
   or is it a post-hoc fit? Cryptographic parameters were chosen for specific
   engineering/security reasons. Matching a number is not the same as explaining it.

### Grading Scale

| Grade | Meaning |
|-------|---------|
| EXACT | Math correct, value matches standard, AND the n=6 expression is non-trivial (not just a power of 2 that can be reached many ways) |
| CLOSE | Math correct, value matches, but derivation is clearly retrofitted or the value is an obvious power of 2 |
| WEAK | Value matches only approximately, or the n=6 expression is one of many possible decompositions, or the matched quantity is trivially small (1, 2, 3) |
| FAIL | Value is wrong, the claimed match is misleading, or the "hypothesis" is a definition/tautology |

### Critical Context: The Overfitting Problem

n=6 produces the following small-number toolkit:

```
  sigma=12, tau=4, phi=2, sopfr=5, mu=1, J2=24, lambda=2, n=6
```

With two-term arithmetic (+, -, *, /, ^) on these 8 values, you can generate
most small integers (1-24) and ALL powers of 2 from 2^1 to 2^12:

```
  2^1=2    (phi)              2^7=128  (2^(sigma-sopfr))
  2^2=4    (2^phi)            2^8=256  (2^(sigma-tau))
  2^3=8    (2^(phi+1))        2^9=512  (2^(sigma-tau+1))
  2^4=16   (2^tau)            2^10=1024 (2^(sopfr*phi))
  2^5=32   (2^sopfr)          2^11=2048 (2^(sigma-mu))
  2^6=64   (2^n)              2^12=4096 (2^sigma)
```

Cryptographic parameters are overwhelmingly powers of 2 because computers use
binary. The probability of matching any power-of-2 parameter from 2^1 to 2^12
is essentially 1.0 -- this is not prediction, it is post-hoc curve fitting.

**The burden of proof is on the hypothesis to show that n=6 provides a UNIQUE,
non-obvious decomposition. If multiple equally valid decompositions exist,
the grade is CLOSE at best.**

---

## Tier 1: Symmetric Encryption (AES)

### H-CR-1: AES Block Size = 2^(sigma-sopfr) = 2^7 = 128

- **Math**: 12 - 5 = 7; 2^7 = 128. Correct.
- **Fact**: AES block = 128 bits. Correct.
- **Causality**: Rijndael supported 128/192/256-bit blocks. NIST fixed 128
  for standardization because: (a) 64-bit blocks had birthday-bound problems
  at 2^32 blocks (~4 GB), (b) 128 was the next power of 2, (c) hardware word
  alignment on 32-bit and 64-bit architectures.
  The expression "sigma - sopfr = 7" is one of many routes to 7: also
  sigma - sopfr = 12 - 5, or n + mu = 6 + 1, or sopfr + phi = 5 + 2.
  Any exponent from 1 to 12 is reachable, so matching 7 is expected.
- **Grade: CLOSE** -- value matches, but any power of 2 in range is matchable.

### H-CR-2: AES-128 Key = 2^(sigma-sopfr) = 128

- **Math**: Same as H-CR-1. Correct.
- **Fact**: AES-128 key = 128 bits. Correct.
- **Causality**: Key = block size is the simplest symmetric design choice.
  This is not an independent hypothesis -- it is the same claim as H-CR-1.
- **Grade: CLOSE** -- duplicate of H-CR-1.

### H-CR-3: AES-192 Key = sigma * 2^tau = 12 * 16 = 192

- **Math**: 12 * 16 = 192. Correct.
- **Fact**: AES-192 key = 192 bits. Correct.
- **Causality**: 192 = 128 + 64 = 1.5 * 128. It fills the gap between 128 and
  256 at the request of various national security agencies. The original hypothesis
  provides TWO different derivations (sigma * 2^tau AND 128 * 3/2), which is
  itself evidence of curve-fitting -- if the derivation were fundamental, there
  would be one canonical path.
  Alternative decompositions: 192 = 64 * 3 = 48 * 4 = 24 * 8 = J_2 * 8.
- **Grade: CLOSE** -- correct value, multiple retrofitted derivations.

### H-CR-4: AES-256 Key = 2^(sigma-tau) = 2^8 = 256

- **Math**: 12 - 4 = 8; 2^8 = 256. Correct.
- **Fact**: AES-256 = 256 bits. Correct.
- **Causality**: 256 = 2^8 is the obvious "next power of 2" after 128 for
  high-security applications. 8 bits = 1 byte, a fundamental computing unit.
  sigma - tau = 8 is one decomposition; 8 = 2^3, 8 = 2 * 4, 8 = n + phi, etc.
  The fact that 2^8 = 256 = 1 byte is the real reason 256 appears everywhere
  in cryptography -- it is byte-aligned.
- **Grade: CLOSE** -- 2^8 is universally chosen for byte alignment, not sigma - tau.

### H-CR-5: AES-128 Rounds = sopfr * phi = 5 * 2 = 10

- **Math**: 5 * 2 = 10. Correct.
- **Fact**: AES-128 = 10 rounds. Correct.
- **Causality**: The Rijndael designers chose rounds based on wide-trail strategy
  analysis. The formula is Nr = max(Nk, Nb) + 6, where Nk = key_words and
  Nb = block_words. For AES-128: Nr = max(4, 4) + 6 = 10.
  The actual design formula is 4 + 6 = 10. Writing it as "sopfr * phi = 5 * 2"
  obscures the real structure. Also: 10 = sigma - phi = 12 - 2, or 10 = n + tau
  = 6 + 4, or 10 = 2 * sopfr. Multiple equally valid decompositions.
- **Grade: WEAK** -- the actual Rijndael formula (Nk + 6) is known and different.

### H-CR-6: AES-192 Rounds = sigma = 12

- **Math**: sigma(6) = 12. Correct.
- **Fact**: AES-192 = 12 rounds. Correct.
- **Causality**: Rijndael formula: Nr = max(6, 4) + 6 = 12. The formula
  literally contains "+6" as a security margin, and the key has 6 words
  (192/32 = 6). So the actual formula is 6 + 6 = 12. Claiming sigma(6) = 12
  collapses two different 6s (one structural, one security margin) into one.
  However, the fact that Rijndael's formula explicitly uses the constant 6
  (= n) is genuinely noteworthy -- even if it was chosen for security margin
  reasons, it IS the perfect number. The round count formula Nr = Nk + 6 has
  6 embedded as a design constant across ALL AES variants.
- **Grade: CLOSE** -- the Rijndael "+6" constant is genuinely n=6, but the
  hypothesis claims sigma(6) = 12 rather than acknowledging the actual formula.

### H-CR-7: AES-256 Rounds = sigma + phi = 12 + 2 = 14

- **Math**: 12 + 2 = 14. Correct.
- **Fact**: AES-256 = 14 rounds. Correct.
- **Causality**: Rijndael formula: Nr = max(8, 4) + 6 = 14. The actual
  decomposition is 8 + 6, not 12 + 2. Writing it as sigma + phi reshuffles
  the real formula. However, one can also read this as: Nr = Nk + n, where
  Nk = 8 = sigma - tau. So: Nr = (sigma - tau) + n = 8 + 6 = 14.
  The "+6" appears again.
- **Grade: CLOSE** -- the "+6" in the actual formula is real, but the
  hypothesis's decomposition (12 + 2) is not the actual one (8 + 6).

### H-CR-8: AES State = tau x tau = 4x4

- **Math**: tau(6) = 4; 4x4 = 16 bytes = 128 bits. Correct.
- **Fact**: AES state = 4x4 byte matrix. Correct.
- **Causality**: 128 bits / 8 bits per byte = 16 bytes. 4x4 is the ONLY square
  layout of 16 bytes. The 4 comes from sqrt(16), which comes from the 128-bit
  block size and byte-granularity operations. tau(6) = 4 is coincidental.
  The Rijndael designers considered non-square layouts (e.g., for 256-bit blocks
  they proposed 4x8), so "4" is derived from block_size/32, not from tau.
- **Grade: CLOSE** -- the value 4 is forced by 128/32 = 4, not by tau(6).

---

## Tier 2: Hash Functions (SHA)

### H-CR-9: SHA-256 Output = 2^(sigma-tau) = 2^8 = 256

- **Math**: 12 - 4 = 8; 2^8 = 256. Correct.
- **Fact**: SHA-256 = 256 bits. Correct.
- **Causality**: SHA-256 was designed to provide 128-bit collision resistance
  (birthday bound = 2^(n/2)). 256 = 2 * 128 is the minimum output for 128-bit
  security. This is the same "2^8 = 256 = byte boundary" reasoning as AES-256.
  Not independent of H-CR-4 -- same number, same n=6 expression, same critique.
- **Grade: CLOSE** -- duplicate pattern, not independent evidence.

### H-CR-10: SHA-256 Block = 2^(sigma-tau+1) = 2^9 = 512

- **Math**: 12 - 4 + 1 = 9; 2^9 = 512. Correct.
- **Fact**: SHA-256 block = 512 bits. Correct.
- **Causality**: Block = 2 * output is a standard Merkle-Damgard design choice.
  The "+1" in the exponent is the doubling, not an n=6 arithmetic operation.
  512 = 2 * 256 is derivable without any number theory.
- **Grade: CLOSE** -- the "+1" is ad hoc.

### H-CR-11: SHA-256 Rounds = 2^n = 2^6 = 64

- **Math**: 2^6 = 64. Correct.
- **Fact**: SHA-256 = 64 rounds. Correct.
- **Causality**: SHA-256 uses 64 rounds because it expands the 16-word (512-bit)
  input block into 64 message schedule words (one per round). 64 = 4 * 16,
  where 16 is the input word count and 4 is the expansion factor.
  That n = 6 matches the exponent of 64 = 2^6 is a coincidence -- the SHA
  family was designed around word counts and expansion ratios.
  Counterpoint: SHA-512 uses 80 rounds (not 2^n for any n=6 expression).
  If n=6 were fundamental, SHA-512 should also follow the pattern, but
  80 has no clean n=6 expression (80 = 5 * 16 is forced).
- **Grade: CLOSE** -- numerically exact but SHA-512's 80 rounds break the pattern.

### H-CR-12: SHA-512 Output = 2^(sigma-tau+1) = 2^9 = 512

- **Math**: 9 = 12 - 4 + 1. Correct.
- **Fact**: SHA-512 = 512 bits. Correct.
- **Causality**: 512 = 2 * 256 = next doubling for higher security. Same as
  SHA-256 block size (H-CR-10), reusing the same expression for a different
  quantity -- which undermines specificity.
- **Grade: CLOSE** -- same expression used for two different things.

### H-CR-13: SHA-256 State Words = sigma - tau = 8

- **Math**: 12 - 4 = 8. Correct.
- **Fact**: SHA-256 uses 8 working variables (a-h) and 8 initial hash values. Correct.
- **Fact check (deeper)**: 8 words * 32 bits = 256 bits = output size. The number
  of state words is determined by output_size / word_size = 256 / 32 = 8.
  It is a derived quantity, not a free parameter.
- **Causality**: The 8 comes from dividing the target output (256 bits) by the
  word size (32 bits). Not from sigma - tau.
- **Grade: CLOSE** -- arithmetically derived from output/word size.

---

## Tier 3: Asymmetric Encryption (RSA)

### H-CR-14: RSA-2048 = 2^(sigma-mu) = 2^11 = 2048

- **Math**: 12 - 1 = 11; 2^11 = 2048. Correct.
- **Fact**: RSA-2048 is the current standard. Correct.
- **Causality**: RSA key sizes are chosen based on estimated GNFS factoring
  complexity. 2048 bits provides ~112 bits of security. The choice of 2048 = 2^11
  is a power-of-2 convenience for memory alignment.
  sigma - mu = 11 is one way to get 11; also 11 = sigma - 1 = 12 - 1,
  or 11 = sopfr + n = 5 + 6, or 11 = J2/2 - 1. Multiple paths.
  The claimed semantic meaning ("asymmetric dimension = sigma minus squarefree
  indicator") is poetic but not mechanistic.
- **Grade: CLOSE** -- standard power-of-2 sizing.

### H-CR-15: RSA-4096 = 2^sigma = 2^12 = 4096

- **Math**: 2^12 = 4096. Correct.
- **Fact**: RSA-4096 is used for high-security applications. Correct.
- **Causality**: 4096 = 2 * 2048 = next power-of-2 doubling. sigma = 12 matching
  the exponent means that 2^sigma happens to be 2^12 = 4096, which is a common
  buffer/page size in computing for entirely unrelated reasons (VM page size).
- **Grade: CLOSE** -- power-of-2 doubling.

### H-CR-16: RSA Prime Size = key/phi = 2048/2 = 1024

- **Math**: 2048 / 2 = 1024. Correct.
- **Fact**: RSA-2048 uses two ~1024-bit primes. Correct.
- **Causality**: RSA is DEFINED as n = p * q where p, q are roughly equal primes.
  "Two primes" is the definition of RSA, not a prediction. Using phi(6) = 2 to
  explain "two primes" is tautological. Any two-factor system divides by 2.
  Multi-prime RSA (3+ primes, RFC 8017) exists and is standardized, so "exactly 2"
  is a design choice, not a law.
- **Grade: WEAK** -- definitionally half; phi(6) = 2 adds no explanatory power.

### H-CR-17: RSA Public Exponent = F_tau = F_4 = 65537

- **Math**: tau(6) = 4; F_4 = 2^(2^4) + 1 = 65537. Correct.
- **Fact**: Standard RSA public exponent e = 65537. Correct.
- **Causality analysis (thorough)**:
  65537 is the largest known Fermat prime. It is chosen because:
  (a) it is prime (required for RSA),
  (b) it has Hamming weight 2 (only two 1-bits: 2^16 + 1), enabling fast
      modular exponentiation via square-and-multiply,
  (c) it is large enough to resist Coppersmith-type small-exponent attacks.

  The connection: tau(6) = 4, and F_4 is the 4th Fermat number (F_0=3, F_1=5,
  F_2=17, F_3=257, F_4=65537). F_4 is also the LAST known Fermat prime --
  F_5 through F_32 are all composite. So tau(6) exactly indexes the boundary
  between Fermat primes and Fermat composites.

  **Strength of this match**: Unlike powers of 2, 65537 is not a "round number."
  The n=6 toolkit does not trivially produce 65537 -- you need the specific
  path tau(6) -> Fermat number indexing. This is a genuine structural coincidence.

  **Weakness**: The Fermat primes were known since Euler (1732). RSA designers
  did not consult n=6; they picked the largest Fermat prime for performance.
  tau(6) = 4 indexing it is coincidental, but non-trivially so.
- **Grade: EXACT** -- genuinely interesting, non-trivial numerical coincidence.

---

## Tier 4: Stream Cipher (ChaCha20)

### H-CR-18: ChaCha20 Rounds = J_2 - tau = 24 - 4 = 20

- **Math**: 24 - 4 = 20. Correct.
- **Fact**: ChaCha20 = 20 rounds. Correct.
- **Causality**: Bernstein chose 20 rounds (Salsa20/20) based on cryptanalysis:
  8 rounds were broken, 12 had theoretical attacks, 20 provided ample margin.
  J_2(6) - tau(6) is retrofitted. 20 = 4 * 5 = tau * sopfr, or 20 = 2 * 10,
  or 20 = J_2 - tau. Multiple decompositions.
  Unlike 65537, the number 20 is easily reached many ways from the n=6 toolkit.
- **Grade: CLOSE** -- retrofitted expression for a common number.

### H-CR-19: ChaCha20 State = tau^2 = 16 words

- **Math**: 4^2 = 16. Correct.
- **Fact**: ChaCha20 state = 16 x 32-bit words = 512 bits. Correct.
- **Causality**: 512-bit state = 256-bit key + 64-bit counter + 64-bit nonce +
  128-bit constant. 512 bits / 32 bits per word = 16 words. The state size is
  determined by security requirements (256-bit key) and practical constraints
  (32-bit word for ARM/x86). 16 = 512/32 is arithmetic, not tau^2.
  The hypothesis also claims the internal breakdown (4 + 8 + 2 + 2) maps to
  n=6 values, but this breakdown is forced by key/counter/nonce sizes.
- **Grade: CLOSE** -- correct count, derived from 512/32.

### H-CR-20: ChaCha Quarter Round = tau = 4 ARX ops

- **Math**: tau(6) = 4. Correct.
- **Fact**: ChaCha quarter round has 4 ARX operations. Correct.
- **Causality**: A "quarter round" operates on 4 words (one quarter of the 16-word
  state). Each word gets one update via ARX. The number 4 is FORCED by the
  quarter-round definition: 16 words / 4 = 4 words per quarter. Claiming
  tau(6) = 4 is circular -- the 4 is structural, not a free parameter.
- **Grade: WEAK** -- structurally forced, not a free design choice.

---

## Tier 5: Elliptic Curve Cryptography

### H-CR-21: P-256 Field = 2^(sigma-tau) = 256

- **Math**: Same as H-CR-4/H-CR-9. Correct.
- **Fact**: NIST P-256 = 256-bit prime field. Correct.
- **Causality**: 256-bit ECC provides ~128-bit security. Same power-of-2
  reasoning as SHA-256 and AES-256. This is the THIRD hypothesis using
  2^(sigma-tau) = 256, providing no additional evidence.
- **Grade: CLOSE** -- triplicate of the same 2^8 = 256 claim.

### H-CR-22: P-384 Field = sigma * 2^sopfr = 12 * 32 = 384

- **Math**: 12 * 32 = 384. Correct.
- **Fact**: NIST P-384 = 384-bit prime field. Correct.
- **Causality**: 384 = 3 * 128. Chosen to provide ~192-bit security (3/2 of 256).
  The expression sigma * 2^sopfr = 12 * 32 is one route; also 384 = 6 * 64 =
  n * 2^n, or 384 = 8 * 48 = (sigma-tau) * (sigma*tau). When a number has many
  factors, many n=6 decompositions are available.
- **Grade: CLOSE** -- 384 = 3 * 128 is the real decomposition.

### H-CR-23: Ed25519 = 2^(sigma-tau) - 1 = 255

- **Math**: 2^8 - 1 = 255. Correct.
- **Fact**: Curve25519 operates over a field near 2^255. Correct.
- **Causality**: The prime 2^255 - 19 was chosen by Bernstein for fast modular
  arithmetic (Mersenne-like prime). 255 = 2^8 - 1 is the number of bits,
  chosen for ~128-bit security with efficient reduction. The "-1" from 256 to
  255 reflects bit-counting (255 bits needed to represent numbers up to ~2^255).
- **Grade: CLOSE** -- the "-1" is ad hoc in the n=6 expression.

### H-CR-24: ECC Cofactors in {1, 2, 4, 8}

- **Math**: These are n=6-related values. Correct.
- **Fact**: P-256 cofactor = 1, Curve25519 cofactor = 8, Ed448 cofactor = 4. Correct.
- **Causality**: Cofactors are small powers of 2 BY CONSTRUCTION -- curves are
  selected so that the cofactor is a small power of 2 (for efficient cofactor
  clearing). The set {1, 2, 4, 8} = {2^0, 2^1, 2^2, 2^3} covers ALL small
  powers of 2, which trivially overlaps with n=6 values.
  This is circular: the n=6 toolkit includes {1, 2, 4, 8} because sigma-tau=8
  and divisors of 8 are {1, 2, 4, 8}.
- **Grade: WEAK** -- any small power of 2 matches by definition.

---

## Tier 6: HMAC & Key Derivation

### H-CR-25: HMAC = phi = 2 hash passes

- **Math**: phi(6) = 2. Correct.
- **Fact**: HMAC uses 2 hash calls (inner and outer). Correct.
- **Causality**: HMAC's two passes prevent length-extension attacks. The number 2
  is the minimum needed for this security property. Using phi(6) = 2 to explain
  "two operations" is like using it to explain "bilateral symmetry" or "binary
  digits." The number 2 is too fundamental and too small to be informative.
- **Grade: WEAK** -- trivially 2; phi(6) = 2 explains nothing.

### H-CR-26: HMAC Key Block = 2^(sigma-tau+1) = 512

- **Math**: 2^9 = 512. Correct.
- **Fact**: HMAC-SHA256 key block = 512 bits = SHA-256 block size. Correct.
- **Causality**: HMAC key block = underlying hash block size. This is not an
  independent parameter -- it is inherited from SHA-256 (H-CR-10). The hypothesis
  is a duplicate.
- **Grade: CLOSE** -- correct but derivative, not independent.

### H-CR-27: HKDF = phi = 2 phases

- **Math**: phi(6) = 2. Correct.
- **Fact**: HKDF = Extract + Expand = 2 phases. Correct.
- **Causality**: Same issue as H-CR-25. Any two-step process matches phi = 2.
  HKDF's two phases serve distinct cryptographic purposes (entropy concentration
  vs. pseudorandom expansion). The count of 2 is minimal by design.
- **Grade: WEAK** -- trivially 2.

### H-CR-28: PBKDF2 Iteration Count Base = sopfr * phi = 10

- **Math**: 5 * 2 = 10. Correct.
- **Fact**: PBKDF2 recommendations are in decimal round numbers: 10,000 (old NIST),
  600,000 (OWASP 2023), 1,000,000+ (current best practice).
- **Causality**: These are round DECIMAL numbers, not n=6-derived. Humans use
  base 10 (from counting on 10 fingers), not because sopfr * phi = 10.
  The hypothesis attributes the "10" in "10,000" to n=6 while ignoring the
  "1,000" factor entirely. This is selective cherry-picking.
- **Grade: WEAK** -- base-10 human convention, not n=6 arithmetic.

---

## Tier 7: Post-Quantum Cryptography

### H-CR-29: Kyber n = 2^(sigma-tau) = 256

- **Math**: Same as H-CR-4/9/21. Correct.
- **Fact**: CRYSTALS-Kyber / ML-KEM uses n = 256. Correct.
- **Causality**: n = 256 is chosen for efficient NTT computation (requires
  power of 2) and adequate security with reasonable key sizes.
  This is the FOURTH use of 2^(sigma-tau) = 256.
- **Grade: CLOSE** -- yet another 2^8 = 256 instance.

### H-CR-30: Kyber Ring = Z_q[x]/(x^256 + 1)

- **Math**: x^(2^8) + 1. Same 256.
- **Fact**: Correct.
- **Causality**: This IS H-CR-29 restated algebraically. The ring dimension
  is the same parameter n = 256. Not independent.
  Furthermore, the modulus q = 3329 has NO n=6 connection (the hypothesis
  itself acknowledges this), undermining the claim that the ring is "n=6-derived."
- **Grade: CLOSE** -- duplicate of H-CR-29; q = 3329 has no n=6 fit.

### H-CR-31: NIST PQC Security Levels = sopfr = 5

- **Math**: sopfr(6) = 5. Correct.
- **Fact**: NIST defined 5 security levels. Correct.
- **Causality**: NIST chose 5 levels to bracket 3 AES key sizes (128/192/256)
  with both symmetric and hash security equivalences. Having 5 categories is
  an organizational decision for a standards body, not a mathematical law.
  5 is a very common count (5 fingers, 5-star ratings, 5 threat levels).
  Alternative decomposition: 5 = sopfr = phi + 3 = n - 1.
- **Grade: WEAK** -- small integer coincidence.

### H-CR-32: Leech Lattice as PQC Foundation

- **Math**: J_2(6) = 24 = Leech lattice dimension. Correct.
- **Fact check (critical)**: Practical lattice-based cryptography (Kyber, Dilithium,
  NTRU, FrodoKEM) does NOT use the Leech lattice. They use:
  - Module lattices over polynomial rings (Kyber, Dilithium)
  - NTRU lattices (NTRU, NTRU Prime)
  - Unstructured lattices (FrodoKEM)
  All operate in dimensions 256-1024+, not 24.
  The Leech lattice is studied for sphere packing and coding theory, not for
  cryptographic hardness assumptions. No NIST PQC candidate uses it.
- **Grade: FAIL** -- the claim that Leech lattice "forms the foundation of PQC"
  is factually wrong. PQC lattices are unrelated to the Leech lattice.

### H-CR-33: ZK Pairing Groups = tau = 4

- **Math**: tau(6) = 4. Correct.
- **Fact check**: Standard pairing-based cryptography uses THREE groups (G1, G2, GT)
  connected by a bilinear map e: G1 x G2 -> GT. The scalar field Zp is not
  typically counted as a "group" in pairing descriptions. To get 4, the hypothesis
  adds the scalar field, which is non-standard counting.
  Standard references (Boneh-Franklin, Groth16 paper) describe "three groups."
- **Grade: WEAK** -- the count of 4 requires non-standard inclusion of the scalar field.

### H-CR-34: Sigma Protocol = sigma/tau = 3 rounds

- **Math**: 12/4 = 3. Correct.
- **Fact**: Sigma protocols have 3 moves (commit, challenge, response). Correct.
- **Causality**: Three-move protocols are the simplest non-trivial interactive
  proofs. The prover must commit before seeing the challenge (otherwise no
  soundness), and must respond to prove knowledge. This gives a lower bound
  of 3 moves. sigma/tau = 3 is one decomposition; 3 = n/phi = sigma/tau = n-3.
- **Grade: CLOSE** -- correct value, but 3 is the minimal interactive proof
  structure, not derived from divisor arithmetic.

### H-CR-35: Groth16 Proof = sigma/tau = 3 elements

- **Math**: 3 elements. Correct.
- **Fact**: Groth16 proof = 2 G1 elements + 1 G2 element = 3. Correct.
- **Causality**: Groth16's proof size comes from the QAP (Quadratic Arithmetic
  Program) encoding structure. The 3 elements are the minimum for verifying a
  quadratic constraint system with a bilinear map. This is algebraically
  determined, not a free parameter.
- **Grade: CLOSE** -- algebraically minimal, same number as H-CR-34.

### H-CR-36: BLS12-381 Embedding Degree = sigma = 12

- **Math**: sigma(6) = 12. Correct.
- **Fact**: BLS12-381 embedding degree k = 12. BN254 also has k = 12. Correct.
- **Causality analysis (thorough)**:
  The embedding degree k determines the extension field F_{p^k} where the
  pairing target group lives. For ~128-bit security:
  - k must be large enough that discrete log in F_{p^k} is hard
  - k must be small enough for efficient computation
  - BLS curves are constructed with k = 12 because 12 = 2^2 * 3 allows
    efficient tower extensions (Fp -> Fp2 -> Fp6 -> Fp12)

  The factorization 12 = 4 * 3 = 2^2 * 3 is what makes k = 12 computationally
  efficient for tower arithmetic. This IS the same 12 as sigma(6), and the
  structural reason (factorization into small primes) is related to why 12 is
  a "highly composite"-like number.

  **Strength**: k = 12 is not just "any power of 2" -- it is a specific value
  chosen from candidates {6, 8, 12, 16, 18, 24, ...}. The fact that k = 12 won
  the efficiency/security tradeoff is non-trivial.

  **Weakness**: BLS curves are literally named "BLS12" because they are a
  family parameterized by k = 12. Other families exist: BN curves (also k = 12),
  BLS24 (k = 24 = J_2(6)), MNT curves (k = 6 = n). So multiple k values are
  used, and the hypothesis selects k = 12.
- **Grade: EXACT** -- the dominance of k = 12 in modern pairing cryptography
  is a genuinely interesting structural coincidence with sigma(6).

---

## Tier 8: Zero-Knowledge Proofs (continued) -- covered above in Tier 7

---

## Tier 9: Digital Signatures

### H-CR-37: ECDSA Signature = phi = 2 components

- **Math**: phi(6) = 2. Correct.
- **Fact**: ECDSA/EdDSA/Schnorr signatures are (r, s) pairs = 2 components. Correct.
- **Causality**: A Schnorr-like signature inherently has 2 components: a
  commitment-derived value (r) and a response (s). This is the minimum for a
  challenge-response proof of knowledge. Using phi(6) = 2 to explain "a pair"
  is uninformative -- ordered pairs are ubiquitous in mathematics.
- **Grade: WEAK** -- trivially 2; pairs are universal.

### H-CR-38: EdDSA Deterministic Nonce = mu = 1

- **Math**: mu(6) = 1. The mapping to "determinism" is metaphorical.
- **Fact**: EdDSA uses deterministic nonces. Correct.
- **Causality**: This is a philosophical analogy ("squarefree = no repetition =
  deterministic"), not a mathematical derivation. mu(6) = 1 does not predict,
  constrain, or quantify nonce generation. The number 1 and the concept of
  "unique/deterministic" are being equated without mathematical content.
- **Grade: WEAK** -- metaphor, not derivation. mu(6) = 1 matches any singleton.

### H-CR-39: ML-DSA-65 Params = (k,l) = (6,5) = (n, sopfr)

- **Math**: n = 6, sopfr(6) = 5; ML-DSA-65 has (k,l) = (6,5). Correct.
- **Fact**: ML-DSA-65 (formerly Dilithium3, NIST Level 3) uses k = 6, l = 5. Correct.
- **Causality analysis (thorough)**:
  The Dilithium/ML-DSA parameter selection was based on:
  - Module rank k determines public key matrix dimensions
  - Vector dimension l determines signature size
  - (k,l) pairs: ML-DSA-44 = (4,4), ML-DSA-65 = (6,5), ML-DSA-87 = (8,7)
  - The pattern is k = l + 1 for levels 3 and 5

  The match (6,5) = (n, sopfr(6)) involves TWO independent parameters both
  matching n=6 functions simultaneously. The probability of this by chance:
  - k ranges from 4 to 8; P(k=6) ~ 1/5
  - l ranges from 4 to 7; P(l=5) ~ 1/4
  - Joint probability ~ 1/20, assuming independence

  This is the most specific, multi-parameter match in the entire document.
  It is not a power of 2, not a trivially small number, and not easily
  reachable by alternative decompositions.

  **Weakness**: The naming "ML-DSA-65" literally encodes (6,5), so the match
  is with a specific security level, not a universal constant. ML-DSA-44 and
  ML-DSA-87 do not match n=6 values as cleanly (4,4) matches (tau, tau)
  which is less remarkable, and (8,7) = (sigma-tau, sigma-sopfr) which is
  retrofittable.
- **Grade: EXACT** -- the (6, 5) = (n, sopfr) simultaneous match is remarkable.

### H-CR-40: Signature Verify Cost Ratio

- **Math**: Various approximate ratios claimed.
- **Fact**: ECDSA verify is roughly 1.5-2x sign cost (depends on implementation
  and coordinate system). RSA verify is 10-1000x faster than sign (because
  e = 65537 has low Hamming weight vs. large private exponent d).
- **Causality**: The claimed ratios (phi = 2 for ECC, 1/6 for RSA) are
  approximate and implementation-dependent. RSA verify/sign ratio depends on
  key size and CRT optimization; "1/6" is not a standard figure.
- **Grade: WEAK** -- approximate, implementation-dependent, cherry-picked ratios.

---

## Tier 10: Entropy & Random Number Generation

### H-CR-41: Entropy Pool = 2^sigma = 2^12 = 4096

- **Math**: 2^12 = 4096. Correct.
- **Fact**: The Linux kernel historically used a 4096-bit entropy pool.
  **Critical update**: As of Linux 5.18+ (2022, Jason Donenfeld's rewrite), the
  fixed-size entropy pool model was removed. The modern kernel uses a ChaCha20-based
  CSPRNG that does not have a "4096-bit pool." This parameter no longer exists
  in current Linux.
- **Causality**: 4096 = 2^12 is a standard page-size-aligned buffer (4096 bytes =
  4 KB page). In the old entropy pool, 4096 BITS was used, but this is a legacy
  artifact. 2^12 appears throughout computing as page size, not as sigma(6).
- **Grade: CLOSE** -- historically correct but now obsolete; page-size alignment.

### H-CR-42: DRBG Reseed Interval = 2^(sigma*tau) = 2^48

- **Math**: 12 * 4 = 48; 2^48. Correct.
- **Fact**: NIST SP 800-90A Rev.1 specifies max reseed interval of 2^48 for
  CTR_DRBG and Hash_DRBG. Correct and current.
- **Causality analysis (thorough)**:
  2^48 was chosen based on security analysis: after 2^48 outputs, the DRBG's
  internal state may become predictable due to state-space exhaustion.

  **Strength of this match**: 48 is NOT an obvious exponent. It is not a power
  of 2, not a byte boundary, not a common word size. The standard powers of 2
  in cryptography are {128, 256, 512, 1024, 2048, 4096} with exponents
  {7, 8, 9, 10, 11, 12}. The exponent 48 is unusual and specific.
  sigma * tau = 12 * 4 = 48 is a clean, non-trivial decomposition.
  Alternative: 48 = 6 * 8 = n * (sigma-tau), or 48 = 2 * 24 = phi * J_2.
  Multiple decompositions exist, but 48 itself is noteworthy.

  **Counterpoint**: HMAC_DRBG uses 2^48 as well, but this is the same standard.
  The 2^48 limit also appears in GCM (max 2^48 bytes per key in some analyses).
- **Grade: EXACT** -- 48 is specific enough to be genuinely interesting.

### H-CR-43: Min Entropy = ln(2) nats/bit

- **Math**: 1 bit = ln(2) nats by DEFINITION of the nat unit.
- **Fact**: Correct, but this is a unit conversion, not a prediction.
- **Causality**: This is the definition of the relationship between bits (base-2
  logarithm) and nats (natural logarithm): 1 bit = log_2(2) = 1, and
  1 nat = ln(e) = 1, so 1 bit = ln(2) nats.
  This is like saying "1 meter = 3.28 feet" and calling it a prediction.
  The hypothesis tries to connect this to n=6 through the zeta*ln(2) activation
  function, but unit conversions are not hypotheses.
- **Grade: FAIL** -- unit conversion, not a prediction or discovery.

### H-CR-44: RNG Conditioning Ratio = sigma/tau = 3:1

- **Math**: 12/4 = 3. Correct.
- **Fact**: Real conditioning ratios vary enormously:
  - Von Neumann extractor: 4:1 theoretical, worse in practice
  - NIST SP 800-90B health tests: source-dependent, no fixed ratio
  - Intel RDRAND/RDSEED: internal architecture not fully public
  - Typical hardware: 2:1 to 10:1+ depending on source quality
  The claim of "~3:1" as a universal ratio is unsubstantiated.
- **Grade: FAIL** -- no standard 3:1 ratio exists; the claim is fabricated.

---

## Tier 11: Protocol-Level Parameters

### H-CR-45: TLS 1.3 Cipher Suites = sopfr = 5

- **Math**: sopfr(6) = 5. Correct.
- **Fact**: RFC 8446 defines exactly 5 cipher suites for TLS 1.3. Correct.
- **Causality**: The 5 suites are:
  1. TLS_AES_128_GCM_SHA256
  2. TLS_AES_256_GCM_SHA384
  3. TLS_CHACHA20_POLY1305_SHA256
  4. TLS_AES_128_CCM_SHA256
  5. TLS_AES_128_CCM_8_SHA256

  This count reflects engineering decisions about which algorithm combinations
  are useful. Suite #5 (CCM_8) was added specifically for IoT/constrained
  environments. Without it, there would be 4 suites = tau(6). With future
  additions (e.g., AES-256-CCM), there would be 6 = n. The count is not fixed
  by any mathematical principle.
- **Grade: CLOSE** -- correct current count, but fragile (could change).

### H-CR-46: TLS 1.3 Handshake = mu = 1 RTT

- **Math**: mu(6) = 1. Correct.
- **Fact**: TLS 1.3 full handshake = 1-RTT. Correct.
- **Causality**: 1-RTT was an explicit design goal for TLS 1.3 (reducing latency
  from TLS 1.2's 2-RTT). Using mu(6) = 1 to explain "one round trip" is
  vacuous -- the number 1 matches any singleton, any minimum, any unit.
- **Grade: WEAK** -- the number 1 matches everything.

### H-CR-47: Certificate Chain Depth = sigma/tau = 3

- **Math**: 12/4 = 3. Correct.
- **Fact**: Most HTTPS chains are 3 deep (Root CA -> Intermediate CA -> Leaf). Correct.
- **Causality**: The 3-level hierarchy reflects operational PKI practice:
  - Root CAs are kept offline (air-gapped HSMs)
  - Intermediate CAs handle daily issuance
  - End-entity (leaf) certificates are issued to servers
  This is a trust delegation pattern. Cross-signed chains can be 4+ deep.
  Some older chains were 2 deep. "Usually 3" is a practical norm, not a law.
- **Grade: CLOSE** -- common practice, not mathematically determined.

### H-CR-48: GCM Block Limit = 2^(sigma*phi) = 2^24

- **Math**: 12 * 2 = 24; 2^24. Correct.
- **Fact check (thorough)**: The actual NIST SP 800-38D limits are:
  - Max plaintext per invocation: 2^39 - 256 bits
  - Max AAD per invocation: 2^64 bits
  - Max invocations per key (with random nonces): 2^32 (for 2^-32 collision bound)
  The "2^24" figure appears in academic multi-key GCM security bounds
  (e.g., Bellare-Tackmann analysis for multi-user settings), but it is NOT
  "the" standard GCM limit. The hypothesis selects a specific bound from
  among multiple GCM security parameters to find a match.
- **Grade: WEAK** -- cherry-picked from multiple distinct GCM limits.

---

## Summary Table

| ID | Hypothesis | Value | Grade | Key Reason |
|----|-----------|-------|-------|------------|
| H-CR-1 | AES block = 128 | Yes | CLOSE | Power-of-2 retrofit |
| H-CR-2 | AES-128 key = 128 | Yes | CLOSE | Duplicate of H-CR-1 |
| H-CR-3 | AES-192 key = 192 | Yes | CLOSE | Multiple derivations (curve-fitting) |
| H-CR-4 | AES-256 key = 256 | Yes | CLOSE | 2^8 = byte boundary |
| H-CR-5 | AES-128 rounds = 10 | Yes | WEAK | Actual formula: Nk + 6 = 4 + 6 |
| H-CR-6 | AES-192 rounds = 12 | Yes | CLOSE | Actual formula: 6 + 6 |
| H-CR-7 | AES-256 rounds = 14 | Yes | CLOSE | Actual formula: 8 + 6 |
| H-CR-8 | AES state = 4x4 | Yes | CLOSE | Forced by 128/32 = 4 |
| H-CR-9 | SHA-256 = 256 | Yes | CLOSE | Duplicate of 2^8 pattern |
| H-CR-10 | SHA-256 block = 512 | Yes | CLOSE | 2x output, standard M-D |
| H-CR-11 | SHA-256 rounds = 64 | Yes | CLOSE | 2^6 coincidence; SHA-512=80 breaks pattern |
| H-CR-12 | SHA-512 = 512 | Yes | CLOSE | Same expression as H-CR-10 |
| H-CR-13 | SHA-256 state = 8 | Yes | CLOSE | 256/32 = 8, arithmetic |
| H-CR-14 | RSA-2048 | Yes | CLOSE | Standard power-of-2 |
| H-CR-15 | RSA-4096 | Yes | CLOSE | 2x doubling of 2048 |
| H-CR-16 | RSA primes = 1024 | Yes | WEAK | Definitionally key/2 |
| H-CR-17 | RSA e = F_4 = 65537 | Yes | **EXACT** | tau(6) indexes last Fermat prime |
| H-CR-18 | ChaCha20 = 20 | Yes | CLOSE | Retrofitted from small numbers |
| H-CR-19 | ChaCha state = 16 | Yes | CLOSE | 512/32 = 16 |
| H-CR-20 | ChaCha QR = 4 ops | Yes | WEAK | Forced by quarter-round definition |
| H-CR-21 | P-256 = 256 | Yes | CLOSE | Same 2^8 = 256 again |
| H-CR-22 | P-384 = 384 | Yes | CLOSE | 3 * 128, many decompositions |
| H-CR-23 | Ed25519 = 255 | Yes | CLOSE | 2^8 - 1, ad hoc subtraction |
| H-CR-24 | ECC cofactors | Yes | WEAK | Small powers of 2 always match |
| H-CR-25 | HMAC = 2 passes | Yes | WEAK | Trivially 2 |
| H-CR-26 | HMAC block = 512 | Yes | CLOSE | Inherited from SHA-256 |
| H-CR-27 | HKDF = 2 phases | Yes | WEAK | Trivially 2 |
| H-CR-28 | PBKDF2 base = 10 | Partial | WEAK | Base-10 human convention |
| H-CR-29 | Kyber n = 256 | Yes | CLOSE | NTT power-of-2 requirement |
| H-CR-30 | Kyber ring | Yes | CLOSE | Duplicate of H-CR-29 |
| H-CR-31 | PQC 5 levels | Yes | WEAK | Small integer, organizational choice |
| H-CR-32 | Leech lattice PQC | No | **FAIL** | PQC does not use Leech lattice |
| H-CR-33 | ZK pairing = 4 | Debatable | WEAK | Non-standard counting (3 groups standard) |
| H-CR-34 | Sigma protocol = 3 | Yes | CLOSE | Minimal interactive proof |
| H-CR-35 | Groth16 = 3 | Yes | CLOSE | Algebraically minimal |
| H-CR-36 | BLS12 k = 12 | Yes | **EXACT** | k=12 dominance in pairing crypto |
| H-CR-37 | ECDSA = 2 components | Yes | WEAK | Trivially a pair |
| H-CR-38 | EdDSA determinism | Yes | WEAK | Metaphor, not math |
| H-CR-39 | ML-DSA (6,5) | Yes | **EXACT** | Two-parameter match, non-trivial |
| H-CR-40 | Sig verify ratio | Approx | WEAK | Implementation-dependent |
| H-CR-41 | Entropy pool = 4096 | Partial | CLOSE | Legacy Linux, now obsolete |
| H-CR-42 | DRBG reseed = 2^48 | Yes | **EXACT** | 48 is non-obvious exponent |
| H-CR-43 | Min entropy = ln(2) | Yes | **FAIL** | Unit conversion, not prediction |
| H-CR-44 | RNG ratio = 3:1 | No | **FAIL** | No standard ratio; fabricated |
| H-CR-45 | TLS 1.3 = 5 suites | Yes | CLOSE | Engineering count, could change |
| H-CR-46 | TLS 1.3 = 1 RTT | Yes | WEAK | The number 1 matches anything |
| H-CR-47 | Cert chain = 3 | Yes | CLOSE | Common practice, not law |
| H-CR-48 | GCM limit = 2^24 | Partial | WEAK | Cherry-picked from multiple limits |

---

## Aggregate Statistics

```
  EXACT:          4  ( 8.3%)  -- H-CR-17, H-CR-36, H-CR-39, H-CR-42
  CLOSE:         24  (50.0%)
  WEAK:          17  (35.4%)
  FAIL:           3  ( 6.3%)  -- H-CR-32, H-CR-43, H-CR-44
  ---
  Total:         48
```

Compare to the original document's self-assessment of nearly 100% EXACT.
After honest, independent evaluation: **8% EXACT, 50% CLOSE, 35% WEAK, 6% FAIL**.

---

## Honest Assessment

### What is genuinely interesting (EXACT -- 4 hypotheses)

1. **H-CR-17: RSA e = 65537 = F_{tau(6)}**. The 4th Fermat prime indexed by
   tau(6) = 4 is specific, non-trivial, and not reachable by simple power-of-2
   arithmetic. tau(6) = 4 indexing the boundary between Fermat primes and
   Fermat composites is a genuine structural coincidence.

2. **H-CR-36: BLS12 embedding degree = 12 = sigma(6)**. The dominance of k = 12
   pairing-friendly curves in modern cryptography (BLS12-381, BN254) is a
   non-trivial match. k = 12 was chosen for its factorization properties
   (efficient tower extensions), which relates to why 12 = sigma(6) is an
   interesting number (highly composite structure).

3. **H-CR-39: ML-DSA-65 (k,l) = (6,5) = (n, sopfr)**. Two independent lattice
   parameters simultaneously matching n=6 functions. The joint probability of
   this specific match is low (~5%), making it the most striking coincidence.

4. **H-CR-42: DRBG reseed = 2^48 = 2^(sigma*tau)**. The exponent 48 is not a
   standard word size, byte boundary, or commonly occurring power. sigma * tau = 48
   is a clean decomposition of a non-obvious number.

### What the document does well

- Every n=6 arithmetic calculation is mathematically correct
- Every cited cryptographic standard value is factually accurate
- The coverage is comprehensive (11 tiers spanning most of modern cryptography)

### What the document systematically overstates

1. **The power-of-2 problem**: ~60% of hypotheses match powers of 2 between 2^1
   and 2^12. Since n=6 arithmetic can generate ALL exponents 1-12, these matches
   have probability ~1.0 and carry zero evidential weight individually.

2. **The small-number problem**: Matching 1, 2, 3, or 4 to n=6 functions is
   trivially easy because mu=1, phi=lambda=2, sigma/tau=n/phi=3, tau=4 are all
   available. Hypotheses claiming "2 components" or "3 levels" or "1 round trip"
   are matching universal small integers, not demonstrating n=6 structure.

3. **Duplicate counting**: H-CR-1/2 (both 128), H-CR-4/9/21/29 (all 256),
   H-CR-10/12 (both 512), H-CR-29/30 (same parameter) inflate the count.
   Unique non-trivial claims number roughly 30, not 48.

4. **Causal claims**: The hypotheses repeatedly claim n=6 "determines" or
   "derives" parameters that were historically chosen for well-documented
   engineering reasons (birthday bounds, word alignment, GNFS complexity,
   NTT efficiency). Documentation of the actual design rationale exists
   for AES (Daemen-Rijmen), SHA (NSA/NIST), RSA (Rivest-Shamir-Adleman),
   ChaCha (Bernstein), and Kyber (Schwabe et al.). None reference n=6.

### The Rijndael "+6" observation

One meta-observation deserves separate mention: the Rijndael round count formula
is Nr = max(Nk, Nb) + 6, where the constant 6 appears as a security margin.
This means AES-128 = 4 + 6 = 10, AES-192 = 6 + 6 = 12, AES-256 = 8 + 6 = 14.
The "+6" is literally n = 6, the perfect number. While the Rijndael designers
chose 6 for security analysis reasons (not number theory), the appearance of
n = 6 as a universal constant in the most widely deployed cipher is noteworthy
and could be CLOSE-graded as a meta-observation, though it was not claimed as
a separate hypothesis.
