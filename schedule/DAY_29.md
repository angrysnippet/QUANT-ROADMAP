# Day 29 — 🌱 Builder World · Build a Stack From Scratch

> Yesterday you learned what a stack is and connected it to recursion. Today you **build one yourself** — because strong programmers know what `stack<int>` is doing underneath.

**Focus:** Stack Using Arrays · Stack Using Linked Lists · Overflow · Underflow · Monotonic Thinking (intro)

---

## Previous Day Review (10 min)
- Explain why recursion needs a stack.
- Recall how the stairs problem became Fibonacci.

---

## Block 1 — C++ (Array-based stack)

Picture the stack growing upward: `top → 30, 20, 10`. Store an `arr[100]` plus an integer `top`.

**Question:** when you push `40`, what changes — the whole stack, or only `top`? (Only `top`.)

**Overflow:** `arr[3]` already holds 10, 20, 30 — can you push again? **Underflow:** the stack is empty — can you pop?

*Why it matters:* a stack is just an array plus one index. Realising that demystifies it — and the overflow/underflow edges teach you to guard boundaries.

**Code from scratch:**
1. `push()`.
2. `pop()`.
3. `top()`.

**Concept check:**
- What single variable tracks the whole stack's state?
- What must `push` check before writing? What must `pop` check first?

---

## Block 2 — DSA (Stack using an array)
**Implement:** push, pop, peek, isEmpty.

**Thinking question:** why are push, pop, and top all **O(1)**?

---

## Block 3 — Quant Thinking (Information storage)

A stack remembers **recent** things.

**Problems:**
1. You walk North, East, South, West — how could a stack help you *undo* the moves?
2. Browser history — why is it basically a stack?
3. Recursion — why must unfinished calls be remembered?

**Hard puzzle:** 8 coins, one heavier, balance scale — minimum weighings? Generalise for 3ⁿ coins. Think information-theoretically (each weighing splits into 3).

**Career connection:** "keep only what's recent and relevant" is how efficient systems — and trading state — avoid drowning in history.

---

## Block 4 — Mathematics (Recurrence relations)
- **Warm-up:** Fibonacci `F(n) = F(n−1) + F(n−2)`.
- **Exercise:** derive the recurrence for climbing stairs (1 or 2 steps) yourself.
- **Challenge:** if you can climb 1, 2, **or 3** steps, find the recurrence.

---

## Block 5 — Python · Student Management System v16
Add an **undo stack**: `history = []`; every action (add/delete/update) is pushed; implement `undo_last_action()` (conceptually).

---

## Block 6 — Linux
**Learn:** `alias`, e.g. `alias ll='ls -la'`. Understand command shortcuts.

---

## Quant Thinking Track — Boundaries
A stack has **overflow** and **underflow.** **Question:** why do systems often fail at *boundaries* rather than in normal cases? Think memory limits, market crashes, bridge capacity, queue overload.

---

## Portfolio Building
`DataStructures/stack/`:
- `array_stack.cpp`
- `overflow_underflow.cpp`

---

## Communication Exercise
In 5 lines, explain: *"How can an array plus one variable behave as a stack?"*

---

## Journal
- What does `top` alone let you manage?
- Why are push/pop/top all O(1)?
- Where else do systems fail at the boundary?

---

## Day 29 Milestone
You're done when you can answer **why a stack needs only one extra variable (`top`)** to manage everything. Get that, and you understand array-based stacks at the core.

---

## Tracker Update (after Day 29)
- Stack → **35%**
- DSA stack → **35%**
- Recurrence relations → **35%**
- Information theory → **50%**
- Recursive thinking → **80%**
- Python data-structure usage → **30%**
