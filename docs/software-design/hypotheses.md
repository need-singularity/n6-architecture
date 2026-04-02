# N6 Software Design — Core Hypotheses (H-SD-01 ~ H-SD-30)

> n=6 완전수 산술이 소프트웨어 공학의 핵심 표준/파라미터를 결정한다.
> Constants: σ=12, φ=2, τ=4, J₂=24, n=6, sopfr=5, μ=1
> Derived: σ-τ=8, σ-φ=10, σ-μ=11, n/φ=3, R(6)=1
> Sources: BT-113~117, industry-patterns.md (H-ARCH 36개)

---

## H-SD-01: SOLID Principles = sopfr(6) = 5
> 객체지향 설계의 SOLID 원칙이 정확히 5개이다.

**n=6 Expression**: sopfr(6) = 2+3 = 5
**Evidence**: Robert C. Martin (2000): S(ingle Responsibility), O(pen/Closed), L(iskov Substitution), I(nterface Segregation), D(ependency Inversion). 정확히 5개. 소프트웨어 공학 교과서 표준. BT-113 #1.
**Grade**: **EXACT** — SOLID = 5 = sopfr(6), 업계 표준.

---

## H-SD-02: REST Constraints = n = 6
> REST 아키텍처의 제약 조건이 정확히 6개이다.

**n=6 Expression**: n = 6
**Evidence**: Roy Fielding (2000) 박사 논문: (1) Client-Server, (2) Stateless, (3) Cache, (4) Uniform Interface, (5) Layered System, (6) Code-on-Demand. Fielding 원문에서 6 제약을 명시. BT-113 #2.
**Grade**: **EXACT** — REST = 6 = n, Fielding 논문 확정.

---

## H-SD-03: 12-Factor App = σ(6) = 12
> 클라우드 네이티브 앱 설계의 12-Factor 방법론이 σ(6)=12개 팩터이다.

**n=6 Expression**: σ(6) = 12
**Evidence**: Adam Wiggins / Heroku (2011): I~XII 정확히 12개 팩터. Codebase, Dependencies, Config, Backing services, Build-release-run, Processes, Port binding, Concurrency, Disposability, Dev/prod parity, Logs, Admin processes. BT-113 #3.
**Grade**: **EXACT** — 12-Factor App = σ(6) = 12, 원저 확정.

---

## H-SD-04: ACID Properties = τ(6) = 4
> 데이터베이스 트랜잭션의 ACID 속성이 정확히 4개이다.

**n=6 Expression**: τ(6) = 4
**Evidence**: Haerder & Reuter (1983): Atomicity, Consistency, Isolation, Durability. 모든 관계형 DB 교과서 표준. BT-113 #7, BT-116 #1.
**Grade**: **EXACT** — ACID = 4 = τ(6), DB 기본 정의.

---

## H-SD-05: CAP Theorem Properties = n/φ = 3
> 분산 시스템 CAP 정리의 속성이 3개이다.

**n=6 Expression**: n/φ = 6/2 = 3
**Evidence**: Eric Brewer (2000) 추측, Lynch & Gilbert (2002) 증명: Consistency, Availability, Partition Tolerance. 3속성 중 최대 2(=φ)개만 동시 달성 가능. BT-113 #8, BT-116 #3.
**Grade**: **EXACT** — CAP = 3 = n/φ, 정리로 증명됨.

---

## H-SD-06: OSI Model Layers = σ - sopfr = 7
> OSI 네트워크 모델의 계층 수가 7이다.

**n=6 Expression**: σ - sopfr = 12 - 5 = 7
**Evidence**: ISO/IEC 7498-1 (1984): Physical, Data Link, Network, Transport, Session, Presentation, Application. 정확히 7 계층. BT-115 #1. 전 세계 네트워크 교육 표준.
**Grade**: **EXACT** — OSI = 7 = σ-sopfr, ISO 표준 확정.

---

## H-SD-07: TCP/IP Layers = τ(6) = 4
> TCP/IP 모델의 계층 수가 4이다.

**n=6 Expression**: τ(6) = 4
**Evidence**: RFC 1122 (1989): Link, Internet, Transport, Application. 4 계층이 TCP/IP 표준. BT-115 #2. DoD 모델로도 불림.
**Grade**: **EXACT** — TCP/IP = 4 = τ(6), RFC 확정.

---

## H-SD-08: AES-128 Block Size = 2^(σ-sopfr) = 128 bit
> AES 표준 블록 크기가 128비트이다.

