# Day 68 — 🌐⚠️ Quant Mind World · Bellman-Ford — When Dijkstra Fails

> Yesterday Dijkstra felt amazing — fast, elegant, greedy. Today, a deeper lesson: **every algorithm has assumptions, and when they break, the algorithm fails.** A crucial engineering insight.

**Focus:** Bellman-Ford · Negative Edge Weights · Relaxation · Negative Cycles · Algorithm Assumptions

---

## Previous Day Review (10 min)
- Explain why Dijkstra uses a priority queue.
- Recall the relaxation step.

---

## The shock
```
A --5--> B
A --2--> C
C --(-10)--> B
```
Dijkstra reports `A→B = 5` and stops happy. But `A→C→B = 2 + (-10) = -8` is far smaller. **Dijkstra can fail** with negative edges.

---

## Block 1 — C++ (Bellman-Ford idea)
Instead of trusting greedy decisions, Bellman-Ford says: **keep improving answers, again and again.** Store `dist[i]` (source = 0, others = INF), exactly like Dijkstra.

*Why it matters:* trading cleverness for brute repetition makes Bellman-Ford *correct* where Dijkstra is merely *fast* — and it can detect negative cycles.

**Code from scratch:** distance array; edge list; one relaxation pass.

---

## Block 2 — DSA (Relaxation)
The same relaxation as Dijkstra: for edge `u→v` with weight `w`, if `dist[u] + w < dist[v]` then update. The difference:
- **Dijkstra:** relaxes *intelligently* (nearest first).
- **Bellman-Ford:** relaxes *everything, repeatedly.*

**Task:** relax `A→B=5, A→C=2, C→B=-10` by hand. **Thinking question:** why might one pass be insufficient?

---

## Block 3 — DSA (Why N−1 passes?)
For `1→2→3→4`, shortest-path info must travel along up to `N−1` edges. Each pass lets information move *one step farther*, so `N−1` passes suffice.

**Task:** draw `1→2→3→4→5` and watch the shortest-path info propagate one hop per pass.

---

## Block 4 — Mathematics (Negative cycles)
`A→B=1, B→C=-3, C→A=1` totals `-1`. Going around repeatedly drives the cost down forever (−1, −2, −3, …) — so **no shortest path exists.**

**Exercise:** compute the cycle cost by hand. **Challenge:** why is shortest path *undefined* inside a negative cycle?

---

## Block 5 — Quant Thinking (Arbitrage)

Currencies USD→EUR→GBP→USD: if the loop turns \$100 into \$101, repeat forever — that's a **negative cycle** in disguise (take −log of exchange rates).

**Problems:** currency arbitrage; risk-free profit loops; market inefficiencies.

**Hard puzzle:** a trading loop yields 0.1% per cycle — what happens repeated thousands of times? Think.

**Career connection:** Bellman-Ford underlies **arbitrage detection** — model assets as a graph, take −log of rates, and a negative cycle *is* a risk-free profit loop.

---

## Block 6 — Python · Student Management System v55
Edges `[("A","B",5), ("A","C",2), ("C","B",-10)]`; implement `relax()`, run `N−1` passes. Bonus: detect a negative cycle (one more pass that still improves ⇒ cycle).

**Linux:** learn `jobs`; run it after launching background tasks. **Question:** why must an OS detect processes that never terminate? Think negative cycles.

---

## Quant Thinking Track — Assumptions Matter
Dijkstra worked *because* of "no negative edges." Violate the assumption → it fails. Models, financial theories, research papers, ML — all valid only under assumptions. **Strong thinkers always ask: what assumptions make this method valid?**

---

## Portfolio Building
`DataStructures/graphs/`:
- `bellman_ford.cpp`
- `negative_cycle_detect.cpp`

---

## Communication Exercise
In 5 lines, explain: *"Why does Bellman-Ford run N−1 relaxation passes?"*

---

## Journal
- Why is one relaxation pass not enough?
- Why is shortest path undefined in a negative cycle?
- How is currency arbitrage a negative cycle?

---

## 🚩 Day 68 Milestone
You're done when you can answer **why Dijkstra fails on negative edges**: it assumes that once a node's shortest distance is finalized it can never improve — and a negative edge violates that. That's why Bellman-Ford exists.

---

## Next 🚀
Day 69 — **Floyd-Warshall — All-Pairs Shortest Paths:** from one source → all destinations to *every* source → *every* destination, with a beautiful DP interpretation. Where graphs and dynamic programming finally meet.

---

## Tracker Update (after Day 68)
- Graph algorithms → **70%**
- DSA: Bellman-Ford → **40%**, relaxation → **80%**, negative cycles → **40%**
- Graph optimization → **65%**
- Arbitrage thinking → **25%**, assumption analysis → **50%**
- Python graph simulations → **50%**
