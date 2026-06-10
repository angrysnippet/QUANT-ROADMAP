# Day 21 — 🚀 Foundation → Data Structures World · Linked Lists Begin

> Your first real data structure built from scratch. Arrays store everything together with index access. Today: **Node → pointer → Node → pointer → Node** — and you finally see *why pointers, memory, and objects matter.*

**Focus:** Linked List Intuition · Nodes · Dynamic Memory · State Representation · Geometric Probability

---

## Previous Day Review (10 min)
- Recall the 100-locker rule (which stay open, and why).
- Re-write a small class with a constructor from memory.

---

## Block 1 — C++ (Linked list intuition)

**The problem with arrays:** in `10 20 30 40 50`, inserting `25` between `20` and `30` forces everything to shift — costly.

**The linked-list idea:** each node stores `[value | next]`:
```
10 -> 20 -> 30 -> 40
```
Every node knows *where the next node lives.*

```cpp
class Node {
public:
    int data;
    Node* next;
};
```
Understand **every word** of that.

*Why it matters:* a linked list trades index access for cheap insert/delete anywhere — and it's the first structure where pointers stop being abstract and become essential.

**Resource:** LearnCpp / linked-list intro.

**Code from scratch:**
1. Create one node and print its `data`.
2. Create two nodes and connect them.
3. Print `10 -> 20` by following the pointer.

**Concept check:**
- What does `Node* next;` hold?
- What marks the end of the list?
- Why is inserting in the middle cheaper than in an array?

**Thinking question:** explain `Node* next;` in plain English — what information does it store, and why?

**Cross-domain mapping:** train coaches, web pages linking to pages, a chain of transactions. Find **3 more** real systems that behave like a linked list.

---

## Block 2 — DSA (Linked list basics)
**Learn:** node, head, tail, traversal.
**Implement:**
1. Create a linked list.
2. Traverse it.
3. Count the nodes.

**Most important question:** why do we need the **head** pointer?

---

## Block 3 — Quant Thinking (State representation)

You solved HH and HT using states. Today, represent **HHH**.

**Problems:**
1. Expected tosses until HHH — don't solve fully, just **define the states** (how many do you need?).
2. A random walk starts at 0 and moves +1 or −1 with equal probability — after 2 moves, what positions are possible?
3. Why is *current position* a **state**?

**Hard puzzle:** a stick is broken at a random point — P(longer piece is at least twice the shorter)? Draw a picture; no formula first.

**Career connection:** "the current position is all you need to know" (the Markov idea) is the foundation of random walks — the baseline model for price movement.

---

## Block 4 — Mathematics (Combinatorial thinking)
- **Warm-up:** choose 2 from 8.
- **Challenge:** handshakes among 20 people — generalise.
- **Insight:** why are handshakes and selections fundamentally the *same* problem?

---

## Block 5 — Python · Project Upgrade (first system design)
Split responsibilities into two classes:
```python
class Student: ...
class StudentManager: ...   # add / search / update / delete
```
Moving behaviour into a manager class is your first taste of **system design.**

---

## Block 6 — Linux
**Learn:** `find`. Practice:
```
find . -name "*.cpp"
find . -name "*.py"
```

---

## Quant Thinking Track — Linked Representation
Arrays: `A B C D` stored together. Linked lists: `A -> B -> C -> D`. **Question:** what real-world systems work like linked lists? Find 3 more beyond trains, web links, and transaction chains.

---

## Portfolio Building
Start a new area — `DataStructures/linked_list/`:
- `single_node.cpp`
- `connect_two_nodes.cpp`
- `traverse_list.cpp`

---

## Communication Exercise
In 5 lines, explain: *"How is a linked list different from an array, and when is each better?"*

---

## Journal
- What does `Node* next;` store?
- Why do we need a head pointer?
- How many states does HHH need?

---

## Day 21 Milestone
You're done when you can explain `Node* next;` in plain English — not the syntax, just *what information it stores and why.*

---

## Tracker Update (after Day 21)
- Linked lists → **10%**
- OOP fundamentals → **95%**
- DSA linked lists → **10%**
- Combinations → **85%**
- Probability fundamentals → **90%**
- State thinking → **80%**
- Python OOP → **75%**
