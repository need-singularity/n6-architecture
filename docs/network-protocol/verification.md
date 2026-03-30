# N6 Network Protocol Hypotheses — Strengthened Independent Verification

Date: 2026-03-30 (revised)

## Methodology

Each hypothesis (H-NP-1 through H-NP-18) is verified against:
1. **Math check**: Does the claimed n=6 derivation hold arithmetically?
2. **Fact check**: Does the predicted value match real-world standards/practice? (primary sources: RFCs, IEEE standards, 3GPP specs)
3. **Uniqueness check**: Could a different n=6 expression produce the same value? How many expressions in the n=6 toolkit yield integers in the relevant range?
4. **Counterfactual**: Does n=28 (next perfect number) produce anything meaningful for the same domain?
5. **Grade**: EXACT / CLOSE / WEAK / FAIL

Grade definitions:
- **EXACT**: Math is correct AND the real-world value matches precisely AND the matched constant is architecturally fixed (not approximate or subjective).
- **CLOSE**: Math is correct AND the real-world value is within 5%, or exact count but plausibly coincidental.
- **WEAK**: Math is correct BUT the match is trivial (n itself, ubiquitous power of 2), cherry-picked, or the derivation path is arbitrary.
- **FAIL**: The real-world value does not match the claim, or the claim is factually wrong.

### Available n=6 expressions (the "toolkit")

To assess cherry-picking risk, we enumerate distinct values producible from pairwise operations on {n=6, sigma=12, tau=4, phi=2, sopfr=5, mu=1, J2=24, lambda=2}:

Single values: 1, 2, 4, 5, 6, 12, 24
Pairwise sums: 3, 6, 7, 8, 13, 14, 16, 17, 25, 26, 28, 29, 36
Pairwise differences: 1, 2, 3, 5, 6, 7, 8, 10, 11, 18, 19, 20, 22, 23
Products: 2, 4, 5, 6, 8, 10, 12, 20, 24, 48, 60, 72, 120, 288...
Powers of 2 from exponents: 2, 4, 8, 16, 32, 64, 128, 256, 1024, 2048, 4096, 65536...

**Coverage**: Through sums, differences, and 2^(expr), most integers 1-30 and most powers of 2 from 2^1 to 2^16 are reachable. This means nearly any small integer or power-of-2 constant in networking can be "derived."

---

## H-NP-1: IPv6 Address Length = 2^(sigma-sopfr) = 128 bits

**Math check**: sigma(6)=12, sopfr(6)=5, 12-5=7, 2^7=128. Correct.

**Fact check**: IPv6 addresses are 128 bits per RFC 8200 (superseding RFC 2460). Exact match.

**Uniqueness check**: 7 = sigma-sopfr is the only natural pairwise difference from the core functions that yields 7. However, 128 = 2^7 is a standard power of 2. The choice of sigma minus sopfr (rather than, say, sigma minus sopfr as a motivated operation) has no structural justification beyond producing 7.

**Counterfactual**: n=28: sigma(28)=56, sopfr(28)=9, difference=47. 2^47 = 140 trillion -- no protocol uses this. The n=6 derivation is unique among perfect numbers.

**Commentary**: The match is exact and non-trivial (128 is not the most "obvious" power of 2 for address space -- 64, 256, and 32 were all candidates during IPv6 design). The derivation path sigma-sopfr is reused for OSI layers (H-NP-7), which is a consistency point but also a sign that this particular difference was noticed and then applied to multiple targets.

**Grade: EXACT**

---

## H-NP-2: TCP 6 Control Flags = n

**Math check**: n=6. Trivially correct.

**Fact check**: RFC 793 (1981) defines exactly 6 control bits: URG, ACK, PSH, RST, SYN, FIN. This is correct. However, RFC 3168 (2001) added ECE and CWR, and RFC 3540 (2003) added NS, bringing the total to 9 in the modern TCP header. The claim selectively counts the "original" set.

**Uniqueness check**: Matching n itself is the lowest-effort match possible. Any protocol feature with count 6 trivially matches.

