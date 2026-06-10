# Day 34 — 🌱 Builder World · Graphs Begin — BFS for Real

> Yesterday: queue → layer exploration → BFS intuition. Today you enter **graph thinking** — one of the biggest milestones in DSA. Social networks, Google Maps, internet routing, trading networks, supply chains: all graphs.

**Focus:** Graph Representation · Nodes & Edges · BFS Traversal · Visited Array · Shortest Path

---

## Previous Day Review (10 min)
- Explain why BFS finds the shortest path with equal-cost moves.
- Recall why a queue needs both front and rear.

---

## Block 1 — C++ (What is a graph?)
```
A ---- B
|      |
C ---- D
```
Nodes: A B C D. Edges: AB, AC, BD, CD.

**Vocabulary:** node (vertex), edge, neighbour, path, degree.

**Representation — adjacency list:**
```cpp
vector<vector<int>> graph;
```

*Why it matters:* the adjacency list is how almost all real graph code stores connections — compact, and fast to list a node's neighbours.

**Code from scratch:**
1. Represent `1–2`, `1–3` as an adjacency list.
2. Print all neighbours of node 1.
3. Count the degree of each node.

**Concept check:**
- For an undirected edge A–B, how many entries go in the adjacency list?
- What does the *degree* of a node count?

---

## Block 2 — DSA (BFS traversal)
```
    1
   / \
  2   3
  |
  4
```
Start at 1 — what order does BFS visit?

**Learn:** the **queue** is the engine; the **visited array** prevents revisiting.

**Implement:** BFS traversal + print the visitation order.

**Thinking question:** what happens if you remove the visited array? (Infinite loops on cycles.)

---

## Block 3 — Quant Thinking (Network thinking)

You know 5 friends; each knows 5 friends. **How many people are at distance 2** from you (roughly)?

**Problems (think BFS):**
1. Start at 1; allowed moves `+1`, `+2`. Reach 7 in the minimum steps.
2. Reach 10 in the minimum steps.

**Hard puzzle:** jugs of 8L, 5L, 3L — measure exactly 4L. Don't solve; **model the states** (each jug-configuration is a node, each pour an edge).

**Career connection:** treating positions/configurations as nodes and moves as edges turns messy puzzles — and market states — into clean shortest-path problems.

---

## Block 4 — Mathematics (Graph distances)
For `A–B–C–D`, what is the distance A → D? **Challenge:** why does BFS always find the shortest path when all edges cost the same? Think in layers.

**Probability exercise:** random walk from 0, +1/−1, after 4 moves — P(back at 0)? Use combinations (need 2 ups and 2 downs).

---

## Block 5 — Python · Student Management System v21
Add a **relationship graph**: `friends = {"A": ["B", "C"]}`; write `show_friends()`. Your first graph-like structure.

---

## Block 6 — Linux
**Learn:** `tree` (install if needed). View a project tree and understand directory hierarchies.

---

## Quant Thinking Track — Exploration
DFS thinks **deep**; BFS thinks **nearest-first.** **Question:** why would a delivery company, a social network, and a routing algorithm each care about nearest-first exploration?

---

## Portfolio Building
Start a new area — `DataStructures/graphs/`:
- `adjacency_list.cpp`
- `bfs_traversal.cpp`

---

## Communication Exercise
In 5 lines, explain: *"What jobs do the queue and the visited array each do in BFS?"*

---

## Journal
- How many adjacency entries does one undirected edge create?
- Why does removing the visited array break BFS?
- How did I model the water-jug states as a graph?

---

## Day 34 Milestone
You're done when you can explain — conceptually — **why BFS needs both a queue and a visited array**, and what problem each one solves.

---

## Tracker Update (after Day 34)
- Graphs → **15%**
- DSA: BFS → **20%**, queue → **70%**
- Graph thinking → **25%**
- Network thinking → **20%**, layer thinking → **40%**
- Python data modeling → **55%**
