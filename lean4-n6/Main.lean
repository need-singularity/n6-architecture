-- Main.lean : N6 Lean4 skeleton 실행
import N6.Basic
import N6.Verification

def main : IO Unit := do
  IO.println "=== N6 Lean4 skeleton v3 M3 ==="
  IO.println s!"σ(6) = {N6.sigma 6}"
  IO.println s!"φ(6) = {N6.phi 6}"
  IO.println s!"τ(6) = {N6.tau 6}"
  IO.println s!"σ(6)·φ(6) = {N6.sigma 6 * N6.phi 6}"
  IO.println s!"6·τ(6)    = {6 * N6.tau 6}"
  IO.println ""
  IO.println "=== Theorem B bounded verification (v3 loop 19) ==="
  IO.println s!"n ∈ [2, 20] 중 σ·φ = n·τ 만족: {N6.theoremBHitsSmall}"
  IO.println "→ 유일하게 n = 6 (Lean4 kernel decide 확인)"
  IO.println "→ Theorem B full proof 는 sorry 유지 (Mathlib v4)"
