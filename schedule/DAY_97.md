# Day 97 — 🎯⚡ Quant Mind World · Greedy Algorithms — When Local Decisions Win

> You met greedy on Day 63; today is a sharper revisit focused on *when greedy actually works* — because greedy and DP solve similar-looking optimization problems with completely different thinking.

**Focus:** Greedy Algorithms · Local vs Global Optimization · Greedy Choice Property · Interval Scheduling · Algorithm Selection

---

## Previous Day Review (10 min)
- Recall the algorithm decision tree.
- State the greedy-vs-DP one-liner (best now vs best future).

---

## Why today matters
Travelling A→B by always choosing what looks best *right now* — does that always give the best final answer? **Sometimes yes, sometimes no.** Knowing *when* is a major milestone.

---

## Block 1 — C++ (Greedy mindset)
Coins `10, 5, 1`, make 28: take largest first → `10,10,5,1,1,1` = 6 coins. (Works *because* these denominations have the greedy-choice property — not all do.)

*Why it matters:* greedy is fast and simple, but only *correct* when the problem's structure guarantees it — recognising that is the skill.

**Code from scratch:** `greedy_coin.cpp` — amount in, coins chosen out.

---

## Block 2 — DSA (Greedy vs DP)
Greedy: best immediate choice, never reconsiders (Activity Selection ✓). DP: many futures, stores results (Knapsack ✓; greedy **fails** here). Local best ≠ global best in general.

**Deep insight:** greedy needs the **greedy-choice property** — *the best local choice always leads to a globally optimal solution.*

---

## Block 3 — DSA (Activity Selection)
Activities `(1,3)(2,4)(3,5)(0,6)(5,7)`, maximise non-overlapping. **Greedy rule: pick the earliest finish time** (leaves the most room). Choose `(1,3)→(3,5)→(5,7)` = 3 activities.

**Task:** sort by finish time and select by hand.

---

## Block 4 — DSA (Interval scheduling pattern)
Keywords *intervals / meetings / schedules / events / time slots* + "maximum non-overlapping" → **sort by end time** (greedy).

**Exercise:** meetings `1-2, 2-3, 1-4, 3-5` — choose the maximum number.

---

## Block 5 — Mathematics (Proof matters)
DP works because it examines all possibilities; greedy works because we can *prove* the local choice is safe.

**Exercise:** why is "earliest finish time" safe in Activity Selection? **Challenge:** why might "earliest start time" fail? (A long early-starting activity can block many shorter ones.)

---

## Block 6 — Quant Thinking (Local decisions)

Tasks of 10 min / 2 hr / 15 min, goal = complete the *most* tasks — do the shortest first? That's greedy scheduling.

**Problems:** meeting scheduling; CPU task scheduling; resource allocation.

**Hard puzzle:** 4 hours, tasks 30m/45m/1h/3h, maximise *count* completed — what ordering, and why?

**Career connection:** greedy appears in scheduling, networking, compression, routing heuristics, and trade execution — chosen because it's fast, simple, and scalable, even when not perfectly optimal.

---

## Block 6.5 — Python · Student Management System v80
Create `greedy_intro.py`: greedy coin change; Activity Selection on `(start, end)` sorted by `end`, selecting the max set. Bonus: compare greedy vs DP on small examples (e.g. coins `1,3,4` for 6 — greedy fails).

**Linux:** learn `head`; practice `head -n 20 file.txt`. **Question:** why sample a file before full processing? Think "quick info before full work."

---

## Quant Thinking Track — Good Enough vs Perfect
DP seeks *guaranteed optimal*; greedy seeks a *fast decision* that's provably optimal for specific structures. The real skill: recognising **when greedy works and when it doesn't.**

---

## Portfolio Building
`Patterns/greedy/`:
- `greedy_coin.cpp`
- `activity_selection.cpp`

---

## Communication Exercise
In 5 lines, explain: *"What property must a problem have for greedy to be correct?"*

---

## Journal
- Why does earliest-finish beat earliest-start?
- Where did greedy coin change fail (which denominations)?
- What real scheduling do I do greedily?

---

## 🚩 Day 97 Milestone
You're done when you can define a greedy algorithm precisely: *a strategy that repeatedly makes the locally best choice, relying on the problem's structure to guarantee a globally optimal result.*

---

## Next 🚀
Day 98 — **Greedy Proofs & Exchange Arguments:** *why* greedy works and how to prove it — where algorithmic thinking becomes mathematical reasoning.

---

## Tracker Update (after Day 97)
- Dynamic programming → **85%**, graphs → **90%**
- Algorithm selection → **50%**, greedy algorithms → **25%**
- Quant thinking → **90%**, systems thinking → **82%**
