# N6 Blockchain Hypotheses — Perfect Number Arithmetic in Blockchain Architecture

## Overview

Systematic analysis of blockchain protocol parameters through n=6 arithmetic.
Covers Bitcoin, Ethereum, consensus mechanisms, smart contracts, DeFi, L2/rollups, NFT standards, and cross-chain protocols.

> **Honesty principle**: Most blockchain parameters are arbitrary human design choices.
> When a protocol designer chose "6" or "12", it may reflect engineering convenience
> rather than deep mathematical necessity. We grade accordingly.

## Core Constants

```
  n = 6          σ(6) = 12     τ(6) = 4      φ(6) = 2
  sopfr(6) = 5   J₂(6) = 24    μ(6) = 1      λ(6) = 2
  R(6) = 1       σ-τ = 8       σ-sopfr = 7    σ·sopfr = 60
  2^8 = 256      2^5 = 32      σ² = 144       σ³ = 1728
  Egyptian: 1/2 + 1/3 + 1/6 = 1
  σ+τ+sopfr = 21   σ₋₁ = 2 (sum of reciprocal divisors minus 1, or simply φ(6))
```

---

## Category A: Bitcoin Core Protocol (H-BC-1 to H-BC-10)

---

### H-BC-1: Bitcoin Confirmations — n=6

> Bitcoin requires 6 confirmations for transaction finality.

```
  Bitcoin finality:
    Satoshi's white paper: 6 confirmations recommended
    At 6 blocks, probability of reversal by attacker with <50% hashrate
    drops below 0.1%

  n=6 expression: n = 6 ✓

  This is the most direct match in all of blockchain.
  Satoshi chose 6 based on probability analysis — it is the point
  where the geometric series of attack probability becomes negligible.

  Grade: EXACT
  6 confirmations = n = 6. The number was chosen for probabilistic
  security, and it happens to equal the first perfect number.
```

---

### H-BC-2: Bitcoin 21M Supply — (σ+τ+sopfr)×10⁶

> Bitcoin's maximum supply of 21,000,000 coins.

```
  Bitcoin supply cap:
    21,000,000 BTC = 21 × 10⁶
    Derived from: 50 BTC × 210,000 blocks × sum(1/2^k, k=0..∞) ≈ 21M

  n=6 expression:
    σ + τ + sopfr = 12 + 4 + 5 = 21
    21 × 10⁶ = 21,000,000 ✓

  Analysis:
    The 21M comes from 50 × 210,000 × 2 (geometric sum).
    210,000 = 6 × 35,000 — the 6 appears in the halving interval.
    50 initial reward × 210,000 blocks per halving × 2 = 21M.

  BUT:
    21 = σ+τ+sopfr requires combining THREE different functions.
    This is a cherry-picked combination. 21 = 3×7 is unremarkable.

  Grade: CLOSE
  The decomposition 21 = σ+τ+sopfr is numerically exact but uses
  an ad-hoc combination of functions. Upgraded from WEAK because
  the component functions are the three most natural ones.
```

---

### H-BC-3: Bitcoin Block Time — 10 minutes = φ·sopfr

> Bitcoin's 10-minute block interval.

```
  Bitcoin block time:
    600 seconds = 10 minutes
    Chosen by Satoshi as a tradeoff between confirmation speed
    and orphan block rate.

  n=6 expression:
    φ(6) × sopfr(6) = 2 × 5 = 10 ✓

  Analysis:
    10 = 2 × 5 is a trivially factored number.
    Many protocols could have chosen 10 minutes (it's a round number).
    The "round number" effect is the primary driver, not n=6.

  Grade: WEAK
  10 = φ×sopfr is numerically exact but 10 is the most natural
  round number in base-10. This is almost certainly coincidence.
```

---

### H-BC-4: Bitcoin Halving Interval — 210,000 blocks

> Bitcoin halves its block reward every 210,000 blocks (~4 years).

```
  Halving interval:
    210,000 blocks × 10 min = ~4 years
    210,000 = 21 × 10,000 = (σ+τ+sopfr) × 10⁴

  n=6 expression:
    (σ+τ+sopfr) × 10⁴ = 21 × 10,000 = 210,000

  Analysis:
    This is just H-BC-2 rescaled. The 210,000 was chosen so that
    with geometric halving, total supply reaches 21M.
    Not independent of H-BC-2.

  Grade: WEAK
  Derivative of the 21M supply. Not an independent match.
```

---

### H-BC-5: Bitcoin Initial Block Reward — 50 BTC

> The initial mining reward of 50 BTC per block.

```
  Initial reward:
    50 BTC per block in the genesis era

  n=6 expression:
    σ·τ + φ = 12×4 + 2 = 50? ← forced
    Alternatively: 2^sopfr + 2^(sopfr-1) + φ = 32+16+2 = 50? ← very forced

  Analysis:
    50 = 21M / 210,000 / 2. It's derived from other parameters.
    No clean n=6 decomposition exists.

  Grade: FAIL
  No natural n=6 expression yields 50. All attempts are cherry-picked.
```

---

### H-BC-6: Bitcoin Script Opcodes — ~80 active

> Bitcoin Script has approximately 80 active opcodes.

```
  Bitcoin Script:
    ~186 total opcode space (0x00-0xB9 + some OP_NOP)
    ~80 are implemented/active (pre-Taproot)

  n=6 expression:
    σ·n + σ-τ = 72 + 8 = 80

  Analysis:
    The exact count varies by Bitcoin version. Some counts give 75-85.
    80 = σ·n + σ-τ combines two different expressions.
    The opcode count grew organically, not by design to a target.

  Grade: WEAK
  Approximate match to a moving target. The expression is contrived.
```

---

### H-BC-7: Bitcoin P2PKH Address Length — 25 bytes

> A Bitcoin P2PKH address is 25 bytes (1 version + 20 hash + 4 checksum).