**Counterfactual**: n=28 control flags would be absurd for a transport header.

**Commentary**: The divisor-to-flag mapping ({1,2,3,6} mapped to SYN/ACK/FIN/RST) is decorative -- there is no structural reason why SYN corresponds to divisor 1 rather than divisor 3. The modern TCP header has 9 flags, not 6. The 2^6=64 flag combination count is numerically correct but all TCP flag sets produce 2^k combinations for k flags; this is not specific to n=6.

**Grade: WEAK** (trivial n-match; modern count is 9, not 6)

---

## H-NP-3: WiFi 6 = Generation n

**Math check**: n=6. Trivially correct.

**Fact check**: WiFi Alliance designated 802.11ax as "WiFi 6" in October 2018. Correct. This naming scheme was retroactively applied: previous generations were 802.11a/b/g/n/ac (not numbered 1-5 by IEEE). The "WiFi 6" name is a marketing decision by the WiFi Alliance, not an IEEE technical constant.

**Uniqueness check**: n=6 itself. Trivial.

**Subsidiary claims check**:
- MU-MIMO 8 streams = sigma-tau: WiFi 6 supports up to 8 MU-MIMO streams. The actual count is configurable (1, 2, 4, or 8). 8 is the *maximum*, not a fixed constant.
- 1024-QAM = 2^10 = 2^(sigma-phi): 1024-QAM was introduced in WiFi 6. However, 1024 = 2^10 is simply the next power of 2 after 512 (2^9 in 802.11ac Wave 2). QAM orders follow powers of 2 by definition.
- WiFi 6E at 6 GHz: The 6 GHz band is defined by radio spectrum allocation, not protocol design.

**Grade: WEAK** (marketing name, not engineering constant; subsidiary claims are post-hoc fits to powers of 2)

---

## H-NP-4: 5G NR tau(6)=4 Optimization Dimensions

**Math check**: tau(6)=4. Correct.

**Fact check**: The claim of "4 optimization dimensions" is a selective framing. ITU-R M.2083 (IMT-2020 vision) defines **8 key capabilities**: peak data rate, user experienced data rate, spectrum efficiency, mobility, latency, connection density, network energy efficiency, area traffic capacity. 3GPP TS 22.261 identifies **3 primary service types**: eMBB, URLLC, mMTC. The "4 dimensions" (Speed/Latency/Density/Reliability) requires cherry-picking from the 8 KPIs and ignoring the other 4.

**Network slicing claim**: 3GPP defines 3 standardized slice/service types (SST) in TS 23.501: eMBB (SST=1), URLLC (SST=2), mMTC (SST=3). V2X is not a separate standardized SST -- it uses URLLC. So the count is 3, not 4.

**Density 10^6 claim**: The target of 10^6 devices/km^2 is an ITU KPI. The exponent 6 is a coincidence of the metric system (it could equally be expressed as 1 device/m^2).

**Counterfactual**: tau(28)=6. Under this framework, 5G should have 6 optimization dimensions -- which is actually closer to the real ITU count of 8 than tau(6)=4 is.

**Grade: FAIL** (ITU defines 8 KPIs, not 4; 3GPP defines 3 slice types, not 4; the claimed count requires cherry-picking)

---

## H-NP-5: DNS Root Servers = sigma(6)+mu(6) = 13

**Math check**: sigma(6)=12, mu(6)=1, sum=13. Correct.

**Fact check**: There are exactly 13 DNS root server identities (A through M), maintained by 12 organizations. This has been unchanged since 1997. The 13 root server identities are a hard architectural constraint: the 512-byte UDP response limit (pre-EDNS) allowed exactly 13 NS records with glue A records in a priming response.

**Uniqueness check**: 13 = sigma+mu is a clean expression. No other simple pairwise operation on core functions yields 13. However, 13 is also 6+7, sopfr+sigma-tau, etc. -- many paths exist when you allow three-term combinations.

**Counterfactual**: n=28: sigma(28)+mu(28) = 56+(-1) = 55. Meaningless for DNS.

