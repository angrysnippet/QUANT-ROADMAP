# Day 79 — 🧠⚡ Quant Mind World · Memoization vs Tabulation — Two Ways to Think in DP

> You met both styles on Days 43–44; today you contrast them head-to-head: **same problem, different thinking.** Two people can solve the identical DP via memoization (top-down) or tabulation (bottom-up).

**Focus:** Memoization · Tabulation · Top-Down · Bottom-Up · DP Efficiency

---

## Previous Day Review (10 min)
- State the 3-question DP recognition test.
- Recall what "state" means.

---

## Why today matters
Naive `Fibonacci(50)` recomputes `F(48)` over and over — exponential waste. DP removes it.

---

## Block 1 — C++ (Memoization)
Solve a state, **store** it; before re-solving, check the cache:
```cpp
int fib(int n){
    if(n<=1) return n;
    if(dp[n]!=-1) return dp[n];
    return dp[n] = fib(n-1)+fib(n-2);
}
```
Memoization = **solve only when needed.**

*Why it matters:* one cache check converts O(2ⁿ) into O(n) — the highest-leverage line in DP.

**Task:** memoize Fibonacci and Climbing Stairs.

---

## Block 2 — DSA (Tabulation)
Start from base cases and build upward:
```cpp
dp[0]=0; dp[1]=1;
for(int i=2;i<=n;i++) dp[i]=dp[i-1]+dp[i-2];
```
Tabulation = **build answers before they're needed.**

**Task:** tabulate Fibonacci and House Robber.

---

## Block 3 — DSA (Compare)
| Feature | Memoization | Tabulation |
|---------|-------------|------------|
| Style | Top-down | Bottom-up |
| Recursion | Yes | No |
| Easier to write | Usually | Sometimes harder |
| Stack usage | Yes | No |
| Often faster | No | Usually |

Memoization computes **only required** states; tabulation computes **all** states.

**Exercise:** write Climbing Stairs both ways.

---

## Block 4 — Mathematics (Computational efficiency)
Naive Fibonacci ≈ O(2ⁿ); DP Fibonacci = O(n). At n=50 that's millions of calls vs 50 states.

**Exercise:** why is storing answers so powerful? **Challenge:** why do overlapping subproblems create inefficiency?

---

## Block 5 — Quant Thinking (Reuse knowledge)

Relearning loops/arrays/functions from scratch every day would be terrible — instead, *learn once, reuse forever.* That's memoization for humans.

**Problems:** learning systems; research notes; trading signals.

**Hard puzzle:** a research team computes "market feature X" today — should future teams recompute it, or reuse? Think DP.

**Career connection:** quant systems lean on **caching** (= memoization) for historical data, features, model outputs, risk calculations — recomputing everything is expensive.

---

## Block 6 — Python · Student Management System v63
Create `dp_methods.py`: Fibonacci by memoization and by tabulation; print execution time for n=40. Bonus: space-optimized Fibonacci using only `prev2`, `prev1` (preview of tomorrow).

**Linux:** learn `time`; practice `time python script.py`. **Question:** why measure performance instead of guessing?

---

## Quant Thinking Track — Store Valuable Work
Weak thinkers repeat effort; strong thinkers store and reuse results — programming (memoization), research (notes), trading (features), learning (knowledge structures). Efficiency often comes from *avoiding repeated work*, not working harder.

---

## Communication Exercise
In 5 lines, explain: *"When would you prefer tabulation over memoization?"*

---

## Journal
- Which style felt more natural for Climbing Stairs?
- Why does memoization skip work tabulation does?
- What's a "feature" I should cache instead of recompute?

---

## 🚩 Day 79 Milestone
You're done when you can answer **the difference between memoization and tabulation** — not "both are DP", but *memoization solves states on demand via recursion; tabulation builds them iteratively from base cases upward.*

---

## Next 🚀
Day 80 — **Space Optimization in DP:** rolling arrays, state compression — *a correct solution isn't always an efficient one.*

---

## Tracker Update (after Day 79)
- DSA: DP recognition → **40%**, memoization → **35%**, tabulation → **35%**
- Complexity analysis → **40%**
- Reuse & optimization → **45%**
- Python DP implementations → **35%**
