# Day 60 — 🔥⚡ Quant Mind World · Sliding Window & Two Pointers — Don't Recompute, Update

> One of the most useful algorithmic patterns in all of DSA — it appears in interviews, competitive programming, data analysis, quant research, and streaming systems.

**Focus:** Sliding Window · Fixed Window · Variable Window · Two Pointers · Incremental Computation

---

## Previous Day Review (10 min)
- Explain why a hash table gives direct lookup.
- Recall why hashing and DP are "cousins."

---

## Why this matters
For `[1,2,3,4,5,6]`, window size 3, beginners recompute each window (`1+2+3`, `2+3+4`, …). Strong programmers think: **old window → remove left → add right → update.** No recomputation.

---

## Block 1 — C++ (Fixed window)
`[1,2,3,4,5]`, `K = 3`, max window sum. Brute force recomputes; sliding window slides:
```
1+2+3 = 6
6 - 1 + 4 = 9
9 - 2 + 5 = 12
```

*Why it matters:* reusing the previous window's work turns an `O(N·K)` scan into `O(N)`.

**Code from scratch:** max sum subarray of size K; average of every window; print all window sums.

---

## Block 2 — DSA (Fixed window)
Pattern: `window_sum += arr[right]; window_sum -= arr[left];`. **Core insight:** you already know the previous window — why recompute?

**Task:** `[2,1,5,1,3,2]`, K=3 — max sum by hand. **Thinking question:** why is sliding window O(N) and not O(N·K)?

---

## Block 3 — DSA (Two pointers)
`[1,2,3,4,5]`, find a pair summing to 7. `L = 1`, `R = 5`: if `1+5 < 7` move L; `2+5 = 7` → found.

**Key idea:** two pointers eliminate large portions of the search space.

**Code from scratch:** pair sum; remove duplicates from a sorted array; two-sum (sorted version).

---

## Block 4 — Mathematics (Incremental computation)
Today's deep idea: instead of *compute again*, **update the existing answer.** `[1,2,3]=6` → moving to `[2,3,4]`: don't recompute, do `6 - 1 + 4 = 9`.

**Exercise:** `[5,2,8,1,4]`, K=2 — all window sums by hand. **Challenge:** why is remembering previous work the key to efficiency (DP, hashing, sliding window)?

---

## Block 5 — Quant Thinking (Streaming data)

Stock prices stream in; you need a **moving average** over the last 50. Recompute 50 numbers every second? No — slide.

**Problems:** (1) moving average; (2) rolling volatility; (3) recent trading volume.

**Hard puzzle:** highest trading volume in any 30-minute period over millions of records — think sliding window.

**Career connection:** moving averages, rolling correlation/variance, recent activity — quant researchers constantly ask *"what happened recently?"*, which is exactly what a sliding window answers.

---

## Block 6 — Python · Student Management System v47
Add a **recent-performance tracker**: `moving_average()` over marks via sliding window; bonus: `best_recent_streak()`.

**Linux:** learn `watch`; practice `watch date` / `watch ls`. Observe continuous updates — think streaming.

---

## Quant Thinking Track — Incremental Thinking
You've seen this repeatedly: DP (remember past *answers*), hashing (remember past *information*), sliding window (remember past *computation*). Same philosophy. **Strong solvers ask:** *what from the previous step can I reuse?* — often turning O(N²) into O(N).

---

## Portfolio Building
Start a new area — `Patterns/sliding_window/`:
- `max_sum_window.cpp`
- `two_pointer_pair_sum.cpp`

---

## Communication Exercise
In 5 lines, explain: *"How does sliding the window avoid recomputing each sum from scratch?"*

---

## Journal
- Why is sliding window O(N)?
- How do two pointers shrink the search space?
- What reuse turned an O(N²) idea into O(N)?

---

## 🏅 60-Day Milestone
Foundations built: ✅ C++ ✅ STL ✅ Recursion ✅ Linked Lists ✅ Stacks & Queues ✅ Graphs ✅ DP ✅ Trees ✅ BSTs ✅ Heaps ✅ Hashing ✅ Sliding Window. A serious foundation.

---

## Next 🚀
Day 61 — **Monotonic Stack:** next/previous greater element, histogram problems — *keep only the useful elements, discard the rest.*

---

## Tracker Update (after Day 60)
- Sliding window → **30%**
- DSA: fixed window → **40%**, two pointers → **30%**
- Incremental computation → **50%**
- Streaming systems → **50%**, rolling calculations → **30%**
- Python moving averages → **30%**
