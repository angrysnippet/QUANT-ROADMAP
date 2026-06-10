# Day 22 — 🚀 Data Structures World · Linked Lists Become Real

> Yesterday: node = `data` + `next` pointer. Today you actually **manipulate memory** — where most students finally understand *why pointers exist.*

**Focus:** Dynamic Memory · `new` · Insert at Head/Tail · Traversal · State Machines

---

## Previous Day Review (10 min)
- Explain `Node* next;` in plain English.
- Recall your 3 real-world "linked list" examples.

---

## Block 1 — C++ (Dynamic memory)

**Study** — before, memory was automatic:
```cpp
int x = 10;
```
Now you request it yourself:
```cpp
int* ptr = new int(10);   // Who owns this? Who deletes it?
```
And nodes are built the same way:
```cpp
Node* n1 = new Node(10);
```

*Why it matters:* `new` gives you memory that outlives the current scope — the only way to grow a structure at runtime — but it also makes *you* responsible for it.

**Resource:** LearnCpp — dynamic allocation.

**Code from scratch:**
1. `Node* first = new Node(10);` then print `first->data`.
2. Build `10 -> 20` using pointers.
3. Build `10 -> 20 -> 30` and print all nodes.

**Concept check:**
- What does `new` return?
- Who is responsible for freeing memory allocated with `new`?
- What's stored in `first` — the node, or its address?

**Thinking question:** explain `Node* head = new Node(10);` — what got created in memory, and what does `head` actually store?

**Cross-domain mapping:** "who owns this resource and who cleans it up" — where does that question appear outside code?

---

## Block 2 — DSA (Traversal)
**Learn:** `head`, and the traversal idiom `while (temp != NULL)`.
**Implement:**
1. Print the linked list.
2. Count nodes.
3. Sum of node values.

**Thinking question:** why *can't* you do `head[3]` like an array?

---

## Block 3 — Quant Thinking (State machines)

**Problems (model, don't solve):**
1. Design states `S0, S1, S2, S3` for reaching **HHH** — what does each mean?
2. Expected tosses until **HT** — only define the states.
3. Random walk from 0, moves +1/−1, after 3 moves — list the possible positions.

**Hard puzzle:** a length-1 stick broken at random — P(longer piece > 3× shorter)? Draw first, then solve.

**Career connection:** "what state am I in, and what can I transition to?" is the literal definition of a state machine — and the skeleton of every Markov model in finance.

---

## Block 4 — Mathematics (Pascal's Triangle)
Construct:
```
1
1 1
1 2 1
1 3 3 1
1 4 6 4 1
```
**Why** is each number `above-left + above-right`? **Challenge:** use the triangle to find C(8, 3) without the formula.

---

## Block 5 — Python · Student Management System v9 (first persistence layer)
Add `save_student()` and `load_students()` using `open()`, `read()`, `write()`. Your data now survives between runs.

---

## Block 6 — Linux
**Learn:** `cat`, `head`, `tail`. Practice `head students.txt`, `tail students.txt` for quick file inspection.

---

## Quant Thinking Track — Dynamic Structures
Array = fixed. Linked list = grows when needed. **Question:** why might a stock-exchange order book prefer linked structures over fixed arrays in some places? Just think — no finance knowledge needed yet.

---

## Portfolio Building
`DataStructures/linked_list/`:
- `new_node.cpp`
- `two_nodes.cpp`
- `three_nodes_print.cpp`

---

## Communication Exercise
In 5 lines, explain: *"What does `new` do, and why does it create a responsibility?"*

---

## Journal
- What does `new` return, and who must free it?
- Why no `head[3]`?
- What did the order-book question make me realise?

---

## Day 22 Milestone
You're done when you can explain `Node* head = new Node(10);` in plain English — what got created in memory and what `head` stores.

---

## Tracker Update (after Day 22)
- Linked lists → **25%**
- Dynamic memory → **25%**
- DSA linked lists → **25%**
- Combinations → **90%**
- State thinking → **85%**
- Python file handling → **15%**
