# Day 4 — 🌱 Foundation World

**Focus:** Vectors (deeper) · Hashing · Counting · Time Complexity

---

## Previous Day Review (10 min)
- Explain the difference between an array and a vector.
- Re-create a frequency count of an array from memory.
- Re-solve one Day 3 vector problem.

---

## Block 1 — C++ (Vectors, deeper)

**Study** — continue vectors. Learn:
- `insert()`
- `erase()`
- `front()`
- `back()`
- `empty()`

*Why it matters:* Knowing the full vector toolkit means you reach for the right method instead of re-implementing it by hand each time.

**Resource:** LearnCpp — *STL std::vector operations*.

**Code from scratch** (no copying, no AI):
1. Count occurrences of every element — e.g. `1 2 2 3 1 1` → `1 -> 3`, `2 -> 2`, `3 -> 1`.
2. Find the unique elements.
3. Merge two vectors.

**Concept check:**
- When does a vector reallocate its memory?
- What does `erase()` do to the elements after the removed one?
- Why is `empty()` preferred over `size() == 0`?

**Thinking question:** You counted occurrences by storing a frequency once and reusing it. How does that avoid re-scanning the array for every element?

**Cross-domain mapping:** A frequency table over trades tells you the most-traded tickers in one pass. Where else does "count once, reuse" save work?

---

## Block 2 — DSA (Hashing basics)

**Idea first:** store a frequency → reuse the frequency → avoid recomputing. This single move turns many O(n²) scans into O(n).

**Solve (Striver A2Z Sheet):**
1. Count frequency of elements.
2. Highest-frequency element.
3. Character frequency in a string.

After each solution, answer in one line — **why did hashing help here?**

**System thinking:** Hashing trades extra memory for speed. When would that trade be a *bad* idea?

---

## Block 3 — Quant Thinking (Arrange vs Select)

**Problems (reason it out):**
1. How many ways can 5 students sit in 5 chairs?
2. How many ways can 3 students be selected from 10?
3. A 4-digit lock, digits 0–9 — how many possible codes?

**Bonus (understand, don't memorise):** Why is *selection* different from *arrangement*?

**Reflection:** Which of the three problems cared about *order*, and which didn't?

**Career connection:** "Does order matter?" is the same question behind pricing ordered vs unordered outcomes in probability and finance.

---

## Block 4 — Mathematics (Algebra)

**Watch:** *3Blue1Brown — Essence of Algebra, Chapter 4.* Think about functions — inputs and outputs.

Write down: **one intuition learned** and **one question remaining.**

---

## Block 5 — Python (Dictionaries)

**Learn:** dictionaries — `freq = {}` and `dict[key]`.
**Resource:** Python Official Tutorial — *Dictionaries*.

**Write:**
1. A frequency counter.
2. A word counter.

**Compare:** a Python `dict` vs a C++ `map` / `unordered_map` — what's the same idea?

---

## Block 6 — Linux

**Learn:** `grep`, `history`, `clear`.

**Lab:** use `grep` to find a word inside a file you created, then use `history` to review the commands you've run today.

---

## Portfolio Building

Extend your toolkit — `FoundationToolkit/hashing/`:
- `frequency_count.cpp`
- `unique_elements.cpp`
- `merge_vectors.cpp`

---

## Communication Exercise

In 5 lines, explain: *"What is hashing, and why does it make counting fast?"*

---

## Journal
- When should I use a vector instead of an array?
- Why is hashing useful — what does it buy me?
- What is the difference between selection and arrangement?
- Where might frequency counting appear in real systems?

---

## Success Criteria
- ✅ Use `insert` / `erase` / `front` / `back` / `empty` confidently.
- ✅ Build a frequency table from scratch.
- ✅ Solve 3 hashing problems and justify why hashing helped.
- ✅ Explain selection vs arrangement.
- ✅ Write a frequency counter in Python.

---

## Tracker Update (after Day 4)
- Arrays & strings → **70%**
- STL vectors & pairs → **40%**
- Hashing & sliding window → **10%**
- Algebra intuition → **25%**
- Probability fundamentals → **15%**
- Python: variables, loops, functions → **40%**
- Python: lists & dictionaries → **20%**
- Linux terminal basics → **30%**
