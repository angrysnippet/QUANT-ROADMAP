# Day 90 — 🚧🧠 Quant Mind World · Grid DP with Obstacles — Handling Invalid States

> Yesterday: Unique Paths, Minimum Path Sum. Today a crucial idea: **not every state is reachable.** Real systems rarely let you use every state — there are obstacles, invalid states, constraints.

**Focus:** Obstacles · Invalid States · Reachability · Constraints · Feasible Solutions

---

## Previous Day Review (10 min)
- State what `dp[i][j]` means in Minimum Path Sum.
- Recall why greedy fails on grids.

---

## Why today matters
```
S . .
. X .
. . E      (X = obstacle)
```
The robot can't pass through `X` — so some paths simply **don't exist.**

---

## Block 1 — C++ (State stays the same)
`dp[i][j]` = number of ways to reach `(i,j)` — **the state meaning doesn't change; only the transitions do.**

*Why it matters:* learning to *exclude* invalid states (not just compute valid ones) is what separates textbook DP from real, constrained problems.

**Code from scratch:** `unique_paths_obstacles.cpp`; store `0 = free`, `1 = obstacle`.

---

## Block 2 — DSA (Unique Paths with obstacles)
Normal transition `dp[i][j] = dp[i-1][j] + dp[i][j-1]`, **but an obstacle cell is unreachable**, so `dp[i][j] = 0` there. For the grid above:
```
1 1 1
1 0 1
1 1 2   → 2 paths
```

**Task:** fill the table by hand.

---

## Block 3 — DSA (Invalid states)
Some states are simply **impossible**: a grid obstacle, a negative Knapsack capacity, an out-of-range string index, a disconnected graph node. DP isn't *compute everything* — it's *compute valid states.*

**Thinking question:** why is `dp[i][j] = 0` the right value for an obstacle cell? (Zero ways pass through it, so it contributes nothing to its neighbours.)

---

## Block 4 — Mathematics (Feasible vs infeasible)
```
S X
X E
```
The destination can't be reached → **0 paths.**

**Exercise:** compute the path grid for `1 0 / 0 1` (1 = obstacle). **Challenge:** why is *detecting impossibility* often as important as finding a solution?

---

## Block 5 — Quant Thinking (Constraints change everything)

You want Stock A, but regulations forbid it → that state is **invalid.** A research plan needing ML before Probability is impossible → invalid.

**Problems:** portfolio restrictions; resource constraints; scheduling constraints.

**Hard puzzle:** 20 hours but some study blocks are unavailable — how does that change the optimization? Think obstacles.

**Career connection:** professional optimization spends enormous effort on **constraint handling** — the best solution is useless if it's infeasible. *Feasibility comes before optimization.*

---

## Block 6 — Python · Student Management System v73
Create `grid_obstacles.py`: grid `[[0,0,0],[0,1,0],[0,0,0]]`; `if obstacle: dp[i][j] = 0`. Bonus: print "Reachable / Not Reachable" for the destination.

**Linux:** learn `mv`; practice `mv old.txt new.txt`, `mv file.txt folder/`. **Question:** why keep files in valid locations? Think state validity.

---

## Quant Thinking Track — Not Every Option Is Available
Weak thinkers assume all choices exist; strong thinkers first ask *what constraints exist?* — money (budget), time (hours), computing (memory), finance (risk limits). Constraints define reality; optimization happens *inside* them.

---

## Portfolio Building
`DP/grid/`:
- `unique_paths_obstacles.cpp`
- `feasibility_check.cpp`

---

## Communication Exercise
In 5 lines, explain: *"Why must an obstacle cell contribute 0 to the DP?"*

---

## Journal
- Paths in the obstacle grid, and why?
- When is detecting impossibility the real goal?
- What constraint invalidates a state in my own plans?

---

## 🚩 Day 90 Milestone
You're done when you can answer **what an invalid state is in DP** — not "a bug", but *a state that can't legally occur under the problem's constraints, so it must contribute nothing (or be ignored).*

---

## Next 🚀
Day 91 — **Grid DP Pattern Recognition Checkpoint:** unify Unique Paths, Minimum Path Sum, and obstacle grids into one framework.

---

## Tracker Update (after Day 90)
- DP: grid DP → **60%**, constraint handling → **35%**, state validation → **40%**
- Feasibility analysis → **60%**
- Constraint modeling → **55%**, optimization under limits → **65%**
- Python grid-DP implementation → **60%**