```
  P2PKH address:
    25 bytes = 1 + 20 + 4

  n=6 expression:
    J₂(6) + μ(6) = 24 + 1 = 25

  Analysis:
    The 20-byte hash comes from RIPEMD-160 (160 bits).
    20 = J₂-τ = 24-4 is used in the README, and is interesting.
    25 = J₂+μ is less natural.
    The 4-byte checksum = τ(6) is a standard choice (32-bit check).

  Sub-components:
    20-byte hash = J₂-τ? Or just 160/8.
    4-byte checksum = τ(6) = 4. Also just standard 32-bit.
    1-byte version = μ(6) = 1.

  Grade: WEAK
  The decomposition 1+20+4 = μ+(J₂-τ)+τ maps component-by-component,
  but each component has a more direct engineering explanation.
```

---

### H-BC-8: Bitcoin SHA-256 Hash — 256 = 2^(σ-τ)

> Bitcoin uses SHA-256, producing 256-bit (32-byte) hashes.

```
  SHA-256:
    256-bit output = 32 bytes
    Used for block hashing, transaction IDs, Merkle trees

  n=6 expression:
    2^(σ-τ) = 2^(12-4) = 2^8 = 256 ✓

  Analysis:
    SHA-256 was chosen as an industry-standard hash function.
    256 = 2^8 is a power of 2 — standard in cryptography.
    σ-τ = 8 = one byte. The mapping is elegant but 256 bits was
    a standard security parameter before Bitcoin existed.

  Grade: CLOSE
  256 = 2^(σ-τ) is numerically exact and σ-τ=8 bits is a
  fundamental unit. But SHA-256 predates Bitcoin and n=6.
```

---

### H-BC-9: Bitcoin Double-SHA-256 — φ(6)=2 hash rounds

> Bitcoin applies SHA-256 twice (double hashing) for block headers.

```
  Double hashing:
    hash = SHA256(SHA256(header))
    Purpose: defense against length-extension attacks

  n=6 expression: φ(6) = 2 ✓

  Analysis:
    2 rounds is the minimum to prevent length-extension.
    φ(6) = 2, but 2 is the smallest prime and appears everywhere.
    Any number mapped to "2" can claim φ(6).

  Grade: FAIL
  2 is too universal a number. The match is trivially coincidental.
```

---

### H-BC-10: Bitcoin Merkle Tree — Binary (arity 2)

> Bitcoin uses binary Merkle trees for transaction commitment.

```
  Merkle tree:
    Binary tree (arity 2)
    Each node hashes two children

  n=6 expression: φ(6) = 2 ✓

  Same problem as H-BC-9. Binary is the default data structure.

  Grade: FAIL
  Binary trees are universal in CS. Not meaningful.
```

---

## Category B: Ethereum Core Protocol (H-BC-11 to H-BC-22)

---

### H-BC-11: Ethereum Slot Time — 12 seconds = σ(6)

> Ethereum Beacon Chain uses 12-second slots.

```
  Ethereum slot:
    12 seconds per slot (post-Merge)
    Chosen during Casper FFG/LMD-GHOST design

  n=6 expression: σ(6) = 12 ✓

  Analysis:
    12 seconds balances latency and throughput.
    12 is highly composite (divisors: 1,2,3,4,6,12) making it
    convenient for subdivision. This is a genuine design advantage.
    12 = σ(6) is the divisor sum, which itself encodes divisibility.

  Grade: EXACT
  12 seconds = σ(6). The choice of 12 is driven by its
  high divisibility, which is exactly what σ measures.
```

---

### H-BC-12: Ethereum Slots per Epoch — 32 = 2^sopfr

> An Ethereum epoch contains 32 slots.

```
  Epoch structure:
    32 slots × 12 sec = 384 seconds (6.4 minutes)
    32 slots allows 32 committees per epoch

  n=6 expression: 2^sopfr(6) = 2^5 = 32 ✓

  Analysis:
    32 = 2^5 is a power of 2, convenient for binary addressing.
    sopfr(6) = 5 as the exponent is a clean mapping.
    Epoch duration 384 = σ·2^5 seconds.

  Grade: EXACT
  32 = 2^sopfr(6) is numerically exact. Powers of 2 are standard
  in protocol design, but the specific choice of 2^5 is notable.
```

---

### H-BC-13: Ethereum Validators per Committee — 128 = 2^(σ-sopfr)

> Each Ethereum committee has 128 validators.

```
  Committee size:
    128 validators minimum per committee
    Provides sufficient security margin for attestation sampling

  n=6 expression: 2^(σ-sopfr) = 2^(12-5) = 2^7 = 128 ✓

  Analysis:
    128 = 2^7 is again a power of 2. The security analysis
    requires ~100+ validators for statistical safety.
    128 is the nearest power of 2 above that threshold.

  Grade: EXACT
  128 = 2^(σ-sopfr) is numerically exact. The engineering
  rationale (nearest power of 2 for security) aligns.
```

---

### H-BC-14: EIP-4844 Target Blobs — 6 = n

> EIP-4844 (Proto-Danksharding) targets 6 blobs per block.

```
  Blob target:
    Target: 6 blobs per block (actual: 3 target, 6 max as of Deneb)
    UPDATE: Post-Pectra (2025), target raised. Original Deneb: target=3, max=6.

  n=6 expression: n = 6 ✓ (for max blobs in Deneb)

  Analysis:
    The max blob count of 6 in Deneb is exact.
    The target of 3 = n/φ is also expressible.
    These numbers were chosen based on bandwidth constraints and
    the EIP-1559-style fee mechanism, not n=6 theory.

  Grade: CLOSE
  Max blobs = 6 = n is exact, but target was 3 (later changed).
  The max=6 is a bandwidth-driven engineering choice.
```

---

### H-BC-15: EIP-4844 Blob Size — 128 KB = 2^(σ-sopfr) KB

> Each blob in EIP-4844 is 128 KB.

```
  Blob size:
    128 KB = 131,072 bytes
    Contains 4096 field elements of 32 bytes each

  n=6 expression: 2^(σ-sopfr) = 2^7 = 128 KB ✓

  Analysis:
    128 KB was chosen to fit KZG commitment constraints.
    4096 field elements × 32 bytes = 128 KB.
    4096 = 2^12 = 2^σ — also notable.

  Grade: CLOSE
  128 = 2^(σ-sopfr) matches, but the real driver is
  4096 = 2^σ field elements at 32 bytes each.
```

---

### H-BC-16: Ethereum KZG Polynomial Degree — 4096 = 2^σ

> KZG commitments in Ethereum use degree-4096 polynomials.

