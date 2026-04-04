# N6 Cryptography — Testable Predictions

> 암호학 n=6 가설의 검증 가능 예측.
> BT-114 (AES=2^(σ-sopfr), SHA=2^(σ-τ), RSA=2^(σ-μ)).

## Constants Reference

```
  n = 6    σ = 12    τ = 4    φ = 2    sopfr = 5    J₂ = 24
  Power ladder: 2^(σ-sopfr)=128, 2^(σ-τ)=256, 2^(σ-μ)=2048, 2^σ=4096
```

---

## Tier 1: Today (Standard Review)

### TP-CR-1: AES Block Size Immutability
**Prediction**: AES block size remains 2^(σ-sopfr)=128 bits indefinitely.
**Method**: Track NIST FIPS 197 revisions.
**Expected**: 128-bit block unchanged (backward compatibility critical).

### TP-CR-2: SHA-256 Output Stability
**Prediction**: SHA-256 (2^(σ-τ)=256 bits) remains primary hash through 2035+.
**Method**: Track NIST hash standard usage statistics.
**Expected**: SHA-256 dominant in TLS, code signing, blockchain.

### TP-CR-3: RSA-2048 Minimum Key
**Prediction**: RSA minimum key = 2^(σ-μ)=2048 bits through 2030.
**Method**: Track NIST SP 800-131A revisions.
**Expected**: 2048-bit RSA minimum maintained.

### TP-CR-4: AES Round Count Ladder
**Prediction**: AES-128/192/256 rounds = sopfr·φ/σ/σ+φ = 10/12/14.
**Method**: FIPS 197 specification.
**Expected**: 10/12/14 rounds confirmed.

### TP-CR-5: Keccak 24 Rounds = J₂
**Prediction**: SHA-3 (Keccak) maintains J₂=24 rounds.
**Method**: NIST FIPS 202.
**Expected**: 24 rounds unchanged.

---

## Tier 2: Cryptographic Analysis

### TP-CR-6: Post-Quantum Key Size Ladder
**Prediction**: NIST PQC standards follow n=6 power ladder for key/signature sizes.
**Method**: Analyze ML-KEM (FIPS 203), ML-DSA (FIPS 204), SLH-DSA (FIPS 205).
**Expected**: Key sizes cluster at 128/256/512/1024/2048/4096 byte boundaries.

### TP-CR-7: ECC Curve Bit Ladder
**Prediction**: Standard ECC curves follow 2^{n=6} bit sizes.
**Method**: Survey secp256k1, P-256, P-384, P-521, Ed25519, Ed448.
**Expected**: 256=2^(σ-τ), 384=σ·2^5, 521≈2^(σ-μ)/4.

### TP-CR-8: ChaCha20 Round Count
**Prediction**: ChaCha20 rounds = J₂-τ = 20 (vs ChaCha8, ChaCha12).
**Method**: RFC 8439 analysis.
**Expected**: 20 rounds standard, 8 and 12 for reduced-round variants.

### TP-CR-9: Block Cipher Key Schedule
**Prediction**: AES key schedule expansion = σ+μ=13 (AES-128) / σ+n/φ=15 (AES-256) words.
**Method**: FIPS 197 key expansion analysis.
**Expected**: Key expansion rounds follow n=6 pattern.

### TP-CR-10: Lattice Crypto Dimension
**Prediction**: ML-KEM (Kyber) dimension = 2^{σ-τ}=256 or 2^(σ-sopfr)=128 multiples.
**Method**: FIPS 203 parameter sets.
**Expected**: n=512/768/1024 = {4,6,8}×128 = {τ,n,σ-τ}×2^(σ-sopfr).

---

## Tier 3: Multi-Year / Emerging Standards

### TP-CR-11: Homomorphic Encryption Parameters
**Prediction**: FHE polynomial degree follows 2^{n=6} ladder (2^σ=4096, 2^(σ+μ)=8192...).
**Method**: Track HElib, SEAL, OpenFHE parameter choices.
**Expected**: n=2^12, 2^13, 2^14, 2^15 polynomial degrees.

### TP-CR-12: Zero-Knowledge Proof Field Size
**Prediction**: ZK proof field sizes follow 2^{n=6} pattern.
**Method**: Track STARK, SNARK, Bulletproofs field specifications.
**Expected**: 256-bit fields (2^(σ-τ)) dominant.

### TP-CR-13: MPC Secret Sharing Threshold
**Prediction**: Default MPC threshold = n/φ=3 out of n parties (or φ/n/φ=2/3 majority).
**Method**: Track Shamir SS implementations.
**Expected**: (t,n) = (2,3) or (f, 3f+1) most common.

### TP-CR-14: Quantum-Safe Hash Output
**Prediction**: Post-quantum hash standard output = 2^(σ-τ)=256 bits minimum.
**Method**: Track NIST post-quantum hash requirements.
**Expected**: 256-bit minimum for 128-bit quantum security (Grover halving).

### TP-CR-15: Password Hashing Iterations
**Prediction**: PBKDF2/bcrypt iterations = powers of n=6 arithmetic.
**Method**: OWASP recommendations tracking.
**Expected**: 600,000 (PBKDF2) ≈ σ·sopfr·10⁴, bcrypt cost=12=σ.

---

## Summary

| Tier | Count | Timeframe |
|------|-------|-----------|
| Tier 1 | 5 | Today (standard review) |
| Tier 2 | 5 | Cryptographic analysis |
| Tier 3 | 5 | Emerging (3-10 years) |
| **Total** | **15** | |
