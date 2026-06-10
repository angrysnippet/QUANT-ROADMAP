# Day 12 — 🌱 Foundation World

**Focus:** Dereferencing · Arrays & Pointers · Memory Visualisation · Conditional Probability

> Yesterday you learned what a pointer is and that variables have addresses. Today you connect **pointers ↔ arrays** — one of the most important days in C++.

---

## Previous Day Review (10 min)
- Explain `int x = 10; int* ptr = &x;` in plain English.
- Recall your 3 "value vs reference" real-life examples.
- Re-solve one Day 11 expected-value problem.

---

## Block 1 — C++ (Arrays & pointers)

**Study** — the dereference operator `*ptr` means *"go to the address stored in `ptr` and get the value."*

**Modify through a pointer:**
```cpp
int x = 10;
int* ptr = &x;
*ptr = 20;   // What is x now? Think before running.
```

**Arrays and pointers** — try printing `arr` and `&arr[0]` and observe.

*Why it matters:* An array name *is* essentially a pointer to its first element — understanding this unlocks how C++ passes and walks over data without copying.

**Resource:** LearnCpp — pointers & arrays.

**Code from scratch:**
1. Change a value using a pointer.
2. Print `arr`, `arr+1`, `arr+2` — observe the addresses.
3. Access elements with `*(arr + i)` instead of `arr[i]`.

**Concept check:**
- Why does `arr` print an address but `arr[0]` a value?
- What does adding 1 to a pointer actually move it by?
- Are `arr[i]` and `*(arr + i)` truly equivalent?

**Thinking question:** Explain `int arr[5]; int* ptr = arr;` in plain English — what's happening in memory?

**Cross-domain mapping:** Pointer arithmetic walks contiguous memory. Where else do you "step through a block by offset" (a spreadsheet column? a tape?)?

---

## Block 2 — DSA (Recursion strengthening)

**Solve:**
1. Check palindrome (recursive).
2. Reverse an array (recursive).
3. Fibonacci.

*Goal:* become comfortable and fast with recursive calls.

**System thinking:** Passing an array to a recursive function — is the array copied each call? Why does that matter?

---

## Block 3 — Quant Thinking (Conditional probability)

**Problems (list outcomes, don't rush):**
1. Bag: 3 red, 2 blue. Pick one without replacement — P(second ball is blue)?
2. Toss 2 coins; given at least one is Head — P(both Heads)?
3. Toss 3 coins; given exactly 2 Heads — P(first coin was Head)?

**Career connection:** "Given what I already know, update the probability" is the seed of Bayesian reasoning — central to quant research.

---

## Block 4 — Mathematics (Counting & probability)

**Question:** How many ways to choose 2 students from 5? Reason before formula.

**Generalise:** can you derive C(n, 2) yourself?

Write down: **one intuition learned** and **one question remaining.**

---

## Block 5 — Python (Functions)

**Learn:** `def add(a, b):` and `return`.

**Write:**
1. Calculator functions — `add()`, `subtract()`, `multiply()`, `divide()`.
2. Max of three numbers (as a function).

---

## Block 6 — Linux

**Learn / practice:**
```
cp file1.txt file2.txt
mv file2.txt backup.txt
```

---

## Quant Thinking Track — Representation
Yesterday: *value ↔ address.* Today: *array ↔ pointer.*

**Question:** Why would a computer prefer passing addresses over copying huge amounts of data? Think in terms of memory, speed, and efficiency.

---

## Portfolio Building
Extend your toolkit — `FoundationToolkit/pointers/`:
- `modify_through_pointer.cpp`
- `pointer_arithmetic.cpp`
- `array_as_pointer.cpp`

---

## Communication Exercise
In 5 lines, explain: *"How is an array related to a pointer?"*

---

## Journal
- What does `*ptr` do, exactly?
- What surprised me about the printed addresses?
- Why is passing an address cheaper than copying?

---

## Day 12 Milestone
You're done when you can explain `int arr[5];` and `int* ptr = arr;` in plain English — what's actually happening in memory.

---

## Tracker Update (after Day 12)
- Pointers & memory → **35%**
- DSA recursion → **70%**
- Probability fundamentals → **70%**
- Conditional probability → **15%**
- Expected value → **40%**
- Python functions → **75%**
