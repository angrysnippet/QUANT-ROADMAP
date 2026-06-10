# Day 11 — 🌱 Foundation World

**Focus:** Memory · Addresses · Pointers · Expected Value · Combinatorics

> Welcome to the first topic that separates *beginner C++* from *actual C++*: **pointers.** Don't memorise `int *ptr;` — today's goal is to understand **why pointers exist at all.**

---

## Previous Day Review (10 min)
- Recall your Day 10 definitions of recursion, expectation, and state.
- Re-run one function from `foundation_revision.cpp` from memory.
- Recall the 50-people birthday estimate.

---

## Block 1 — C++ (Pointers — intuition first)

**Study** — variables live *somewhere* in memory:
```cpp
int x = 10;
// Address   Value
// 1000      10
```
- `&x` — the **address-of** operator: "give me the location of `x`".
- `int* ptr = &x;` — `ptr` stores the **address** of `x`, not its value.

*Why it matters:* Pointers are how C++ shares and modifies data without copying it — the basis of dynamic memory, linked structures, and efficiency.

**Resource:** LearnCpp — *Introduction to pointers*, *address-of operator*, *dereference operator*.

**Code from scratch:**
1. Print `x` and `&x` — observe the difference.
2. Store an address inside a pointer and print `ptr`.
3. Print `*ptr` — and understand why it gives back the value.

**Concept check:**
- What's the difference between `ptr`, `&x`, and `*ptr`?
- What does dereferencing (`*`) actually do?
- Why might two pointers point at the same address?

**Thinking question:** Explain `int x = 10; int* ptr = &x;` in plain English — what's happening *in memory*?

**Cross-domain mapping:** A value is like a house; a pointer is like its address. Find **3 more** real-life "thing vs reference-to-thing" pairs.

---

## Block 2 — DSA (Complexity revision)

Before going deeper, revise. Solve **without notes**:
1. Frequency counter.
2. Binary search.
3. Reverse an array.

After each, answer — **O(?)** for time and space.

**System thinking:** Which of these three is the bottleneck if the input grows to 100 million elements?

---

## Block 3 — Quant Thinking (Expected value)

**Problems (calculate, don't answer emotionally):**
1. Fair die: you win ₹100 if a 6 appears; it costs ₹10 to play. Would you play?
2. Coin: Head → +₹3, Tail → −₹2. Expected gain per play?
3. Generalise: with win probability `p` / amount `W` and loss probability `q` / amount `L`, write the expected-value formula.

**Career connection:** `E = pW − qL` is the skeleton of every bet, trade, and insurance premium.

---

## Block 4 — Mathematics (Combinations)

Learn formally: a **combination** is a *selection*, not an *arrangement*.

**Problem:** choose 3 students from 8. Derive the reasoning **before** reaching for the formula.

Write down: **one intuition learned** and **one question remaining.**

---

## Block 5 — Python (Functions revision)

**Write:**
1. Binary search.
2. Frequency counter (using a dictionary).

---

## Block 6 — Linux (Revision day)

Be comfortable with: `pwd`, `ls`, `cd`, `mkdir`, `rm`. No new commands — just fluency.

---

## Quant Thinking Track — Representation

**Question:** A variable stores a **value**; a pointer stores an **address**. That's *representation* — the same information held in different forms. Find **3 real-life examples** (e.g. *house vs house address*).

---

## Portfolio Building

Extend your toolkit — `FoundationToolkit/pointers/`:
- `address_and_value.cpp`
- `pointer_basics.cpp`
- `dereference.cpp`

---

## Communication Exercise

In 5 lines, explain: *"What is the difference between a variable and a pointer?"*

---

## Journal
- What does `&` mean, and what does `*` mean?
- Why do pointers exist — what problem do they solve?
- What were my 3 "value vs reference" examples?

---

## Day 11 Milestone
You're done when you can explain `int x = 10; int* ptr = &x;` in plain English — not syntax, not definitions, just **what is happening in memory.**

---

## Tracker Update (after Day 11)
- Pointers & memory → **15%**
- Arrays → **90%**
- Binary search → **50%**
- Recursion → **55%**
- Probability fundamentals → **65%**
- Expected value → **35%**
