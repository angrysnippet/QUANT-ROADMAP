# Day 9 — 🌱 Foundation World

**Focus:** Recursion Mastery · Stack Intuition · Fibonacci Thinking · Expected Value

> Today recursion becomes a *tool* instead of a concept. You connect it to one of the most important ideas in algorithms: **stack thinking.**

---

## Previous Day Review (10 min)
- Explain *State A / State B* in the HH-expectation problem.
- Re-trace `factorial(4)` on paper.
- Re-solve one Day 8 recursion problem from memory.

---

## Block 1 — C++ (The call stack)

**Study** — learn:
- What happens when a function calls another function?
- What information is remembered?
- What is the call stack?

*Why it matters:* Once you can *see* the stack, recursion stops being mysterious and you can reason about depth, order, and memory.

**Resource:** continue LearnCpp — *Recursion*.

**Dry run (most important).** Trace on paper and draw the stack manually:
```cpp
fun(3) { cout << 3; fun(2); }
fun(2) { cout << 2; fun(1); }
```

**Code from scratch:**
1. Factorial (recursive).
2. Fibonacci (recursive).
3. Power function — compute `2^n` recursively.

**Concept check:**
- How deep does the stack get for `fib(5)`?
- Why does naive recursive Fibonacci repeat work?
- What is the base case for the power function?

**Thinking question:** For each problem, write its **Base case** and **Recursive case** explicitly.

**Cross-domain mapping:** Fibonacci grows from its own previous terms. Could a market process depend only on its recent past the same way?

---

## Block 2 — DSA (Recursion practice)

**Solve (Striver A2Z Sheet):**
1. Fibonacci number.
2. Reverse a string recursively.
3. Check palindrome.
4. Sum of digits.

After each problem write — **Base case:** … **Recursive case:** …

**System thinking:** Naive `fib(n)` is exponential. What single idea (hint: remember results) would make it linear?

---

## Block 3 — Quant Thinking (Expected value)

**Problems (build the equation yourself):**
1. Expected rolls until the first 6.
2. Expected tosses until the first Head.
3. Expected tosses until the first Tail. Compare your answers.

**Hard puzzle:** A game pays +₹10 on Head, −₹5 on Tail, fair coin. Should you play? Think **expectation** before probability.

**Career connection:** A positive-expectation game can still lose many times in a row — separating *edge* from *variance* is the heart of trading.

---

## Block 4 — Mathematics (Sequences)

**Watch:** *3Blue1Brown* — continue the Algebra playlist. Focus on recursive sequences and growth patterns.

**Thinking exercise:** For Fibonacci `1 1 2 3 5 8 13 …`, why does each term depend only on the previous terms? Could a market process behave similarly?

---

## Block 5 — Python (Functions + recursion)

**Write:**
1. Factorial.
2. Fibonacci.
3. Sum of digits.

---

## Block 6 — Linux

**Learn:** `grep`, `sort`, `uniq`.

**Lab:**
```
sort data.txt
uniq data.txt
```
See how Linux processes data with small composable tools.

---

## Quant Thinking Track — Expected Value

**Question:** Why can a game be **profitable on average** while **losing several times in a row**? This is one of the most important ideas in trading and quant finance.

---

## Weekly Challenge (Day 9 Special) — 100 doors
100 closed doors. Pass 1 toggles every door, pass 2 every 2nd door, … pass 100 every 100th door. Which doors stay open? You touched a simpler version earlier — now **derive the general rule completely** (hint: count each door's divisors).

---

## Portfolio Building

Extend your toolkit — `FoundationToolkit/recursion/`:
- `fibonacci.cpp`
- `power.cpp`
- `sum_digits.cpp`

---

## Communication Exercise

In 5 lines, explain: *"Why does a positive expected value not guarantee winning every time?"*

---

## Journal
- What finally clicked about the call stack?
- Which expected-value result surprised me?
- Which doors stay open, and why exactly?

---

## 🚩 Phase 1 Checkpoint
By Day 9 you should comfortably know: ✅ Functions ✅ Arrays ✅ Vectors ✅ Maps ✅ Sets ✅ Binary Search basics ✅ Recursion basics. That's a solid first flag on your roadmap.

---

## Tracker Update (after Day 9)
- Recursion basics → **75%**
- DSA recursion → **60%**
- Probability fundamentals → **55%**
- Recursive thinking → **45%**
- State thinking → **35%**
- Expected value → **20%**
- Python functions → **60%**
