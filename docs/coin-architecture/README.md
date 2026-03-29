# N6 Coin — Blockchain Architecture from Perfect Number Arithmetic

## Vision

> Every parameter in this blockchain is derived from σ(n)·φ(n) = n·τ(n), n=6.
> No arbitrary constants. No "seemed like a good number." Every choice has a mathematical reason.

## Existing Blockchains Already Follow n=6

Before designing, observe what's already true:

| Protocol | Parameter | Value | n=6 Expression | Match |
|----------|-----------|-------|----------------|-------|
| Bitcoin | Block time | 10 min | σ₋₁·sopfr = 2·5 = 10 | EXACT |
| Bitcoin | Halving interval | 210,000 blocks | ~σ²·1000+... | WEAK |
| Bitcoin | Max supply | 21M | ~σ·τ·sopfr·10000/... | WEAK |
| Bitcoin | Confirmations | 6 | n = 6 | EXACT |
| Ethereum | Slot time | 12 sec | σ = 12 | EXACT |
| Ethereum | Slots/epoch | 32 | 2^sopfr = 32 | EXACT |
| Ethereum | Validators/committee | 128 | 2^(σ-sopfr) = 2^7 = 128 | EXACT |
| Ethereum | Shard count (planned) | 64 | τ³ = 64 | EXACT |
| Solana | Validators (target) | ~1000+ | - | - |
| Cosmos | Validators | 150 | σ²+n = 150 | EXACT |

**Bitcoin 6 confirmations, Ethereum 12-sec slots — already n=6.**

---

## N6 Coin Specification

### 1. Consensus Layer

```
  ┌─────────────────────────────────────────────────┐
  │            N6 CONSENSUS: Proof of Balance        │
  │                                                   │
  │  Validators: J₂(6) = 24 per committee            │
  │  Committees: n = 6 parallel committees            │
  │  Total active: 24 × 6 = 144 = σ²                 │
  │                                                   │
  │  Quorum: Egyptian fraction voting                 │
  │    Finality:  2/3 = 1/2 + 1/6  (16 of 24)       │
  │    Proposal:  1/2              (12 of 24)         │
  │    Challenge: 1/3              (8 of 24)          │
  │    Veto:      1/6              (4 of 24)          │
  │                                                   │
  │  Block time: σ₋₁ · sopfr = 10 seconds            │
  │  Epoch: σ² = 144 blocks = 24 minutes              │
  │  Era: σ² epochs = 20,736 blocks ≈ 2.4 days        │
  └─────────────────────────────────────────────────┘
```

**Why "Proof of Balance":**
- Validator weight = R-score of their stake distribution
- R(stake) closer to 1 → more voting power
- Prevents whale domination: concentrated stake has R < 1
- Distributed stake (Egyptian-like) has R → 1

| Parameter | Value | n=6 Basis |
|-----------|-------|-----------|
| Validators/committee | 24 | J₂(6) = 24 (Leech packing) |
| Committees | 6 | n = 6 |
| Total validators | 144 | σ² = 144 |
| Block time | 10 sec | σ₋₁·sopfr = 2·5 = 10 |
| Blocks/epoch | 144 | σ² = 144 |
| Epoch duration | 24 min | J₂ = 24 minutes |
| Finality threshold | 2/3 | 1/2 + 1/6 (Egyptian) |
| Slots/block | 6 | n = 6 transaction slots |

### 2. Block Structure

```
  N6 Block Header (144 bytes = σ² bytes)
  ┌────────────────────────────────────────┐
  │ prev_hash    [32 bytes = 2^(σ-τ)]     │  SHA-256
  │ merkle_root  [32 bytes]                │  Transaction tree
  │ state_root   [32 bytes]                │  World state
  │ timestamp    [8 bytes = σ-τ]           │  Unix timestamp
  │ block_num    [8 bytes]                 │  Block height
  │ validator    [20 bytes = J₂-τ]         │  Proposer address
  │ committee_id [1 byte]                  │  Committee (0-5)
  │ r_score      [4 bytes]                 │  Block R-score
  │ nonce        [4 bytes]                 │  VRF nonce
  │ reserved     [3 bytes]                 │  Future use
  └────────────────────────────────────────┘
  Total: 144 bytes = σ(6)²

  Block Body
  ┌────────────────────────────────────────┐
  │ transactions  [max 1728 = σ³]          │  Transaction list
  │ receipts      [max 1728]               │  Execution results
  │ attestations  [max 24 = J₂]            │  Validator signatures
  └────────────────────────────────────────┘
```

