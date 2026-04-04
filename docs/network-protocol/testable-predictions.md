# N6 Network Protocol — Testable Predictions

> 네트워크 프로토콜 n=6 가설의 검증 가능 예측.
> BT-115 (OSI=σ-sopfr=7, TCP/IP=τ=4), BT-47 (interconnect gen counts).

## Constants Reference

```
  n = 6    σ = 12    τ = 4    φ = 2    sopfr = 5    J₂ = 24
  σ-sopfr = 7  σ-τ = 8  σ-φ = 10  σ-μ = 11
```

---

## Tier 1: Today (RFC / Standard Review)

### TP-NP-1: OSI 7-Layer Stability
**Prediction**: OSI model maintains σ-sopfr=7 layers with no additions.
**Method**: Track ISO/IEC 7498 revisions.
**Expected**: 7 layers unchanged since 1984.

### TP-NP-2: TCP/IP 4-Layer Stability
**Prediction**: TCP/IP maintains τ=4 layers as practical model.
**Method**: RFC 1122 and successors.
**Expected**: 4 layers (Link, Internet, Transport, Application).

### TP-NP-3: TCP Original 6 Flags = n
**Prediction**: TCP original 6 control flags (URG, ACK, PSH, RST, SYN, FIN) = n.
**Method**: RFC 793 analysis.
**Expected**: 6 original flags confirmed.

### TP-NP-4: DNS Root Servers = σ+μ = 13
**Prediction**: DNS root servers remain 13 = σ+μ.
**Method**: IANA root-servers.org.
**Expected**: 13 root server letters (A-M).

### TP-NP-5: HTTP Methods = σ-τ = 8 or σ-sopfr = 7
**Prediction**: HTTP standard methods cluster at 7-8.
**Method**: RFC 9110 (HTTP Semantics).
**Expected**: 8 methods (GET, HEAD, POST, PUT, DELETE, CONNECT, OPTIONS, TRACE) or 7 common.

---

## Tier 2: Protocol Analysis (Multi-Standard)

### TP-NP-6: WiFi Channel Count = σ to σ+μ
**Prediction**: 2.4GHz WiFi uses σ+μ=13 channels (US) or σ=12 (Japan).
**Method**: IEEE 802.11 channel allocation by region.
**Expected**: 11-14 channels depending on region, centered on σ.

### TP-NP-7: IPv4 Header = J₂-τ = 20 bytes
**Prediction**: IPv4 minimum header = J₂-τ = 20 bytes.
**Method**: RFC 791.
**Expected**: 20 bytes minimum header.

### TP-NP-8: IPv6 Header = J₂+σ+τ = 40 bytes
**Prediction**: IPv6 fixed header = 40 bytes.
**Method**: RFC 8200.
**Expected**: 40 bytes = J₂-τ × φ = 20 × 2.

### TP-NP-9: TLS 1.3 Cipher Suites = sopfr
**Prediction**: TLS 1.3 defines sopfr=5 cipher suites.
**Method**: RFC 8446 Section 8.4.
**Expected**: 5 cipher suites (3 mandatory + 2 optional).

### TP-NP-10: BGP Path Attributes
**Prediction**: Well-known mandatory BGP path attributes = τ=4.
**Method**: RFC 4271.
**Expected**: ORIGIN, AS_PATH, NEXT_HOP, (LOCAL_PREF) = 3-4.

---

## Tier 3: Future Protocol Evolution

### TP-NP-11: QUIC vs TCP Convergence
**Prediction**: QUIC maintains τ=4 key protocol features (multiplexing, encryption, migration, 0-RTT).
**Method**: Track RFC 9000 implementations.
**Expected**: 4 core features stable.

### TP-NP-12: HTTP/3 Stream Types
**Prediction**: HTTP/3 uni-directional stream types = τ=4 or fewer.
**Method**: RFC 9114.
**Expected**: Control, Push, QPACK Encoder, QPACK Decoder = 4 types.

### TP-NP-13: 5G NR Numerology
**Prediction**: 5G NR subcarrier spacing options = sopfr=5 (15/30/60/120/240 kHz).
**Method**: 3GPP TS 38.211.
**Expected**: 5 numerology options (μ=0..4).

### TP-NP-14: Ethernet Speed Ladder
**Prediction**: Ethernet speed ladder follows n=6 power pattern.
**Method**: IEEE 802.3 standard evolution.
**Expected**: 10/100/1G/10G/100G/400G → decades of 10× jumps.

### TP-NP-15: SDN Controller Architecture
**Prediction**: SDN architecture = n/φ=3 planes (data/control/management).
**Method**: ONF architecture documents.
**Expected**: 3-plane architecture standard.

---

## Summary

| Tier | Count | Timeframe |
|------|-------|-----------|
| Tier 1 | 5 | Today (RFC review) |
| Tier 2 | 5 | Multi-standard analysis |
| Tier 3 | 5 | Future evolution |
| **Total** | **15** | |