**Subsidiary claims**: The "majority = 7 = sigma-sopfr" claim is numerically correct (ceil(13/2) = 7) but DNS root servers do not use majority voting. They are independently operated Anycast clusters. This mapping has no operational meaning.

**Commentary**: This is a strong match. 13 is a non-obvious fixed architectural constant, not a round number or power of 2. The derivation 12+1 = "complete structure plus one unit of redundancy" has at least a poetic parallel to fault tolerance. The engineering explanation (512-byte UDP constraint) is the actual cause, but the coincidence is notable.

**Grade: EXACT**

---

## H-NP-6: HTTP Methods = sigma(6)-tau(6) = 8

**Math check**: sigma(6)=12, tau(6)=4, difference=8. Correct.

**Fact check**: RFC 7231 (HTTP/1.1 Semantics) defines 8 methods: GET, HEAD, POST, PUT, DELETE, CONNECT, OPTIONS, TRACE. RFC 5789 adds PATCH. The hypothesis table lists PATCH but omits CONNECT -- this is factually incorrect bookkeeping. The real RFC 7231 count is 8 (without PATCH), or 9 (with PATCH from a separate RFC).

**Uniqueness check**: 8 = sigma-tau is one of many ways to get 8 from the toolkit (also: sigma-tau, 2^3, tau*phi, etc.). The formula sigma-tau is also used for H-NP-17 (Ethernet preamble), creating a collision.

**Formula reuse problem**: Using sigma-tau=8 for both HTTP methods and Ethernet preamble bytes undermines any claim that the formula has domain-specific meaning. If sigma-tau "means" something about HTTP semantics, it should not simultaneously "mean" something about clock synchronization.

**Grade: CLOSE** (RFC 7231 count is 8, but the table contains an error swapping CONNECT for PATCH; formula reuse weakens the claim)

---

## H-NP-7: OSI 7 Layers = sigma(6)-sopfr(6)

**Math check**: sigma(6)=12, sopfr(6)=5, difference=7. Correct.

**Fact check**: ISO/IEC 7498-1 (OSI Reference Model) defines exactly 7 layers. This is a fixed architectural standard from 1984.

**Uniqueness check**: 7 = sigma-sopfr is the same derivation as H-NP-1's exponent. This is internally consistent (both use sigma-sopfr) but means the formula is doing double duty.

**Counterfactual**: n=28: sigma(28)-sopfr(28) = 56-9 = 47 layers. Meaningless.

**Subsidiary claim**: TCP/IP 4 layers = tau(6) is a genuine secondary match. The practical TCP/IP model has 4 layers (Link, Internet, Transport, Application), matching tau(6)=4.

**Commentary**: The value 7 is exact and the OSI model is a foundational standard. However, the layer-by-layer mapping (Layer 4 = tau(6), Layer 2 = phi(6), etc.) is inevitable: the integers 1 through 7 overlap heavily with n=6 function values {1, 2, 4, 5} simply by counting. The real question is whether OSI "had to" have 7 layers -- historically, the number was debated (some proposals had 5, others 8).

**Grade: EXACT** (architecturally fixed at 7; derivation is clean; same sigma-sopfr as H-NP-1 is a consistency point)

---

## H-NP-8: Ethernet MTU 1500 = 6 x 250

**Math check**: 1500/6 = 250. Correct. 250 = 2 x 5^3 = phi(6) x sopfr(6)^3. Arithmetically valid.

**Fact check**: IEEE 802.3 Ethernet MTU is 1500 bytes. Correct. Jumbo frames are 9000 bytes; 9000/1500 = 6. Correct.

**Title formula check**: The title claims 1500 = n x (sigma-tau)^(sigma-sopfr-tau-1) = 6 x 8^(7-4-1) = 6 x 8^2 = 6 x 64 = 384. **This does not equal 1500. The title formula is wrong.**

