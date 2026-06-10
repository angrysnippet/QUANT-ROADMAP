# Day 96 — 🧠⚔️ Quant Mind World · The Algorithm Selection Framework — Choosing the Right Tool

> End of your first major algorithms phase (Days 1–95). Now the most valuable skill: **algorithm selection.** The hardest question is rarely *how do I implement it?* — it's *which algorithm should I use?*

**Focus:** Algorithm Selection · Greedy vs DP · Graph vs DP · Optimization Frameworks · Systems Thinking

---

## Previous Day Review (10 min)
- Give your one-sentence definition of DP.
- Name the five DP families.

---

## Block 1 — C++ · Revision Sprint (no notes)
Skeletons from memory: `bfs(src)`, `dfs(node)`, `binarySearch(...)`, a DP array, Dijkstra (`priority_queue`). Goal: recognise tools instantly.

---

## Block 2 — DSA · The Algorithm Decision Tree
Ask in order:
1. **Shortest path?** unweighted → BFS; weighted → Dijkstra; negative edges → Bellman-Ford.
2. **Max/min/best/count with future decisions?** → DP.
3. **Best local choice, provably safe?** → Greedy.
4. **Connectivity / components?** → DFS / BFS / DSU.
5. **Fast search in sorted structure?** → Binary search.

---

## Block 3 — DSA · Greedy vs DP
**Greedy:** best choice now, never revisit (Activity Selection). **DP:** explore many futures, store results (Knapsack). Greedy asks *what looks best now?*; DP asks *what creates the best future?*

**Exercise:** classify Knapsack / Coin Change / Activity Selection / Minimum Path Sum as Greedy or DP.

---

## Block 4 — Mathematics (Complexity awareness)
Recognise instantly: Linear O(n); Logarithmic O(log n); Quadratic O(n²); Graph traversal O(V+E); Dijkstra O((V+E) log V).

**Exercise:** why is O(log n) dramatically faster than O(n) for large inputs?

---

## Block 5 — Quant Thinking (Tool selection)

Mastery isn't *knowing the hammer* — it's *knowing when to use it.*

**Problems:** shortest route → Dijkstra; max reward under budget → Knapsack; dependency optimization → Graph DP.

**Hard puzzle:** 100 study hours across DSA/Probability/Python/Projects — this resembles **Knapsack**, not graph traversal. Why?

**Career connection:** quant researchers rarely ask *"can you implement Dijkstra?"* — they ask *"can you model the problem?"* Success comes from **problem formulation**, not coding speed (portfolio → Knapsack-like; signal similarity → Sequence DP; pipelines → Graph DP; routing → graph algorithms).

---

## Block 6 — Python · Student Management System v79
Create `algorithm_selector.py`: a menu (`1 Shortest Path / 2 Optimization / 3 Matching / 4 Connectivity / 5 Search`) that takes a description and outputs **suggested algorithm + reason.** Bonus: store a keyword→algorithm map.

**Linux:** learn `man`; practice `man ls / grep / find`. **Question:** why is reading documentation a superpower?

---

## Quant Thinking Track — Algorithms Are Tools
Weak thinker: *problem → random algorithm.* Strong thinker: *problem → structure → pattern → algorithm.* That's the beginning of engineering maturity.

---

## 📊 Mastery Tracker (after Day 96)
Graphs **90%** · DP **85%** · Pattern recognition **92%** · Algorithm selection **40%** (new) · Quant thinking **88%** · Systems thinking **80%**.

---

## Communication Exercise
In 5 lines, explain: *"Why does problem formulation matter more than coding speed?"*

---

## Reflection Journal
- Which decision-tree branch do I hesitate on?
- Greedy vs DP — which still confuses me?
- What recent problem did I solve with the wrong tool first?

---

## 🚩 Day 96 Milestone
You're done when you reliably ask *"what kind of problem is this?"* **before** *"how do I solve it?"* — the habit that separates problem solvers from algorithm memorisers.

---

## Next 🚀
Day 97 — **Greedy Algorithms (revisited):** greedy choice property, exchange arguments, interval scheduling — why some optimization needs DP while other problems yield to simple local decisions.

---

## Tracker Update (after Day 96)
- Graphs → **90%**, dynamic programming → **85%**, pattern recognition → **92%**
- Algorithm selection → **40%**, quant thinking → **88%**, systems thinking → **80%**