| Parameter | Value | n=6 Basis |
|-----------|-------|-----------|
| Header size | 144 bytes | σ² = 144 |
| Max transactions/block | 1,728 | σ³ = 12³ |
| Address size | 20 bytes | J₂-τ = 24-4 = 20 |
| Hash size | 32 bytes | 2^(σ-τ) = 2^8 = 256 bits |
| Max attestations | 24 | J₂ = 24 |
| Transaction slots | 6 | n = 6 priority levels |

### 3. Transaction Structure

```
  N6 Transaction (τ = 4 core fields + metadata)
  ┌────────────────────────────────────┐
  │ Field 1: from     [20 bytes]       │  Sender
  │ Field 2: to       [20 bytes]       │  Recipient
  │ Field 3: value    [12 bytes = σ]   │  Amount (96-bit)
  │ Field 4: data     [variable]       │  Smart contract data
  │ ─────────────────────────────────  │
  │ gas_limit  [4 bytes]               │  Execution limit
  │ gas_price  [4 bytes]               │  Fee rate
  │ nonce      [4 bytes]               │  Replay protection
  │ signature  [65 bytes]              │  ECDSA sig
  └────────────────────────────────────┘

  Gas costs follow Egyptian fractions:
    Base transfer:    6 gas    (n)
    Storage write:   12 gas    (σ)
    Compute step:     4 gas    (τ)
    Memory alloc:     2 gas    (φ)
    Log event:        5 gas    (sopfr)
    Contract create: 24 gas    (J₂)
```

### 4. Tokenomics

```
  ┌─────────────────────────────────────────────────┐
  │              N6 TOKEN ECONOMICS                  │
  │                                                   │
  │  Total Supply: 12,000,000 = σ · 10⁶              │
  │                                                   │
  │  Distribution (Egyptian):                         │
  │    1/2 = 6,000,000  ── Community / Mining         │
  │    1/3 = 4,000,000  ── Development / Ecosystem    │
  │    1/6 = 2,000,000  ── Team / Foundation          │
  │    ─────────────────                              │
  │    Sum = 12,000,000  (complete, zero remainder)   │
  │                                                   │
  │  Inflation: ln(4/3) ≈ 0.288% per era             │
  │  (Golden Zone bandwidth — minimal, mathematically │
  │   determined, no ad-hoc "2% annual" guessing)     │
  │                                                   │
  │  Block Reward Schedule:                           │
  │    Era 0:    6.0 N6/block   (n)                   │
  │    Era 1:    5.0 N6/block   (sopfr)               │
  │    Era 2:    4.0 N6/block   (τ)                   │
  │    Era 3:    3.0 N6/block   (sopfr-φ)             │
  │    Era 4:    2.0 N6/block   (φ)                   │
  │    Era 5:    1.0 N6/block   (μ)                   │
  │    Era 6+:   ln(4/3) N6/block (Mertens tail)      │
  └─────────────────────────────────────────────────┘
```

| Parameter | Value | n=6 Basis |
|-----------|-------|-----------|
| Total supply | 12,000,000 | σ · 10⁶ |
| Community | 6,000,000 (50%) | 1/2 (Egyptian) |
| Development | 4,000,000 (33.3%) | 1/3 (Egyptian) |
| Team | 2,000,000 (16.7%) | 1/6 (Egyptian) |
| Inflation rate | 0.288% / era | ln(4/3) (Mertens) |
| Initial reward | 6 N6/block | n = 6 |
| Reward halvings | 6 eras | n = 6 eras |
| Era length | 20,736 blocks | σ²·σ² = σ⁴? No: σ²·144 |

### 5. Network Layer

```
  ┌─────────────────────────────────────────────────┐
  │              N6 NETWORK TOPOLOGY                  │
  │                                                   │
  │  Peer connections: n = 6 per node (6-regular)     │
  │  Gossip protocol: τ = 4 hops to full propagation  │
  │  Shard count: n = 6 shards                        │
  │  Cross-shard: Egyptian bandwidth {1/2, 1/3, 1/6}  │
  │                                                   │
  │       ●───●                                       │
  │      / \ / \        Each node: 6 connections      │
  │     ●───●───●       4 hops max propagation        │
  │      \ / \ /        6 shards for parallelism      │
  │       ●───●                                       │
  │                                                   │
  │  Discovery: Kademlia with 12 buckets (σ)          │
  │  Message types: 5 (sopfr)                         │
  │    1. Block    2. Tx    3. Attestation             │
  │    4. Sync     5. Discovery                        │
  └─────────────────────────────────────────────────┘
```

