# Day 43 — 🚀 Quant Mind World · Memoization — Your First Real DP

> Yesterday you discovered repeated work → repeated states → waste. Today you fix it. This is the birth of actual DP.

**Focus:** Memoization · Top-Down DP · State Storage · Fibonacci Optimization · Computational Thinking

---

## Previous Day Review (10 min)
- Explain why naive Fibonacci is exponential.
- Recall how many times `fib(2)` appeared in `fib(8)`.

---

## Block 1 — C++ (Store the answers)

`fib(40)` was painfully slow because `fib(38)`, `fib(37)`, … were recomputed many times. **Solution:** store answers — `dp[5]` holds `fib(5)`. New rule: *before computing `fib(n)`, ask "have I solved this already?"*

*Why it matters:* one cache lookup turns exponential work into linear — the single highest-leverage line in algorithms.

**Code from scratch:**
1. Recursive Fibonacci with a call counter.
2. Memoized Fibonacci using `vector<int> dp(n+1, -1);`.
3. Compare runtimes — run `fib(45)` and observe the huge difference.

**Concept check:**
- What does `dp[n] == -1` signify?
- After memoization, how many *distinct* states get computed for `fib(n)`?

---

## Block 2 — DSA (The heart of DP)
```cpp
if (dp[n] != -1) return dp[n];
```
That one line is the heart of DP.

**Task 1:** memoized Fibonacci. **Task 2:** count calls again and compare.

**Thinking question:** why does every state now get solved only **once**?
**Challenge:** can memoization be applied to the staircase `Ways(n)`?

---

## Block 3 — Quant Thinking (State storage)

Expected tosses until **HH**, states Start → H → HH. If state `H` appears multiple times, should you recompute? **No — reuse.**

**Problems (draw states):**
1. Staircase 1 or 2 steps — ways to reach 7.
2. Ways to reach 10.
3. Identify the repeated states.

**Hard puzzle:** moves `+1`, `+2`, `+3` — ways to reach 20? Don't solve — **design the states.**

**Career connection:** storing `state → answer` once and reusing it is the same idea you'll later meet as value tables in reinforcement learning and quant systems.

---

## Block 4 — Mathematics (Recurrence → DP)
You derived `Ways(n) = Ways(n−1) + Ways(n−2)`. Now you *store* the answers. **Why does storing not change correctness — only efficiency?**

**Exercise:** find `Ways(6)` by hand. **Challenge:** with steps 1, 2, 3, write `Ways(n) = ?`

---

## Block 5 — Python · Student Management System v30
Add **cached reports**: `report_cache = {}`; reuse when a report already exists; track `cache_hits` and `cache_misses` — this *is* memoization in software.

---

## Block 6 — Linux
**Learn:** `nano` or `vim` (pick one). Goal: edit files directly from the terminal.

---

## Quant Thinking Track — The State Table
DP is essentially **state → answer**, stored in a table:
- Fibonacci: `n → fib(n)`
- Staircase: `n → ways(n)`
- Probability: `state → expected value`

**Question:** why is DP basically a *memory system* for problem-solving?

---

## Portfolio Building
`DP/`:
- `fib_memoized.cpp`
- `staircase_memoized.cpp`

---

## Communication Exercise
In 5 lines, explain: *"What does the line `if (dp[n] != -1) return dp[n];` accomplish?"*

---

## Journal
- Why does each state compute only once now?
- How big was the `fib(45)` speedup?
- What's the `state → answer` table for the staircase?

---

## 🚩 Day 43 Milestone
You're done when you can answer **why memoization makes Fibonacci fast** — not "because DP is faster", but *because each state is computed once and then reused.*

---

## Tracker Update (after Day 43)
- Dynamic programming → **15%**
- DSA memoization → **25%**
- Recurrence relations → **80%**
- Optimization thinking → **30%**, state reuse → **40%**
- Python caching concepts → **30%**
