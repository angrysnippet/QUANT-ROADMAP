# Day 18 — 🌱 Foundation World · Constructor Overloading & the `this` Pointer

> Yesterday you initialised objects with constructors and started thinking in blueprints and instances. Today: how an object **understands itself** — exactly what the `this` pointer does.

**Focus:** Constructor Overloading · `this` Pointer · Object Lifecycle · Counting & Probability

---

## Previous Day Review (10 min)
- Explain why a constructor is practically useful.
- Recall the blueprint-vs-instance distinction.

---

## Block 1 — C++ (Overloading & `this`)

**Constructor overloading** — one class, several constructors:
```cpp
Student s1;
Student s2("Aditya");
Student s3("Aditya", 101);
```
**Question:** why might that flexibility be useful?

**The `this` pointer:**
```cpp
class Student {
    string name;
public:
    Student(string name) { this->name = name; }  // which name is which?
};
```
The object uses `this` to refer to **itself** — `this->name` is the object's field, plain `name` is the parameter.

*Why it matters:* `this` resolves the ambiguity when a parameter and a member share a name, and overloading lets one type be created in several convenient ways.

**Resource:** LearnCpp — overloading & `this`.

**Code from scratch:**
1. `Student()` — default.
2. `Student(string n)`.
3. `Student(string n, int r)`.
4. Print a message whenever a constructor runs — watch objects get created.

**Concept check:**
- How does the compiler pick which overload to call?
- In `this->name = name;`, which side is the member?
- What is `this` actually a pointer *to*?

**Thinking question:** explain in plain English why `this->name = name;` works, and what problem overloading solves.

**Cross-domain mapping:** "self-reference" — an object describing itself. Where else does a thing refer to itself?

---

## Block 2 — DSA (Binary search completion)
**Solve:**
1. Floor of square root.
2. Find peak element (intuition).
3. Binary search on the answer (introduction).

**Thinking question:** when can binary search be used on an *answer* rather than an array? Look for **monotonic behaviour.**

---

## Block 3 — Quant Thinking (Information theory begins)

**Problems:**
1. 16 cards, one marked — minimum yes/no questions?
2. Generalise: for 2ⁿ objects, how many yes/no questions?
3. 27 balls, one heavier, balance scale — minimum weighings? (Connect to the 8-ball puzzle.)

**Hard puzzle:** 100 people each shake hands once — how many handshakes? Then derive the formula for n people.

**Career connection:** "How many questions to pin down one of N possibilities?" is `log₂ N` — the literal definition of information, and the lens behind efficient search and signal.

---

## Block 4 — Mathematics (Combinatorics)
Calculate 7C2, 8C3, 10C2.

**Challenge:** give an *intuitive* argument for why nCr = nC(n−r) must always hold.

---

## Block 5 — Python · Student Management System v6
Add `calculate_grade()` returning A/B/C/D from marks, alongside the constructor, `display`, and update-marks features.

---

## Block 6 — Linux
**Learn:** `chmod`. Practice `chmod +x file.sh`. Understand read / write / execute at a high level.

---

## Quant Thinking Track — Self-Reference
Yesterday: object ← blueprint. Today: object refers to itself. **Question:** find **3 examples of self-reference outside programming** (recursion, dictionaries that define words with words, expectation equations…).

---

## Portfolio Building
Extend your toolkit — `FoundationToolkit/oop/`:
- `overloaded_constructors.cpp`
- `this_pointer.cpp`
- `constructor_trace.cpp`

---

## Communication Exercise
In 5 lines, explain: *"What does the `this` pointer refer to, and why do we need it?"*

---

## Journal
- How does the compiler choose between overloaded constructors?
- What does `this` point to?
- What were my 3 self-reference examples?

---

## Day 18 Milestone
You're done when you can explain — in plain English — why `this->name = name;` works, and what problem constructor overloading solves.

---

## Tracker Update (after Day 18)
- OOP fundamentals → **60%**
- Constructors → **60%**
- `this` pointer → **25%**
- Binary search → **95%**
- Combinations → **70%**
- Information thinking → **15%**
- Combinatorial thinking → **35%**
- Python OOP basics → **45%**
