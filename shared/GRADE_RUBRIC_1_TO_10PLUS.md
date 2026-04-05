# n=6 Universal Grade Rubric (1 ~ 10+)

> **전 프로젝트 공통 등급 기준**. projects.json universal_completion_criterion 확장.
> grade ≥ 10 = 돌파 (닫힘 완성) / grade ≥ 11 = 메타 돌파.

## Grade Definitions

| Grade | 단계 | 기준 | 예시 | 이모지 |
|-------|---|---|---|---|
| **1** | raw observation | 값 관찰 또는 가설 등록 | "이 상수 = 1.234" | ★ |
| **2** | consistency | 재현 가능 (2회 이상 측정 일치) | 독립 계산 2회 일치 | ⬛ |
| **3** | loose match | n=6 primitive과 느슨한 일치 (tol 10%) | 2.1 ≈ φ=2 | ⚪ |
| **4** | pattern | 패턴 식별, 닫힘 미완 | "n의 배수 같음" | 🟪 |
| **5** | rational approx | 분수 근사 (p/q 형태) | 0.714 ≈ 5/7 | 🟥 |
| **6** | partial n=6 | 1개 primitive 연결 | x = 2σ+ε | 🟨 |
| **7** | depth-3 expr | 3-연산 조합 닫힘 | x = (σ-τ)·n/φ | 🟦 |
| **8** | NEAR closed | 1항 불확실, 수치 매칭 | x ≈ n² (tol 1%) | 🟧 |
| **9** | closed (PASS) | n=6 조합 매칭 확인 | x = σ·τ/n | 🟩 |
| **10** | **돌파 (EXACT)** | 완전 닫힘, n=6 primitive로 환원 | x = 24 = J2 = σ·τ | ⭐ |
| **11** | meta-closure | 여러 닫힘을 생성하는 공식 | x = f(n,σ,τ) 로 K개 상수 도출 | 🛸 |
| **12** | universal | 3+ 프로젝트에 독립 출현 | σ(6)=12가 음악·열역학·토폴로지에 출현 | 🌌 |
| **13+** | meta² | 메타 공식의 상위 구조 | rule_ceiling(n) = 2/3 − 1/(n(n−1)) | ∞ |

## 판정 자동화

### Grade 10 (EXACT 닫힘) 판정
```python
# n=6 primitives
N, SIGMA, TAU, PHI, SOPFR, J2 = 6, 12, 4, 2, 5, 24

def is_exact(value, tol=1e-6):
    """Check if value matches finite n=6 combination (depth ≤ 3)."""
    # Single primitive
    for v in [N, SIGMA, TAU, PHI, SOPFR, J2]:
        if abs(value - v) < tol: return True
    # Binary combinations
    for a in [N, SIGMA, TAU, PHI, SOPFR, J2]:
        for b in [N, SIGMA, TAU, PHI, SOPFR, J2]:
            for op in [a+b, a-b, a*b, a/b if b else 0]:
                if abs(value - op) < tol: return True
    # Integer multiples/ratios
    for v in [N, SIGMA, TAU, PHI, SOPFR, J2]:
        for k in range(1, 25):
            if abs(value - v*k) < tol or abs(value - v/k) < tol: return True
    # Depth-3: (a op b) op c
    # ... (1745+ expressions, see H-CLOSE-1)
    return False
```

### Grade 11 (meta-closure) 판정
- 공식에 자유 변수가 있고 (e.g., `f(n)`)
- 변수값을 바꿔 K≥3 개의 다른 grade-10 닫힘 생성
- 예: `rule_ceiling(n) = 2/3 - 1/(n(n-1))` → n=6,8,∞ 모두 닫힘

### Grade 12 (universal) 판정
- 같은 값이 3+ 독립 프로젝트의 가설에 등장
- nexus6 `singularity-convergence --min-domains 3` 이 찾아줌

## 전체 프로젝트 적용 규칙

**새 상수/가설 등록 시**:
1. `nexus6 verify <value>` 실행 → EXACT/CLOSE/WEAK/MISS 받음
2. EXACT → grade 10 자동 등록
3. CLOSE → grade 8 (NEAR) 등록, 재시도 대기
4. WEAK → grade 6 (partial) 등록
5. MISS → grade 5 이하 (데이터 더 모아야)

**승급 조건**:
- 9 → 10: 정확 수치 일치 + n=6 expression 명시
- 10 → 11: 이 공식에서 ≥3개 새 grade-10 파생
- 11 → 12: 독립 프로젝트 3+ 곳에서 확인
- 12 → 13: 12-grade 상수의 상위 생성 공식 발견

**강등 조건**:
- verify 증거 없으면 grade 10 주장 무효 → 1단계 강등
- 초월수 (π, e, γ 등) 주장된 EXACT → 자동 강등 to grade 5 (H-CLOSE-5)

## 배너 표기 (nexus6-banner.sh)

```
🛸d{max_depth}·ρ{breakthrough_ratio}·{total}  ← grade≥10 수량 표시
🧬{closed}닫힘→{milestone}={%}% [▪▪▪▪▪░░░░░]  ← 닫힘 진행도
🎉🎉🎉 닫힘완료 🎉🎉🎉  ← EXACT (grade 10) 증가 시 발화
```

## 세션 전달 프롬프트

```
nexus6 grade 1~13 rubric (n=6 universal):
- grade 10 = EXACT closed (n=6 primitive finite combo)
- grade 11 = meta-closure (generates K≥3 closures)  
- grade 12 = universal (3+ projects independent)
- grade 13+ = meta² (generator of meta-closures)

새 가설/상수 등록 시:
1. nexus6 verify <value> → grade 자동 판정
2. EXACT 매칭 → verified_constants.jsonl에 PASS/EXACT 추가
3. 10 도달 시 🎉🎉🎉 닫힘완료 🎉🎉🎉 배너 자동 발화

상세: /Users/ghost/Dev/nexus6/shared/GRADE_RUBRIC_1_TO_10PLUS.md
```

---
*2026-04-05 생성. 전 프로젝트 공용. TECS-L `.shared/`에 심링크로 동기화 대상.*