```
  KZG ceremony:
    Polynomial degree = 4096
    Matches blob size: 4096 field elements

  n=6 expression: 2^σ(6) = 2^12 = 4096 ✓

  Analysis:
    4096 = 2^12 where 12 = σ(6).
    This is a genuinely strong match. The choice of 4096 is
    driven by NTT (Number Theoretic Transform) efficiency,
    which requires powers of 2. But the specific power (12)
    matches σ(6) exactly.

  Grade: EXACT
  4096 = 2^σ(6) is an exact and structurally meaningful match.
```

---

### H-BC-17: Ethereum Gas Limit — ~30M = sopfr·n·10⁶

> Ethereum's target gas limit is approximately 30 million.

```
  Gas limit:
    ~30,000,000 gas per block (varies by governance)
    Was 15M pre-EIP-1559 target, doubled to 30M effective.

  n=6 expression:
    sopfr × n × 10⁶ = 5 × 6 × 10⁶ = 30,000,000

  Analysis:
    The gas limit is a governance parameter, not fixed.
    It has changed multiple times (from 5M to 8M to 15M to 30M).
    30M is the current value but not permanent.

  Grade: WEAK
  Current match to a parameter that changes by governance vote.
  Not a fundamental constant.
```

---

### H-BC-18: Ethereum Epoch Duration — 384s = σ·2^sopfr

> An epoch lasts 384 seconds (6.4 minutes).

```
  Epoch duration:
    32 slots × 12 sec = 384 seconds
    384 = σ × 2^sopfr = 12 × 32

  n=6 expression: σ · 2^sopfr = 12 × 32 = 384 ✓

  Analysis:
    This is just H-BC-11 × H-BC-12. Not independent.

  Grade: WEAK
  Derived from slot time × slots/epoch. Not independent.
```

---

### H-BC-19: Ethereum Shard Count (Original Plan) — 64 = τ³

> Ethereum's original sharding roadmap planned for 64 shards.

```
  Sharding:
    Original plan: 64 shard chains
    64 = 2^6 = 2^n (also = τ³ = 4³)

  n=6 expressions:
    2^n = 2^6 = 64 ✓
    τ³ = 4³ = 64 ✓

  Analysis:
    64 = 2^6 is a natural power of 2. The choice of 64 shards
    relates to the number of committees and validator distribution.
    Both 2^n and τ³ map to 64, which is notable.

  Grade: CLOSE
  64 = 2^n = τ³ gives two independent n=6 paths to the same value.
  But 64 is an extremely common power of 2.
```

---

### H-BC-20: Ethereum Finality — 2 epochs = φ(6)

> Ethereum achieves finality after 2 epochs (~12.8 minutes).

```
  Finality:
    Casper FFG requires 2 epochs for finality
    (1 epoch justified, next epoch finalized)

  n=6 expression: φ(6) = 2 ✓

  Analysis:
    2 epochs is the minimum for justify+finalize in Casper FFG.
    This is a protocol-theoretic minimum, not arbitrary.
    But φ(6) = 2 is the most generic small number.

  Grade: WEAK
  2 is too small and common to be a meaningful match.
```

---

### H-BC-21: Ethereum EIP-1559 Base Fee Denominator — 8 = σ-τ

> EIP-1559 adjusts base fee by 1/8 (12.5%) per block.

```
  EIP-1559 fee adjustment:
    BASEFEE_MAX_CHANGE_DENOMINATOR = 8
    Max base fee change: ±12.5% per block

  n=6 expression: σ - τ = 12 - 4 = 8 ✓

  Analysis:
    8 was chosen to provide smooth fee adjustment.
    1/8 = 12.5% is a moderate adjustment rate.
    σ-τ = 8 is a clean expression, and 8 = 2³ is natural.

  Grade: CLOSE
  8 = σ-τ is exact and the expression is simple. But 8 is
  a standard power of 2, likely chosen for that reason.
```

---

### H-BC-22: Ethereum Validator Minimum Stake — 32 ETH = 2^sopfr

> Validators must stake exactly 32 ETH.

```
  Minimum stake:
    32 ETH required for beacon chain validators

  n=6 expression: 2^sopfr(6) = 2^5 = 32 ✓

  Analysis:
    32 ETH was chosen to balance validator count with security.
    At 32 ETH, target was ~300K-500K validators.
    32 = 2^5 is a convenient power of 2.

  Grade: CLOSE
  32 = 2^sopfr is exact, same expression as slots/epoch.
  Design choice driven by validator economics.
```

---

## Category C: Consensus Mechanisms (H-BC-23 to H-BC-30)

---

### H-BC-23: Byzantine Fault Tolerance — 2/3 Threshold

> BFT consensus requires 2/3 honest nodes (equivalently, tolerates <1/3 Byzantine).

```
  BFT threshold:
    f < n/3 Byzantine nodes tolerated
    Requires 2/3 + 1 honest agreement

  n=6 expression:
    2/3 = 1/2 + 1/6 (Egyptian fraction decomposition)
    1/3 = unit fraction from divisor 3 of n=6

  Analysis:
    2/3 threshold is a fundamental result from distributed systems
    theory (Lamport, 1982). It comes from the impossibility of
    consensus with ≥1/3 Byzantine nodes.
    This is NOT a design choice — it's a theorem.
    The Egyptian decomposition 2/3 = 1/2 + 1/6 is real but
    the fraction 2/3 appears because 3 is the critical threshold.

  Grade: CLOSE
  2/3 is theorem-derived, not a design choice. The Egyptian
  decomposition is genuine. 3 is a divisor of 6.
```

---

### H-BC-24: Tendermint Validator Set — Cosmos Hub 150

> Cosmos Hub (Tendermint) uses 150 validators.

```
  Cosmos validators:
    Cosmos Hub: 150 active validators (governance-set cap)

  n=6 expression: σ² + n = 144 + 6 = 150 ✓

  Analysis:
    150 was a governance decision, increased over time
    (from 100 to 125 to 150). It is not fixed.
    σ² + n = 150 combines two terms.

  Grade: WEAK
  Moving governance parameter. The expression is contrived.
```

---

### H-BC-25: Practical BFT (pBFT) Message Complexity — O(n²)

