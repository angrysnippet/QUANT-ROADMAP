# Day 8 — 🌱 Foundation World

**Focus:** Recursion Practice · Induction Intuition · State Thinking · Python Lists

> Yesterday recursion was a concept. Today it becomes **problem-solving** — the part most people find hard at first.

---

## Previous Day Review (10 min)
- Explain why recursion needs a base case.
- Re-write recursive factorial from memory.
- Recall the one-sentence link between factorial, Fibonacci, and expected tosses.

---

## Block 1 — C++ (Recursion, deeper)

**Study** — learn:
- Recursive call-stack visualisation
- Dry-running recursion
- Recursive-tree intuition

*Why it matters:* Being able to *trace* recursion on paper is what turns it from "magic" into a tool you control.

**Resource:** continue LearnCpp — *Recursion*.

**Most important exercise (on paper):** take `factorial(4)` and draw the descent, then unwind:
```
factorial(4)
 └ factorial(3)
    └ factorial(2)
       └ factorial(1)   ← base case, then unwind back up
```

**Code from scratch:**
1. Print `1 2 3 4 5` using recursion.
2. Print `5 4 3 2 1` using recursion.
3. Sum of an array using recursion.

**Concept check:**
- In the two print problems, *where* does the print happen — before or after the recursive call? Why does that flip the order?
- What does each frame hold while it waits?

**Thinking question:** What information does each recursive call remember? (This builds your stack intuition.)

**Cross-domain mapping:** A recursive tree branches like a decision tree of market scenarios. Where else do you see branching that "unwinds"?

---

## Block 2 — DSA (Recursion patterns)

**Solve (Striver A2Z Sheet):**
1. Reverse an array using recursion.
2. Check palindrome using recursion.
3. Sum of an array using recursion.

After each solution, answer — **what does each recursive call remember?**

**System thinking:** Reversing an array with two pointers vs recursion — which uses extra memory, and where does it go?

---

## Block 3 — Quant Thinking (State thinking)

You started this with the HH-expectation and coin problems — today we formalise it.

**Problems:**
1. Expected rolls until the first 6. Let `E` = expected rolls; build the equation yourself.
2. A rabbit population doubles every month starting from 1 — how many after 12 months? Then generalise.
3. You're on stair 0 and can move 1 or 2 steps — how many ways to reach stair 4? (Think, don't code.)

**Career connection:** "Define the states, then write how they relate" is the core move behind Markov models and many trading problems.

---

## Block 4 — Mathematics (Sequences & patterns)

**Watch:** *3Blue1Brown — Essence of Algebra, Chapter 7.* Focus on recursive sequences and pattern growth.

Write down: **one intuition learned** and **one question remaining.**

---

## Block 5 — Python (Lists, properly)

**Learn:** `append()`, `remove()`, slicing.
**Resource:** Python — *Lists* tutorial.

**Write:**
1. Reverse a list.
2. Largest element.
3. Palindrome check.

---

## Block 6 — Linux

**Learn:** `chmod`, `ls -l`. Understand read / write / execute permissions — just understand them, no need to master.

---

## Quant Thinking Track — State Thinking

**Question:** Why did we create *State A* and *State B* in the HH-expectation problem? Could we solve it without states? Think deeply.

---

## Portfolio Building

Extend your toolkit — `FoundationToolkit/recursion/`:
- `reverse_recursive.cpp`
- `palindrome_recursive.cpp`
- `array_sum_recursive.cpp`

---

## Communication Exercise

In 5 lines, explain: *"What does the call stack do during recursion?"*

---

## Journal
- What pattern did I notice across the recursion problems?
- Why does the order of the print statement change the output?
- Where might "state thinking" help in real systems?

---

## Day 8 Milestone
You're done when you can solve **expected rolls until the first 6** by building the equation yourself — that proves you understand recursive expectation, not just memorised it.

---

## Tracker Update (after Day 8)
- Recursion basics → **50%**
- DSA recursion → **35%**
- State thinking → **15%**
- Recursive thinking → **30%**
- Probability fundamentals → **45%**
- Python: lists & dictionaries → **35%**
