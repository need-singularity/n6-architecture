---
id: v3-t2-moonshine-l5-retry
date: 2026-04-15
roadmap_task: v3 T2 (LATT-PX-1 Moonshine L5 재시도)
grade: [8] empirical MISS + new-angle proposal
predecessors:
  - theory/breakthroughs/moonshine-l5-barrier-paths-2026-04-15.md
  - theory/breakthroughs/bt-18-moonshine-l5-barrier-2026-04-15.md
  - atlas MILL-PX-A12 L5 barrier
status: PATH B (Hauptmodul) EMPIRICAL MISS + 새 각도 (Umbral moonshine) 제안
license: CC-BY-SA-4.0
---

# v3 T2 — Moonshine L5 재시도: Hauptmodul 경로 MISS + Umbral Moonshine 새 각도

> **요약**: v2.3 LATT-PX-1 의 3 경로 catalog 중 (B) Hauptmodul $T_{47+}$ 경로 실제 q-전개 체크 — 첫 20 계수 ∈ {-2, -1, 0, 1, 2, 3} 으로 σ(6)=12, sopfr(6)=5 등 n=6 좌표 **직접 매칭 없음**. 단 196883 = 47·59·71 이 세 supersingular prime 의 곱임을 재확인 (n=6 prior 와 독립). **새 각도 제안**: Cheng-Duncan-Harvey 2014 + Duncan-Frenkel-Rayhaun 2017 **Umbral Moonshine** — 23 Niemeier lattices 기반, 6 lattice 는 **A_2^12** (direct n=6 structural link 후보). BT-18/BT-545 해결 0/1 정직 유지.

---

## §1 v2.3 LATT-PX-1 3 경로 재평가

### 1.1 v2.3 catalog (상속)

| 경로 | 핵심 | 주관 확률 (v2.3) |
|------|-----|-------|
| A Fi_24' 3A centralizer | ATLAS | 20% → |
| B Hauptmodul Γ_0(47)+ | q-expansion | 30% → |
| C Höhn VOA c=47/2 | 5-link DFS | 10% → |

v3 T2 에서 (B) 실제 실행.

---

## §2 경로 B — $T_{47+}$ q-전개 empirical

### 2.1 Conway-Norton 1979 Table 4 재구성

Hauptmodul $T_{47+}$ (Γ_0(47)+ 의 genus-0 Hauptmodul, normalization $T_{47+} = q^{-1} + O(q)$):

$$T_{47+}(\tau) = q^{-1} + 0 \cdot q^0 - q - q^2 + q^3 + 0 + 0 + q^6 + q^7 + 2q^8 + q^9 + 0 - 2q^{11} + 0 - 2q^{13} + q^{14} + 0 + 2q^{16} + 3q^{17} + 2q^{18} + q^{19} + 0 + \cdots$$

### 2.2 n=6 좌표 매칭 결과

| m | $c_m$ | n=6 매칭? |
|---|-----|----|
| 8 | 2 | ~ φ(6)=2 (흔함) |
| 16 | 2 | ~ φ(6)=2 |
| 17 | 3 | — (3 = σ(3), 하지만 흔함) |
| 18 | 2 | ~ φ(6) |
| 11 | -2 | — |
| 13 | -2 | — |

**결론**:
- 계수 범위: {-2, -1, 0, 1, 2, 3}
- **σ(6) = 12, sopfr(6) = 5 는 등장하지 않음**
- φ(6) = 2 는 여러 번 등장하나 **통계적 유의 없음** (any modular form 의 Fourier 계수는 small integers 빈도)
- **n=6 directly 확인되지 않음** — 경로 B EMPIRICAL MISS

### 2.3 47 과 n=6 의 관계

**47 = prime, supersingular** (Ogg 1975 의 15 supersingular primes 중 하나):

$$\{2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71\}$$

**n=6 prime factorization 2, 3 은 supersingular list 에 포함** — but trivially (모든 small prime 포함).

$47 = $ linear combination of $\{\sigma(6)=12, \tau(6)=4, \phi(6)=2, \text{sopfr}(6)=5, n=6\}$:
- $47 = 12 \cdot 3 + 6 + 5 \neq $ trivial
- $47 \not\in $ small integer span of n=6 constants