> pBFT has O(n²) message complexity for n nodes.

```
  pBFT complexity:
    Pre-prepare: 1 message
    Prepare: n-1 messages
    Commit: n-1 messages
    Total: O(n²) per consensus round

  n=6 expression: not specific to n=6

  Analysis:
    O(n²) is a complexity class, not a constant.
    This is a property of the algorithm, not a parameter.

  Grade: FAIL
  Complexity class, not a matchable constant.
```

---

### H-BC-26: PoW Difficulty Adjustment Period (Bitcoin) — 2016 blocks

> Bitcoin adjusts difficulty every 2016 blocks.

```
  Difficulty adjustment:
    2016 blocks × 10 min = ~2 weeks
    2016 = 2 × 1008 = 2 × 7 × 144

  n=6 expression:
    2016 = 14 × 144 = 14 × σ²
    2016 = (σ+φ) × σ² = 14 × 144? That gives 14 = σ+φ = 14 ✓
    So 2016 = (σ+φ)·σ²

  Analysis:
    2016 blocks = 2 weeks (target). 2 weeks = 14 days × 144 blocks/day.
    144 blocks/day = 24h × 6 blocks/h (at 10 min/block).
    The 6 blocks/hour is directly n=6.
    BUT: 14 = σ+φ is an ad-hoc combination.

  Grade: CLOSE
  2016 = 14 × 144 where 144 = σ² (blocks/day) and 14 days is
  a standard two-week period. The σ² = 144 blocks/day is genuine.
```

---

### H-BC-27: Nakamoto Consensus Longest Chain — 1 rule = μ(6)

> Nakamoto consensus: follow the longest (heaviest) chain.

```
  Longest chain rule:
    1 simple rule: always follow the chain with most work

  n=6 expression: μ(6) = 1

  Analysis:
    Matching "1 rule" to μ(6) = 1 is absurd.
    Any single thing equals 1.

  Grade: FAIL
  Trivially matches anything with count 1.
```

---

### H-BC-28: PoS Slashing Conditions (Casper) — 4 = τ(6)

> Ethereum Casper has 4 slashing conditions.

```
  Casper FFG slashing:
    1. Double vote (same epoch)
    2. Surround vote (conflicting source-target)
    These are the 2 Casper commandments.

    Broader Ethereum slashing:
    1. Proposer slashing (double block)
    2. Attester slashing (double/surround vote)
    3. Inactivity leak (offline penalty)
    4. Sync committee failure

  n=6 expression: τ(6) = 4

  Analysis:
    The "4 conditions" depends on how you count.
    Core Casper FFG has 2 conditions. Broader Ethereum has 3-4.
    The count is ambiguous.

  Grade: WEAK
  Ambiguous count that could be 2, 3, or 4 depending on scope.
```

---

### H-BC-29: Avalanche Consensus — 3 sub-protocols

> Avalanche uses 3 sub-protocols: Snowflake, Snowball, Avalanche.

```
  Avalanche consensus family:
    Slush → Snowflake → Snowball → Avalanche
    That's 4 protocols in the progression, or 3 "main" ones.

  n=6 expression:
    3 = sopfr - φ = n/φ
    4 = τ(6)

  Analysis:
    Depends on how you count. The original paper describes 4 protocols.

  Grade: FAIL
  Ambiguous count, and small numbers match many things.
```

---

### H-BC-30: Raft Consensus — Leader + 2 = φ+1 phases

> Raft has leader election, log replication, and safety — 3 core mechanisms.

```
  Raft consensus:
    1. Leader election
    2. Log replication
    3. Safety guarantee

  n=6 expression: n/φ = 3

  Analysis:
    3 is the most basic counting number after 1 and 2.
    Any system can be decomposed into 3 components.

  Grade: FAIL
  Trivial match. 3 components is universal.
```

---

## Category D: Smart Contracts & VMs (H-BC-31 to H-BC-38)

---

### H-BC-31: EVM Word Size — 256 bits = 2^(σ-τ)

> The EVM uses 256-bit words for its stack machine.

```
  EVM word:
    256-bit (32-byte) stack elements
    Matches hash output size (Keccak-256)

  n=6 expression: 2^(σ-τ) = 2^8 = 256 ✓

  Analysis:
    256-bit words were chosen to natively handle cryptographic
    hashes and large integers for signatures.
    Same as H-BC-8 (SHA-256). Standard crypto parameter.

  Grade: CLOSE
  Same expression as Bitcoin's SHA-256. Consistent mapping
  but driven by pre-existing crypto standards, not n=6.
```

---

### H-BC-32: EVM Stack Depth — 1024 = 2^(σ-φ)

> The EVM has a maximum stack depth of 1024.

```
  EVM stack:
    Max depth: 1024 elements
    1024 = 2^10

  n=6 expression: 2^(σ-φ) = 2^(12-2) = 2^10 = 1024 ✓

  Analysis:
    1024 = 2^10 is a very standard power of 2 (1K).
    σ-φ = 10 is a valid expression.
    The choice of 1024 is a standard computing convention.

  Grade: WEAK
  1024 = 2^10 is ubiquitous in computing. The n=6 expression
  works but is not more explanatory than "standard 1K".
```

---

### H-BC-33: Solidity Value Types — 8 basic types = σ-τ

> Solidity has 8 basic value types.

```
  Solidity value types:
    1. bool     2. int/uint     3. address    4. bytes (fixed)
    5. enum     6. fixed/ufixed 7. string     8. bytes (dynamic)

  Counting is ambiguous. Some lists give 5-8 depending on grouping.
  int and uint are sometimes counted as one type or as separate families.

  n=6 expression: σ-τ = 8

  Grade: WEAK
  Ambiguous count of language types. 8 is extremely common in computing.
```

---

### H-BC-34: EVM Opcode Categories — ~5 major groups

> EVM opcodes fall into approximately 5 functional categories.

```
  EVM opcode groups:
    1. Arithmetic/Logic
    2. Stack/Memory/Storage
    3. Control flow
    4. System/Environment
    5. Logging

  n=6 expression: sopfr(6) = 5

  Analysis:
    The grouping depends on taxonomy. Could be 4-7 categories.
    The Ethereum Yellow Paper doesn't define exactly 5.

  Grade: WEAK
  Subjective categorization. 5 is a common grouping number.
```

