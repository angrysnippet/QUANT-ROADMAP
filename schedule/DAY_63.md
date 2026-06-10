# Day 63 — 🔥🎯 Quant Mind World · Greedy Algorithms — When the Best Immediate Choice Wins

> DP tries many possibilities, compares, then chooses. **Greedy** says: *choose the best option NOW, never look back.* Sometimes that's beautiful; sometimes it completely fails. Learning the difference is today's goal.

**Focus:** Greedy Algorithms · Local vs Global Optimum · Activity Selection · Interval Scheduling · Decision Making

---

## Previous Day Review (10 min)
- Explain Binary Search on Answer in one line.
- Recall the "decision version first" reframing.

---

## Block 1 — C++ (Activity Selection)
Activities `(1,3), (2,4), (3,5), (6,8)` — maximum non-overlapping set? **Greedy rule:** pick the activity with the **earliest finishing time.** Choose `(1,3) → (3,5) → (6,8)` = 3 activities.

*Why it matters:* finishing early leaves the most room for future activities — a greedy choice you can actually *prove* is safe.

**Code from scratch:** sort by end time; activity selection; print the chosen activities.

---

## Block 2 — DSA (Local vs global optimum)
Greedy chooses the best *immediate* move; DP chooses the best *overall future.*
- **Greedy works:** Activity Selection.
- **Greedy fails:** House Robber `[2,7,9,3,1]` — grabbing `9` isn't enough; you need future planning.

**Thinking question:** why can Activity Selection be solved greedily but House Robber cannot? **Task:** compare greedy vs DP for House Robber, Coin Change, Activity Selection.

---

## Block 3 — DSA (Interval scheduling)
Meetings `(1,2), (2,4), (3,5), (6,7)` — maximum count? **Greedy: sort by end time**, always pick the earliest-finishing meeting.

**Deep insight:** finishing early leaves more room for the future. **Challenge:** why doesn't picking the *shortest* meeting always work? Think carefully.

---

## Block 4 — Mathematics (Exchange arguments)
The maths behind greedy: assume an optimal solution exists; can you swap part of it for the greedy choice and stay optimal? If yes, greedy is correct. (For activity selection, the earliest-finishing activity always fits into *some* optimal solution.)

**Exercise:** why is "finish earliest" better than "start earliest"? **Challenge:** construct a case where "earliest start" fails.

---

## Block 5 — Quant Thinking (Decision making)

100 trading opportunities, choose 10 — always take the highest immediate profit? Maybe, maybe not.

**Problems:** project selection, research topic selection, capital allocation.

**Hard puzzle:** each project has cost, profit, duration — can greedy always maximise profit? Think.

**Career connection:** greedy shows up in job scheduling, routing, compression, trade execution. But a strong quant asks *"why is greedy **valid** here?"* — not just *"can I use greedy?"*

---

## Block 6 — Python · Student Management System v50
Add **scholarship allocation**: students `(name, merit_score)`, greedy rule = highest score first, allocate limited scholarships. Bonus: compare with a fuller optimization approach.

**Linux:** learn `crontab` conceptually — scheduled tasks. **Question:** why might OS schedulers use greedy decisions?

---

## Quant Thinking Track — Greedy = Commitment
DP keeps options open; greedy commits immediately. Benefit: fast, simple, efficient. Risk: may miss better future choices. Investing, careers, research, business. **Question:** when is committing early a strength, and when is it dangerous?

---

## Portfolio Building
`Patterns/greedy/`:
- `activity_selection.cpp`
- `interval_scheduling.cpp`

---

## Communication Exercise
In 5 lines, explain: *"Why does 'earliest finishing time' maximise the number of activities?"*

---

## Journal
- Why does greedy work for activities but fail for House Robber?
- Why "finish earliest" beats "start earliest"?
- Where might committing early hurt me?

---

## 🚩 Day 63 Milestone
You're done when you can define a greedy algorithm correctly — not "takes the largest thing", but *repeatedly makes the best local decision, hoping local choices lead to a globally optimal solution.*

---

## Next 🚀
Day 64 — **Greedy vs Dynamic Programming:** how to *tell* which a problem needs — comparing Coin Change, Activity Selection, Knapsack, House Robber.

---

## Tracker Update (after Day 63)
- Greedy algorithms → **25%**
- DSA: activity selection → **40%**, interval scheduling → **30%**
- Exchange arguments → **20%**
- Local vs global optimization → **50%**
- Python resource-allocation modeling → **60%**