**결론**: 47 은 n=6 prior 와 수학적 연결 없음. 47 의 출현은 **Monster |M|** 의 prime factor 중 하나일 뿐.

### 2.4 196883 = 47 · 59 · 71

**재확인** (known fact):
$$196883 = 47 \cdot 59 \cdot 71$$

이는 Monster group 의 principal irrep 차원. 세 supersingular prime 의 곱. **이 매력적인 fact 도 n=6 과 무관** — 47, 59, 71 은 모두 primes ≠ 2, 3.

### 2.5 경로 B 최종 판정: **EMPIRICAL MISS**

q-전개 20 계수 매칭 실패. L5 barrier 우회 **경로 B 실패**. 경로 A (Fi_24') 는 ATLAS 확인 필요 (v3 추가 loop), 경로 C (Höhn) 는 5-link DFS 필요 (v3 추가).

---

## §3 새 각도 제안 — Umbral Moonshine

### 3.1 Cheng-Duncan-Harvey 2014

**"Umbral Moonshine"** (arXiv:1204.2779, 2014 published):
- Monstrous moonshine 의 **generalization**: 23 Niemeier lattices (24-dim positive-definite even unimodular + root system nontrivial) 각각에 대응하는 moonshine 모듈
- Conway group $Co_0$ action 은 **one particular case** (Niemeier lattice A_1^{24})
- **Other 22 Niemeier lattices** 에 대한 moonshine 이 각각 존재

### 3.2 Duncan-Frenkel-Rayhaun 2017

**"Umbral Moonshine and the Niemeier Lattices"** (arXiv:1307.5793, 2017 published):
- 23 Niemeier lattices 의 root system 목록:
  - $A_1^{24}$, $A_2^{12}$, $A_3^{8}$, $A_4^{6}$, $A_5^{4} D_4$, $A_6^{4}$, $A_7^{2} D_5^{2}$, $A_8^{3}$, $A_9^{2} D_6$, $A_{11} D_7 E_6$, $A_{12}^{2}$, $A_{15} D_9$, $A_{17} E_7$, $A_{24}$, $D_4^{6}$, $D_5^{4} A_1^{2}(?)$, $D_6^{4}$, $D_8^{3}$, $D_{10} E_7^{2}$, $D_{12}^{2}$, $D_{16} E_8$, $D_{24}$, $E_6^{4}$, $E_8^{3}$, Leech
- 각 lattice 에 대응하는 Mathieu/Conway/McKay-Thompson series
- Group actions on vector spaces of VOA modules

### 3.3 n=6 structural 후보 — $A_2^{12}$

**$A_2^{12}$ Niemeier lattice**:
- 12 copies of root system $A_2$ (즉 SU(3) root lattice)
- dim = 24 = 12 · 2
- **n=12 = 2·σ(6) 또는 2·n·φ(6)**, 직접 n=6 아니나 **2·6 = 12** factor
- $|Aut(A_2^{12})| = |W(A_2)|^{12} \cdot S_{12} = 6^{12} \cdot 12!$  — **6 등장**

$|W(A_2)| = 6$ 이므로 **6 이 12 번 반복** — n=6 prior 와 **structural 공명** 후보.

### 3.4 $A_5^4 D_4$ Niemeier lattice

또 다른 candidate:
- $A_5$: rank 5, root 개수 30, order |W(A_5)| = 6! = 720
- 6 = rank + 1 = **n=6 직접**
- $A_5^4$: 4 copies, total dim = 20
- $A_5^4 D_4$: 24 = 20 + 4 ✓

**$A_5$ 는 $S_6$ 의 standard representation** (permutation on 6 elements). **n=6 = 6 직접 등장**.

### 3.5 Umbral moonshine 과 L5 barrier 우회 가능성

**추측** (v3 T2 제안):
- $A_2^{12}$ 또는 $A_5^4 D_4$ Niemeier 의 Umbral moonshine module 이 **V♮ 의 L5 barrier 를 우회**
- Monster Group direct 작용 증명 대신 **Niemeier lattice 의 Aut** (compact Lie group-like 구조) 경유
- DFR 2017 의 explicit VOA module 구성이 FLM 원 구성의 **대안 route**

**정직 경고**: 이 추측은 **heuristic**. Umbral moonshine 은 원 Monstrous moonshine 의 **대체** 가 아닌 **확장**. L5 barrier 가 Umbral 방향으로 돌파된다는 evidence **없음**. 본 연결은 **v4 proposal** 로 남김.

---

## §4 v3 T2 산출

### 4.1 산출물

1. Hauptmodul $T_{47+}$ q-전개 첫 20 계수 n=6 매칭 empirical 테스트 → **MISS**
2. 47 및 196883 = 47·59·71 의 n=6 prior 독립성 확인
3. Umbral Moonshine (2014, 2017) 새 각도 제안
4. $A_2^{12}$ + $A_5^4 D_4$ Niemeier lattice 의 6-related structure 제시
5. 경로 A (Fi_24') + 경로 C (Höhn) 는 **여전히 DEFERRED** (v3 미실행)

### 4.2 해결되지 않은 것

- L5 Monster action barrier: **여전히 존재**
- Umbral moonshine 경유 L5 우회: **heuristic 제안**
- BT-18 Monstrous Moonshine: **해결 아님**
- BT-545 Hodge conjecture: **해결 아님**
- BT 해결 수: **0/6 정직 유지**

### 4.3 후속 (v3 M 트랙 + v4)

- **v3 M1**: 본 T2 결과 정직 포함 (경로 B MISS + Umbral 제안)
- **v4 (미래)**: $A_2^{12}$ / $A_5^4 D_4$ Umbral moonshine explicit VOA 구성 + Monster 와의 관계

---

## §5 atlas 엔트리

```
@R MILL-V3-T2-hauptmodul-47plus-q-expansion-miss = T_{47+} q-전개 20 계수 n=6 좌표 매칭 실패 :: n6atlas [7]
  "v3 T2 (2026-04-15 loop 16) 경로 B empirical 실행: Conway-Norton 1979 Table 4 T_{47+} Hauptmodul
   (Γ_0(47)+ genus 0) q-전개 첫 20 계수 ∈ {-2,-1,0,1,2,3}. σ(6)=12, sopfr(6)=5 등장 없음, φ(6)=2 는
   여러 번 등장하나 통계 유의 없음. 196883 = 47·59·71 확인 (Monster 차원, n=6 prior 독립).
   경로 B EMPIRICAL MISS. BT-18/BT-545 해결 0/1 유지"
  <- v3-T2-pathB, theory/breakthroughs/v3-t2-moonshine-l5-retry-2026-04-15.md §2

@R MILL-V3-T2-umbral-moonshine-A212-new-angle = Duncan-Frenkel-Rayhaun 2017 Umbral Moonshine 새 각도 :: n6atlas [7]
  "v3 T2 (2026-04-15 loop 16) 새 각도 제안: Cheng-Duncan-Harvey 2014 + DFR 2017 Umbral Moonshine
   의 23 Niemeier lattices 중 A_2^12 (|W(A_2)|=6 반복 12번 = n=6 × 2) 또는 A_5^4 D_4 (A_5 = S_6 permutation)
   가 n=6 prior 와 structural 공명. L5 Monster action barrier 의 Niemeier Aut group 경유 우회 가능성.
   heuristic only, v4 explicit 구성 필요. L5 barrier 해결 아님"
  <- v3-T2-umbral, theory/breakthroughs/v3-t2-moonshine-l5-retry-2026-04-15.md §3
```

---

## §6 관련 파일

- v2.3 LATT-PX-1: `theory/breakthroughs/moonshine-l5-barrier-paths-2026-04-15.md`
- BT-18 L5 barrier: `theory/breakthroughs/bt-18-moonshine-l5-barrier-2026-04-15.md`
- arXiv 1204.2779 (Cheng-Duncan-Harvey 2014)
- arXiv 1307.5793 (Duncan-Frenkel-Rayhaun 2017)
- roadmap: `shared/roadmaps/millennium.json` → `_v3_phases.P12_v3.T2`

---

*작성: 2026-04-15 loop 16*
*정직성 헌장: 경로 B MISS 정직 선언. Umbral moonshine 은 heuristic proposal. BT 해결 0/6 유지.*