**Uniqueness check**: 1500 is divisible by 6, but also by 2, 3, 4, 5, 10, 12, 15, 20, 25, 30, 50, 60, 75, 100, 125, 150, 250, 300, 375, 500, 750. Divisibility by 6 is not selective. The decomposition phi(6) x sopfr(6)^3 is a multi-step construction that could fit many numbers.

**Historical context**: The 1500-byte MTU was chosen in 1980 by the Xerox/DEC/Intel consortium as a compromise: small enough for low latency on 10 Mbps shared media, large enough for reasonable throughput, with RAM cost constraints. The value was pragmatic, not mathematical.

**Jumbo frame 6x ratio**: 9000 = 6 x 1500 is genuinely interesting. IEEE 802.3 does not standardize jumbo frames; the 9000 value is a de facto convention. Some NICs support 9216 or 9014 bytes, not exactly 9000.

**Grade: WEAK** (title formula is mathematically wrong; divisibility by 6 is not selective; jumbo frame ratio is interesting but not standardized)

---

## H-NP-9: TCP Initial Window = sigma(6)-phi(6) = 10

**Math check**: sigma(6)=12, phi(6)=2, difference=10. Correct.

**Fact check**: RFC 6928 (2013) recommends IW=10 MSS-sized segments. The historical progression is:
- RFC 2001 (1997): IW=1
- RFC 2414 (1998): IW=2 (experimental)
- RFC 3390 (2002): IW = min(4*MSS, max(2*MSS, 4380)) -- effectively IW=2 for large MSS, IW=3 for MSS 1096-2190, IW=4 for MSS<=1095
- RFC 6928 (2013): IW=10

**Omission**: The hypothesis maps 1->2->4->10 to mu->phi->tau->(sigma-phi), but RFC 3390 also allowed IW=3 (for certain MSS values). The sequence is really 1, 2, 3-or-4, 10 -- the "3" breaks the clean mapping.

**Uniqueness check**: 10 = sigma-phi is one way. Also 10 = n+tau, sopfr+sopfr, 2*sopfr, etc. Multiple paths to 10 exist.

**Commentary**: The current standard value of 10 is exact. The 4-step "convergence" narrative (1->2->4->10) is the most compelling structural claim in the document, but it requires suppressing the IW=3 case. The value 10 is also simply a "round number" that Google found empirically optimal through large-scale A/B testing.

**Grade: CLOSE** (10 matches exactly; progression narrative requires omitting IW=3; value is empirically determined, not structurally mandated)

---

## H-NP-10: BGP AS Path Length = tau(6) = 4

**Math check**: tau(6)=4. Correct.

**Fact check**: Average AS path length measurements (APNIC, RIPE RIS, RouteViews):
- 2015-2020 range: ~3.8-4.3 hops
- 2023-2025 range: ~3.5-4.0 hops (declining due to IXP growth)
The claim of "approximately 4" is within range but approximate.

**Peering claim check**: "Optimal 6 peering relationships per AS" -- this is unsupported. CAIDA data shows massive variation: stub ASes may peer with 1-3 providers, while large transit networks peer with hundreds or thousands. There is no empirical evidence for 6 being optimal.

**Tier-1 ISP claim**: The hypothesis claims ~12-16 Tier-1 ISPs near sigma(6)=12. CAIDA's AS ranking consistently identifies ~15-17 Tier-1 networks. The range 12-16 is stated broadly enough to capture this, but sigma(6)=12 is at the low end.

**Counterfactual**: tau(28)=6. An AS path length of ~6 was actually closer to reality in the early 2000s. The declining trend toward ~3.5 actually moves *away* from tau(6)=4.

**Grade: CLOSE** (approximate match to ~4; peering claim is unsupported; trend is moving below 4)

---

## H-NP-11: QUIC Multiplexed Streams = J_2(6) = 24

**Math check**: J_2(6) = 6^2 x prod(1-1/p^2) for p|6 = 36 x (3/4)(8/9) = 36 x 24/36 = 24. Correct.

