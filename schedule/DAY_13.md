# Day 13 — 🌱 Foundation World

**Focus:** Pointer Arithmetic · Pass by Value vs Reference · Efficiency · Bayesian Intuition

> Yesterday: arrays and pointers are closely related, and you modified values through pointers. Today you learn **why references exist** — one of the most useful ideas in modern C++.

---

## Previous Day Review (10 min)
- Explain `int arr[5]; int* ptr = arr;` in plain English.
- Re-solve one Day 12 conditional-probability problem.
- Recall why passing an address beats copying.

---

## Block 1 — C++ (References)

**Study — pass by value:**
```cpp
void update(int x) { x = 100; }   // Does the original change? Why not?
```

**Pass by reference:**
```cpp
void update(int& x) { x = 100; }  // Now what changes?
```

**Reference variable:**
```cpp
int x = 10;
int& y = x;   // Is y another variable, or another name for x?
```

*Why it matters:* References let a function work on the *real* data (not a copy) with clean syntax — the everyday tool for efficiency and in-place updates.

**Resource:** LearnCpp — references & pass-by-reference.

**Code from scratch:**
1. Pass by value — observe.
2. Pass by reference — observe.
3. Swap two numbers, first by value then by reference, and compare.

**Concept check:**
- Why does pass-by-value leave the original untouched?
- Is a reference its own object in memory?
- When would swap-by-value silently "fail"?

**Thinking question:** In `int x = 10; int& y = x;`, what does `y` actually represent?

**Cross-domain mapping:** A reference is a second *name* for the same thing — like a nickname. Where else does one object have several names?

---

## Block 2 — DSA (Recursion completion)

**Solve:**
1. Print subsequences (basic understanding).
2. Sum of digits.
3. Count digits recursively.

**Most important question:** what information is stored on the stack during these calls?

---

## Block 3 — Quant Thinking (Bayesian thinking begins)

**Problems (draw the tree):**
1. Box: 9 good bulbs, 1 defective — P(defective) on one pick? (warm-up)
2. Box A (8 good / 2 defective) or Box B (4 good / 6 defective), chosen at random, then a bulb drawn — P(defective)?
3. Toss 2 coins; told "at least one is Head" — P(both Heads)? List the outcomes.

**Career connection:** Two-stage "pick a box, then a bulb" trees are exactly how you reason about hidden states behind observed data — the backbone of Bayesian inference.

---

## Block 4 — Mathematics (Combinations, deeper)

**Question:** How many ways to select 4 people from 10? Derive before formula, then verify with C(10, 4).

Write down: **one intuition learned** and **one question remaining.**

---

## Block 5 — Python (Lists + functions) · Student Marks Manager v2

Build with functions:
- `add_student()`
- `search_student()`
- `show_students()`

Use lists, dictionaries, and functions together.

---

## Block 6 — Linux

**Learn:** `grep` — think of it as a *search engine for files.*
```
grep "main" *.cpp
grep "student" data.txt
```

---

## Quant Thinking Track — Efficiency Thinking
**Question:** Why use `int& x` instead of copying a huge object? Imagine copying 1 integer vs 1 million integers — would they cost the same?

---

## Portfolio Building
Extend your toolkit — `FoundationToolkit/references/`:
- `pass_by_value.cpp`
- `pass_by_reference.cpp`
- `swap_compare.cpp`

---

## Communication Exercise
In 5 lines, explain: *"What is the difference between pass by value and pass by reference?"*

---

## Journal
- Why didn't pass-by-value change the original?
- What does a reference really "point to"?
- Where might in-place updates matter for performance?

---

## Day 13 Milestone
You're done when you can explain `int x = 10; int& y = x;` in plain English — what exists in memory and what `y` actually represents.

---

## Tracker Update (after Day 13)
- Pointers & memory → **55%**
- DSA recursion → **85%**
- Probability fundamentals → **75%**
- Bayesian thinking → **15%**
- Expected value → **45%**
- Python functions → **85%**