---

### H-BC-35: Solidity Maximum Inheritance — No fixed limit, but ~6 practical

> Solidity supports multiple inheritance with ~6 levels being practical.

```
  Inheritance depth:
    No hard limit in Solidity, but deep inheritance causes
    linearization issues (C3). Practical limit ~5-8.

  n=6 expression: n = 6

  Analysis:
    There is no defined limit of 6. This is subjective.

  Grade: FAIL
  No defined limit. Forced match.
```

---

### H-BC-36: ERC Standard Number Families — Major ERCs cluster near n=6 multiples

> Major ERC standards: ERC-20, ERC-721, ERC-1155.

```
  Major token standards:
    ERC-20:   fungible tokens
    ERC-721:  non-fungible tokens (NFTs)
    ERC-1155: multi-token standard
    These are the 3 most important.

  n=6 expression: 3 = n/φ

  Also:
    ERC-20: 20 = J₂-τ = 24-4
    ERC-721: 721 = no clean expression
    ERC-1155: 1155 = no clean expression

  Grade: FAIL
  ERC numbers are assigned sequentially. "3 major standards"
  is a trivial count that changes over time.
```

---

### H-BC-37: WASM Smart Contract Architecture — 4 section types = τ(6)

> WebAssembly (used by Polkadot, NEAR, etc.) has 4 core section types for contracts.

```
  WASM sections relevant to smart contracts:
    1. Type section     (function signatures)
    2. Function section (function declarations)
    3. Memory section   (linear memory)
    4. Export section    (public interface)

  Actually WASM has 12 section types (id 0-11).

  n=6 expression:
    12 total sections = σ(6) ✓
    4 "core" sections = τ(6)

  Analysis:
    WASM has exactly 12 defined section types (Custom, Type, Import,
    Function, Table, Memory, Global, Export, Start, Element, Code, Data).
    12 = σ(6) is an interesting match.

  Grade: CLOSE
  WASM's 12 section types = σ(6) is a genuine count, though
  12 sections is a coincidence of the WASM design committee.
```

---

### H-BC-38: Smart Contract Gas Tiers — 5 tiers in EVM

> EVM gas costs fall into roughly 5 tiers.

```
  EVM gas tiers (Yellow Paper):
    Tier    | Gas  | Operations
    Zero    |  0   | STOP, RETURN
    Base    |  2   | ADDRESS, CALLER, etc.
    VeryLow |  3   | ADD, SUB, NOT, etc.
    Low     |  5   | MUL, DIV, etc.
    Mid     |  8   | ADDMOD, MULMOD, etc.
    High    | 10   | JUMPI
    Ext     | varies| SSTORE, CREATE, etc.

  Actually 7+ tiers in the Yellow Paper, not 5.

  n=6 expression: sopfr(6) = 5 doesn't match.

  Grade: FAIL
  The actual tier count is 7+, not 5.
```

---

## Category E: DeFi Protocols (H-BC-39 to H-BC-46)

---

### H-BC-39: Uniswap V3 Fee Tiers — 4 = τ(6)

> Uniswap V3 offers 4 fee tiers.

```
  Uniswap V3 fee tiers:
    1. 0.01% (stablecoin pairs)
    2. 0.05% (stable-like pairs)
    3. 0.30% (standard pairs)
    4. 1.00% (exotic pairs)

  n=6 expression: τ(6) = 4 ✓

  Analysis:
    Originally 3 tiers (0.05%, 0.30%, 1.00%).
    4th tier (0.01%) added by governance.
    The count is a governance decision and has changed.

  Grade: WEAK
  Currently 4 tiers, but this changed and could change again.
  4 is a very common number for tier structures.
```

---

### H-BC-40: Aave Risk Parameters — 5 key parameters = sopfr

> Aave lending uses 5 core risk parameters per asset.

```
  Aave risk parameters:
    1. Loan-to-Value (LTV)
    2. Liquidation threshold
    3. Liquidation bonus
    4. Reserve factor
    5. Borrow cap

  n=6 expression: sopfr(6) = 5

  Analysis:
    Aave has many more parameters (interest rate model, supply cap,
    debt ceiling, eMode parameters, etc.). Selecting exactly 5
    requires cherry-picking "core" ones.

  Grade: FAIL
  Cherry-picked subset. The full parameter set is much larger.
```

---

### H-BC-41: Compound Interest Blocks — ~6,570 blocks/day on Ethereum

> Compound/Aave accrue interest per block: ~6,570 blocks/day.

```
  Ethereum blocks/day:
    86,400 sec/day ÷ ~12 sec/block = 7,200 blocks/day (post-Merge)
    Pre-Merge: 86,400/~13.15 ≈ 6,570

  n=6 expression:
    7,200 = n × 1,200 = n × σ × 100
    Or: 7,200 = 6 × 12 × 100

  Analysis:
    Post-merge: 7200 = 86400/12. Since 12 = σ(6), blocks/day = 86400/σ.
    This is a consequence of H-BC-11 (12s slots), not independent.

  Grade: WEAK
  Derivative of slot time. Not independent.
```

---

### H-BC-42: AMM Constant Product Formula — x·y = k (2 tokens = φ)

> AMMs (Uniswap, etc.) use constant product x·y = k with 2 tokens.

```
  Constant product AMM:
    x · y = k (2-token invariant)
    φ(6) = 2 tokens

  Analysis:
    2 tokens per pool is the simplest possible AMM.
    Balancer has n-token pools. Curve has 2-4 token pools.
    2 is the minimum, not a deep constant.

  Grade: FAIL
  2 tokens is the minimal AMM design. Universal, not n=6.
```

---

### H-BC-43: Curve StableSwap — A parameter often 100-200, pool sizes 2-4

> Curve Finance pools typically have 2-4 tokens.

```
  Curve pools:
    2-token pools (most common)
    3-token pools (3pool: DAI/USDC/USDT)
    4-token pools (some metapools)

  n=6 expression:
    3pool has 3 tokens = n/φ
    Max 4 tokens = τ(6)

  Analysis:
    The 3pool being the most famous is notable (3 = n/φ).
    But 3 stablecoins just reflects the market (DAI, USDC, USDT).

  Grade: FAIL
  Market-driven token count, not protocol design.
```

