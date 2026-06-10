# Day 16 — 🌱 Foundation World · Classes & Encapsulation

> Yesterday you grouped related *data* with structs. Today you add **behaviour** to data — the real beginning of OOP.

**Focus:** Classes · Objects · Public vs Private · Encapsulation · Probability Trees · Software Design

---

## Previous Day Review (10 min)
- Answer: why is a `Student` better than separate variables?
- Re-create the `Student` struct from memory.

---

## Block 1 — C++ (Classes)

**Study** — a `class` is a struct that can also bundle rules. Before:
```cpp
struct Student { string name; int roll; };
```
Now:
```cpp
class Student {
public:
    string name;
    int roll;
};
```

**Access specifiers:** `public:` vs `private:`. **Question:** why shouldn't anyone be able to do `marks = -100;` directly?

**Encapsulation:** *Data + Rules = Object.*

*Why it matters:* Hiding data behind rules (so it can't enter an invalid state) is what makes large programs safe to change — the core promise of OOP.

**Resource:** LearnCpp — classes, access specifiers, encapsulation.

**Code from scratch:**
1. A `class Student` with `name`, `roll`, `marks`.
2. Add a `display()` method.
3. Add `setMarks()` that **rejects negative marks.**

**Concept check:**
- What can `public` members do that `private` can't?
- Why route changes through `setMarks()` instead of touching `marks` directly?
- What invalid state does the guard prevent?

**Thinking question:** Practically — *why would a software engineer choose a class over a struct?*

**Cross-domain mapping:** A `BankAccount` that refuses a negative balance is encapsulation. What other entities need rules guarding their data?

---

## Block 2 — DSA (Binary search revision)
**Solve:**
1. Binary search.
2. First occurrence.
3. Last occurrence.

*Goal:* get comfortable with searching *patterns*, not just the basic case.

---

## Block 3 — Quant Thinking (Probability trees)

**Problems (draw the tree):**
1. Bag: 2 red, 3 blue. Pick 2 without replacement — P(both red)?
2. Box A chosen 70% (80% good bulbs) vs Box B 30% (50% good bulbs) — P(good bulb)?
3. Fair coin tossed twice — P(exactly one Head)? Solve by listing outcomes *and* by counting, then compare.

**Hard puzzle:** Roll two dice — which is more likely, (A) sum = 7 or (B) at least one die is 4? Reason before calculating.

**Career connection:** Tree diagrams that weight each branch by its probability are the visual form of the law of total probability — everywhere in risk modelling.

---

## Block 4 — Mathematics (Pascal's Triangle)
Study the first few rows and observe the patterns. **Question:** why do combinations appear inside Pascal's Triangle? Try to discover it before reading.

**Exercise:** find 5C0, 5C1, 5C2, 5C3 using the triangle.

---

## Block 5 — Python · Student Record System v4
Convert the logic into a class:
```python
class Student:
    ...
```
Features: add student, display student, update marks. **First exposure to OOP in Python.**

---

## Block 6 — Linux
**Learn:** `nano` (or `vim`, optional). Practice: create a file, edit it, save it.

---

## Quant Thinking Track — Abstraction
A real student has height, weight, friends, hobbies, marks, phone number. **Question:** why does your program store only `name`, `roll`, `marks`? That's **abstraction** — learning *what to ignore.*

---

## Portfolio Building
Extend your toolkit — `FoundationToolkit/oop/`:
- `student_class.cpp`
- `student_display.cpp`
- `student_setmarks_guard.cpp`

---

## Communication Exercise
In 5 lines, explain: *"What does encapsulation protect, and why?"*

---

## Journal
- What's the practical difference between a struct and a class?
- Why guard `marks` behind `setMarks()`?
- What did I choose to abstract away in my Student model?

---

## Day 16 Milestone
You're done when you can give the **practical** answer (not the textbook one) to: *what is the difference between a struct and a class — and why would an engineer choose a class?*

---

## Tracker Update (after Day 16)
- OOP fundamentals → **25%**
- Pointers & memory → **75%**
- Binary search → **75%**
- Probability fundamentals → **90%**
- Combinations → **40%**
- Modelling thinking → **25%**
- Abstraction thinking → **40%**
- Python OOP basics → **15%**
