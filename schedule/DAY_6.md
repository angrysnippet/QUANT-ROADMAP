# Day 6 — 🌱 Foundation World

**Focus:** Binary Search · Sorted Data · Logarithmic Thinking · Conditional Probability

> Your first true **algorithmic thinking** day. Binary search is an idea that stays with you for your entire career.

---

## Previous Day Review (10 min)
- Explain when to use a `map` vs a `set`.
- Re-solve one Day 5 hashing problem from memory.
- Recall your two decompositions of 13 × 17.

---

## Block 1 — C++ (Searching)

**Study** — learn:
- Linear search
- Binary search
- *Why* binary search is faster

**Think first.** To find `8` in `1 2 3 4 5 6 7 8 9 10`:
- *Method 1* — check 1, then 2, then 3 … (linear).
- *Method 2* — jump to the middle, ask "is target > 5?", and **half the array disappears.**

That halving idea is huge.

*Why it matters:* Throwing away half your remaining options per step is the template behind every log-time algorithm.

**Resource:** LearnCpp — *Algorithms & searching*.

**Code from scratch** (no copying, no AI):
1. Linear search.
2. Binary search.
3. A version that counts the comparisons each makes (so you can *see* the difference).

**Concept check:**
- What must be true about the data before binary search works?
- What is the search space after each comparison?
- Why is the worst case about `log₂(n)` steps?

**Thinking question:** Why does binary search *require* sorted data, while linear search doesn't?

**Cross-domain mapping:** Looking up a price by date in a sorted price history is binary search. Where else do you "search a sorted thing" in daily life?

---

## Block 2 — DSA (Binary search)

**Solve (Striver A2Z Sheet):**
1. Binary search.
2. Lower-bound intuition.
3. Search-insert position.

After each problem, answer — **why does binary search require sorting?**

**System thinking:** For 1 billion sorted records, roughly how many comparisons does binary search need? (Reason from `log₂`.)

---

## Block 3 — Quant Thinking (Information reduction)

Yesterday: 3-way groupings. Today: binary reduction.

**Problems (reason, don't rush):**
1. A number is between 1 and 1000 — minimum yes/no questions to pin it down?
2. How many yes/no questions for 1 million numbers?
3. 27 balls, one heavier, a balance scale — how many weighings? (Reuse the grouping idea from Day 5.)

**Reflection:** Each yes/no question (or weighing) gives you one "unit" of information. How does that connect to `log` of the search space?

**Career connection:** "How much information does one observation give me?" is the heart of signal vs noise in quant research.

---

## Block 4 — Mathematics (Conditional probability)

**Watch:** *3Blue1Brown — Probability* (the first relevant probability video).

**Problem:** A bag has 3 red and 2 blue balls. Draw one, then a second *without replacement*. What's the probability the second ball is blue? Think carefully about how the first draw changes the second.

Write down: **one intuition learned** and **one question remaining.**

---

## Block 5 — Python (Functions)

**Learn:** defining functions, e.g. `def search(arr, x):`.
**Resource:** Python Official Tutorial — *Defining Functions*.

**Write:**
1. Linear search as a function.
2. Binary search as a function.

**Compare:** how does wrapping logic in a function change how you reuse and test it?

---

## Block 6 — Linux

**Learn:** `mkdir`, `rmdir`, `tree`.

**Lab:** build this structure for your work and view it with `tree`:
```
quant/
├── cpp
├── dsa
├── math
└── python
```

---

## Quant Thinking Track — Abstraction

**Question:** What do **binary search**, the **prisoner puzzle**, and the **100-locker problem** have in common? Not the solutions — the underlying *thinking pattern*. Try to name it.

*Why:* Seeing the shared structure beneath different problems is what makes you fast on problems you've never met.

---

## Portfolio Building

Extend your toolkit — `FoundationToolkit/searching/`:
- `linear_search.cpp`
- `binary_search.cpp`
- `compare_comparisons.cpp`

---

## Communication Exercise

In 5 lines, explain: *"Why is binary search O(log n) and not O(n)?"*

---

## Journal
- Why is binary search O(log n) and not O(n)?
- Which yes/no-question puzzle clicked, and which didn't?
- What abstract pattern links today's three quant problems?
- Where might binary search appear in real software systems?

---

## Day 6 Checkpoint
Day 6 is complete when you can explain, **in your own words and without memorising**, why binary search is O(log n) rather than O(n).

---

## Tracker Update (after Day 6)
- STL vectors & pairs → **80%**
- Maps, sets, iterators → **40%**
- Binary search → **20%**
- Probability fundamentals → **30%**
- Decomposition thinking → **35%**
- Abstraction thinking → **20%**
