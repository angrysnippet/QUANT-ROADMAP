# Day 58 — 🌳🔥 Quant Mind World · Heap Applications & Top-K Problems

> Yesterday: heap property, min/max heap, priority queue, insert/delete. Today you learn *why heaps are actually used* — where they become powerful.

**Focus:** Top-K Problems · K Largest Elements · Running Data Streams · Priority Optimization · Heap Applications

---

## Previous Day Review (10 min)
- Explain why a heap beats sorting for "the most important element."
- Recall the heap array index relationships.

---

## Block 1 — C++ (Top-K)
For `[5, 2, 9, 1, 7, 8, 3]`, find the largest 3:
- **Method 1:** sort everything — `O(N log N)`.
- **Method 2:** maintain a heap of size `K = 3` — much better.

**Key insight:** you don't need everything sorted, only the **best K.**

*Why it matters:* this single pattern handles "top trades", "top results", "top students" at scale — without sorting the whole dataset.

**Code from scratch:** K largest via priority queue; K smallest; compare with the sorting approach.

---

## Block 2 — DSA (The Top-K pattern)
For `N = 1,000,000`, `K = 10` — do you really sort a million numbers for the top 10? No.

**Algorithm — maintain a min heap of size K:**
```
for each number:
    if heap size < K:      insert
    else if num > heap top: pop top, insert num
```
At the end, the heap holds the **top K**.

**Task:** run `[4, 8, 1, 9, 2, 7, 10]` for top 3 by hand. **Thinking question:** why is the heap size only `K`, not `N`?

---

## Block 3 — DSA (Running median)
Numbers arrive one by one and you need the median after each. Sorting every time is terrible. **Solution — two heaps:** a max heap for the lower half, a min heap for the upper half.

**Thinking question:** why does keeping two *balanced* heaps help? **Challenge:** insert `5, 2, 10` and track the median.

---

## Block 4 — Mathematics (Efficiency)
For `N = 1,000,000`, top 10: sorting ≈ `N log N`; heap ≈ `N log K`. Since `K = 10` is tiny, that's a huge win.

**Exercise:** compare `log₂(1,000,000)` (≈ 20) and `log₂(10)` (≈ 3.3). **Challenge:** why does *reducing problem size* often matter more than faster hardware?

---

## Block 5 — Quant Thinking (Continuous ranking)

1000 stocks, you need the top 20 momentum names *continuously* — sort everything, or maintain a heap?

**Problems:** (1) top 5 of 100,000 students; (2) maintain best trading opportunities in real time; (3) news feed showing top 10 posts.

**Hard puzzle:** 1 million research papers, keep the most relevant 50 continuously — design the system. Think heap.

**Career connection:** quant research (top signals), trading (best opportunities), risk (largest exposures), monitoring (most active assets) all continuously maintain a **Top K** with heaps.

---

## Block 6 — Python · Student Management System v45
Use `heapq`; implement `top_k_students(k)` over `[(A,95), (B,80), (C,99)]`; bonus: maintain top students dynamically.

**Linux:** learn `sort -n`; practice `sort -n marks.txt`. **Question:** why sort an entire file if you only need the top few?

---

## Quant Thinking Track — Streaming Thinking
Traditional: *receive → store everything → process later.* Heap: *receive → continuously maintain best candidates.* Stock screeners, search engines, trading dashboards, social feeds. **Question:** why is real-time maintenance often better than periodic recomputation?

---

## Portfolio Building
`DataStructures/heaps/`:
- `top_k_largest.cpp`
- `running_median_two_heaps.cpp`

---

## Communication Exercise
In 5 lines, explain: *"Why does a size-K heap beat sorting for a Top-K query?"*

---

## Journal
- Why heap size K and not N?
- How do two heaps give a running median?
- `log₂(1,000,000)` vs `log₂(10)` — how big is the gap?

---

## 🚩 Day 58 Milestone
You're done when you can answer **why use a heap for Top-K**: you don't need everything sorted, only the best K — and a heap maintains exactly that efficiently.

---

## Next 🌳⚡
Day 59 — **Hash Tables & Unordered Maps:** hashing, collisions, `unordered_map`, frequency counting — *trade memory for speed.*

---

## Tracker Update (after Day 58)
- Heaps → **50%**
- DSA: priority queue → **60%**, Top-K pattern → **40%**, running median → **20%**
- Complexity analysis → **60%**
- Streaming systems → **30%**, priority thinking → **65%**
- Python heapq → **50%**
