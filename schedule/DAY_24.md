# Day 24 — 🚀 Data Structures World · Deletion & Memory Management

> Yesterday you inserted, searched, and traversed. Today you remove things **safely** — the day `new` finally meets `delete`.

**Focus:** Delete Head/Tail/by Value · Memory Leaks · Ownership Thinking

---

## Previous Day Review (10 min)
- Explain why head-insert is O(1).
- Re-implement search on a linked list from memory.

---

## Block 1 — C++ (`delete` & leaks)

**The problem:**
```cpp
Node* temp = new Node(10);   // memory allocated
```
What happens if you never delete it? **Learn:** `delete temp;` returns the memory.

**Memory leak:**
```cpp
Node* temp = new Node(10);
temp = new Node(20);   // what happened to the first node?
```
The first node is now unreachable *and* unfreed — a leak.

*Why it matters:* every `new` needs a matching `delete`. Leaks are invisible until your program slowly eats all memory — a real production failure mode.

**Code from scratch:**
1. Create a node, delete it, print messages.
2. `10 -> 20 -> 30`, delete head → `20 -> 30`.
3. Delete the tail.

**Concept check:**
- After `delete temp;`, is it safe to use `temp`?
- Why must you save `head->next` *before* deleting `head`?
- What makes a node a "leak"?

**Thinking question:** explain *conceptually* why this safely removes the first node:
```cpp
Node* temp = head;
head = head->next;
delete temp;
```

**Cross-domain mapping:** "if nobody owns it, who cleans it up?" — apply that to shared files, public goods, databases.

---

## Block 2 — DSA (Deletion)
**Implement:**
1. Delete head.
2. Delete tail.
3. Delete first occurrence — `10 -> 20 -> 30 -> 20`, delete `20` → `10 -> 30 -> 20`.

**Thinking question:** why is deleting from a linked list often easier than from an array?

---

## Block 3 — Quant Thinking (Markov state thinking)

**Problems (model, mostly):**
1. Define the minimum states for **HTH** — model only.
2. Random walk from 0, +1/−1, after 4 moves — P(returning to 0)?
3. Roll a die repeatedly — expected rolls until the first even number. Build the state equation.

**Hard puzzle:** the 100-prisoners problem (seen before) — explain *conceptually* why the cycle-following strategy beats random guessing. Think information, don't calculate.

**Career connection:** "the future depends only on the current state" is the Markov property — the simplifying assumption behind most tractable financial models.

---

## Block 4 — Mathematics (Probability & counting)
- **Warm-up:** choose 3 from 8.
- **Challenge:** a committee with President, VP, Secretary from 10 people — how many ways? **Why** is this different from ordinary selection? (Order/role matters → permutation.)

---

## Block 5 — Python · Student Management System v11
Add `delete_student()`, `update_marks()`, and **auto-save** on every modification — start thinking about the **data lifecycle.**

---

## Block 6 — Linux
**Learn:** `rm`, `rm -r` — delete files and folders. Practice **safely**, on test folders only.

---

## Quant Thinking Track — Ownership
**Question:** if nobody owns a piece of memory, who deletes it? This sounds like programming, but the same idea governs shared resources, public goods, databases, and financial risk. **Ownership** is a deep systems concept.

---

## Portfolio Building
`DataStructures/linked_list/`:
- `delete_head.cpp`
- `delete_tail.cpp`
- `delete_value.cpp`

---

## Communication Exercise
In 5 lines, explain: *"What is a memory leak, and how does `delete` prevent it?"*

---

## Journal
- Why save `head->next` before deleting `head`?
- Why is list deletion easier than array deletion?
- Where does "ownership" matter outside code?

---

## Day 24 Milestone
You're done when you can explain *conceptually* why
```cpp
Node* temp = head;
head = head->next;
delete temp;
```
safely removes the first node.

---

## Tracker Update (after Day 24)
- Linked lists → **55%**
- Dynamic memory → **50%**
- DSA linked lists → **55%**
- Combinations → **95%**
- State thinking → **95%**
- Information thinking → **25%**
- Python file handling → **50%**