**Fact check**: QUIC (RFC 9000) does not define a fixed stream concurrency. The initial_max_bidi_streams and initial_max_uni_streams transport parameters are negotiated per connection.
- Chromium default: initial_max_bidi_streams = 100
- Firefox (Neqo): default 100
- nginx quic module: default 256
- Cloudflare quiche: default 100

The claim that "effective active streams are ~24" has no empirical basis in published measurements. HTTP Archive data (2024-2025) shows median page loads request 60-80 resources, not 20-30 as claimed.

**Grade: FAIL** (no standard, implementation default, or empirical measurement supports 24; actual defaults are 100-256; page resource counts are 60-80)

---

## H-NP-12: TLS Handshake = phi(6) = 2 RTT

**Math check**: phi(6)=2. Correct.

**Fact check**: TLS 1.2 full handshake requires 2 round trips. TLS 1.3 full handshake requires 1 round trip. TLS 1.3 0-RTT resumption requires 0 round trips. All correct.

**Uniqueness check**: phi(6)=2 = lambda(6)=2. The value 2 is the most common small integer in engineering. Any protocol involving a challenge-response pattern uses 2 RTTs (SSH, DTLS 1.2, IPsec IKEv1 phase 1, etc.). Matching 2 to phi(6) is trivially easy.

**Commentary**: The progression 2->1->0 mapped to phi->mu->0 assigns three different n=6 functions to three consecutive protocol versions, then runs out of functions and maps to 0 directly. The post-quantum TLS prediction (revert to 2-RTT) is speculative: ML-KEM handshakes in TLS 1.3 still complete in 1-RTT despite larger key sizes.

**Grade: WEAK** (2 is too common a value; any challenge-response protocol matches phi(6); progression mapping is ad hoc)

---

## H-NP-13: TCP State Machine = sigma(6)-mu(6) = 11 States

**Math check**: sigma(6)=12, mu(6)=1, difference=11. Correct.

**Fact check**: RFC 793 TCP state diagram defines exactly 11 states: CLOSED, LISTEN, SYN-SENT, SYN-RECEIVED, ESTABLISHED, FIN-WAIT-1, FIN-WAIT-2, CLOSE-WAIT, CLOSING, LAST-ACK, TIME-WAIT. Exact match. This count has not changed since 1981.

**Uniqueness check**: 11 = sigma-mu is the only simple pairwise difference yielding 11. The value 11 is a prime, making it harder to reach through multiplication. This is one of the more selective matches.

**Counterfactual**: n=28: sigma(28)-mu(28) = 56-(-1) = 57 states. Meaningless.

**Formula reuse**: sigma-mu=11 is also the exponent for H-NP-16 (RSA-2048 = 2^11). Using the same formula for TCP states and RSA key size is a consistency concern -- but at least the raw value (11) and derived value (2^11) are distinguished by the 2^x operation.

**Commentary**: This is the strongest match in the document. 11 is not a round number, not a power of 2, and not a trivially common count. The TCP state machine is a precisely defined, architecturally fixed constant from the most important transport protocol. The derivation 12-1 = "complete structure minus the base unit" has an intuitive interpretation: 12 represents total structural capacity, minus 1 for the degenerate/null state, leaving 11 active states.

**Grade: EXACT** (precise, non-obvious, architecturally fixed; strongest hypothesis)

---

## H-NP-14: Port Number Space = 2^(sigma+tau) = 2^16 = 65536

**Math check**: sigma(6)+tau(6) = 12+4 = 16, 2^16 = 65536. Correct.

**Fact check**: TCP and UDP port fields are 16-bit unsigned integers (0-65535). 65536 port values total. Correct.

**Uniqueness check**: 16-bit fields are the most common word size in networking and computing. The value 16 can be reached many ways: sigma+tau=16, 2*sigma-sigma=12+4, 4*tau, etc. TCP/UDP ports being 16-bit is part of a broader pattern: Ethernet type fields (16-bit), IP identification (16-bit), TCP window (16-bit), UDP length (16-bit), etc. ALL of these would "match" sigma+tau=16.

