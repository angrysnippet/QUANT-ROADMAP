# Day 5 — 🌱 Foundation World

**Focus:** Maps · Sets · Iterators · Combinations · Decomposition

---

## Previous Day Review (10 min)
- Explain why hashing makes counting O(n) instead of O(n²).
- Re-build a frequency table from memory.
- Re-solve one Day 4 hashing problem.

---

## Block 1 — C++ (Maps & sets)

**Study** — learn:
- `map<int,int>`
- `unordered_map<int,int>`
- `set<int>`

Concepts: key–value storage, automatic sorting (`map`), uniqueness (`set`).

*Why it matters:* These containers replace pages of manual bookkeeping with one well-chosen data structure — and they're your first real "powerful STL tools".

**Resource:** LearnCpp — STL *map* and *set* sections.

**Code from scratch** (no copying, no AI):
1. Store frequencies using a `map` — e.g. `1 2 2 3 3 3` → `1 -> 1`, `2 -> 2`, `3 -> 3`.
2. Remove duplicates from an array using a `set`.
3. Find the first repeating element.

**Concept check:**
- What's the difference between `map` and `unordered_map`?
- Why does a `set` automatically reject duplicates?
- When does sorted order (in `map`) actually help you?

**Thinking question:** A `set` and a sorted `map` of counts can both "deduplicate". When would you reach for each?

**Cross-domain mapping:** A `map<ticker, volume>` summarises a whole trading day. What real-world records are naturally key–value?

---

## Block 2 — DSA (Hash maps)

**Solve (Striver A2Z Sheet):**
1. Count frequency.
2. Highest-frequency number.
3. Find the missing number using hashing.

After every solution, answer — **why did hashing help here?**

**System thinking:** When would you prefer `unordered_map` (faster average) over `map` (sorted, slower)? When does the ordering pay for itself?

---

## Block 3 — Quant Thinking (Combinations)

Yesterday you saw arrangements and selections — today we formalise.

**Problems (derive yourself):**
1. How many ways to choose 2 students from 10?
2. How many committees of 3 from 8 people?
3. Toss 5 coins — **P(exactly 2 heads)**?

**Hard thinking:** 10 people each shake hands once — how many handshakes? Can you derive the **general formula for n people**?

**Career connection:** Combinations underpin probability, which underpins risk and pricing models in quant finance.

---

## Block 4 — Mathematics (Algebra)

**Watch:** *3Blue1Brown — Essence of Algebra, Chapter 5* (one chapter only — don't binge). Focus on functions and transformations.

Write down: **one intuition learned** and **one question remaining.**

---

## Block 5 — Python (Dictionaries — mini project)

**Learn:** dictionary basics — `freq = {}` and `dict[key]`.

**Mini project — Frequency Counter.** Given input lines:
```
apple
banana
apple
orange
banana
apple
```
Output:
```
apple : 3
banana : 2
orange : 1
```

**Compare:** how does this Python solution mirror the C++ `map` version from Block 1?

---

## Block 6 — Linux

**Learn:** `find`, `which`, `man`.

**Lab:** run each one for real —
```
man ls
which g++
find . -name "*.cpp"
```

---

## Quant Thinking Track — Decomposition

**Question:** A rectangle is `13 × 17`. *Without multiplying directly*, compute the area by decomposition — e.g. `13 × 17 = 13 × (20 − 3)`. Find more than one method.

*Why:* Breaking a hard number into easy pieces is the core mental-math move in quant interviews.

---

## Portfolio Building

Extend your toolkit — `FoundationToolkit/maps_sets/`:
- `map_frequency.cpp`
- `set_dedup.cpp`
- `first_repeating.cpp`

---

## Communication Exercise

In 5 lines, explain: *"When should I use a `map`, and when a `set`?"*

---

## Journal
- How is a `map` different from a `set` — and when would I pick each?
- Which counting idea finally clicked today?
- What was my cleanest decomposition of 13 × 17?
- Where might key–value maps appear in real software systems?

---

## Success Criteria
- ✅ Use `map`, `unordered_map`, and `set` confidently.
- ✅ Deduplicate and count using STL containers.
- ✅ Derive C(n, k) reasoning yourself.
- ✅ Solve 3 hashing problems.
- ✅ Decompose 13 × 17 two different ways.

---

## Tracker Update (after Day 5)
- STL vectors & pairs → **60%**
- Maps, sets, iterators → **20%**
- Hashing & sliding window → **25%**
- Algebra intuition → **35%**
- Probability fundamentals → **20%**
- Decomposition thinking → **25%**
- Puzzle-solving fluency → **15%**