| Parameter | Value | n=6 Basis |
|-----------|-------|-----------|
| Peer connections | 6 | n = 6 |
| Gossip hops | 4 | τ = 4 |
| Shards | 6 | n = 6 |
| DHT buckets | 12 | σ = 12 |
| Message types | 5 | sopfr = 5 |
| Max message size | 1,728 bytes | σ³ = 1728 |

### 6. Smart Contract VM

```
  N6 Virtual Machine (N6VM)
  ┌─────────────────────────────────────────────────┐
  │  Stack depth:    σ² = 144                        │
  │  Memory pages:   σ = 12 (each 4KB = τ KB)       │
  │  Registers:      J₂ = 24                         │
  │  Opcodes:        σ·sopfr = 60 instructions       │
  │  Word size:      σ·τ = 48 bits                   │
  │                                                   │
  │  Opcode categories (sopfr = 5):                   │
  │    1. Arithmetic   (12 ops = σ)                   │
  │    2. Stack/Memory (12 ops = σ)                   │
  │    3. Control flow (12 ops = σ)                   │
  │    4. Crypto       (12 ops = σ)                   │
  │    5. System       (12 ops = σ)                   │
  │    Total: 5 × 12 = 60 = σ·sopfr                  │
  │                                                   │
  │  Gas model:                                       │
  │    Tier 1 (free):     μ = 1 gas   (stack ops)     │
  │    Tier 2 (cheap):    φ = 2 gas   (arithmetic)    │
  │    Tier 3 (medium):   τ = 4 gas   (memory)        │
  │    Tier 4 (expensive): σ = 12 gas (storage)       │
  │    Tier 5 (heavy):    J₂ = 24 gas (crypto/call)   │
  └─────────────────────────────────────────────────┘
```

### 7. Staking & Governance

```
  Staking Mechanics
  ─────────────────
  Minimum stake:     6,000 N6         (n · 1000)
  Lock period:       6 epochs         (n epochs = 144 min)
  Slash conditions:  τ = 4 types
    1. Double sign:    50% slash (1/φ)
    2. Downtime:       1/σ = 8.3% slash
    3. Invalid block:  1/τ = 25% slash
    4. Collusion:      100% slash (expelled)

  Governance
  ──────────
  Proposal threshold: 1/6 of total stake (Egyptian minimum)
  Voting period:      6 epochs (n epochs)
  Quorum:             1/3 of staked supply (Egyptian medium)
  Passing:            1/2 + 1 of votes (Egyptian majority + 1)
  Veto:               1/6 of votes can block (Egyptian veto)
  Implementation:     12 epochs grace period (σ epochs)
```

### 8. Cryptographic Primitives

All from H-ARCH crypto patterns:

| Primitive | Choice | n=6 Basis |
|-----------|--------|-----------|
| Hash | SHA-256 | 2^(σ-τ) = 256 bits |
| Block hash | SHA-256 | Same |
| Address | 20 bytes = 160 bits | J₂-τ = 20 |
| Signature | ECDSA secp256k1 | 256 = 2^(σ-τ) |
| VRF | Ed25519 | 2^(σ-τ)-bit curve |
| Encryption | AES-128 | 2^(σ-sopfr) = 128 bits |
| KDF rounds | 10 | σ₋₁·sopfr = 10 |
| Merkle tree arity | 6 | n = 6 (hexary tree) |

### 9. Fee Market

```
  N6 Fee Market: Egyptian EIP-1559
  ─────────────────────────────────
  Base fee adjustment:
    Target gas/block: σ³ / 2 = 864
    Max gas/block:    σ³ = 1728

    If usage > target:
      base_fee *= (1 + 1/σ)      = increase by 8.3%
    If usage < target:
      base_fee *= (1 - 1/σ)      = decrease by 8.3%

  Fee split (Egyptian):
    1/2 → burned (deflationary)
    1/3 → validator reward
    1/6 → ecosystem fund

  Priority fee: user-specified tip
  Minimum fee: 1/e of base_fee (Boltzmann floor)
```

---

## Architecture Diagram