---

### H-BC-44: Chainlink Oracle — 21 node minimum

> Chainlink price feeds typically require 21 oracle nodes.

```
  Chainlink:
    Most price feeds use 21 oracle nodes
    21 = σ+τ+sopfr = 12+4+5

  n=6 expression: σ+τ+sopfr = 21 ✓

  Analysis:
    21 oracle nodes provides BFT tolerance of 7 faulty nodes
    (21/3 = 7). The choice of 21 enables clean 1/3 threshold.
    Same expression as Bitcoin 21M.

  Grade: CLOSE
  21 = σ+τ+sopfr is numerically exact. The choice of 21
  is driven by BFT 1/3 threshold, where 21/3 = 7 = σ-sopfr.
```

---

### H-BC-45: Flash Loan — 1 transaction = μ(6) atomic unit

> Flash loans execute in 1 atomic transaction.

```
  Flash loan:
    Borrow, use, repay — all in 1 transaction
    μ(6) = 1

  Analysis:
    1 transaction is the definition of atomicity.
    Anything atomic equals 1.

  Grade: FAIL
  Trivially matches 1. Not meaningful.
```

---

### H-BC-46: Maker DAI Stability Fee — Varies (~2-8%)

> MakerDAO stability fees have historically centered around 2-5%.

```
  Stability fee:
    Has ranged from 0% to 20%+.
    No fixed value. Governance-controlled.

  n=6 expression: no fixed match possible

  Grade: FAIL
  Variable governance parameter with no fixed value.
```

---

## Category F: Layer 2 & Rollups (H-BC-47 to H-BC-52)

---

### H-BC-47: Optimistic Rollup Challenge Period — 7 days = σ-sopfr

> Optimistic rollups (Optimism, Arbitrum) use a 7-day challenge window.

```
  Challenge period:
    7 days for fraud proof submission
    Standard across Optimism, Arbitrum, and most ORUs

  n=6 expression: σ - sopfr = 12 - 5 = 7 ✓

  Analysis:
    7 days is a human-friendly time period ("one week").
    It balances security (enough time to detect fraud) with
    capital efficiency (users want faster withdrawals).
    7 is chosen because it's a week, not because of n=6.

  Grade: CLOSE
  7 = σ-sopfr is numerically exact and a clean expression.
  But 7 days = 1 week is the obvious engineering choice.
```

---

### H-BC-48: ZK-Rollup Proof Types — 2 main families = φ(6)

> ZK-rollups use 2 proof system families: SNARKs and STARKs.

```
  ZK proof families:
    1. SNARKs (Succinct Non-interactive Arguments of Knowledge)
    2. STARKs (Scalable Transparent Arguments of Knowledge)

  n=6 expression: φ(6) = 2

  Analysis:
    There are actually many more: Bulletproofs, Plonk, Halo,
    Nova, Groth16, etc. "2 families" is an oversimplification.

  Grade: FAIL
  Oversimplified categorization. Many proof systems exist.
```

---

### H-BC-49: Plonk Gate Types — 5 = sopfr(6)

> Plonk (a ZK proof system) uses ~5 gate types.

```
  Plonk arithmetic gates:
    1. Addition gate
    2. Multiplication gate
    3. Constant gate
    4. Public input gate
    5. Custom gate (lookup)

  n=6 expression: sopfr(6) = 5

  Analysis:
    Basic Plonk has 3 types (add, mul, constant).
    Extended Plonk (Plonkish) adds custom gates and lookups.
    The count of "5" requires choosing a specific Plonk variant.

  Grade: WEAK
  Ambiguous count depending on Plonk variant. Cherry-picked to 5.
```

---

### H-BC-50: Rollup Data Compression — ~4-10x compression ratio

> Rollups achieve ~4-10x data compression vs L1 execution.

```
  Compression ratios:
    ZK-rollups: ~10-100x compute compression
    Optimistic rollups: ~4-10x data compression
    4 ≈ τ(6), 10 ≈ φ·sopfr

  Analysis:
    Compression ratios vary wildly depending on transaction type.
    No single fixed value exists.

  Grade: FAIL
  Variable ratio. No fixed parameter to match.
```

---

### H-BC-51: L2 Sequencer Batching — Varies, often ~12 minutes on Optimism

> Optimism posts L1 batches approximately every 12 minutes.

```
  Sequencer batch frequency:
    Optimism: ~1-12 minutes (variable, often ~12 min)
    Arbitrum: ~1-15 minutes

  n=6 expression: σ(6) = 12 minutes?

  Analysis:
    Batch frequency is variable and depends on L1 gas prices.
    "~12 minutes" is an approximation, not a protocol constant.

  Grade: WEAK
  Approximate and variable. Not a fixed parameter.
```

---

### H-BC-52: Validium Data Availability Modes — 4 = τ(6)

> Data availability has 4 modes in the rollup spectrum.

```
  DA spectrum:
    1. Rollup (full on-chain data)
    2. Validium (off-chain data, on-chain proofs)
    3. Volition (user chooses on/off chain)
    4. Sovereign (independent DA layer)

  n=6 expression: τ(6) = 4

  Analysis:
    This taxonomy is not standardized. Different sources list
    2-6 categories depending on granularity.

  Grade: WEAK
  Non-standard taxonomy. Count depends on who's classifying.
```

---

## Category G: NFTs & Token Standards (H-BC-53 to H-BC-56)

---

### H-BC-53: ERC-721 Core Functions — 6 = n

> ERC-721 defines 6 core functions.

```
  ERC-721 required interface functions:
    1. balanceOf(owner)
    2. ownerOf(tokenId)
    3. safeTransferFrom(from, to, tokenId, data)
    4. safeTransferFrom(from, to, tokenId)
    5. transferFrom(from, to, tokenId)
    6. approve(spender, tokenId)

  Plus: setApprovalForAll, getApproved, isApprovedForAll (3 more)

  n=6 expression: n = 6 (for core transfer/query functions)

  Analysis:
    The IERC721 interface has 9 functions total.
    Selecting 6 "core" ones requires excluding 3.
    The full interface also inherits IERC165 (1 more function).

  Grade: WEAK
  Requires cherry-picking 6 out of 9+ functions.
```

