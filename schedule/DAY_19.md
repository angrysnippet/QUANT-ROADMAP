# Day 19 — 🌱 Foundation World · Destructors, Static Members & Object Lifetime

> You know structs, classes, objects, constructors, overloading, and `this`. Today: **what happens when an object dies**, and **can all objects share something together?**

**Focus:** Destructors · Static Members · Object Lifetime · Counting · Expected Value

---

## Previous Day Review (10 min)
- Explain what `this` points to.
- Recall your 3 self-reference examples.

---

## Block 1 — C++ (Destructors & static members)

**Destructor** — runs when an object is destroyed:
```cpp
~Student() { cout << "Destroyed"; }
```
Think: **Birth → constructor, Life → methods, Death → destructor.**

**Static members** — shared across *all* objects. If students all belong to one university, each object shouldn't store the name separately:
```cpp
class Student {
public:
    static int count;   // one value shared by every object
};
```

*Why it matters:* Destructors let an object clean up after itself (freeing memory, closing files); static members model facts that belong to the *class*, not any one instance.

**Resource:** LearnCpp — destructors & static members.

**Code from scratch:**
1. Constructor that prints `Created`.
2. Destructor that prints `Destroyed` — observe the order.
3. A student counter tracking total students created via a static variable.

**Concept check:**
- In what order are objects destroyed?
- Why is a count better as `static` than as a per-object field?
- What kind of work belongs in a destructor?

**Thinking question:** why does the destructor for the *last-created* local object often run *first*?

**Cross-domain mapping:** a static "total trades today" belongs to the system, not one trade. What other shared-across-all facts fit this?

---

## Block 2 — DSA (Mixed revision)
**Solve:**
1. Frequency count.
2. Binary search.
3. Recursion — sum of digits.

**Thinking question:** which topic felt hardest — arrays, binary search, or recursion? Write *why.*

---

## Block 3 — Quant Thinking (Expected value)

**Problems:**
1. Fair die: win ₹60 on a 6, lose ₹10 otherwise — expected value?
2. Coin: Head → +₹5, Tail → −₹3 — expected gain?
3. Generalise: expected value across n outcomes — write the formula.

**Hard puzzle:** you solved expected tosses until **HH**. Now do expected tosses until **HT** — do the states change? Think carefully (the answer is surprisingly different).

**Career connection:** HH vs HT having different waiting times is a famous lesson that "obvious" symmetry can be false — exactly the traps quant interviews set.

---

## Block 4 — Mathematics (Counting)
- **Warm-up:** handshakes among 15 people?
- **Challenge:** derive the formula for n people without memorising.
- **Deeper:** why is handshake counting the same as nC2?

---

## Block 5 — Python · Student Management System v7
Add a `total_students` counter that tracks the number of objects — mimic static behaviour in Python (a class attribute).

---

## Block 6 — Linux
**Learn:** `ps`, `top`. Just understand the idea of **running processes** — no deep system knowledge needed yet.

---

## Quant Thinking Track — Shared State
**Question:** what's common between static variables, a population count, total students, and "number of trades today"? All represent something **shared across many individual objects.**

---

## Portfolio Building
Extend your toolkit — `FoundationToolkit/oop/`:
- `destructor_order.cpp`
- `student_counter_static.cpp`

---

## Communication Exercise
In 5 lines, explain: *"What's the difference between a normal member and a static member?"*

---

## Journal
- In what order did my objects get destroyed, and why?
- Did HT take the same expected tosses as HH?
- What other "shared state" exists in real systems?

---

## Day 19 Milestone
You're done when you can explain, in plain English — **constructor** (object is born), **destructor** (object dies), **static variable** (shared by all objects).

---

## Tracker Update (after Day 19)
- OOP fundamentals → **80%**
- Constructors → **80%**
- Static members → **30%**
- Destructors → **30%**
- Binary search → **95%**
- Recursion → **90%**
- Combinations → **80%**
- Expected value → **70%**
- State thinking → **60%**
- Python OOP basics → **60%**
