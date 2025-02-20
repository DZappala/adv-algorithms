= Divide and Conquer

== template
Prob($A_{1~n}$)
  if $n == 1$
    return solution(Conquer)
  else
    left_solution = Prob($A_{1~floor{n/2}}$)
    right_solution = Prob($A_{floor{n/2}+1~n}$)
  return combine(left_solution, right_solution)

- divide recurrence relation
- copy and paste, then fix