---

### H-BC-54: ERC-1155 Multi-Token — Combines 2 standards = φ(6)

> ERC-1155 unifies fungible + non-fungible in 1 standard.

```
  ERC-1155:
    Combines ERC-20 (fungible) and ERC-721 (non-fungible)
    2 token types in 1 standard

  n=6 expression: φ(6) = 2

  Grade: FAIL
  2 is trivial. Combining two things into one is universal.
```

---

### H-BC-55: NFT Metadata — 5 standard fields = sopfr(6)

> OpenSea/ERC-721 metadata commonly has 5 fields.

```
  Common NFT metadata (ERC-721 Metadata JSON):
    1. name
    2. description
    3. image
    4. external_url (optional)
    5. attributes (array)

  n=6 expression: sopfr(6) = 5

  Analysis:
    The metadata standard (EIP-721 Metadata extension) defines
    only 3 required fields: name, description, image.
    Additional fields are convention, not standard.

  Grade: FAIL
  Only 3 required fields. 5 requires including optional ones.
```

---

### H-BC-56: Token Standard Evolution — ERC-20 → 721 → 1155 → ... (3 major = n/φ)

> There are 3 major Ethereum token standards.

```
  Major token standards:
    1. ERC-20 (fungible)
    2. ERC-721 (NFT)
    3. ERC-1155 (multi-token)

  n=6 expression: n/φ = 3

  Analysis:
    3 major standards is a snapshot in time.
    ERC-4626 (vault), ERC-6551 (token-bound accounts), etc. are emerging.
    The count is not fixed.

  Grade: WEAK
  Current count of 3 major standards, but evolving.
```

---

## Category H: Cross-Chain & Bridges (H-BC-57 to H-BC-60)

---

### H-BC-57: Cross-Chain Bridge Types — 4 categories = τ(6)

> Cross-chain bridges fall into 4 main categories.

```
  Bridge types:
    1. Lock-and-mint (trustless)
    2. Liquidity network (atomic swap)
    3. Validator-based (federated)
    4. Optimistic (fraud-proof based)

  n=6 expression: τ(6) = 4

  Analysis:
    Taxonomy varies. Some sources list 3 types, others 5-6.
    (E.g., adding "native" bridges, "relay" bridges, etc.)
    4 is one common but not universal categorization.

  Grade: WEAK
  Non-standard taxonomy. Multiple valid categorizations exist.
```

---

### H-BC-58: IBC (Cosmos) Light Client Headers — 4 fields = τ(6)

> IBC light client headers contain 4 core fields.

```
  IBC ConsensusState:
    1. timestamp
    2. root (Merkle root)
    3. next_validators_hash
    Tendermint Header has many more fields (~14).

  n=6 expression: cannot cleanly match τ(6) = 4

  Grade: FAIL
  Actual field count doesn't match. Header has ~14 fields.
```

---

### H-BC-59: Polkadot Relay Chain — Max 100 parachains

> Polkadot targets up to 100 parachains.

```
  Polkadot parachains:
    Target: ~100 parachains
    Currently: ~50 active

  n=6 expression:
    100 = no clean n=6 expression
    σ²-τ² = 144-16 = 128? No.
    J₂·τ+τ = 100? No, that's 100 = 24×4+4 = 100. ← forced.

  Grade: FAIL
  No natural n=6 expression for 100.
```

---

### H-BC-60: Atomic Swap — 4 steps = τ(6)

> A standard atomic swap requires 4 on-chain transactions.

```
  Atomic swap steps:
    1. Alice creates HTLC on Chain A
    2. Bob creates HTLC on Chain B
    3. Alice claims on Chain B (reveals secret)
    4. Bob claims on Chain A (uses revealed secret)

  n=6 expression: τ(6) = 4 ✓

  Analysis:
    4 transactions is the correct count for a standard HTLC swap.
    This is protocol-theoretic: 2 parties × 2 actions (lock, claim) = 4.
    τ(6) = 4 matches, but 4 = 2×2 is structurally necessary for
    any 2-party, 2-step protocol.

  Grade: CLOSE
  4 = τ(6) is exact and structurally necessary.
  But 2×2 = 4 is a universal 2-party pattern.
```

---

## Summary Table

