# Day 47 — 🚀 Quant Mind World · Coin Change — The First Real State-Design Problem

> Yesterday: House Robber → take/skip → optimization DP. Today, a problem that teaches one of the hardest DP skills: **choosing the correct state.**

**Focus:** Coin Change · State Design · Counting DP · Order vs Combination · DP Modelling

---

## Previous Day Review (10 min)
- State the 3-question framework (state / choices / transition).
- Recall why greedy fails on House Robber.

---

## Block 1 — C++ (Coin Change)

Coins `[1, 2, 5]`, target `5`. How many ways to make 5? E.g. `1+1+1+1+1`, `1+1+1+2`, `1+2+2`, `5`.

**Crucial question:** are `1+2+2` and `2+1+2` different? **For today, no** — order doesn't matter (it's a *combination* count).

*Why it matters:* whether order matters silently changes the entire algorithm — getting the state and loop order right is the whole lesson.

**Code from scratch:** recursive, memoized, bottom-up Coin Change.

---

## Block 2 — DSA (State design)
Today's lesson: **DP is mostly about defining the state correctly.**

- **State:** `dp[i]` = ways to make amount `i`.
- **Transition:** for every coin, update `dp[current_amount]` from smaller amounts.

**Task:** build the table for coins `[1, 2]`, target `5`.

**Thinking question (extremely important):** why is *coin-first then amount* (combinations) different from *amount-first then coin* (permutations)?

---

## Block 3 — Quant Thinking (Representation)

Many quant problems become *target state → reach it → count ways.*

**Problems:**
1. Moves 1, 2 — reach 10.
2. Invest ₹1, ₹2, ₹5 repeatedly — how many ways to reach ₹20?
3. Compare "order matters" vs "order doesn't matter."

**Hard puzzle:** coins 1, 3, 4, target 20 — *minimum* number of coins. Don't solve — design the state.

**Career connection:** "how many ways to assemble a target from building blocks" underlies counting portfolios, payout structures, and combinatorial allocations.

---

## Block 4 — Mathematics (Counting structures)
**How is Fibonacci different from Coin Change?** Fibonacci has one recurrence path; Coin Change has *many* possible contributors per state.

**Exercise:** coins 1, 2, target 4 — list all valid combinations. **Challenge:** coins 1, 2, 5, target 10 — estimate before coding.

---

## Block 5 — Python · Student Management System v34
Add a **scholarship calculator**: `scholarship_levels = [1000, 2000, 5000]` — how many ways can an amount be built? A mini Coin-Change simulation; bonus: display all combinations.

---

## Block 6 — Linux
**Learn:** `head`, `tail`. Practice on `students.txt` — very useful for logs (beginning vs end of file).

---

## Quant Thinking Track — State Design
Beginners ask *"how do I solve it?"* Strong DP thinkers ask: **what information completely describes my current situation?** That's the *state.* Examples: Fibonacci → n; staircase → current stair; Coin Change → current amount; trading → position + cash + inventory (years later).

---

## Portfolio Building
`DP/`:
- `coin_change_recursive.cpp`
- `coin_change_memo.cpp`
- `coin_change_tabulation.cpp`

---

## Communication Exercise
In 5 lines, explain: *"Why does loop order decide whether Coin Change counts combinations or permutations?"*

---

## Journal
- What is the state for Coin Change?
- Why does coin-first vs amount-first matter?
- Order matters vs not — which problems are which?

---

## 🚩 Day 47 Milestone
You're done when you can answer **what a state is** — not "some DP variable", but *the minimum information needed to completely describe where I currently am in the problem.*

---

## Tracker Update (after Day 47)
- Dynamic programming → **55%**
- DSA: DP foundations → **60%**, state design → **45%**
- Counting DP → **30%**
- State design → **60%**, optimization thinking → **65%**
- Python DP modeling → **25%**
