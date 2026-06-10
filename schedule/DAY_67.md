# Day 67 — 🌐⚡ Quant Mind World · Dijkstra's Algorithm — Finding the Cheapest Path

> Yesterday: connect everything at minimum cost (MST). Today: travel from A to B at minimum cost (shortest path) — one of the most famous algorithms in computer science.

**Focus:** Shortest Paths · Dijkstra's Algorithm · Priority Queues · Graph Optimization · Greedy

---

## Previous Day Review (10 min)
- Explain why Kruskal needs DSU.
- Recall why an MST never contains a cycle.

---

## Block 1 — C++ (Setup)
```
A --4-- B
|       |
2       3
|       |
C --1-- D
```
A→D via B = `4+3 = 7`; via C = `2+1 = 3`. Answer: **3.**

**Representation:** `vector<pair<int,int>> graph[n];` storing `(neighbour, weight)`. Keep `distance[node]` = best known distance from the source; start `distance[source] = 0`, all others `INF`.

*Why it matters:* Dijkstra combines three things you already know — a **heap** (priority queue) + **greedy** + **graphs** — into the workhorse shortest-path algorithm.

**Code from scratch:** weighted graph; distance array; print distances.

---

## Block 2 — DSA (The greedy insight)
Once a node is *extracted from the priority queue* with its best distance, no later path can improve it (with non-negative weights). That's the greedy step.

**Algorithm:** start from source → pop the minimum-distance node → relax its edges → repeat.

**Task:** run Dijkstra by hand on `A-B=1, A-C=4, B-C=2`. **Thinking question:** why always process the nearest node first?

---

## Block 3 — DSA (Relaxation)
The key operation. If `dist[B] = 10` but `dist[A] + weight(A,B) = 3 + 2 = 5` is smaller, update it:
```cpp
if (dist[u] + w < dist[v]) dist[v] = dist[u] + w;
```

**Task:** perform a relaxation by hand. **Thinking question:** why does Dijkstra repeatedly *improve* answers?

---

## Block 4 — Mathematics (Greedy correctness)
Dijkstra is greedy because it always picks the currently closest node — and with **non-negative weights** that choice is safe.

**Exercise:** `A→B=2, A→C=5, B→C=1` — shortest path A→C? **Challenge:** why would negative weights break Dijkstra? (Just think — tomorrow's topic.)

---

## Block 5 — Quant Thinking (Cost optimization)

A trading order routed A→B→execution costs 7; C→execution costs 3 — best route is the shortest path.

**Problems:** GPS navigation; internet routing; package delivery.

**Hard puzzle:** 1000 servers, each connection has a latency — find the fastest route. Think Dijkstra.

**Career connection:** network routing, trading-infra latency minimization, and market connectivity all reduce to "minimum-cost route" once **cost is modelled as edge weights.**

---

## Block 6 — Python · Student Management System v54
Students connected by collaboration strengths `(student1, student2, weight)`; keep `distance = {}`; simulate the shortest collaboration chain. Bonus: use `import heapq` for the priority queue.

**Linux:** learn `netstat` (or conceptually). **Question:** why do networks care about routes and paths? Think Dijkstra.

---

## Quant Thinking Track — Incremental Improvement
Dijkstra: *best known answer → find a better route → update.* DP improves a state, optimization improves a solution, research improves a hypothesis. Strong thinkers rarely find the perfect answer at once — they repeatedly improve estimates.

---

## Portfolio Building
`DataStructures/graphs/`:
- `dijkstra.cpp`
- `relaxation.cpp`

---

## Communication Exercise
In 5 lines, explain: *"Why does Dijkstra need a priority queue?"*

---

## Journal
- Why process the nearest unexplored node first?
- What does relaxation do?
- Why are non-negative weights required?

---

## 🚩 Day 67 Milestone
You're done when you can answer **why Dijkstra uses a priority queue** — not "because tutorials do", but *because we repeatedly need the currently closest unexplored node, and a heap provides that efficiently.* It connects heap + greedy + graphs into one algorithm.

---

## Next 🚀
Day 68 — **Bellman-Ford & negative weights:** the shocking discovery that *Dijkstra can fail*, and the lesson that every algorithm has assumptions.

---

## Tracker Update (after Day 67)
- Graph algorithms → **60%**
- DSA: Dijkstra → **40%**, priority-queue applications → **60%**, shortest paths → **40%**
- Graph optimization → **55%**
- Route optimization → **50%**, cost modeling → **40%**
- Python heap-based algorithms → **60%**