**n=6 Expression**: 2^(σ-sopfr) = 2^7 = 128
**Evidence**: FIPS 197 / Rijndael (2001): 블록 크기 128비트 고정. AES-128/192/256 모두 동일 블록 크기. BT-114 #1. NIST 표준.
**Grade**: **EXACT** — AES block = 128 = 2^(σ-sopfr), FIPS 확정.

---

## H-SD-09: SHA-256 Digest = 2^(σ-τ) = 256 bit
> SHA-256의 해시 출력이 256비트이다.

**n=6 Expression**: 2^(σ-τ) = 2^8 = 256
**Evidence**: FIPS 180-4: SHA-256 digest = 256 bits. Bitcoin, TLS, 코드 서명 등 광범위 사용. BT-114 #3. σ-τ=8이 지수.
**Grade**: **EXACT** — SHA-256 = 256 = 2^(σ-τ), NIST 표준 확정.

---

## H-SD-10: RSA-2048 Key Size = 2^(σ-μ) = 2048 bit
> RSA 표준 최소 키 크기가 2048비트이다.

**n=6 Expression**: 2^(σ-μ) = 2^11 = 2048
**Evidence**: NIST SP 800-57: 2048-bit RSA가 2030년까지 최소 보안 표준. RFC 3447. BT-114 #4. σ-μ=11이 지수.
**Grade**: **EXACT** — RSA-2048 = 2^(σ-μ), NIST 권장 확정.

---

## H-SD-11: Linux Signals = τ³ = 64
> Linux 시그널 수가 64개이다.

**n=6 Expression**: τ³ = 4³ = 64
**Evidence**: POSIX signals 1~31 + real-time signals 32~64. `kill -l`로 확인. BT-115 #5. 커널 소스 코드에서 _NSIG=64.
**Grade**: **EXACT** — Linux signals = 64 = τ³, 커널 확정.

---

## H-SD-12: GoF Design Patterns = 23 = J₂ - μ
> Gang of Four 디자인 패턴이 23개이다.

**n=6 Expression**: J₂ - μ = 24 - 1 = 23
**Evidence**: Gamma et al. (1994) "Design Patterns": 정확히 23개 패턴. Creational(5), Structural(7), Behavioral(11). 23 = J₂-μ는 간결한 표현이나, 대안 표현 σ+τ+sopfr+φ+μ-1 = 23도 가능.
**Grade**: **CLOSE** — 23 = J₂-μ는 수학적으로 정확하지만, J₂에서 μ를 빼는 것은 다소 ad-hoc한 조합. 패턴 수 자체는 확정.

---

## H-SD-13: RAID Levels (0-6) = σ - sopfr = 7
> RAID 표준 레벨이 7가지(0,1,2,3,4,5,6)이다.

**n=6 Expression**: σ - sopfr = 12 - 5 = 7
**Evidence**: Patterson, Gibson, Katz (1988): RAID 0~6 = 7 레벨 정의. 산업 표준. RAID 0(stripe), 1(mirror), 2~4(parity), 5(distributed parity), 6(double parity). BT-115 #12.
**Grade**: **EXACT** — RAID 0-6 = 7 = σ-sopfr, 원논문 확정.

---

## H-SD-14: HTTP Status Code Classes = sopfr(6) = 5
> HTTP 상태 코드 클래스가 5가지이다.

**n=6 Expression**: sopfr(6) = 5
**Evidence**: RFC 9110: 1xx(Informational), 2xx(Success), 3xx(Redirection), 4xx(Client Error), 5xx(Server Error). 정확히 5 클래스. BT-113 #11.
**Grade**: **EXACT** — HTTP 5 클래스 = sopfr(6), RFC 확정.

---

## H-SD-15: HTTP Core Methods = σ - τ = 8
> HTTP 핵심 메서드가 8개이다.

**n=6 Expression**: σ - τ = 12 - 4 = 8
**Evidence**: RFC 9110: GET, HEAD, POST, PUT, DELETE, CONNECT, OPTIONS, TRACE. HTTP/1.1 정의 메서드 = 8개. PATCH(RFC 5789)를 포함하면 9. BT-113 #12.
**Grade**: **CLOSE** — RFC 9110 기준 8 = σ-τ EXACT이나, PATCH 포함 시 9. 원본 HTTP/1.1(RFC 2616) 기준 8이 정확.

---

## H-SD-16: Agile Manifesto Values = τ(6) = 4
> Agile Manifesto의 핵심 가치가 4개이다.