**Well-known port claim**: 0-1023 = 1024 = 2^10 = 2^(sigma-phi). Arithmetically correct, but 1024 = 2^10 is a ubiquitous computing boundary (1 KB).

**Commentary**: Matching any 16-bit field to sigma+tau is not impressive because 16-bit is the default field width in networking protocol headers. This is less about n=6 and more about 16-bit computing being standard.

**Grade: WEAK** (16-bit fields are ubiquitous; sigma+tau=16 is unimpressive when everything in networking is 16-bit)

---

## H-NP-15: HTTP Status Code Classes = sopfr(6) = 5

**Math check**: sopfr(6) = 2+3 = 5. Correct.

**Fact check**: HTTP defines exactly 5 status code classes: 1xx (Informational), 2xx (Success), 3xx (Redirection), 4xx (Client Error), 5xx (Server Error). This is per RFC 9110 (current) and unchanged since HTTP/1.0. Exact match.

**Uniqueness check**: 5 = sopfr(6) = 2+3. Also 5 = sopfr, tau+mu, n-mu, etc. The number 5 is extremely common in categorization systems (5-point Likert scales, 5 severity levels, 5 DEFCON levels, etc.). Human designers frequently partition things into 5 categories.

**Counterfactual**: sopfr(28) = 2+7 = 9. 9 HTTP status classes would be excessive; 5 is appropriate. But this tells us more about human categorization tendencies than about n=6.

**Commentary**: The match is exact and the constant is fixed. However, 5-category systems are so common in human design that this match carries low information content. The subsidiary claim that the decomposition 5 = 2+3 maps to "client-side (2,4) vs. server-side (3,5)" categories is a forced pattern -- 1xx (Informational) does not fit either side cleanly.

**Grade: CLOSE** (exact value match, but 5 categories is a human design tendency, not a structural necessity; low information content)

---

## H-NP-16: RSA Minimum Key Size = 2^(sigma-mu) = 2^11 = 2048 bits

**Math check**: sigma(6)-mu(6) = 12-1 = 11, 2^11 = 2048. Correct.

**Fact check**: NIST SP 800-57 Part 1 Rev. 5 (2020) specifies RSA-2048 as the minimum key size for key establishment through 2030. CA/Browser Forum Baseline Requirements mandate RSA-2048 minimum for TLS certificates. Exact match.

**Uniqueness check**: 11 = sigma-mu is the same expression as H-NP-13. RSA key sizes are conventionally powers of 2 (512, 1024, 2048, 3072, 4096). The only question is which power. 2^11 happens to be the current minimum, but this is a moving target: RSA-1024 (2^10) was the minimum until ~2013; RSA-3072 or RSA-4096 may become the minimum as quantum computing advances.

**Time-sensitivity**: The "match" to the current standard is epoch-dependent. In 2010, the match would have been 2^10 = sigma-phi. By 2035, it might be 2^12 = sigma. The hypothesis implicitly assumes the current era is "canonical," which is unjustified.

**Subsidiary progression**: 512=2^9, 1024=2^10, 2048=2^11, 4096=2^12 maps to exponents 9, 10, 11, 12. The claim that these correspond to n=6 expressions requires mapping: 9 = "non-standard" (as the document admits), 10 = sigma-phi, 11 = sigma-mu, 12 = sigma. The first term (9) already breaks the pattern.

**Grade: CLOSE** (current standard matches 2^11 exactly, but RSA key sizes are inherently powers of 2, and the minimum is a moving target; formula reuse with H-NP-13)

---

## H-NP-17: Ethernet Frame Preamble = sigma(6)-tau(6) = 8 Bytes

**Math check**: sigma(6)-tau(6) = 12-4 = 8. Correct.

**Fact check**: Ethernet preamble is 7 bytes of 10101010 pattern + 1 byte SFD (10101011) = 8 bytes total. Per IEEE 802.3. Correct. MAC addresses are 6 bytes (EUI-48). Correct. Minimum frame size is 64 bytes. Correct.

