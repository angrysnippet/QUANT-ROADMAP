# Day 7 — 🌱 Foundation World

**Focus:** Recursion · Self-reference · Recursive Thinking · Expected Value

> Until now every problem was *input → loop → answer*. Today you meet **recursion** — one of the most important ideas in algorithms, trees, dynamic programming, and quantitative reasoning.

---

## Previous Day Review (10 min)
- Explain why binary search is O(log n).
- Re-solve one Day 6 search problem from memory.
- Recall the abstraction pattern linking binary search and the locker puzzle.

---

## Block 1 — C++ (Recursion)

**Study** — learn:
- What is recursion?
- Base case
- Recursive case
- The call stack (intuition only)

*Why it matters:* A function that calls itself lets you describe a big problem as a smaller copy of itself — the foundation of trees, DP, and divide-and-conquer.

**Resource:** LearnCpp — *Recursion*.

**Code from scratch** (no copying, no AI):
1. Factorial — `5! = 5 × 4 × 3 × 2 × 1`.
2. Sum of the first N numbers.
3. Print `5 4 3 2 1` using recursion.

**Concept check:**
- What is the base case, and what happens without one?
- What does each recursive call "remember"?
- What gets *smaller* on every call?

**Thinking question:** The expected-tosses-for-HH problem said *"the future state looks like the original state."* Why is that the same idea as recursion?

**Cross-domain mapping:** Where else does "the rest of the problem looks like the whole problem" show up — compound interest? nested folders?

---

## Block 2 — DSA (Recursion basics)

**Solve (Striver A2Z Sheet):**
1. Print N to 1.
2. Print 1 to N.
3. Sum of N numbers.
4. Factorial.

After each problem, answer — **what becomes smaller after each recursive call?** (This question is crucial.)

**System thinking:** Each recursive call uses a stack frame. What happens if the recursion never reaches its base case?

---

## Block 3 — Quant Thinking (Recursive thinking)

**Problems (build the equation — don't Google):**
1. A frog climbs 3 steps each day, slides 2 down each night, in a 10-step well. When does it escape?
2. Flip a fair coin — expected tosses until the first Head? Use the A/B state style.
3. Roll a die repeatedly — expected rolls until the first 6?

**Career connection:** Recursive expectation ("the expected future equals one step plus the expected remainder") is exactly how many pricing and game-value problems are set up.

---

## Block 4 — Mathematics (Sequences)

**Watch:** *3Blue1Brown — Essence of Algebra, Chapter 6.* Focus on patterns and recursive definitions.

Write down: **one intuition learned** and **one question remaining.**

---

## Block 5 — Python (Functions + recursion)

**Write:**
1. Recursive factorial.
2. Recursive sum.

**Compare:** how does the Python recursion read against your C++ version?

---

## Block 6 — Linux

**Learn:** `head`, `tail`, `wc`.

**Lab:**
```
wc file.txt
head file.txt
tail file.txt
```

---

## Quant Thinking Track — Recursion Thinking

**Question:** In **one sentence**, what do factorial, Fibonacci, expected tosses, and recursive fractions all have in common?

---

## Portfolio Building

Extend your toolkit — `FoundationToolkit/recursion/`:
- `factorial.cpp`
- `sum_n.cpp`
- `print_countdown.cpp`

---

## Communication Exercise

In 5 lines, explain: *"Why must every recursion have a base case?"*

---

## Journal
- What clicked about recursion today?
- Which expected-value problem was hardest to set up?
- Where might "a problem inside the same problem" appear in finance?

---

## Day 7 Milestone
You're done when you can explain — in your own words, not textbook definitions — **why recursion must have a base case.**

---

## Tracker Update (after Day 7)
- Recursion basics → **25%**
- DSA recursion → **15%**
- Algebra intuition → **45%**
- Probability fundamentals → **40%**
- Recursive thinking → **15%**