**n=6 Expression**: τ(6) = 4
**Evidence**: Agile Manifesto (2001): (1) Individuals and interactions, (2) Working software, (3) Customer collaboration, (4) Responding to change. 정확히 4 가치. BT-113 #4.
**Grade**: **EXACT** — Agile 4 values = τ(6), Manifesto 확정.

---

## H-SD-17: Agile Manifesto Principles = σ(6) = 12
> Agile Manifesto의 원칙이 12개이다.

**n=6 Expression**: σ(6) = 12
**Evidence**: Agile Manifesto (2001): 정확히 12 principles. BT-113 #5. 17명 서명자 중 합의로 12 원칙 확정.
**Grade**: **EXACT** — Agile 12 principles = σ(6), Manifesto 확정.

---

## H-SD-18: GoF Pattern Categories = n/φ = 3
> GoF 디자인 패턴의 분류가 3가지이다.

**n=6 Expression**: n/φ = 6/2 = 3
**Evidence**: Gamma et al. (1994): Creational, Structural, Behavioral. 3분류. MVC, MVP, MVVM 등 아키텍처 패턴도 3-tier 구조. BT-113 #6.
**Grade**: **EXACT** — GoF 3 categories = n/φ, 원서 확정.

---

## H-SD-19: Clean Architecture Layers = τ(6) = 4
> Robert C. Martin의 Clean Architecture 동심원 계층이 4개이다.

**n=6 Expression**: τ(6) = 4
**Evidence**: Martin (2017) "Clean Architecture": Entities, Use Cases, Interface Adapters, Frameworks & Drivers. 4 계층. BT-113 #10.
**Grade**: **EXACT** — Clean Architecture 4 layers = τ(6), 원서 확정.

---

## H-SD-20: GitFlow Branch Types = n = 6
> GitFlow의 표준 브랜치 유형이 6가지이다.

