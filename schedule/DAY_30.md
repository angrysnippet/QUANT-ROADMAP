# Day 30 — 🚩 Builder World · Stack Mastery Checkpoint

> First 30 days complete. You've built: ✅ Arrays ✅ Vectors ✅ Maps & Sets ✅ Binary Search ✅ Recursion ✅ Pointers ✅ OOP Foundations ✅ Linked Lists ✅ Stack Basics. This is your first **Builder World Checkpoint.**

**Focus:** Stack using Linked List · Balanced Parentheses · Recursion = Hidden Stack · First Monthly Audit

---

## Previous Day Review (10 min)
- Explain why an array stack needs only `top`.
- Recall the overflow/underflow boundary idea.

---

## Block 1 — C++ (Stack using a linked list)

Yesterday: array stack (fixed size). **Problem with arrays:** the size is fixed. A linked-list stack grows freely:
```
TOP → 30 → 20 → 10 → NULL
```
**Key insight:** push at head = O(1), pop from head = O(1) — perfect for a stack.

**Code from scratch:** `push()`, `pop()`, `peek()` using a linked list.

**Concept check:**
- Why is the *head* the right end to push/pop for a stack?
- What does the array version gain and lose vs the linked version?

---

## Block 2 — DSA (Balanced parentheses)

Valid: `()`, `(())`, `()()`, `(()())`. Invalid: `(`, `)(`, `(()`.

**Think first:** in `(()())`, when you see `(`, what must you remember? When `)` appears, where do you find its match? *That's why the stack exists.*

**Solve:** balanced parentheses, valid parentheses (LeetCode), remove adjacent duplicates (intuition).

---

## Block 3 — Quant Thinking (Memory & state)

**Problems:**
1. Toss until **HTH** — define the states (don't solve).
2. Random walk from 0, after 6 steps — P(back at 0)? Think combinatorially.
3. Expected tosses until the first Tail — build the equation.

**Hard puzzle:** 100 prisoners each get a random number on their forehead; everyone sees others, nobody sees their own — can they deduce their own number? Think about **information flow.**

**Career connection:** "what's the smallest state that captures everything I need to predict the future?" is the modelling instinct behind every tractable quant model.

---

## Block 4 — Mathematics (Recurrence & state)
- **Warm-up:** staircase (1 or 2 steps) — ways to reach stair 6.
- **Challenge:** 1, 2, or 3 steps allowed — derive the recurrence.
- **General question:** why do recursive *counting* problems so often become recurrence relations?

---

## Block 5 — Python · Student Management System v17
**Actually implement** the undo stack now: `history = []`, push add/delete/update operations, and implement `undo()` for at least one action.

---

## Block 6 — Linux (Mini challenge — no notes)
From memory, use `ls`, `cd`, `mkdir`, `cp`, `mv`, `rm`, `grep`, `find`, `wc`, `history` to build a small project folder from scratch.

---

## Quant Thinking Track — Recursion = Hidden Stack
When `factorial(5)` calls `factorial(4)`, then `factorial(3)`, then `factorial(2)` — what must be remembered each time, and **where is it stored?** This is the deepest stack insight so far: the call stack *is* a stack.

---

## 🚩 30-Day Audit
You should now comfortably know — **C++:** functions, arrays, vectors, maps, binary search, recursion, pointers, references, OOP fundamentals, linked lists, stack basics. **Maths:** combinations, probability basics, expected value, state thinking, recurrence relations. **Python:** functions, dictionaries, OOP basics, file handling.

---

## Reflection Journal
- Which audit item still feels shaky?
- Linked-list stack vs array stack — when would I choose each?
- Where exactly does the call stack store paused work?

---

## 🏅 Reward Unlocked — Builder Rank I
You're past the "learning syntax" stage. You now build and understand data structures — a major transition.

---

## Tracker Update (after Day 30)
- Linked lists → **100%**
- Stack → **60%**
- DSA: linked lists → **100%**, stack → **60%**
- Probability → **100%**, combinatorics → **100%**
- Recurrence relations → **50%**
- State thinking → **100%**, information theory → **60%**, invariants → **40%**
- Python: file handling → **85%**, OOP → **80%**
