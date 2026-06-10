# Day 35 — 🚀 Builder World · DFS — Going Deep Instead of Wide

> Yesterday: BFS → queue → layer by layer. Today: **DFS** — the second fundamental graph traversal.

**Focus:** DFS · Recursion in Graphs · Connected Components · Graph Exploration · Stack Thinking Returns

---

## Previous Day Review (10 min)
- Explain the jobs of the queue and visited array in BFS.
- Recall how you modelled the water-jug puzzle as a graph.

---

## Block 1 — C++ (BFS vs DFS)
```
    1
    |
    2
   / \
  3   4
  |
  5
```
- **BFS** visits `1 2 3 4 5` — layer-wise.
- **DFS** visits `1 2 3 5 4` — deep first.

**Key idea:** BFS uses a **queue**; DFS uses **recursion** (or an explicit **stack**).

*Why it matters:* DFS riding on recursion ties together three things you already know — recursion → stack → DFS.

**Code from scratch:**
1. Represent the graph with `vector<vector<int>> graph;`.
2. DFS traversal — print the visitation order.
3. Compare BFS vs DFS on the same graph; observe the difference.

**Concept check:**
- Where is DFS's "stack" when you write it recursively?
- Why can two valid DFS orders differ depending on neighbour order?

---

## Block 2 — DSA (DFS traversal)
**Implement:** DFS using recursion, a visited array, count of nodes visited.

**Thinking question:** remove the visited array — what happens on this graph?
```
1 → 2
↑   ↓
4 ← 3
```
(It loops forever — the cycle is never escaped.)

---

## Block 3 — Quant Thinking (Exploration strategies)

You're in a maze. **Strategy A:** explore nearest rooms first (BFS). **Strategy B:** keep going deeper until blocked (DFS).

**Problems:**
1. Which strategy finds the shortest path faster?
2. Which uses less memory?
3. Searching a huge decision tree — which might be preferable?

**Hard puzzle:** 3 missionaries and 3 cannibals, a boat holding 2 — how do all cross safely? Don't solve fully; **model the states.**

**Career connection:** BFS vs DFS is the trade between *shortest answer* and *low memory* — the same trade you weigh when searching large strategy/decision trees.

---

## Block 4 — Mathematics (Connected components)
```
1 -- 2 -- 3      4 -- 5      6
```
How many separate groups exist? **Challenge:** if DFS starts at 1, can it ever reach 5? Why not?

**Probability exercise:** random walk from 0, +1/−1, after 6 steps — P(back at 0)? Use combinations (3 ups, 3 downs).

---

## Block 5 — Python · Student Management System v22
Add a **friend network**:
```python
friends = {"A": ["B","C"], "B": ["A","D"]}
```
Implement `show_network()` — find all friends connected to a student (DFS intuition).

---

## Block 6 — Linux
**Learn:** `man`, e.g. `man grep`. Manual pages are one of the most useful Linux skills.

---

## Quant Thinking Track — Connected Components
Social network — Group A: `A-B-C-D`, Group B: `E-F`. **Question:** can information spread from A to F? This idea appears in networks, epidemiology, markets, and supply chains.

---

## Portfolio Building
`DataStructures/graphs/`:
- `dfs_traversal.cpp`
- `bfs_vs_dfs.cpp`

---

## Communication Exercise
In 5 lines, explain: *"Why does DFS pair naturally with recursion?"*

---

## Journal
- Where is DFS's stack hidden in the recursive version?
- BFS vs DFS — which for shortest path, which for low memory?
- Can A reach F across components — why or why not?

---

## Day 35 Milestone
You're done when you can state BFS = "explore nearest first", DFS = "explore deepest first", and — more importantly — explain **why DFS naturally pairs with recursion**, connecting recursion → stack → DFS.

---

## Tracker Update (after Day 35)
- Graphs → **30%**
- DSA: BFS → **30%**, DFS → **25%**
- Graph thinking → **40%**
- Network thinking → **40%**, state thinking → **100%**
- Python data modeling → **65%**