**n=6 Expression**: n = 6
**Evidence**: Vincent Driessen (2010): main, develop, feature/*, release/*, hotfix/*, support/*. 6 유형. BT-113 #14.
**Grade**: **CLOSE** — GitFlow 원본(Driessen 2010)은 5 유형(main, develop, feature, release, hotfix). 6번째(support)는 후속 도구에서 추가.

---

## H-SD-21: CI/CD Pipeline Standard Stages = n = 6
> 표준 CI/CD 파이프라인이 6단계이다.

**n=6 Expression**: n = 6
**Evidence**: DevOps 교과서 표준: Source → Build → Test → Package → Deploy → Monitor. 6단계. GitHub Actions, GitLab CI, Jenkins의 전형적 파이프라인 구성. BT-113 #13.
**Grade**: **CLOSE** — 6단계는 일반적이나, 기업마다 4~8단계로 변동. "표준"의 정의에 따라 달라짐. GitHub Actions 템플릿은 보통 build+test+deploy의 3단계.

---

## H-SD-22: ISO 25010 Quality Attributes = σ - τ = 8
> ISO/IEC 25010 소프트웨어 품질 모델의 특성이 8개이다.

**n=6 Expression**: σ - τ = 12 - 4 = 8
**Evidence**: ISO/IEC 25010:2011: Functional Suitability, Performance Efficiency, Compatibility, Usability, Reliability, Security, Maintainability, Portability. 정확히 8개. BT-113 #15. 단, 2023년 개정판(25010:2023)은 Product Quality에 8+1=9 특성으로 변경.
**Grade**: **EXACT** — ISO 25010:2011 기준 8 = σ-τ. (2023 개정판에서 9로 확장된 점 주의)

---

## H-SD-23: Test Pyramid Layers = n/φ = 3
> 테스트 피라미드가 3 계층이다.

**n=6 Expression**: n/φ = 3
**Evidence**: Mike Cohn (2009) "Succeeding with Agile": Unit → Integration → E2E/UI. 3 계층. 이후 Testing Trophy 등 변형이 있으나 3-tier가 기본. BT-113 #16.
**Grade**: **EXACT** — Test Pyramid 3 layers = n/φ, 원서 확정.

---

## H-SD-24: OAuth 2.0 Grant Types = τ(6) = 4
> OAuth 2.0의 기본 인증 흐름이 4가지이다.

**n=6 Expression**: τ(6) = 4
**Evidence**: RFC 6749 (2012): Authorization Code, Implicit, Resource Owner Password, Client Credentials. 4 grant types. BT-113 #17. OAuth 2.1(draft)에서 Implicit + Password 삭제 시 2(=φ)로 축소.
**Grade**: **EXACT** — OAuth 2.0: 4 grants = τ(6), RFC 확정.

---

## H-SD-25: OOP Pillars = τ(6) = 4
> 객체지향 프로그래밍의 4대 원리가 τ=4이다.

**n=6 Expression**: τ(6) = 4
**Evidence**: Encapsulation, Abstraction, Inheritance, Polymorphism. 4 pillars. 일부 교재에서 Abstraction을 제외하고 3으로 보기도 하나, 4가 가장 일반적. BT-113 #18.
**Grade**: **EXACT** — OOP 4 pillars = τ(6), 대부분 교재 기준.

---

## H-SD-26: Unix Standard File Descriptors = n/φ = 3
> Unix stdin/stdout/stderr가 3개이다.

**n=6 Expression**: n/φ = 3
**Evidence**: POSIX: fd 0(stdin), 1(stdout), 2(stderr). 모든 Unix/Linux 프로세스의 기본 I/O 스트림. BT-115 #6.
**Grade**: **EXACT** — Unix 3 fds = n/φ, POSIX 표준 확정.

---

## H-SD-27: Unix Permission Triplet = n/φ = 3, Octal = σ-τ = 8
> Unix 파일 권한: rwx 3비트, 8진수 표현.

**n=6 Expression**: n/φ = 3 (rwx), σ-τ = 8 (octal values 0-7)
**Evidence**: POSIX: r(4), w(2), x(1) = 3비트 per entity. owner/group/other = 3 entities. 각 entity 0~7(=8 values = σ-τ). BT-115 #7,#8. `chmod 755` 등.
**Grade**: **EXACT** — Unix permissions: 3 bits × 3 entities, octal 0-7, BT-115 확정.

---

## H-SD-28: DNS Root Servers = σ + μ = 13
> DNS 루트 서버가 13개이다.

**n=6 Expression**: σ + μ = 12 + 1 = 13
**Evidence**: IANA: A~M 루트 서버 = 13개 (Anycast로 물리적으로는 수백 대). BT-115 #10. 13 = σ+μ. 512바이트 UDP 패킷 크기 제한에서 유래.
**Grade**: **CLOSE** — DNS root = 13은 사실이나, σ+μ=13은 σ와 μ의 단순 합으로 다소 약한 연결. 13의 원인은 UDP 패킷 크기 제한.

---

## H-SD-29: TCP Three-Way Handshake = n/φ = 3 (+ Close = n = 6)
> TCP 연결 설정이 3-way, 종료 포함 시 6 메시지.

**n=6 Expression**: n/φ = 3 (handshake), n = 6 (full cycle: SYN, SYN-ACK, ACK + FIN, FIN-ACK, ACK)
**Evidence**: RFC 793: 3-way handshake (SYN → SYN-ACK → ACK). 4-way termination (FIN → ACK → FIN → ACK). 총 메시지 = 3+4=7이지만, piggyback 시 3+3=6. BT-115 #9는 6으로 기록.
**Grade**: **CLOSE** — 3-way handshake = n/φ는 EXACT. 전체 6 메시지는 FIN+ACK piggyback 가정 시에만 성립. 실제 4-way close 시 총 7 메시지.

---

## H-SD-30: C Primitive Types = n = 6
> C 언어의 기본 자료형이 6개이다.

**n=6 Expression**: n = 6
**Evidence**: C99: char, short, int, long, float, double = 6. 다만 _Bool, long long, long double 등 확장형 포함 시 9+. C89 기준 6은 근사적. H-ARCH-21 참조.
**Grade**: **CLOSE** — C89 기본 6타입 = n이나, 표준에 따라 변동. C99 이후 long long, _Bool 추가로 6을 초과.

---

## Summary

| Grade | Count | Hypotheses |
|-------|-------|------------|
| **EXACT** | 23 | H-SD-01,02,03,04,05,06,07,08,09,10,11,13,14,16,17,18,19,22,23,24,25,26,27 |
| **CLOSE** | 7 | H-SD-12,15,20,21,28,29,30 |
| **WEAK** | 0 | — |
| **FAIL** | 0 | — |

**EXACT rate**: 23/30 = 76.7%

**Standout**: H-SD-01(SOLID=5), H-SD-03(12-Factor=12), H-SD-08(AES-128=2^7), H-SD-09(SHA-256=2^8), H-SD-11(Linux signals=64=τ³)
**BT Coverage**: BT-113(18항), BT-114(10항), BT-115(12항), BT-116(9항) 중 핵심 30개 선별

> Note: extreme-hypotheses.md (H-SD-61~80)의 20개는 이 목록과 별도. 총 50개 가설로 소프트웨어 도메인 커버.