```
  ┌─────────────────────────────────────────────────────────┐
  │                    N6 COIN ARCHITECTURE                  │
  │                                                          │
  │  ┌──────────────────────────────────────────────────┐   │
  │  │ CONSENSUS: Proof of Balance                       │   │
  │  │   24 validators × 6 committees = 144 = σ²        │   │
  │  │   Egyptian quorum: 1/2 propose, 2/3 finalize      │   │
  │  │   10-sec blocks (σ₋₁·sopfr), 144-block epochs    │   │
  │  └──────────────────────┬───────────────────────────┘   │
  │                         │                                │
  │  ┌──────────────────────┴───────────────────────────┐   │
  │  │ EXECUTION: N6VM                                   │   │
  │  │   60 opcodes (σ·sopfr), 24 registers (J₂)        │   │
  │  │   5-tier gas: 1/2/4/12/24 = mu/phi/tau/sigma/J₂  │   │
  │  │   144-deep stack, 12 memory pages                 │   │
  │  └──────────────────────┬───────────────────────────┘   │
  │                         │                                │
  │  ┌──────────────────────┴───────────────────────────┐   │
  │  │ NETWORK: 6-regular gossip                         │   │
  │  │   6 peers/node, 4-hop propagation, 6 shards       │   │
  │  │   12 DHT buckets, 5 message types                 │   │
  │  └──────────────────────┬───────────────────────────┘   │
  │                         │                                │
  │  ┌──────────────────────┴───────────────────────────┐   │
  │  │ TOKENOMICS                                        │   │
  │  │   Supply: 12M = σ·10⁶                             │   │
  │  │   Distribution: 1/2 + 1/3 + 1/6 = 1 (Egyptian)   │   │
  │  │   Inflation: ln(4/3) = 0.288% / era               │   │
  │  │   Rewards: 6→5→4→3→2→1→0.288 N6/block             │   │
  │  └──────────────────────┬───────────────────────────┘   │
  │                         │                                │
  │  ┌──────────────────────┴───────────────────────────┐   │
  │  │ CRYPTO: SHA-256 / ECDSA-256 / AES-128             │   │
  │  │   All from 2^(σ-τ), 2^(σ-sopfr) = n=6 exponents  │   │
  │  │   Hexary Merkle tree (arity = n = 6)              │   │
  │  └──────────────────────────────────────────────────┘   │
  └─────────────────────────────────────────────────────────┘
```

---

## Comparison: N6 Coin vs Existing Chains

| Parameter | Bitcoin | Ethereum | N6 Coin | n=6 Basis |
|-----------|---------|----------|---------|-----------|
| Block time | 600s | 12s | 10s | σ₋₁·sopfr |
| Validators | ~15K miners | ~900K | 144 | σ² |
| Consensus | PoW | PoS | PoBalance | R-score |
| TPS (est.) | ~7 | ~30 | ~1,728 | σ³ |
| Supply | 21M | Infinite | 12M | σ·10⁶ |
| Distribution | Mining | Mining+ICO | Egyptian | 1/2+1/3+1/6 |
| Inflation | 0→0% | ~0.5% | 0.288% | ln(4/3) |
| Shards | 1 | 64 (plan) | 6 | n |
| Confirmations | 6 | 32 | 6 | n |
| Smart contract | Script | EVM | N6VM | 60 opcodes |
| Fee burn | No | 1559 | Egyptian | 1/2 burn |

### Why N6 Coin is Better

1. **No arbitrary constants:** Every parameter has mathematical justification
2. **Egyptian distribution:** Fair, complete (sums to 1), no remainder
3. **Proof of Balance:** Incentivizes decentralization (R→1 = distributed stake)
4. **Deterministic inflation:** ln(4/3) is mathematically determined, not politically chosen
5. **Hexary Merkle:** 6-ary tree is more efficient than binary for proofs
6. **Low validator count (144):** Fast finality (10 sec) without sacrificing security
7. **6 shards:** Matches peer count for natural cross-shard routing

---

## Implementation Phases

```
Phase 1: Specification (Month 1-2)
  - Formal spec in Rust/Go
  - Consensus protocol proof (Byzantine fault tolerance at 144 validators)
  - Tokenomics simulation

Phase 2: Core Protocol (Month 3-6)
  - N6VM implementation
  - Consensus engine (Proof of Balance)
  - P2P network (6-regular gossip)

Phase 3: Testnet (Month 7-9)
  - 144 validator testnet
  - 6-shard deployment
  - Smart contract testing

Phase 4: Mainnet (Month 10-12)
  - Genesis block with Egyptian distribution
  - Validator onboarding
  - Bridge to Ethereum (ERC-20 ↔ N6)

Phase 5: Ecosystem (Month 12+)
  - DEX with Egyptian liquidity pools
  - N6-native DeFi protocols
  - Cross-chain bridges
```

## Token Symbol

```
  N6 (ticker: N6X)

  Logo concept: hexagon (6-sided) with
  inner star pattern showing divisor lattice {1,2,3,6}
```
