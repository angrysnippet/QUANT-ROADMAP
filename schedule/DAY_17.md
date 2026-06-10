# Day 17 — 🌱 Foundation World · Constructors & Object Lifecycle

> Yesterday: classes, public/private, encapsulation. Today a practical problem — when you write `Student s;`, *how does the object get initialised?* That's what **constructors** are for.

**Focus:** Constructors · Object Initialisation · `this` Pointer (intro) · Combinatorics · Probability Intuition

---

## Previous Day Review (10 min)
- Give the practical difference between a struct and a class.
- Re-create the `Student` class with a `setMarks()` guard from memory.

---

## Block 1 — C++ (Constructors)

**Study** — a constructor initialises an object *as it's created.* Before:
```cpp
Student s;
s.name = "Aditya"; s.roll = 101;
```
After:
```cpp
Student s("Aditya", 101);
```

**Default constructor:**
```cpp
Student() { marks = 0; }
```
**Parameterised constructor:**
```cpp
Student(string n, int r) { name = n; roll = r; }
```

*Why it matters:* Constructors guarantee an object is never born in a half-set-up, invalid state — initialisation and creation become one step.

**Resource:** LearnCpp — constructors.

**Code from scratch:**
1. `Student` with `name`, `roll`; constructor initialises both.
2. Create 3 students and print them.
3. A `BankAccount` (name, balance) whose constructor initialises the account.

**Concept check:**
- What runs first when `Student s("Aditya", 101);` executes?
- When is the default constructor used?
- What's the risk of an object created without a constructor?

**Thinking question:** *Why* is a constructor useful — what practical problem does it solve (not "because OOP says so")?

**Cross-domain mapping:** Opening a `BankAccount` with a starting balance is a constructor. What other real entities must be "set up correctly" at creation?

---

## Block 2 — DSA (Binary search mastery)
**Solve:**
1. Search in a rotated sorted array (intuition only).
2. Square root using binary search.
3. First and last occurrence.

*Goal:* understand *when* binary search applies.

---

## Block 3 — Quant Thinking (Combinatorial thinking)

**Problems (derive before formula):**
1. Arrange 5 people in a line — how many ways?
2. Choose 3 from 10 — how many ways?
3. A committee of 4 from 12 — how many?

**Hard puzzle:** 8 identical balls, one heavier, a balance scale — minimum weighings? Think information-theoretically (each weighing has 3 outcomes).

**Career connection:** Counting arrangements vs selections is the grammar of probability — and probability is the grammar of quant.

---

## Block 4 — Mathematics (Pascal → combinations)
Compute 6C2, 6C3, 7C3 by reasoning, then verify with the formula.

**Thinking question:** why is nCr = nC(n−r)? Don't memorise — *visualise* (choosing who's *in* = choosing who's *out*).

---

## Block 5 — Python · Student Management System v5
Convert to `class Student:` with a constructor and `display()`. Create 3 student objects.

---

## Block 6 — Linux
**Learn:** `tar`, `zip`, `unzip`. Practice: create a folder, compress it, extract it. (Useful for shipping projects later.)

---

## Quant Thinking Track — Object Thinking
**Question:** what's the difference between `Student` and `Aditya`? One is a **blueprint**, one is an **instance.** This distinction is everywhere: Class → Object, Model → Data, Formula → Example, Distribution → Outcome.

---

## Portfolio Building
Extend your toolkit — `FoundationToolkit/oop/`:
- `student_constructor.cpp`
- `three_students.cpp`
- `bank_account.cpp`

---

## Communication Exercise
In 5 lines, explain: *"What does a constructor do, and why is it cleaner than setting fields by hand?"*

---

## Journal
- What practical problem does a constructor solve?
- When is the default vs parameterised constructor used?
- What's the blueprint-vs-instance distinction in my own words?

---

## Day 17 Milestone
You're done when you can answer **why a constructor is useful** in terms of the *practical problem it solves*, not "because OOP says so."

---

## Tracker Update (after Day 17)
- OOP fundamentals → **45%**
- Constructors → **25%**
- Binary search → **85%**
- Combinations → **55%**
- Probability fundamentals → **90%**
- Abstraction → **50%**
- Combinatorial thinking → **25%**
- Python OOP basics → **30%**
