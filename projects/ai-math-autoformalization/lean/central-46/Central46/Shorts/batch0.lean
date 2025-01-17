import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring
import Mathlib.Data.Real.Basic
import Mathlib.Tactic.Explode

namespace Example1
def h : 1 + 1 = 2 := sorry
end Example1

namespace Example2
variable (x : ℝ)

def h1 : x ^ 2 ≥ 0 := sorry
end Example2

namespace Example3
variable (x1 : ℝ)

def h2 : x1 ^ 2 + 1 ≥ 2 * x1 := sorry
end Example3
