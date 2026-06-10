# Day 99 — ⚠️🧠 Quant Mind World · Greedy Failures — When Local Decisions Destroy Global Optimality

> Yesterday: *why* greedy works. Today, equally important: **when greedy fails.** Many students become dangerous after learning greedy, assuming local best = global best. Today is about developing **skepticism.**

**Focus:** Greedy Failures · Counterexamples · Greedy vs DP · Algorithm Selection · Critical Thinking

---

## Previous Day Review (10 min)
- State the exchange argument.
- Recall why evidence isn't proof.

---

## Why today matters
Capacity 50; items `(10,60), (20,100), (30,120)`. Greedy-by-value grabs 120 first — *looks smart.* But greedy isn't always optimal here — this is where many problems become **DP, not greedy.**

---

## Block 1 — C++ (Counterexample thinking)
Create `greedy_failure.cpp`: store `weight[]`, `value[]`; try **greedy by value** (highest first) and compare against the **optimal** computed by hand.

*Why it matters:* knowing when *not* to use a tool is as valuable as knowing when to use it — and a single counterexample is decisive.

**Goal:** see that the best immediate choice can lose to a better *combination.*

---

## Block 2 — DSA (The key failure: 0/1 Knapsack)
Greedy struggles when **choices interact** — taking an item changes remaining capacity, which changes future options. That coupling is exactly where **DP shines** (it considers combinations, not just the next step).

**Deep insight:** greedy fails when *the current choice significantly changes future possibilities.*

---

## Block 3 — DSA (Recognising dangerous problems)
When you see *take / skip*, ask: **does taking this change future opportunities?** If yes → be careful → often a DP candidate.
- **Knapsack:** taking item A changes remaining capacity → DP.
- **Activity Selection:** earliest-finish leaves max future room → greedy works.

**Exercise:** classify Knapsack / Activity Selection / Coin Change / Minimum Path Sum as greedy-safe or DP-needed.

---

## Block 4 — Mathematics (Counterexamples)
A counterexample is *one case that breaks a rule.* Against "always take the largest item," you need only **one failure** to disprove it.

**Exercise:** why does one counterexample outweigh 1000 successes? **Challenge:** invent your own greedy rule, then try to break it.

---

## Block 5 — Quant Thinking (Local ≠ global success)

Spending *all* study time on DSA gives the biggest *immediate* gain — but does it maximise 3-year quant growth? Probably not; you also need Probability, Statistics, Python, Projects. That's the DP perspective.

**Problems:** career planning; investment allocation; research prioritization.

**Hard puzzle:** 100 hours — (A) all on DSA vs (B) 60 DSA / 20 Probability / 20 Python — which creates more future opportunities? Think long-term.

**Career connection:** many quant failures are **local** optimization beating out **global**: overfitting (great now, fails later), concentrated portfolios (great today, risky tomorrow), research shortcuts. Immediate reward can mislead.

---

## Block 6 — Python · Student Management System v82
Create `greedy_vs_dp.py`: map examples to technique + reason (Activity Selection → greedy; Knapsack → DP; Minimum Path Sum → DP). Bonus: `is_greedy_safe(problem_type)` with explanation.

**Linux:** learn `less`; practice `less largefile.txt`. **Question:** why load info gradually instead of all at once? Think resource efficiency.

---

## Quant Thinking Track — Think Beyond the Next Step
Weak thinker: *what helps most today?* Strong thinker: *what creates the best future state?* That difference is **greedy vs DP** in one sentence.

---

## Portfolio Building
`Patterns/greedy/`:
- `greedy_failure_knapsack.cpp`
- `greedy_vs_dp_table.cpp`

---

## Communication Exercise
In 5 lines, explain: *"What feature of a problem warns you greedy will fail?"*

---

## Journal
- Where did greedy-by-value lose to a combination?
- Which of the four problems are DP-needed, and why?
- Where have I over-optimized for *today*?

---

## 🚩 Day 99 Milestone
You're done when you can answer **when greedy usually fails** — *when current decisions significantly affect future opportunities, so local optimization diverges from global optimization.*

---

## Next 🚀
Day 100 — **Algorithm Selection Master Checkpoint:** unify Graphs, DP, Greedy, Binary Search, and Trees into one decision framework — the shift from *learning algorithms* to *thinking like a problem solver.*

---

## Tracker Update (after Day 99)
- Greedy: fundamentals → **60%**, proof techniques → **45%**, failure recognition → **50%**
- Dynamic programming → **88%**, algorithm selection → **65%**
- Quant thinking → **94%**, systems thinking → **88%**