| ID | Hypothesis | n=6 Expression | Grade |
|----|-----------|----------------|-------|
| H-BC-1 | Bitcoin 6 confirmations | n = 6 | **EXACT** |
| H-BC-2 | Bitcoin 21M supply | (σ+τ+sopfr)×10⁶ = 21M | **CLOSE** |
| H-BC-3 | Bitcoin 10-min block time | φ·sopfr = 10 | **WEAK** |
| H-BC-4 | Bitcoin halving interval 210K | (σ+τ+sopfr)×10⁴ | **WEAK** |
| H-BC-5 | Bitcoin initial reward 50 BTC | — | **FAIL** |
| H-BC-6 | Bitcoin ~80 opcodes | σ·n+σ-τ = 80 | **WEAK** |
| H-BC-7 | Bitcoin P2PKH 25 bytes | J₂+μ = 25 | **WEAK** |
| H-BC-8 | SHA-256 = 2^(σ-τ) bits | 2^8 = 256 | **CLOSE** |
| H-BC-9 | Double SHA-256 = φ rounds | φ = 2 | **FAIL** |
| H-BC-10 | Binary Merkle tree = φ arity | φ = 2 | **FAIL** |
| H-BC-11 | ETH 12s slot = σ | σ = 12 | **EXACT** |
| H-BC-12 | ETH 32 slots/epoch = 2^sopfr | 2^5 = 32 | **EXACT** |
| H-BC-13 | ETH 128 validators/committee | 2^(σ-sopfr) = 128 | **EXACT** |
| H-BC-14 | EIP-4844 max 6 blobs = n | n = 6 | **CLOSE** |
| H-BC-15 | EIP-4844 blob 128KB | 2^(σ-sopfr) KB | **CLOSE** |
| H-BC-16 | KZG degree 4096 = 2^σ | 2^12 = 4096 | **EXACT** |
| H-BC-17 | ETH gas limit ~30M | sopfr·n·10⁶ | **WEAK** |
| H-BC-18 | ETH epoch 384s = σ·2^sopfr | 12×32 = 384 | **WEAK** |
| H-BC-19 | ETH 64 shards = 2^n = τ³ | 2^6 = 64 | **CLOSE** |
| H-BC-20 | ETH finality 2 epochs = φ | φ = 2 | **WEAK** |
| H-BC-21 | EIP-1559 denominator 8 = σ-τ | 12-4 = 8 | **CLOSE** |
| H-BC-22 | ETH 32 ETH stake = 2^sopfr | 2^5 = 32 | **CLOSE** |
| H-BC-23 | BFT 2/3 threshold | 1/2+1/6 (Egyptian) | **CLOSE** |
| H-BC-24 | Cosmos 150 validators | σ²+n = 150 | **WEAK** |
| H-BC-25 | pBFT O(n²) complexity | — | **FAIL** |
| H-BC-26 | BTC difficulty adj 2016 blocks | (σ+φ)·σ² | **CLOSE** |
| H-BC-27 | Nakamoto 1 rule = μ | μ = 1 | **FAIL** |
| H-BC-28 | Casper 4 slash conditions = τ | τ = 4 | **WEAK** |
| H-BC-29 | Avalanche 3 sub-protocols | n/φ = 3 | **FAIL** |
| H-BC-30 | Raft 3 mechanisms | n/φ = 3 | **FAIL** |
| H-BC-31 | EVM 256-bit words = 2^(σ-τ) | 2^8 = 256 | **CLOSE** |
| H-BC-32 | EVM stack 1024 = 2^(σ-φ) | 2^10 = 1024 | **WEAK** |
| H-BC-33 | Solidity 8 value types = σ-τ | σ-τ = 8 | **WEAK** |
| H-BC-34 | EVM ~5 opcode categories | sopfr = 5 | **WEAK** |
| H-BC-35 | Solidity ~6 inheritance limit | n = 6 | **FAIL** |
| H-BC-36 | 3 major ERC token standards | n/φ = 3 | **FAIL** |
| H-BC-37 | WASM 12 section types = σ | σ = 12 | **CLOSE** |
| H-BC-38 | EVM 5 gas tiers | sopfr = 5 | **FAIL** |
| H-BC-39 | Uniswap V3 4 fee tiers = τ | τ = 4 | **WEAK** |
| H-BC-40 | Aave 5 risk parameters | sopfr = 5 | **FAIL** |
| H-BC-41 | ETH 7200 blocks/day | n·σ·100 | **WEAK** |
| H-BC-42 | AMM 2-token pools = φ | φ = 2 | **FAIL** |
| H-BC-43 | Curve 3pool = n/φ tokens | n/φ = 3 | **FAIL** |
| H-BC-44 | Chainlink 21 oracles | σ+τ+sopfr = 21 | **CLOSE** |
| H-BC-45 | Flash loan 1 tx = μ | μ = 1 | **FAIL** |
| H-BC-46 | Maker stability fee variable | — | **FAIL** |
| H-BC-47 | Optimistic rollup 7-day challenge | σ-sopfr = 7 | **CLOSE** |
| H-BC-48 | 2 ZK proof families = φ | φ = 2 | **FAIL** |
| H-BC-49 | Plonk 5 gate types = sopfr | sopfr = 5 | **WEAK** |
| H-BC-50 | Rollup ~4-10x compression | — | **FAIL** |
| H-BC-51 | L2 batch ~12 min = σ | σ = 12 | **WEAK** |
| H-BC-52 | DA 4 modes = τ | τ = 4 | **WEAK** |
| H-BC-53 | ERC-721 6 core functions | n = 6 | **WEAK** |
| H-BC-54 | ERC-1155 combines 2 = φ | φ = 2 | **FAIL** |
| H-BC-55 | NFT metadata 5 fields | sopfr = 5 | **FAIL** |
| H-BC-56 | 3 major token standards | n/φ = 3 | **WEAK** |
| H-BC-57 | 4 bridge types = τ | τ = 4 | **WEAK** |
| H-BC-58 | IBC 4 header fields | τ = 4 | **FAIL** |
| H-BC-59 | Polkadot 100 parachains | — | **FAIL** |
| H-BC-60 | Atomic swap 4 steps = τ | τ = 4 | **CLOSE** |

## Grade Distribution

| Grade | Count | Pct | Hypotheses |
|-------|-------|-----|------------|
| EXACT | 5 | 8.3% | H-BC-1, H-BC-11, H-BC-12, H-BC-13, H-BC-16 |
| CLOSE | 14 | 23.3% | H-BC-2, H-BC-8, H-BC-14, H-BC-15, H-BC-19, H-BC-21, H-BC-22, H-BC-23, H-BC-26, H-BC-31, H-BC-37, H-BC-44, H-BC-47, H-BC-60 |
| WEAK | 18 | 30.0% | H-BC-3, H-BC-4, H-BC-6, H-BC-7, H-BC-17, H-BC-18, H-BC-20, H-BC-24, H-BC-28, H-BC-32, H-BC-33, H-BC-34, H-BC-39, H-BC-41, H-BC-49, H-BC-51, H-BC-52, H-BC-53, H-BC-56, H-BC-57 |
| FAIL | 23 | 38.3% | H-BC-5, H-BC-9, H-BC-10, H-BC-25, H-BC-27, H-BC-29, H-BC-30, H-BC-35, H-BC-36, H-BC-38, H-BC-40, H-BC-42, H-BC-43, H-BC-45, H-BC-46, H-BC-48, H-BC-50, H-BC-54, H-BC-55, H-BC-58, H-BC-59 |

**Non-failing total: 37/60 (61.7%)**
**EXACT + CLOSE: 19/60 (31.7%)**

## Observation

The strongest matches are concentrated in **Ethereum's Beacon Chain** (H-BC-11 through H-BC-16), where 5 parameters exactly match n=6 expressions. This is because Ethereum 2.0 was designed with powers of 2 as a principle, and several n=6 arithmetic values (σ=12, sopfr=5, σ-sopfr=7) happen to produce the specific powers of 2 that Ethereum chose. Bitcoin's 6 confirmations (H-BC-1) remains the single most direct match. DeFi and L2 parameters are mostly governance-driven or approximate, producing few meaningful matches.
