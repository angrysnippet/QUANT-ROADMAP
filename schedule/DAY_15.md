# Day 15 — 🚀 Foundation → Advanced Foundation · OOP Begins

> 14 days of functions, arrays, STL, binary search, recursion, pointers, and references. Today begins the next milestone: **Objects.** Programs stop being *input → process → output* and start **modelling real-world entities in code.**

**Focus:** Structs · Classes · Objects · Modelling Systems · Counting · Expected Value

---

## Previous Day Review (10 min)
- Recall which C++ tasks you marked weak on Day 14 — re-attempt one.
- Explain "state representation" in one line.

---

## Block 1 — C++ (Structs)

**Study** — a `struct` groups related data. Instead of:
```cpp
string name; int roll; double marks;
```
you get:
```cpp
Student s;   // one entity holding all three
```

*Why it matters:* Modelling a "thing" as one type (instead of loose variables) is the first step from coding *operations* to designing *systems*.

**Resource:** LearnCpp — structs & user-defined types.

**Code from scratch:**
1. Create a `Student` with `name`, `roll`, `marks`.
2. Input 3 students and print them.
3. Find the student with the highest marks.

**Concept check:**
- What does a `struct` let you pass around in one go?
- Why is `Student s;` clearer than three separate variables?
- How would you make an array/vector of students?

**Thinking question:** Why is grouping into a `Student` useful once you have *many* students with *many* fields?

**Cross-domain mapping:** A `Student` models a real entity. What financial entities are natural structs — a `Trade`? an `Order`?

---

## Block 2 — DSA (Mixed revision)
**Solve** (keep old knowledge alive):
1. Largest element.
2. Binary search.
3. Palindrome (recursion).

---

## Block 3 — Quant Thinking (Expected value, deeper)

**Problems:**
1. Coin: Head → ₹10, Tail → ₹0 — expected value?
2. Toss twice; win ₹20 only if both heads — expected value?
3. Generalise: win probability `p`, reward `R` — what is the expectation?

**Hard puzzle:** Monty Hall — 3 doors, 1 car, 2 goats. You pick one, the host opens a goat door. Should you switch? Reason it, don't search.

**Career connection:** `E = p · R` is the atom of every pricing decision; Monty Hall is the classic lesson that intuition about probability is often wrong.

---

## Block 4 — Mathematics (Combinatorics)
- **Warm-up:** how many ways to choose 2 people from 6?
- **Challenge:** how many ways to arrange 6 people in a line?

Think explicitly about **selection vs arrangement** — which one cares about order?

---

## Block 5 — Python · Student Record System (upgrade)

Move to a list of dictionaries:
```python
students = []   # each: {"name": ..., "roll": ..., "marks": ...}
```
Features: add student, search student, highest marks.

---

## Block 6 — Linux
**Learn:** `history` and `!!`. Revise `grep`, `cp`, `mv`.

---

## Quant Thinking Track — Modelling
A "Student" exists in real life *and* as an object in code. **Question:** what other real-life things could be objects? Think `BankAccount`, `OrderBook`, `Trade`, `Stock`. This is the first step toward **systems thinking.**

---

## Portfolio Building
Extend your toolkit — `FoundationToolkit/oop/`:
- `student_struct.cpp`
- `students_input_print.cpp`
- `highest_marks.cpp`

---

## Communication Exercise
In 5 lines, explain: *"Why model a Student as one type instead of separate variables?"*

---

## Journal
- What does a struct make easier?
- Did switching change the Monty Hall odds — and why?
- Which real-world entities did I list as objects?

---

## Day 15 Milestone
You're ready to move on when you can clearly answer: **why is a `Student` better than separate variables for name, roll, and marks?**

---

## Tracker Update (after Day 15)
- OOP fundamentals → **10%**
- Pointers & memory → **70%**
- Arrays → **95%**
- Binary search → **65%**
- Recursion → **90%**
- Probability fundamentals → **85%**
- Expected value → **60%**
- Modelling thinking → **10%**
- Student Record project → **60%**
