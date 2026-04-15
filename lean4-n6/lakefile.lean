import Lake
open Lake DSL

package «n6» where
  -- v3 M3: n=6 Theorem B Lean4 formalization skeleton

lean_lib «N6» where
  -- σ(n) · φ(n) = n · τ(n)  iff  n = 6  (n ≥ 2)
  -- 3 independent proofs (algebraic / analytic / combinatorial)
  -- 본 라이브러리는 skeleton — 실제 증명 단계는 v4 장기 과제

@[default_target]
lean_exe «n6exe» where
  root := `Main
