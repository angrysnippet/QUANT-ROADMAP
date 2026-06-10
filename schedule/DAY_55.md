# Day 55 — 🌳🚀 Quant Mind World · BST Operations — Search, Insert & Delete

> Yesterday (Day 54): the BST property, searching a BST, and why it beats a plain binary tree. Today, the three core operations that make a BST useful — **search, insert, delete.**

**Focus:** BST Operations · Recursive Search · Insertion · Deletion · Ordered Thinking

---

## Previous Day Review (10 min)
- Explain why BST search ignores half the tree each step.
- Recall why inorder traversal of a BST is sorted.

---

## Block 1 — C++ (Search)
```
          8
        /   \
       3     10
      / \      \
     1   6      14
        / \     /
       4   7   13
```
**BST rule:** `Left < Root < Right`, always. To find `7`: `8 → 3 → 6 → 7` — one path.

*Why it matters:* search, insert, and delete all reuse this single "go left or right by comparison" walk.

**Code from scratch:**
1. `bool search(Node* root, int x)`.
2. Count comparisons made.
3. Compare BST search vs linear search.

---

## Block 2 — DSA (Insertion)
Insert `5` into `8 / (3 \ 6)`: `5 < 8 → 5 > 3 → 5 < 6 → insert as 6's left child.`

**Key insight:** insertion follows *exactly the same path as search* — you walk down until you fall off, and attach there.

**Task:** manually insert `2`, `9`, `15` into today's BST.

**Thinking question:** why can insertion be done without touching most nodes?

---

## Block 3 — DSA (Deletion — the first hard operation)
- **Case 1 — leaf** (e.g. delete `1`): just remove it.
- **Case 2 — one child** (e.g. `8 \ 10`, delete `8`): promote the child.
- **Case 3 — two children** (e.g. `8 / (3, 10)`, delete `8`): hardest — replace with the **inorder successor** (or predecessor).

**Thinking question:** why can't you simply remove a two-child node directly? (Its two subtrees would be orphaned — the successor preserves ordering.)

---

## Block 4 — Quant Thinking (Ordered systems)

Student marks `70 50 90 40 60 80 95` — which makes searching easier, an unsorted list or a BST?

**Problems:**
1. Build a BST by inserting `10, 20, 30, 40, 50` in order — what happens?
2. Why is that bad?
3. How can insertion *order* affect performance?

**Hard puzzle:** insert `1..7` into an empty BST and draw the result. What does it resemble? (A linked list — height 6.)

**Career connection:** the realisation "**how data is organised determines performance**" is foundational — it returns in databases, trading engines, order books, and memory management. A quant developer asks: *how should data be organised so future operations are fast?*

---

## Block 5 — Mathematics (Height matters)
A perfect BST of 7 nodes has height 2; a chain-like BST of 7 nodes has height 6. Search cost ≈ `O(height)`.

**Exercise:** why does worst-case search take `O(h)`? **Challenge:** a perfect BST with `2ᵏ − 1` nodes — what's its height? (`k − 1`.)

---

## Block 6 — Python · Student Management System v42
Create `class StudentNode` (`name`, `roll`, `left`, `right`), implement `insert_student()`, and bonus: search a student by roll using BST logic.

**Linux:** learn `sort`; practice `sort marks.txt`. **Question:** how might a BST help internally with sorting? (Inorder = sorted.)

---

## Quant Thinking Track — Balance
A balanced BST searches fast; a degenerate chain searches slowly. **Deep question:** why do real systems spend huge effort *maintaining balance* — teams, databases, learning roadmaps, capital allocation? Because **structure → efficiency.**

---

## Portfolio Building
`DataStructures/trees/`:
- `bst_insert.cpp`
- `bst_delete.cpp`

---

## Communication Exercise
In 5 lines, explain: *"Why is deleting a two-child node the hardest BST operation?"*

---

## Journal
- Where did inserting `2, 9, 15` land them?
- What does inserting `1..7` produce, and why is it bad?
- Why does the inorder successor preserve the BST property?

---

## 🚩 Day 55 Milestone
You're done when you can answer **why BST search doesn't explore the whole tree**: the BST property eliminates half the possibilities at every step — the tree version of binary search.

---

## Next 🌳🔥
Day 56 — **Balanced Trees & AVL:** BST + balance = guaranteed fast operations, and why inserting `1 2 3 4 5 6` is a disaster that AVL trees automatically fix.

---

## Tracker Update (after Day 55)
- BST → **45%**
- DSA: BST search → **100%**, BST insert → **80%**, BST delete → **30%**
- Tree height analysis → **60%**
- Ordered systems → **40%**, structural efficiency → **35%**
- Python tree data structures → **50%**