**Formula reuse**: sigma-tau=8 is the same formula as H-NP-6 (HTTP methods). This is the most direct formula collision in the hypothesis set. Two unrelated constants (HTTP method count, Ethernet preamble bytes) are "derived" from the same formula, which means at least one derivation is meaningless.

**Subsidiary matches**:
- MAC address = 6 bytes = n: Genuine and well-known. EUI-48 is fixed.
- Minimum frame 64 bytes = 2^6 = 2^n: The minimum frame size of 64 bytes exists to ensure collision detection on 10 Mbps Ethernet with maximum cable length. 64 = 2^6 is exact.
- EUI-64 = 8 bytes = sigma-tau: EUI-64 is used in IPv6 interface identifiers.

**Commentary**: The preamble is 8 bytes for clock synchronization -- the value is determined by the number of bit transitions needed for PLL lock at 10 Mbps. The MAC=6 bytes and min frame=2^6=64 are the genuinely interesting matches here and arguably stronger than the preamble claim.

**Grade: CLOSE** (individual values match, but sigma-tau=8 is reused from H-NP-6; MAC=6 and frame=2^6 are the real highlights)

---

## H-NP-18: Browser Concurrent Connections = n = 6

**Math check**: n=6. Trivially correct.

**Fact check**: All major browsers default to 6 concurrent connections per origin for HTTP/1.1:
- Chromium: `kMaxSocketsPerGroup = 6` (net/socket/client_socket_pool.cc)
- Firefox: `network.http.max-persistent-connections-per-server = 6`
- Safari: 6 per origin

This is well-documented and has been stable since ~2008.

**Historical context**: RFC 2616 (1999) recommended max 2 persistent connections per server. Browsers experimented with higher values. Opera tried 8, IE tried 4, and eventually all converged on 6 through empirical testing. Google's research showed 6 as the sweet spot balancing parallelism against TCP congestion and server load.

**Uniqueness check**: n=6 itself. Trivial derivation. Any protocol feature equaling 6 matches.

**Commentary**: The empirical convergence on 6 is genuinely interesting -- multiple independent engineering teams found the same optimum. However, the n=6 "derivation" is non-existent; the number IS n. The hypothesis does not explain WHY 6 is optimal through number theory; it merely observes the coincidence. The claim about 1/2+1/3+1/6 bandwidth distribution has no empirical measurement to support it.

**Grade: WEAK** (real engineering constant determined empirically, but derivation is trivially n itself)

---

## Revised Summary Table

| ID | Hypothesis | Claimed Value | Real Value | Math OK | Grade | Change |
|----|-----------|--------------|------------|---------|-------|--------|
| H-NP-1 | IPv6 address bits | 128 | 128 (RFC 8200) | Yes | **EXACT** | -- |
| H-NP-2 | TCP control flags | 6 | 6 (RFC 793) / 9 (modern) | Yes | **WEAK** | -- |
| H-NP-3 | WiFi generation | 6 | 6 (marketing name) | Yes | **WEAK** | -- |
| H-NP-4 | 5G optimization dims | 4 | 8 (ITU M.2083) / 3 (3GPP slices) | Yes | **FAIL** | was WEAK |
| H-NP-5 | DNS root servers | 13 | 13 | Yes | **EXACT** | -- |
| H-NP-6 | HTTP methods | 8 | 8 (RFC 7231) / 9 (with PATCH) | Yes | **CLOSE** | -- |
| H-NP-7 | OSI layers | 7 | 7 (ISO 7498) | Yes | **EXACT** | -- |
| H-NP-8 | Ethernet MTU | 1500 | 1500 | Yes | **WEAK** | -- |
| H-NP-9 | TCP initial window | 10 | 10 (RFC 6928) | Yes | **CLOSE** | -- |
| H-NP-10 | BGP AS path length | 4 | ~3.5-4.2 | Yes | **CLOSE** | -- |
| H-NP-11 | QUIC streams | 24 | defaults 100-256 | Yes | **FAIL** | was UNVERIFIABLE |
| H-NP-12 | TLS handshake RTT | 2 | 2 (TLS 1.2 only) | Yes | **WEAK** | -- |
| H-NP-13 | TCP states | 11 | 11 (RFC 793) | Yes | **EXACT** | -- |
| H-NP-14 | Port number space | 65536 | 65536 | Yes | **WEAK** | -- |
| H-NP-15 | HTTP status classes | 5 | 5 | Yes | **CLOSE** | was EXACT |
| H-NP-16 | RSA min key size | 2048 | 2048 (NIST current) | Yes | **CLOSE** | was EXACT |
| H-NP-17 | Ethernet preamble | 8 bytes | 8 bytes | Yes | **CLOSE** | -- |
| H-NP-18 | Browser connections | 6 | 6 | Yes | **WEAK** | -- |

