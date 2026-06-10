# Day 49 — 🚀 Quant Mind World · Knapsack — The Most Famous DP Problem

> Your first truly *quant-like* DP problem. Knapsack isn't about counting — it's about **limited resources + maximum reward.** That idea appears in portfolio allocation, capital allocation, budgeting, resource scheduling, and quant optimization.

**Focus:** Knapsack · Resource Allocation · Capacity States · Optimization Under Constraints · Decision Science

---

## Previous Day Review (10 min)
- Explain why LIS uses "ending at i."
- Recall the difference between subsequence and subarray.

---

## Block 1 — C++ (0/1 Knapsack)

Bag capacity **5 kg.** Items:

| Item | Weight | Value |
|------|--------|-------|
| A | 1 | 6 |
| B | 2 | 10 |
| C | 3 | 12 |

What's the maximum value you can carry? You **can't take everything** — capacity limits you, which is what makes optimization interesting. For each item: **take** or **skip** (same philosophy as House Robber).

*Why it matters:* "maximise reward subject to a budget" is the single most common shape of real optimization — this is its canonical form.

**Code from scratch:** recursive, memoized, bottom-up Knapsack.

---

## Block 2 — DSA (State design — your first 2D DP)
**State:** `dp[i][w]` = max value using the first `i` items with capacity `w`.

**Transition** (if item `i` fits — take or skip):
```
dp[i][w] = max( dp[i-1][w], value[i] + dp[i-1][w - wt[i]] )
```

**Task:** draw the complete DP table for `weights = [1, 2]`, `values = [5, 10]`, `capacity = 3`.

---

## Block 3 — Quant Thinking (Capital allocation)

You have ₹100. Strategies:

| Strategy | Cost | Profit |
|----------|------|--------|
| A | 20 | 30 |
| B | 50 | 80 |
| C | 40 | 60 |

How should capital be allocated? **This is Knapsack.**

**Problems:**
1. Budget 10; projects cost `3 4 5`, value `30 50 60` — best selection?
2. What changes if projects can be **repeated**? (Unbounded knapsack.)

**Hard puzzle:** hire researchers, each with a cost and an expected research value — how would you allocate a fixed budget? Model it; don't solve.

**Career connection:** a quant fund allocating capital across strategies to maximise return *is* a knapsack — this problem is the bridge from DSA to real quant optimization.

---

## Block 4 — Mathematics (Optimization under constraints)
**Why is Knapsack harder than Fibonacci?** Fibonacci is one recurrence; Knapsack is *state + constraint + choice.*

**Exercise:** capacity 4, items `(1,1), (3,4), (4,5)` — find the optimum by hand. **Challenge:** if capacity doubles, does optimal value always double? Think carefully (no — it's not linear).

---

## Block 5 — Python · Student Management System v36
Add **scholarship allocation**: `budget = 100000`, students as `(cost, impact)` — which set maximises total impact? Mini-Knapsack; bonus: display the chosen students.

---

## Block 6 — Linux
**Learn:** `cat`, `less`, `more`. Practice `cat file.txt`, `less file.txt` for viewing large files.

---

## Quant Thinking Track — Resource Allocation
Most real optimization looks like **limited resource → many choices → maximum reward.** A quant fund (capital → strategies → return), research (time → projects → knowledge), student life (24h → activities → growth). Notice: **even your roadmap is a knapsack** — limited time across C++, DSA, Maths, Python, Linux for maximum future return.

---

## Portfolio Building
`DP/`:
- `knapsack_recursive.cpp`
- `knapsack_memo.cpp`
- `knapsack_tabulation.cpp`

---

## Communication Exercise
In 5 lines, explain: *"Why does Knapsack need both the item index and the remaining capacity in its state?"*

---

## Journal
- Why is a 1D state insufficient for Knapsack?
- Does doubling capacity double the value?
- What real allocation in my life is a knapsack?

---

## 🚩 Day 49 Milestone
You're done when you can answer **why Knapsack needs a 2D state**: the current item alone isn't enough, and remaining capacity alone isn't enough — you need *both.* Your first multi-dimensional state design.

---

## Next 🚀
Day 50 — **DP Checkpoint & Pattern Recognition:** connect Fibonacci, Stairs, House Robber, Coin Change, LIS, and Knapsack into one framework, and learn to *recognise the pattern before writing code.*

---

## Tracker Update (after Day 49)
- Dynamic programming → **75%**
- DSA: 2D DP → **20%**, knapsack → **25%**
- Optimization DP → **60%**
- Resource allocation → **40%**, optimization thinking → **75%**
- Python optimization modeling → **35%**
