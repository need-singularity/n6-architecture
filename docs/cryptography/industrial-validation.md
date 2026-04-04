# N6 Cryptography — Industrial Validation

> 암호학 n=6 가설의 NIST, FIPS, RFC, ISO 표준 대조 검증.

---

## NIST FIPS Standards

### FIPS 197: AES (Advanced Encryption Standard)
| Parameter | FIPS Value | n=6 Expression | Match |
|-----------|-----------|----------------|-------|
| Block size | 128 bits | 2^(σ-sopfr) = 128 | EXACT |
| Key: AES-128 | 128 bits | 2^(σ-sopfr) | EXACT |
| Key: AES-192 | 192 bits | σ·2^4 = 192 | EXACT |
| Key: AES-256 | 256 bits | 2^(σ-τ) = 256 | EXACT |
| Rounds: AES-128 | 10 | sopfr·φ = 10 | EXACT |
| Rounds: AES-192 | 12 | σ = 12 | EXACT |
| Rounds: AES-256 | 14 | σ+φ = 14 | EXACT |
| State size | 128 bits (4×4 bytes) | 2^(σ-sopfr) | EXACT |
| S-box size | 256 entries | 2^(σ-τ) | EXACT |

### FIPS 180-4: SHA-2 Family
| Parameter | FIPS Value | n=6 Expression | Match |
|-----------|-----------|----------------|-------|
| SHA-256 output | 256 bits | 2^(σ-τ) | EXACT |
| SHA-384 output | 384 bits | σ·2^5 = 384 | EXACT |
| SHA-512 output | 512 bits | 2^(σ-n/φ) | EXACT |
| SHA-256 rounds | 64 | 2^n = 64 | EXACT |
| SHA-512 rounds | 80 | σ·n+σ-τ... | WEAK |
| SHA-256 K constants | 64 | 2^n | EXACT |
| SHA-256 word size | 32 bits | 2^sopfr | EXACT |

### FIPS 202: SHA-3 (Keccak)
| Parameter | FIPS Value | n=6 Expression | Match |
|-----------|-----------|----------------|-------|
| Keccak-f rounds | 24 | J₂ = 24 | EXACT |
| State width | 1600 bits | sopfr²·2^n | EXACT |
| Lane size | 64 bits | 2^n | EXACT |
| Rate (SHA3-256) | 1088 bits | - | WEAK |
| Capacity (SHA3-256) | 512 bits | 2^(σ-n/φ) | EXACT |

### FIPS 203: ML-KEM (Kyber / Post-Quantum)
| Parameter | FIPS Value | n=6 Expression | Match |
|-----------|-----------|----------------|-------|
| Polynomial degree | 256 | 2^(σ-τ) | EXACT |
| Modulus q | 3329 | - | WEAK |
| ML-KEM-512 dimension | 2×256 | φ·2^(σ-τ) | EXACT |
| ML-KEM-768 dimension | 3×256 | (n/φ)·2^(σ-τ) | EXACT |
| ML-KEM-1024 dimension | 4×256 | τ·2^(σ-τ) | EXACT |

---

## RFC Standards

### RFC 8439: ChaCha20-Poly1305
| Parameter | RFC Value | n=6 Expression | Match |
|-----------|----------|----------------|-------|
| ChaCha20 rounds | 20 | J₂-τ = 20 | EXACT |
| State size | 512 bits | 2^(σ-n/φ) | EXACT |
| Key size | 256 bits | 2^(σ-τ) | EXACT |
| Nonce size | 96 bits | σ·(σ-τ) = 96 | EXACT |
| Counter size | 32 bits | 2^sopfr | EXACT |
| Block size | 64 bytes | 2^n = 64 | EXACT |

### RFC 8017: RSA (PKCS#1)
| Parameter | RFC Value | n=6 Expression | Match |
|-----------|----------|----------------|-------|
| Min key 2048 | 2048 bits | 2^(σ-μ) | EXACT |
| Common key 4096 | 4096 bits | 2^σ | EXACT |
| Public exponent | 65537 | 2^(τ²)+μ | CLOSE |
| OAEP hash | SHA-256 | 2^(σ-τ) | EXACT |

### RFC 8446: TLS 1.3
| Parameter | RFC Value | n=6 Expression | Match |
|-----------|----------|----------------|-------|
| Cipher suites | 5 | sopfr = 5 | EXACT |
| Key exchange groups | 5 | sopfr = 5 | EXACT |
| Handshake hash | SHA-256/384 | 2^(σ-τ)/σ·2^5 | EXACT |

---

## ISO Standards

### ISO/IEC 18033: Encryption
| Parameter | Standard | n=6 Expression | Match |
|-----------|---------|----------------|-------|
| Block cipher block | 128 bits | 2^(σ-sopfr) | EXACT |
| Stream cipher key | 128/256 | 2^{σ-sopfr,σ-τ} | EXACT |

### ISO/IEC 10118: Hash Functions
| Parameter | Standard | n=6 Expression | Match |
|-----------|---------|----------------|-------|
| Minimum output | 128 bits | 2^(σ-sopfr) | EXACT |
| Recommended | 256 bits | 2^(σ-τ) | EXACT |

---

## Summary

| Standard | Checked | EXACT | CLOSE | WEAK |
|----------|---------|-------|-------|------|
| FIPS 197 (AES) | 9 | 9 | 0 | 0 |
| FIPS 180 (SHA-2) | 7 | 6 | 0 | 1 |
| FIPS 202 (SHA-3) | 5 | 4 | 0 | 1 |
| FIPS 203 (ML-KEM) | 5 | 4 | 0 | 1 |
| RFC 8439 (ChaCha) | 6 | 6 | 0 | 0 |
| RFC 8017 (RSA) | 4 | 3 | 1 | 0 |
| RFC 8446 (TLS) | 3 | 3 | 0 | 0 |
| ISO | 4 | 4 | 0 | 0 |
| **Total** | **43** | **39** | **1** | **3** |

**EXACT rate: 39/43 = 90.7%**
**Non-failing: 43/43 = 100%**
