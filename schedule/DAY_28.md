# Day 28 — 🚀 Builder World · Stack — The Hidden Structure Behind Recursion

> A new chapter. You've used a stack all along without seeing it — when you called `factorial(5)`, a stack was already at work. Today you finally see it.

**Focus:** Stack Intuition · LIFO · Stack Operations · Recursion ↔ Stack · State Compression

---

## Previous Day Review (10 min)
- Recall the binary-vs-ternary information idea from Day 27.
- Re-implement reverse-a-list from memory.

---

## Block 1 — C++ (What is a stack?)

Picture a stack of plates: plate 1, 2, 3, 4. Which comes out first? **The last one placed.** That's **LIFO** — Last In, First Out.

**Operations:** `push()`, `pop()`, `top()`, `empty()`, `size()`.

*Why it matters:* a stack is the natural structure for anything that must "undo back to the most recent thing" — including the function call stack itself.

**Resource:** use the STL `stack` today — don't implement from scratch yet.

**Code from scratch:**
1. Push 10, 20, 30; print the top.
2. Pop one element; observe.
3. Print the stack contents.

**Concept check:**
- After pushing 10, 20, 30, what does `top()` return?
- What does `pop()` return — the element, or nothing?
- Why is LIFO the *wrong* model for a queue at a ticket counter?

---

## Block 2 — DSA (Stack basics)
**Implement (via STL):** push, pop, top.

**Problem — balanced parentheses (intuition only):** are `()`, `(())`, `(()())` valid? **Question:** why is a stack *perfect* for matching brackets?

---

## Block 3 — Quant Thinking (State compression)

**Problems (draw the tree, find the pattern):**
1. Climb stairs taking 1 or 2 steps — how many ways to reach stair 4?
2. How many ways to reach stair 5?
3. Generalise — can you see **Fibonacci** appearing?

**Hard puzzle:** 100 prisoners, a light bulb, no communication — can they determine when everyone has visited the room? Think about **information storage** (one prisoner as a counter).

**Career connection:** "the number of ways to reach state n depends only on a few earlier states" is the recurrence idea at the heart of dynamic programming and option pricing lattices.

---

## Block 4 — Mathematics (Recurrence relations)
You know `1 1 2 3 5 8`. Today ask **why.** Define Fibonacci recursively, then define *ways-to-climb-stairs* recursively.

**Thinking question:** why do so many systems depend only on their previous states?

---

## Block 5 — Python · Student Management System v15
Add an **undo feature** (conceptually): store recent actions (add / delete / update) on `stack = []`. The idea matters more than perfect code.

---

## Block 6 — Linux
**Learn:** `history`. Linux command history is essentially a stack — try `history` and `!!` and observe.

---

## Quant Thinking Track — LIFO Thinking
**Question:** why does recursion *naturally* use a stack? `factorial(5)` calls `factorial(4)` … `factorial(1)`. Which call finishes **first**, and which **last**? Think deeply.

---

## Portfolio Building
Start a new area — `DataStructures/stack/`:
- `stl_stack_demo.cpp`
- `push_pop_top.cpp`

---

## Communication Exercise
In 5 lines, explain: *"What does LIFO mean, and why does it match recursion?"*

---

## Journal
- Which recursive call finishes first?
- Where did Fibonacci show up in the stairs problem?
- Why is a stack ideal for bracket matching?

---

## Day 28 Milestone
You're done when you can answer — conceptually, not "because the compiler does it" — **why recursion requires a stack**: what information must be remembered while deeper calls finish?

---

## Tracker Update (after Day 28)
- Stack → **15%**
- DSA stack → **15%**
- Linked lists → **95%**
- Recurrence relations → **15%**
- Information theory → **40%**
- Recursive thinking → **75%**
- Python data-structure usage → **15%**
