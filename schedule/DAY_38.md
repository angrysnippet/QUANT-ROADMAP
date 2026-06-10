# Day 38 — 🚀 Builder World · Multi-Source BFS & Number of Islands

> Yesterday: matrix → graph, cells = nodes, moves = edges. Today, one of the most important BFS patterns: **Multi-Source BFS** — start from *many* nodes at once.

**Focus:** Multi-Source BFS · Flood Fill · Number of Islands · Spread Processes · Contagion Models

---

## Previous Day Review (10 min)
- Explain why a matrix is a graph.
- Recall why BFS gives shortest paths.

---

## Block 1 — C++ (Multi-source BFS)
```
0 0 0
0 1 0
0 0 0      (1 = fire, 0 = empty)
```
Fire spreads every minute. **Insight:** push **all** fire cells into the queue initially, then run BFS. Every cell's distance becomes its time-to-burn from the *nearest* source.

*Why it matters:* seeding the queue with many sources computes "distance to nearest source" for the whole grid in one pass — exactly what spread/contagion problems need.

**Code from scratch:**
1. Normal BFS.
2. Push multiple starting nodes into the queue.
3. Track each cell's distance from the nearest source.

**Concept check:**
- Why does seeding all sources at distance 0 give nearest-source distances?
- Would running single-source BFS from each source separately give the same answer (just slower)?

---

## Block 2 — DSA (Flood fill)
```
1 1 1
1 2 2
1 2 3
```
Start at (0,0), replace all connected `1`s with `9`.

**Implement:** flood fill via BFS, and via DFS.

**Thinking question:** why do *both* work — what is actually being explored? (A connected component.)

---

## Block 3 — DSA (Number of islands)
```
1 1 0 0
1 0 0 1
0 0 1 1     (1 = land, 0 = water)
```
How many separate islands? **Key idea:** every island is a **connected component** inside a grid graph.

**Implement:** count the islands. **Thinking question:** why is this secretly the same problem as connected components in a graph?

---

## Block 4 — Quant Thinking (Spread processes)

**Problems (think BFS):**
1. A rumour starts from 3 people; each tells 2 new people/day — how does it spread?
2. A disease starts from 5 infected people — model the spread.
3. Market news reaches some traders first, then spreads — what structure models this?

**Hard puzzle:** 100 prisoners and a light bulb (revisited) — think in terms of **state, information, propagation.**

**Career connection:** contagion, rumour spread, and information diffusion are all layer-by-layer BFS — the same math behind how a shock propagates through a market.

---

## Block 5 — Mathematics (Grid combinatorics)
From (0,0) to (3,3), moving only Right or Down — how many paths? Derive it, then verify with C(6,3). **Challenge:** generalise for an (n, m) grid → C(n+m, n).

---

## Block 6 — Python · Student Management System v25
Add **study-group analysis**: `count_groups()` over `{"A":["B","C"], "B":["A"], "D":["E"]}` — literally connected components. Then find the largest study group.

---

## Quant Thinking Track — Spread
DFS thinks *explore*; BFS thinks *spread.* **Question:** why do epidemics, information spread, and heat diffusion all resemble BFS? Think in layers.

---

## Portfolio Building
`DataStructures/graphs/`:
- `multi_source_bfs.cpp`
- `flood_fill.cpp`
- `number_of_islands.cpp`

---

## Communication Exercise
In 5 lines, explain: *"Why is Number of Islands the same problem as counting connected components?"*

---

## Journal
- Why does seeding all sources give nearest-source distance?
- What is flood fill actually exploring?
- Grid → graph: what changed, what stayed the same?

---

## Day 38 Milestone
You're done when you can explain why **Number of Islands** is exactly **connected components** — the only difference being *graph → grid.*

---

## Tracker Update (after Day 38)
- Graphs → **70%**
- DSA: BFS → **70%**, DFS → **60%**, grid graphs → **50%**, connected components → **60%**
- Graph thinking → **80%**, combinatorics → **100%**
- Spread models → **25%**, network thinking → **65%**
- Python graph modeling → **35%**
