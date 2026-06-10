# Day 46 — 🚀 Quant Mind World · House Robber — Your First Optimization DP

> Until now Fibonacci and Stairs were *counting* problems. Today you **maximise** something — where DP starts feeling like real problem-solving.

**Focus:** Choice-Based DP · Take / Skip · Optimization · State Design · Decision Making

---

## Previous Day Review (10 min)
- State the 4-step DP recipe.
- Recall why climbing stairs is Fibonacci.

---

## Block 1 — C++ (House Robber)

Houses `[2, 7, 9, 3, 1]`. Rule: you **can't rob adjacent houses.** Maximum money?

**Think like a human:** at each house, **take** or **skip** — only two choices. For `2 7 9`: `2 + 9 = 11` vs `7` → best is `11`.

**Core insight:** DP often begins with *"at this state, what choices exist?"*

*Why it matters:* this is the leap from counting to *decision-making* — the template behind most optimization DP.

**Code from scratch:** recursive, memoized, then bottom-up House Robber.

---

## Block 2 — DSA (Choice-based DP — your new framework)
1. **State:** `dp[i]` = max money from the first `i` houses.
2. **Choices at house i:** take or skip.
3. **Transition:** if you take house `i`, house `i-1` is off-limits, so
   `dp[i] = max(dp[i-1], dp[i-2] + value[i])`.

**Task:** fill the DP table by hand for `[2, 7, 9, 3, 1]`.

---

## Block 3 — Quant Thinking (Decision making)

At every state: *current situation → possible decisions → best future outcome.* This appears in investing, trading, inventory management, scheduling, and reinforcement learning.

**Problems:**
1. Daily profits `[3, 2, 7, 10]`, can't trade consecutive days — max profit?
2. `[5, 1, 1, 5]` — max?
3. Identify the state, the choices, and the transition.

**Hard puzzle:** stairs with rewards `[1, 4, 2, 7, 3]`, can't take adjacent rewards — model as DP.

**Career connection:** "best decision now, accounting for future consequences" is the literal shape of dynamic hedging and reinforcement learning.

---

## Block 4 — Mathematics (Optimization recurrences)
Before: `Ways(n) = Ways(n-1) + Ways(n-2)` — count everything. Now: `Best(n) = max(...)` — choose the best. **Why is optimization harder than counting?**

**Exercise:** build the DP table for `[1, 2, 3, 1]`. **Challenge:** find the optimum for `[2, 1, 1, 2]`.

---

## Block 5 — Python · Student Management System v33
Add a **top-performer cache**: store `best_student`; when marks change, update it intelligently. Goal: maintain the optimal answer efficiently — optimization DP in miniature.

---

## Block 6 — Linux
**Learn:** `zip`, `unzip`. Practice:
```
zip project.zip project/*
unzip project.zip
```

---

## Quant Thinking Track — Local Choice vs Global Optimum
Greedy "take the biggest value" fails: for `[2, 7, 9, 3, 1]`, grabbing `9` isn't enough. DP weighs **current choice + future consequences.** This recurs later in portfolio optimization, dynamic hedging, RL, and trading systems.

---

## Portfolio Building
`DP/`:
- `house_robber_recursive.cpp`
- `house_robber_memo.cpp`
- `house_robber_tabulation.cpp`

---

## Communication Exercise
In 5 lines, explain: *"Why can't a greedy 'take the biggest' strategy solve House Robber?"*

---

## Journal
- What are the state, choices, and transition here?
- Why is maximising harder than counting?
- Where did greedy fail, and why did DP win?

---

## 🚩 Day 46 Milestone
You're done when you can answer, for *any* DP problem: **(1) what is the state? (2) what choices do I have? (3) how do choices lead to new states?** That three-step framework solves a huge fraction of beginner and intermediate DP.

---

## Tracker Update (after Day 46)
- Dynamic programming → **45%**
- DSA: DP foundations → **50%**, choice-based DP → **25%**
- Optimization recurrences → **25%**
- Decision thinking → **30%**, optimization thinking → **60%**
- Python efficient state management → **20%**