## Revised Score Distribution

| Grade | Count | Hypotheses |
|-------|-------|-----------|
| EXACT | 4 | H-NP-1, H-NP-5, H-NP-7, H-NP-13 |
| CLOSE | 6 | H-NP-6, H-NP-9, H-NP-10, H-NP-15, H-NP-16, H-NP-17 |
| WEAK | 6 | H-NP-2, H-NP-3, H-NP-8, H-NP-12, H-NP-14, H-NP-18 |
| FAIL | 2 | H-NP-4, H-NP-11 |

## Changes from Previous Verification

| ID | Old Grade | New Grade | Reason for Change |
|----|-----------|-----------|-------------------|
| H-NP-4 | WEAK | **FAIL** | ITU defines 8 KPIs, not 4; 3GPP defines 3 slice types, not 4. The claimed count is wrong. |
| H-NP-11 | UNVERIFIABLE | **FAIL** | Implementation defaults (100-256) and HTTP Archive data (60-80 resources) directly contradict the claim of 24. |
| H-NP-15 | EXACT | **CLOSE** | 5-category systems are a human cognitive tendency (Likert scales, severity levels), not a structural necessity. Low information content. |
| H-NP-16 | EXACT | **CLOSE** | RSA key sizes are inherently powers of 2; the minimum is era-dependent (was 1024, now 2048, will be 3072+). Matching the current value is time-sensitive. |

## Overall Assessment

**4 of 18 hypotheses receive EXACT grades** (down from 6), with 2 new FAILs.

### Strongest matches (EXACT):
1. **H-NP-13 (TCP 11 states)**: sigma-mu=11. Non-obvious value, fixed since 1981, prime number, architecturally constrained. Best hypothesis in the set.
2. **H-NP-5 (DNS 13 root servers)**: sigma+mu=13. Non-obvious value, fixed since 1997, physically constrained by UDP packet size.
3. **H-NP-7 (OSI 7 layers)**: sigma-sopfr=7. Fixed standard, though 7 is somewhat "round."
4. **H-NP-1 (IPv6 128 bits)**: 2^(sigma-sopfr)=128. Clean derivation, important constant.

### Systemic issues:
1. **Cherry-picking**: The n=6 expression toolkit covers most integers 1-24 and most powers of 2 up to 2^16. Finding a match is not remarkable; failing to find one would be.
2. **Formula reuse**: sigma-tau=8 maps to both HTTP methods and Ethernet preamble. sigma-mu=11 maps to both TCP states and RSA key exponent. This undermines domain-specific meaning.
3. **Trivial matches**: 6 hypotheses use n=6 directly or match ubiquitous 16-bit/power-of-2 values. These carry zero information.
4. **No predictions**: Every derivation was constructed after knowing the answer. A genuine test would predict a currently unknown network constant.
5. **Honest base rate**: With ~30 reachable integers and ~50 well-known networking constants, finding 4-6 exact matches is expected by chance alone (~10-12% hit rate on ~50 targets).

**Bottom line**: TCP 11 states and DNS 13 root servers are genuinely striking coincidences. The remaining matches range from plausible-but-unremarkable to wrong. The framework's explanatory power is limited by its flexibility: too many formulas, too many targets, no mechanism for selecting which formula applies where.
