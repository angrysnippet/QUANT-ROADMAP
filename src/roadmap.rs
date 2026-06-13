//! Static roadmap data ported verbatim from `spec.html`:
//! - `DAILY_BLOCKS`: the eight daily routine blocks and their checkable items
//!   (21 items total). Daily-check state is keyed `"{block_index}_{item_index}"`
//!   per date.
//! - `DAYS`: the day-by-day strategy schedule. `schedule_start` maps "Day 1"
//!   onto a calendar date; each subsequent day follows one calendar day later.

use std::collections::HashMap;

pub struct DailyBlock {
    pub title: &'static str,
    pub color: &'static str,
    pub items: &'static [&'static str],
}

pub const DAILY_BLOCKS: &[DailyBlock] = &[
    DailyBlock {
        title: "Block 1 — C++ theory",
        color: "#7c6fff",
        items: &[
            "Read LearnCpp for current topic only",
            "Type code from scratch (don't copy)",
            "Modify and break the code intentionally",
            "Predict output before running",
        ],
    },
    DailyBlock {
        title: "Block 2 — Implementation",
        color: "#4ecdc4",
        items: &[
            "5 easy problems on Code360",
            "1 thinking / struggle problem",
            "Debug independently before seeking help",
        ],
    },
    DailyBlock {
        title: "Block 3 — LeetCode gym",
        color: "#ff6b6b",
        items: &[
            "2–3 easy LeetCode problems (topic-wise)",
            "Focus on pattern recognition",
            "Debug and understand before moving on",
        ],
    },
    DailyBlock {
        title: "Block 4 — Mathematics",
        color: "#ffd166",
        items: &[
            "Watch 3Blue1Brown (pause and think actively)",
            "Khan Academy practice: functions / logs / sequences",
        ],
    },
    DailyBlock {
        title: "Block 5 — Python",
        color: "#06d6a0",
        items: &[
            "Study Python official tutorial or Py4E",
            "Write 2–3 tiny scripts (calculator, even/odd etc.)",
        ],
    },
    DailyBlock {
        title: "Block 6 — Linux",
        color: "#8b90a0",
        items: &["Linux Journey lesson", "Compile a C++ file from terminal"],
    },
    DailyBlock {
        title: "Block 7 — Quant thinking",
        color: "#ff9f43",
        items: &[
            "1 puzzle (brain teaser, logic, or math puzzle)",
            "1 probability question (classic or self-invented)",
            "1 reasoning problem (pattern, sequence, or argument)",
        ],
    },
    DailyBlock {
        title: "Block 8 — Journal",
        color: "#c77dff",
        items: &["Fill in today's journal", "Note the weakest concept of the day"],
    },
];

/// Total checkable items across every daily block (21).
pub fn total_daily_items() -> usize {
    DAILY_BLOCKS.iter().map(|b| b.items.len()).sum()
}

/// One block within a strategy day. `content` is raw HTML ported verbatim from
/// spec.html and rendered via `dangerous_inner_html` (styled by `.block-content`
/// & `.block-tag` rules in main.css).
pub struct StrategyBlock {
    pub color: &'static str,
    pub title: &'static str,
    pub time: &'static str,
    pub content: &'static str,
}

pub struct StrategyDay {
    pub id: i64,
    pub phase: &'static str,
    pub title: &'static str,
    /// Authored full-day schedule as Markdown. When non-empty the Strategy view
    /// renders it as a single page; when empty it falls back to the
    /// `blocks`/`success` tile breakdown below.
    pub schedule_md: &'static str,
    pub blocks: &'static [StrategyBlock],
    /// "By the end of the day you should be able to…" checklist items.
    pub success: &'static [&'static str],
}

/// `(background, foreground)` colours for a phase tag. Mirrors `PHASE_COLOR_MAP`.
pub fn phase_color(phase: &str) -> (&'static str, &'static str) {
    match phase {
        "Phase 2" => ("rgba(78,205,196,0.12)", "var(--accent2)"),
        "Phase 3" => ("rgba(255,209,102,0.12)", "var(--accent4)"),
        "Phase 4" => ("rgba(255,107,107,0.12)", "var(--accent3)"),
        "Phase 5" => ("rgba(199,125,255,0.12)", "var(--accent5)"),
        "Phase 6" => ("rgba(46,204,113,0.14)", "#2ecc71"),
        "Phase 7" => ("rgba(255,140,66,0.14)", "#ff8c42"),
        "Phase 8" => ("rgba(77,157,224,0.14)", "#4d9de0"),
        "Phase 9" => ("rgba(232,67,147,0.14)", "#e84393"),
        "Phase 10" => ("rgba(6,182,212,0.14)", "#06b6d4"),
        "Phase 11" => ("rgba(132,204,22,0.14)", "#84cc16"),
        "Phase 12" => ("rgba(139,92,246,0.14)", "#8b5cf6"),
        "Phase 13" => ("rgba(245,158,11,0.14)", "#f59e0b"),
        "Phase 14" => ("rgba(20,184,166,0.14)", "#14b8a6"),
        "Phase 15" => ("rgba(99,102,241,0.14)", "#6366f1"),
        _ => ("rgba(124,111,255,0.12)", "var(--accent)"),
    }
}

/// Authored Day 1 schedule (from `finaly daily schedule/DAY_1.docx`), as Markdown.
const DAY1_SCHEDULE: &str = r#"# Day 1 (Today)

## Block 1 — C++

- **Study:** Read [LearnCpp Arrays Fundamentals](https://www.learncpp.com) (chapters on *Arrays*, *Array indexing*, and *Array length*, 45–60 min). *Why:* Arrays let you store collections of elements in contiguous memory; mastering them is foundational for all programming and technical interviews.
- **Implement:** Without copying or using AI, write these functions from scratch:
  - Sum of an array.
  - Largest element in an array.
  - Smallest element in an array.

  (This builds your coding fluency and problem-solving with arrays.)
- **Challenge (Break the Code):** Analyze this code snippet:

  ```cpp
  int arr[5];
  cout << arr[10];
  ```

  **Question:** What happens when you run it, and why? *(This tests your understanding of array bounds and memory access.)*

## Block 2 — DSA (Arrays)

- **Problems (Striver A2Z Sheet):** Solve the following using C++:
  - Largest element in an array.
  - Second largest element in an array.
  - Check if an array is sorted.

  *Note:* These problems reinforce array manipulation skills and will appear in coding interviews.

## Block 3 — Quant Thinking (Probability)

- **Puzzle:** (The first two puzzles – locker and coin symmetry – are assumed completed.) Now solve this: You roll two dice. Which outcome is more likely?
  - A. Sum = 8
  - B. At least one die shows a 4

  **Instruction:** Think it through before calculating. *Why:* This exercise sharpens probabilistic reasoning and intuitive problem-solving, key skills for quantitative analysis.

## Block 4 — Mathematics (Algebra)

- **Watch:** *3Blue1Brown: Essence of Algebra – Chapter 1* (one video). Pause frequently to understand. *Why:* A strong intuition for algebra will help with algorithmic thinking and data modeling later.

## Block 5 — Python

- **Study:** Go through the *Python Official Tutorial – Introduction* section. *Why:* Python is widely used in software engineering and quantitative fields; early familiarity will pay off.
- **Practice:** Write two small programs:
  - A simple calculator (e.g. add/subtract/multiply/divide two numbers).
  - An even/odd number checker.

  (These exercises build basic Python syntax and problem-solving.)

## Block 6 — Linux / Tools

- **Setup:** Install or open a terminal/command-line interface. **Run Commands:**

  ```
  pwd
  ls
  mkdir test
  cd test
  ```

  *Why:* Navigating the filesystem and using the terminal are essential development skills.
- **Compile & Run:** Create a C++ file named `main.cpp` (even with a simple `int main() { return 0; }`). Then run:

  ```
  g++ main.cpp
  ./a.out
  ```

  *Why:* Compiling and running code in the terminal is a practical skill for coding projects.
- **Journal:** At the end of the day, write brief reflections:
  - What clicked today?
  - What confused me?
  - What patterns or connections did I notice?

  (Reflecting helps consolidate learning and identify gaps.)
"#;

/// Authored Day 2 schedule (from `finaly daily schedule/DAY 2.txt`).
const DAY2_SCHEDULE: &str = r#"# Day 2
🌱 Foundation World

## Previous Day Review (10 min)

- Review Day 1 notes.
- Re-solve one Day 1 problem from memory.

## Block 1 — C++

**Study** — Continue Arrays. Learn:
- Array Traversal
- Frequency Counting
- Linear Search

**Resource:** LearnCpp — Arrays and Loops

**Code from scratch:**
- Problem 1 — Frequency of a Number
- Problem 2 — Sum of Even Elements
- Problem 3 — Reverse an Array

**Concept check:**
- What is an array?
- Why are arrays stored contiguously?
- Why is indexing O(1)?
- Why is searching O(n)?

**Thinking question:** Why is `arr[n]` O(1) while searching is O(n)?

**Cross-domain mapping:** Imagine an array stores stock prices for 1000 days. How would traversal, search, and frequency counting help analyze the data?

## Block 2 — DSA

**Striver Arrays** — solve:
1. Largest Element
2. Second Largest Element
3. Remove Duplicates from Sorted Array

For every solution, write: **Brute force**, **Better**, **Optimal**.

**System thinking:** If the array contained 1 million values, would linear search still be efficient? Why?

## Block 3 — Quant Thinking

- **Problem 1 — Die roll:** P(6) vs P(Even)
- **Problem 2 — 3 coin tosses:** P(exactly 2 heads)
- **Problem 3 — Birthday problem**

No calculations — only intuition.

**Career connection:** Probability is one of the foundations of quantitative finance and risk modelling.

## Block 4 — Mathematics

**Watch:** 3Blue1Brown — *Essence of Algebra*, Chapter 2. Pause often. Write:
- One new intuition learned.
- One question you still have.

## Block 5 — Python

**Study:** `if` · `else` · loops

**Resource:** Python Official Tutorial

**Write:**
1. Multiplication Table Generator
2. Prime Number Checker

**Research habit:** Spend 5 minutes exploring one additional page in the official Python docs.

## Block 6 — Linux

**Learn:** `touch` · `rm` · `cp` · `mv`

**Lab:**
- Create `test.txt`
- Copy `test.txt` → `backup.txt`
- Move `backup.txt` → `notes/`
- Delete `test.txt`

## Portfolio Building

Create `FoundationToolkit/` and store:
- `frequency.cpp`
- `reverse.cpp`
- `even_sum.cpp`

## Communication Exercise

In 5 lines, explain: *"Why is array indexing O(1) but searching O(n)?"*

## Journal

- What pattern did I notice in arrays?
- Which quant puzzle surprised me?
- What took longest to debug?
- Where might arrays appear in finance or software systems?

## Success Criteria

- ✅ Reverse an array
- ✅ Explain O(1) vs O(n)
- ✅ Solve 3 array problems
- ✅ Derive P(exactly 2 heads)
- ✅ Write a prime checker
- ✅ Explain array complexity in your own words
- ✅ Complete Linux file lab
"#;

/// Authored Day 3 schedule (from `finaly daily schedule/DAY 3.txt`).
const DAY3_SCHEDULE: &str = r#"# Day 3
🌱 Foundation World

**Focus:** STL Vectors · Array Thinking · Counting · Probability

## Previous Day Review (10 min)

- Reverse an array from memory
- Explain O(1) vs O(n)
- Re-solve one Day 2 array problem

## Block 1 — C++

**Study** — Learn:
- What is a vector?
- Why a vector instead of an array?
- `push_back()`
- `size()`
- indexing

**Why it matters:** Vectors are dynamic arrays. Most modern C++ code uses vectors instead of raw arrays.

**Resource:** LearnCpp — Introduction to std::vector

**Code from scratch:**
- Problem 1 — Input N numbers into a vector, then print them.
- Problem 2 — Find the largest element in a vector.
- Problem 3 — Find the sum of vector elements.

**Concept check:**
- What is a vector?
- How is it different from an array?
- What does `push_back()` do?
- What does `size()` return?
- Can vectors grow automatically?

**Thinking question:** Why does `vector.push_back(x)` feel easier than `int arr[100]`? What problem is a vector solving?

**Cross-domain mapping:** Imagine storing stock prices, transaction history, or temperatures. Why might vectors be useful?

## Block 2 — DSA

**Topic:** Basic array patterns.

**Solve:**
1. Move all zeros to the end
2. Left-rotate the array by one place
3. Find the missing number (1 to N)

After every solution write: **Observation**, **Pattern**, **Time complexity**.

**System thinking:** If you had to store 10 million stock prices, would you choose an array or a vector? Why?

## Block 3 — Quant Thinking

**Goal: counting**

- **Problem 1** — How many 3-digit numbers can be formed from 1, 2, 3, 4 without repetition?
- **Problem 2** — 10 people each shake hands exactly once. How many handshakes?
- **Problem 3** — A password is 2 letters then 2 digits. How many possibilities?

Build the reasoning. Do not search formulas.

**Reflection:** What changed when repetition was allowed versus not allowed?

**Career connection:** Counting and combinatorics later become the foundation for probability, risk models, and quantitative finance.

## Block 4 — Mathematics

**Watch:** 3Blue1Brown — *Essence of Algebra*, Chapter 3. Focus on variables, relationships, and abstraction. Write one intuition learned and one question remaining.

## Block 5 — Python

**Learn:** Lists · list indexing · loops over lists.

**Resource:** Python Official Tutorial — Data Structures

**Write:**
1. Largest element in a list
2. Reverse a list

**Compare:** Python list vs C++ vector.

## Block 6 — Linux

**Learn:** `cat` · `echo` · `nano`

**Lab:**
- Create a file
- Write text into it
- Read it back
- Edit it

## Portfolio Building

In `FoundationToolkit/vectors/`, store:
- `largest.cpp`
- `sum.cpp`
- `input_output.cpp`

## Communication Exercise

Explain vectors to a beginner in 5 lines.

## Journal

- What is the difference between an array and a vector?
- What counting pattern appeared repeatedly?
- Which problem required the most thinking?
- Where might vectors appear in real software systems?

## Success Criteria

- ✅ Use vectors confidently
- ✅ Explain vector vs array
- ✅ Solve 3 DSA problems
- ✅ Complete the counting exercises
- ✅ Write the Python list programs
- ✅ Complete the Linux file lab
- ✅ Explain vectors in your own words
"#;

pub const DAYS: &[StrategyDay] = &[
    StrategyDay {
        id: 1,
        phase: "Phase 1",
        title: "Arrays Fundamentals",
        schedule_md: DAY1_SCHEDULE,
        blocks: &[
            StrategyBlock { color: "#7c6fff", title: "Block 1 — C++", time: "45–60 mins", content: r#"<div class="block-content">
        <span class="block-tag tag-resource">📖 Resource</span>
        <a href="https://www.learncpp.com/cpp-tutorial/arrays-part-i/" target="_blank">LearnCpp — Arrays Part I</a><br/>
        <span class="block-tag tag-resource">📖 Resource</span>
        <a href="https://www.learncpp.com/cpp-tutorial/array-indexing-and-static-array-length/" target="_blank">LearnCpp — Array Indexing &amp; Length</a>
        <br/><br/>
        <span class="block-tag tag-task">✍ Implement</span>
        <ul><li>Sum of all array elements</li><li>Largest element in array</li><li>Smallest element in array</li></ul>
        <span class="block-tag tag-think">🔥 Break The Code</span>
        <ul><li>Write <code>int arr[5]; cout&lt;&lt;arr[10];</code></li><li>Ask: What happens? Why? (undefined behaviour)</li></ul>
        <em>No AI. No copying. Debug yourself first.</em>
      </div>"# },
            StrategyBlock { color: "#4ecdc4", title: "Block 2 — DSA", time: "1–1.5 hrs", content: r#"<div class="block-content">
        <span class="block-tag tag-resource">📖 Resource</span>
        <a href="https://takeuforward.org/data-structure/introduction-to-arrays-data-structures/" target="_blank">Striver A2Z — Arrays</a><br/><br/>
        <span class="block-tag tag-practice">💻 Practice</span>
        <ul>
          <li><a href="https://www.naukri.com/code360/problems/largest-element-in-the-array-largest-element-in-the-array_5026279" target="_blank">Largest Element — Code360</a></li>
          <li><a href="https://www.naukri.com/code360/problems/ninja-and-the-second-order-elements_6573455" target="_blank">Second Largest Element — Code360</a></li>
          <li><a href="https://www.naukri.com/code360/problems/ninja-and-the-sorted-check_6581957" target="_blank">Check if Array Sorted — Code360</a></li>
        </ul>
        Read → think manually → brute force → code → debug yourself.
      </div>"# },
            StrategyBlock { color: "#ff9f43", title: "Block 3 — Quant Thinking", time: "30 mins", content: r#"<div class="block-content">
        <span class="block-tag tag-think">🧩 Puzzle 1</span> Locker problem — revisit. Was your reasoning correct?<br/><br/>
        <span class="block-tag tag-think">🧩 Puzzle 2</span> Coin symmetry problem — revisit.<br/><br/>
        <span class="block-tag tag-think">🎲 Probability</span>
        <ul><li>Roll two dice. What is more likely?</li><li><strong>A.</strong> Sum = 8</li><li><strong>B.</strong> At least one die shows 4</li></ul>
        <em>Draw the sample space first. Think about "at least".</em><br/><br/>
        <span class="block-tag tag-resource">📖 Resource</span>
        <a href="https://www.khanacademy.org/math/statistics-probability/probability-library/basic-theoretical-probability/a/basic-theoretical-probability-article" target="_blank">Khan Academy — Basic Probability</a>
      </div>"# },
            StrategyBlock { color: "#4ecdc4", title: "Block 4 — Mathematics", time: "1–1.5 hrs", content: r#"<div class="block-content">
        <span class="block-tag tag-resource">▶ Watch</span>
        <a href="https://www.youtube.com/watch?v=fNk_zzaMoSs" target="_blank">3Blue1Brown — Essence of Algebra, Ch.1</a><br/><br/>
        Pause every 2–3 mins. Ask: <em>what changed? what pattern? what does this mean geometrically?</em><br/>
        One video only. Do NOT binge.
      </div>"# },
            StrategyBlock { color: "#ffd166", title: "Block 5 — Python", time: "45 mins", content: r#"<div class="block-content">
        <span class="block-tag tag-resource">📖 Resource</span>
        <a href="https://docs.python.org/3/tutorial/introduction.html" target="_blank">Python Official Tutorial — Introduction</a><br/><br/>
        <span class="block-tag tag-task">✍ Write</span>
        <ul><li>Calculator (add, subtract, multiply, divide)</li><li>Even/Odd checker</li></ul>
        No libraries. Pure logic. 2–3 tiny scripts only.
      </div>"# },
            StrategyBlock { color: "#8b90a0", title: "Block 6 — Linux", time: "20–30 mins", content: r#"<div class="block-content">
        <span class="block-tag tag-resource">📖 Resource</span>
        <a href="https://linuxjourney.com/" target="_blank">Linux Journey — Grasshopper</a><br/><br/>
        <span class="block-tag tag-task">✍ Run in terminal</span>
        <ul>
          <li><code>pwd</code> — print working directory</li>
          <li><code>ls</code> — list files</li>
          <li><code>mkdir test</code> then <code>cd test</code></li>
          <li><code>g++ main.cpp</code> then <code>./a.out</code></li>
        </ul>
      </div>"# },
            StrategyBlock { color: "#c77dff", title: "Block 7 — Journal", time: "15 mins", content: r#"<div class="block-content">
        Open the Journal tab and answer all 5 questions.<br/><br/>
        <span class="block-tag tag-think">💭 Key questions</span>
        <ul><li>What clicked today?</li><li>What confused me most?</li><li>What pattern did I discover?</li></ul>
      </div>"# },
        ],
        success: &[
            "Print the sum, largest and smallest of an array",
            "Reverse an array by hand and in code",
            "Work out a basic two-dice probability",
            "Write a calculator and even/odd checker in Python",
            "Run pwd / ls / mkdir / cd and compile with g++",
        ],
    },
    StrategyDay {
        id: 2,
        phase: "Phase 1",
        title: "Array Traversal, Frequency & Search",
        schedule_md: DAY2_SCHEDULE,
        blocks: &[
            StrategyBlock { color: "#7c6fff", title: "Block 1 — C++", time: "45–60 mins", content: r#"<div class="block-content">
        <span class="block-tag tag-task">📚 Continue arrays</span> Array traversal · frequency counting · linear search.<br/><br/>
        <span class="block-tag tag-resource">📖 Resource</span>
        <a href="https://www.learncpp.com/cpp-tutorial/arrays-and-loops/" target="_blank">LearnCpp — Arrays and Loops</a><br/>
        <em>Code from scratch — don't copy.</em>
        <br/><br/>
        <span class="block-tag tag-task">✍ Write</span>
        <ul>
          <li><strong>P1.</strong> Frequency of a number in an array<br/><code>1 2 3 2 5 2</code>, target <code>2</code> → answer <code>3</code></li>
          <li><strong>P2.</strong> Sum of even elements</li>
          <li><strong>P3.</strong> Reverse an array</li>
        </ul>
        <span class="block-tag tag-think">🧠 Thinking question</span>
        Why is <code>arr[n]</code> O(1) while searching for an element is O(n)? <em>Don't search. Think.</em>
      </div>"# },
            StrategyBlock { color: "#4ecdc4", title: "Block 2 — DSA", time: "1–1.5 hrs", content: r#"<div class="block-content">
        <span class="block-tag tag-resource">📖 Resource</span>
        <a href="https://takeuforward.org/data-structure/introduction-to-arrays-data-structures/" target="_blank">Striver A2Z — Arrays</a><br/><br/>
        <span class="block-tag tag-practice">💻 Solve (Easy)</span>
        <ul>
          <li>Find Largest Element</li>
          <li>Find Second Largest Element</li>
          <li>Remove Duplicates from Sorted Array</li>
        </ul>
        <span class="block-tag tag-think">📝 After each problem, write</span>
        <ul><li><strong>Brute force:</strong></li><li><strong>Better:</strong></li><li><strong>Optimal:</strong></li></ul>
        <em>This habit is extremely valuable — build it now.</em>
      </div>"# },
            StrategyBlock { color: "#ff9f43", title: "Block 3 — Quant Thinking", time: "30 mins", content: r#"<div class="block-content">
        <span class="block-tag tag-think">🎲 Problem 1 (warmup)</span>
        Roll one die. Would you rather win if <strong>A)</strong> the number is 6, or <strong>B)</strong> the number is even? What are the probabilities?<br/><br/>
        <span class="block-tag tag-think">🪙 Problem 2</span>
        Toss 3 coins. Probability of exactly 2 heads? Try deriving it.<br/><br/>
        <span class="block-tag tag-think">🎂 Problem 3 (interesting)</span>
        A room has 23 people. Probability that at least two share a birthday — guess only (no calculation): &lt;10% · ~25% · ~50% · &gt;90%? Write your guess.
      </div>"# },
            StrategyBlock { color: "#4ecdc4", title: "Block 4 — Mathematics", time: "1–1.5 hrs", content: r#"<div class="block-content">
        <span class="block-tag tag-resource">▶ Watch</span>
        <a href="https://www.youtube.com/c/3blue1brown" target="_blank">3Blue1Brown — Essence of Algebra, Chapter 2</a><br/><br/>
        Only one chapter. Pause frequently and ask what each step means.
      </div>"# },
            StrategyBlock { color: "#ffd166", title: "Block 5 — Python", time: "45 mins", content: r#"<div class="block-content">
        <span class="block-tag tag-task">📚 Study</span> <code>if</code> · <code>else</code> · loops<br/><br/>
        <span class="block-tag tag-resource">📖 Resource</span>
        <a href="https://docs.python.org/3/tutorial/controlflow.html" target="_blank">Python Tutorial — Control Flow Tools</a><br/><br/>
        <span class="block-tag tag-task">✍ Write</span>
        <ul><li>Script 1 — Multiplication table generator</li><li>Script 2 — Prime number checker</li></ul>
      </div>"# },
            StrategyBlock { color: "#8b90a0", title: "Block 6 — Linux", time: "20–30 mins", content: r#"<div class="block-content">
        <span class="block-tag tag-task">✍ Learn &amp; actually use</span>
        <ul>
          <li><code>touch</code> — create files</li>
          <li><code>mv</code> — move / rename files</li>
          <li><code>cp</code> — copy files</li>
          <li><code>rm</code> — delete files</li>
        </ul>
        Create files, move them, delete them. Use them for real.
      </div>"# },
            StrategyBlock { color: "#c77dff", title: "Block 7 — Journal", time: "15 mins", content: r#"<div class="block-content">
        <span class="block-tag tag-think">💭 Answer</span>
        <ul><li>What pattern did I notice in arrays?</li><li>Which quant puzzle surprised me?</li><li>What took longest to debug?</li></ul>
      </div>"# },
        ],
        success: &[
            "Reverse an array yourself",
            "Explain O(1) vs O(n)",
            "Solve 3 array problems (frequency, even sum, reverse)",
            "Derive probability of exactly 2 heads in 3 tosses",
            "Write a prime checker in Python",
        ],
    },
    StrategyDay {
        id: 3,
        phase: "Phase 1",
        title: "STL Vectors, Counting & Patterns",
        schedule_md: DAY3_SCHEDULE,
        blocks: &[
            StrategyBlock { color: "#7c6fff", title: "Block 1 — C++", time: "45–60 mins", content: r#"<div class="block-content">
        <span class="block-tag tag-task">📚 Learn</span> What is a vector? Why vector instead of array? · <code>push_back()</code> · <code>size()</code> · indexing.<br/><br/>
        <span class="block-tag tag-resource">📖 Resource</span>
        <a href="https://www.learncpp.com/cpp-tutorial/introduction-to-stdvector-and-list-constructors/" target="_blank">LearnCpp — Introduction to std::vector</a><br/>
        <em>Code from scratch.</em>
        <br/><br/>
        <span class="block-tag tag-task">✍ Write</span>
        <ul>
          <li><strong>P1.</strong> Input N numbers into a vector, then print them</li>
          <li><strong>P2.</strong> Find the largest element in a vector</li>
          <li><strong>P3.</strong> Find the sum of vector elements</li>
        </ul>
        <span class="block-tag tag-think">🧠 Thinking question</span>
        Why does <code>vector.push_back(x)</code> feel easier than <code>int arr[100]</code>? What problem is vector solving?
      </div>"# },
            StrategyBlock { color: "#4ecdc4", title: "Block 2 — DSA", time: "1–1.5 hrs", content: r#"<div class="block-content">
        <span class="block-tag tag-task">📐 Topic</span> Basic array patterns.<br/><br/>
        <span class="block-tag tag-practice">💻 Solve</span>
        <ul>
          <li>Move all zeros to the end</li>
          <li>Left-rotate the array by one place</li>
          <li>Find the missing number (1 to N)</li>
        </ul>
        <span class="block-tag tag-think">📝 After every solution, write</span>
        <ul><li><strong>Observation:</strong></li><li><strong>Pattern:</strong></li><li><strong>Time complexity:</strong></li></ul>
      </div>"# },
            StrategyBlock { color: "#ff9f43", title: "Block 3 — Quant Thinking", time: "30 mins", content: r#"<div class="block-content">
        <span class="block-tag tag-think">🔢 Goal: counting</span><br/><br/>
        <span class="block-tag tag-think">Problem 1</span> How many 3-digit numbers can be formed using 1, 2, 3, 4 without repetition?<br/><br/>
        <span class="block-tag tag-think">Problem 2</span> How many handshakes occur if 10 people shake hands exactly once?<br/><br/>
        <span class="block-tag tag-think">Problem 3</span> A password is 2 letters then 2 digits. How many possibilities exist?<br/><br/>
        <em>Don't search formulas. Build the reasoning.</em>
      </div>"# },
            StrategyBlock { color: "#4ecdc4", title: "Block 4 — Mathematics", time: "1–1.5 hrs", content: r#"<div class="block-content">
        <span class="block-tag tag-resource">▶ Watch</span>
        <a href="https://www.youtube.com/c/3blue1brown" target="_blank">3Blue1Brown — Essence of Algebra, Chapter 3</a><br/><br/>
        Focus on: variables · relationships · abstraction. One chapter, pause often.
      </div>"# },
            StrategyBlock { color: "#ffd166", title: "Block 5 — Python", time: "45 mins", content: r#"<div class="block-content">
        <span class="block-tag tag-task">📚 Learn</span> Lists · list indexing · loops over lists.<br/><br/>
        <span class="block-tag tag-resource">📖 Resource</span>
        <a href="https://docs.python.org/3/tutorial/datastructures.html" target="_blank">Python Tutorial — Data Structures</a><br/><br/>
        <span class="block-tag tag-task">✍ Write</span>
        <ul><li>Script 1 — Find the largest element in a list</li><li>Script 2 — Reverse a list</li></ul>
      </div>"# },
            StrategyBlock { color: "#8b90a0", title: "Block 6 — Linux", time: "20–30 mins", content: r#"<div class="block-content">
        <span class="block-tag tag-task">✍ Learn &amp; use</span>
        <ul>
          <li><code>cat</code> — read a file</li>
          <li><code>echo</code> — print / write text</li>
          <li><code>nano</code> — edit a file</li>
        </ul>
        Create a file, write text into it, read it, then edit it — for real.
      </div>"# },
            StrategyBlock { color: "#c77dff", title: "Block 7 — Journal", time: "15 mins", content: r#"<div class="block-content">
        <span class="block-tag tag-think">💭 Answer</span>
        <ul><li>What is the difference between an array and a vector?</li><li>What counting pattern appeared repeatedly?</li><li>Which problem required the most thinking?</li></ul>
      </div>"# },
        ],
        success: &[
            "Input numbers into a vector and print them",
            "Find the largest element and sum of a vector",
            "Explain why a vector is easier than a raw array",
            "Count arrangements (3-digit numbers, handshakes, passwords)",
            "Use cat / echo / nano to create and edit a file",
        ],
    },
    // ── Days 4–66: authored Markdown schedules (rendered from /schedule/*.md) ──
    StrategyDay { id: 4, phase: "Phase 1", title: "Vectors & Hashing Basics", schedule_md: include_str!("../schedule/DAY_4.md"), blocks: &[], success: &[] },
    StrategyDay { id: 5, phase: "Phase 1", title: "Maps, Sets & Combinations", schedule_md: include_str!("../schedule/DAY_5.md"), blocks: &[], success: &[] },
    StrategyDay { id: 6, phase: "Phase 1", title: "Binary Search & Information", schedule_md: include_str!("../schedule/DAY_6.md"), blocks: &[], success: &[] },
    StrategyDay { id: 7, phase: "Phase 1", title: "Recursion & Expected Value", schedule_md: include_str!("../schedule/DAY_7.md"), blocks: &[], success: &[] },
    StrategyDay { id: 8, phase: "Phase 1", title: "Recursion Practice & State Thinking", schedule_md: include_str!("../schedule/DAY_8.md"), blocks: &[], success: &[] },
    StrategyDay { id: 9, phase: "Phase 1", title: "Recursion Mastery & Stack Intuition", schedule_md: include_str!("../schedule/DAY_9.md"), blocks: &[], success: &[] },
    StrategyDay { id: 10, phase: "Phase 1", title: "First Milestone Review", schedule_md: include_str!("../schedule/DAY_10.md"), blocks: &[], success: &[] },
    StrategyDay { id: 11, phase: "Phase 1", title: "Pointers — Intuition First", schedule_md: include_str!("../schedule/DAY_11.md"), blocks: &[], success: &[] },
    StrategyDay { id: 12, phase: "Phase 1", title: "Arrays & Pointers", schedule_md: include_str!("../schedule/DAY_12.md"), blocks: &[], success: &[] },
    StrategyDay { id: 13, phase: "Phase 1", title: "References & Bayesian Intuition", schedule_md: include_str!("../schedule/DAY_13.md"), blocks: &[], success: &[] },
    StrategyDay { id: 14, phase: "Phase 1", title: "Review Flag 2", schedule_md: include_str!("../schedule/DAY_14.md"), blocks: &[], success: &[] },
    StrategyDay { id: 15, phase: "Phase 2", title: "OOP Begins — Structs", schedule_md: include_str!("../schedule/DAY_15.md"), blocks: &[], success: &[] },
    StrategyDay { id: 16, phase: "Phase 2", title: "Classes & Encapsulation", schedule_md: include_str!("../schedule/DAY_16.md"), blocks: &[], success: &[] },
    StrategyDay { id: 17, phase: "Phase 2", title: "Constructors & Object Lifecycle", schedule_md: include_str!("../schedule/DAY_17.md"), blocks: &[], success: &[] },
    StrategyDay { id: 18, phase: "Phase 2", title: "Constructor Overloading & this", schedule_md: include_str!("../schedule/DAY_18.md"), blocks: &[], success: &[] },
    StrategyDay { id: 19, phase: "Phase 2", title: "Destructors & Static Members", schedule_md: include_str!("../schedule/DAY_19.md"), blocks: &[], success: &[] },
    StrategyDay { id: 20, phase: "Phase 2", title: "OOP Checkpoint", schedule_md: include_str!("../schedule/DAY_20.md"), blocks: &[], success: &[] },
    StrategyDay { id: 21, phase: "Phase 2", title: "Linked Lists Begin", schedule_md: include_str!("../schedule/DAY_21.md"), blocks: &[], success: &[] },
    StrategyDay { id: 22, phase: "Phase 2", title: "Dynamic Memory & Linked Lists", schedule_md: include_str!("../schedule/DAY_22.md"), blocks: &[], success: &[] },
    StrategyDay { id: 23, phase: "Phase 2", title: "Linked List Insertion & Search", schedule_md: include_str!("../schedule/DAY_23.md"), blocks: &[], success: &[] },
    StrategyDay { id: 24, phase: "Phase 2", title: "Linked List Deletion & Memory", schedule_md: include_str!("../schedule/DAY_24.md"), blocks: &[], success: &[] },
    StrategyDay { id: 25, phase: "Phase 2", title: "Reverse a Linked List", schedule_md: include_str!("../schedule/DAY_25.md"), blocks: &[], success: &[] },
    StrategyDay { id: 26, phase: "Phase 2", title: "Fast & Slow Pointers", schedule_md: include_str!("../schedule/DAY_26.md"), blocks: &[], success: &[] },
    StrategyDay { id: 27, phase: "Phase 2", title: "Linked List Checkpoint", schedule_md: include_str!("../schedule/DAY_27.md"), blocks: &[], success: &[] },
    StrategyDay { id: 28, phase: "Phase 2", title: "Stacks — LIFO", schedule_md: include_str!("../schedule/DAY_28.md"), blocks: &[], success: &[] },
    StrategyDay { id: 29, phase: "Phase 2", title: "Build a Stack From Scratch", schedule_md: include_str!("../schedule/DAY_29.md"), blocks: &[], success: &[] },
    StrategyDay { id: 30, phase: "Phase 2", title: "Stack Checkpoint", schedule_md: include_str!("../schedule/DAY_30.md"), blocks: &[], success: &[] },
    StrategyDay { id: 31, phase: "Phase 2", title: "Queues — FIFO", schedule_md: include_str!("../schedule/DAY_31.md"), blocks: &[], success: &[] },
    StrategyDay { id: 32, phase: "Phase 2", title: "Circular Queue", schedule_md: include_str!("../schedule/DAY_32.md"), blocks: &[], success: &[] },
    StrategyDay { id: 33, phase: "Phase 2", title: "Deque & BFS Intuition", schedule_md: include_str!("../schedule/DAY_33.md"), blocks: &[], success: &[] },
    StrategyDay { id: 34, phase: "Phase 2", title: "Graphs Begin — BFS", schedule_md: include_str!("../schedule/DAY_34.md"), blocks: &[], success: &[] },
    StrategyDay { id: 35, phase: "Phase 2", title: "DFS — Depth First", schedule_md: include_str!("../schedule/DAY_35.md"), blocks: &[], success: &[] },
    StrategyDay { id: 36, phase: "Phase 2", title: "Connected Components & Cycles", schedule_md: include_str!("../schedule/DAY_36.md"), blocks: &[], success: &[] },
    StrategyDay { id: 37, phase: "Phase 2", title: "Grid Graphs & Shortest Paths", schedule_md: include_str!("../schedule/DAY_37.md"), blocks: &[], success: &[] },
    StrategyDay { id: 38, phase: "Phase 2", title: "Multi-Source BFS & Islands", schedule_md: include_str!("../schedule/DAY_38.md"), blocks: &[], success: &[] },
    StrategyDay { id: 39, phase: "Phase 2", title: "Directed Graphs & Topological Sort", schedule_md: include_str!("../schedule/DAY_39.md"), blocks: &[], success: &[] },
    StrategyDay { id: 40, phase: "Phase 2", title: "Directed Cycles & Scheduling", schedule_md: include_str!("../schedule/DAY_40.md"), blocks: &[], success: &[] },
    StrategyDay { id: 41, phase: "Phase 2", title: "Graph Foundation Checkpoint", schedule_md: include_str!("../schedule/DAY_41.md"), blocks: &[], success: &[] },
    StrategyDay { id: 42, phase: "Phase 3", title: "Dynamic Programming Begins", schedule_md: include_str!("../schedule/DAY_42.md"), blocks: &[], success: &[] },
    StrategyDay { id: 43, phase: "Phase 3", title: "Memoization", schedule_md: include_str!("../schedule/DAY_43.md"), blocks: &[], success: &[] },
    StrategyDay { id: 44, phase: "Phase 3", title: "Bottom-Up DP", schedule_md: include_str!("../schedule/DAY_44.md"), blocks: &[], success: &[] },
    StrategyDay { id: 45, phase: "Phase 3", title: "Climbing Stairs — DP Recipe", schedule_md: include_str!("../schedule/DAY_45.md"), blocks: &[], success: &[] },
    StrategyDay { id: 46, phase: "Phase 3", title: "House Robber — Optimization DP", schedule_md: include_str!("../schedule/DAY_46.md"), blocks: &[], success: &[] },
    StrategyDay { id: 47, phase: "Phase 3", title: "Coin Change — State Design", schedule_md: include_str!("../schedule/DAY_47.md"), blocks: &[], success: &[] },
    StrategyDay { id: 48, phase: "Phase 3", title: "Longest Increasing Subsequence", schedule_md: include_str!("../schedule/DAY_48.md"), blocks: &[], success: &[] },
    StrategyDay { id: 49, phase: "Phase 3", title: "Knapsack", schedule_md: include_str!("../schedule/DAY_49.md"), blocks: &[], success: &[] },
    StrategyDay { id: 50, phase: "Phase 3", title: "DP Checkpoint", schedule_md: include_str!("../schedule/DAY_50.md"), blocks: &[], success: &[] },
    StrategyDay { id: 51, phase: "Phase 3", title: "Trees Begin", schedule_md: include_str!("../schedule/DAY_51.md"), blocks: &[], success: &[] },
    StrategyDay { id: 52, phase: "Phase 3", title: "Tree Traversals", schedule_md: include_str!("../schedule/DAY_52.md"), blocks: &[], success: &[] },
    StrategyDay { id: 53, phase: "Phase 3", title: "Height, Depth & Diameter", schedule_md: include_str!("../schedule/DAY_53.md"), blocks: &[], success: &[] },
    StrategyDay { id: 54, phase: "Phase 3", title: "Binary Search Trees Begin", schedule_md: include_str!("../schedule/DAY_54.md"), blocks: &[], success: &[] },
    StrategyDay { id: 55, phase: "Phase 3", title: "BST Operations", schedule_md: include_str!("../schedule/DAY_55.md"), blocks: &[], success: &[] },
    StrategyDay { id: 56, phase: "Phase 3", title: "Balanced & AVL Trees", schedule_md: include_str!("../schedule/DAY_56.md"), blocks: &[], success: &[] },
    StrategyDay { id: 57, phase: "Phase 3", title: "Heaps & Priority Queues", schedule_md: include_str!("../schedule/DAY_57.md"), blocks: &[], success: &[] },
    StrategyDay { id: 58, phase: "Phase 3", title: "Heap Applications & Top-K", schedule_md: include_str!("../schedule/DAY_58.md"), blocks: &[], success: &[] },
    StrategyDay { id: 59, phase: "Phase 3", title: "Hash Tables", schedule_md: include_str!("../schedule/DAY_59.md"), blocks: &[], success: &[] },
    StrategyDay { id: 60, phase: "Phase 3", title: "Sliding Window & Two Pointers", schedule_md: include_str!("../schedule/DAY_60.md"), blocks: &[], success: &[] },
    StrategyDay { id: 61, phase: "Phase 3", title: "Monotonic Stack", schedule_md: include_str!("../schedule/DAY_61.md"), blocks: &[], success: &[] },
    StrategyDay { id: 62, phase: "Phase 3", title: "Binary Search on Answer", schedule_md: include_str!("../schedule/DAY_62.md"), blocks: &[], success: &[] },
    StrategyDay { id: 63, phase: "Phase 3", title: "Greedy Algorithms", schedule_md: include_str!("../schedule/DAY_63.md"), blocks: &[], success: &[] },
    StrategyDay { id: 64, phase: "Phase 3", title: "Greedy vs Dynamic Programming", schedule_md: include_str!("../schedule/DAY_64.md"), blocks: &[], success: &[] },
    StrategyDay { id: 65, phase: "Phase 3", title: "Disjoint Set Union", schedule_md: include_str!("../schedule/DAY_65.md"), blocks: &[], success: &[] },
    StrategyDay { id: 66, phase: "Phase 3", title: "MST & Kruskal's Algorithm", schedule_md: include_str!("../schedule/DAY_66.md"), blocks: &[], success: &[] },
    StrategyDay { id: 67, phase: "Phase 4", title: "Dijkstra's Algorithm", schedule_md: include_str!("../schedule/DAY_67.md"), blocks: &[], success: &[] },
    StrategyDay { id: 68, phase: "Phase 4", title: "Bellman-Ford & Negative Weights", schedule_md: include_str!("../schedule/DAY_68.md"), blocks: &[], success: &[] },
    StrategyDay { id: 69, phase: "Phase 4", title: "Floyd-Warshall — All-Pairs Shortest Paths", schedule_md: include_str!("../schedule/DAY_69.md"), blocks: &[], success: &[] },
    StrategyDay { id: 70, phase: "Phase 4", title: "Kahn's Algorithm — Topological Sort", schedule_md: include_str!("../schedule/DAY_70.md"), blocks: &[], success: &[] },
    StrategyDay { id: 71, phase: "Phase 4", title: "Directed Cycle Detection & Deadlocks", schedule_md: include_str!("../schedule/DAY_71.md"), blocks: &[], success: &[] },
    StrategyDay { id: 72, phase: "Phase 4", title: "Strongly Connected Components", schedule_md: include_str!("../schedule/DAY_72.md"), blocks: &[], success: &[] },
    StrategyDay { id: 73, phase: "Phase 4", title: "Bridges & Articulation Points", schedule_md: include_str!("../schedule/DAY_73.md"), blocks: &[], success: &[] },
    StrategyDay { id: 74, phase: "Phase 4", title: "Tarjan's Algorithm", schedule_md: include_str!("../schedule/DAY_74.md"), blocks: &[], success: &[] },
    StrategyDay { id: 75, phase: "Phase 4", title: "Graph Pattern Recognition", schedule_md: include_str!("../schedule/DAY_75.md"), blocks: &[], success: &[] },
    StrategyDay { id: 76, phase: "Phase 4", title: "Graph Interview Patterns", schedule_md: include_str!("../schedule/DAY_76.md"), blocks: &[], success: &[] },
    StrategyDay { id: 77, phase: "Phase 4", title: "Graph Mastery Checkpoint", schedule_md: include_str!("../schedule/DAY_77.md"), blocks: &[], success: &[] },
    StrategyDay { id: 78, phase: "Phase 4", title: "DP Revisited — Optimization Thinking", schedule_md: include_str!("../schedule/DAY_78.md"), blocks: &[], success: &[] },
    StrategyDay { id: 79, phase: "Phase 4", title: "Memoization vs Tabulation", schedule_md: include_str!("../schedule/DAY_79.md"), blocks: &[], success: &[] },
    StrategyDay { id: 80, phase: "Phase 4", title: "Space Optimization in DP", schedule_md: include_str!("../schedule/DAY_80.md"), blocks: &[], success: &[] },
    StrategyDay { id: 81, phase: "Phase 4", title: "0/1 Knapsack — King of DP", schedule_md: include_str!("../schedule/DAY_81.md"), blocks: &[], success: &[] },
    StrategyDay { id: 82, phase: "Phase 4", title: "Knapsack Variations", schedule_md: include_str!("../schedule/DAY_82.md"), blocks: &[], success: &[] },
    StrategyDay { id: 83, phase: "Phase 4", title: "Longest Increasing Subsequence", schedule_md: include_str!("../schedule/DAY_83.md"), blocks: &[], success: &[] },
    StrategyDay { id: 84, phase: "Phase 4", title: "Longest Common Subsequence", schedule_md: include_str!("../schedule/DAY_84.md"), blocks: &[], success: &[] },
    StrategyDay { id: 85, phase: "Phase 4", title: "Edit Distance", schedule_md: include_str!("../schedule/DAY_85.md"), blocks: &[], success: &[] },
    StrategyDay { id: 86, phase: "Phase 4", title: "Advanced Sequence DP", schedule_md: include_str!("../schedule/DAY_86.md"), blocks: &[], success: &[] },
    StrategyDay { id: 87, phase: "Phase 4", title: "DP Pattern Recognition Checkpoint", schedule_md: include_str!("../schedule/DAY_87.md"), blocks: &[], success: &[] },
    StrategyDay { id: 88, phase: "Phase 4", title: "Grid DP — Unique Paths", schedule_md: include_str!("../schedule/DAY_88.md"), blocks: &[], success: &[] },
    StrategyDay { id: 89, phase: "Phase 4", title: "Minimum Path Sum", schedule_md: include_str!("../schedule/DAY_89.md"), blocks: &[], success: &[] },
    StrategyDay { id: 90, phase: "Phase 4", title: "Grid DP with Obstacles", schedule_md: include_str!("../schedule/DAY_90.md"), blocks: &[], success: &[] },
    StrategyDay { id: 91, phase: "Phase 4", title: "Grid DP Checkpoint", schedule_md: include_str!("../schedule/DAY_91.md"), blocks: &[], success: &[] },
    StrategyDay { id: 92, phase: "Phase 4", title: "Graph DP (DP on DAGs)", schedule_md: include_str!("../schedule/DAY_92.md"), blocks: &[], success: &[] },
    StrategyDay { id: 93, phase: "Phase 4", title: "Longest Path in a DAG", schedule_md: include_str!("../schedule/DAY_93.md"), blocks: &[], success: &[] },
    StrategyDay { id: 94, phase: "Phase 4", title: "Graph DP Checkpoint", schedule_md: include_str!("../schedule/DAY_94.md"), blocks: &[], success: &[] },
    StrategyDay { id: 95, phase: "Phase 4", title: "DP Master Checkpoint", schedule_md: include_str!("../schedule/DAY_95.md"), blocks: &[], success: &[] },
    StrategyDay { id: 96, phase: "Phase 4", title: "Algorithm Selection Framework", schedule_md: include_str!("../schedule/DAY_96.md"), blocks: &[], success: &[] },
    StrategyDay { id: 97, phase: "Phase 4", title: "Greedy Algorithms", schedule_md: include_str!("../schedule/DAY_97.md"), blocks: &[], success: &[] },
    StrategyDay { id: 98, phase: "Phase 4", title: "Greedy Proofs & Exchange Arguments", schedule_md: include_str!("../schedule/DAY_98.md"), blocks: &[], success: &[] },
    StrategyDay { id: 99, phase: "Phase 4", title: "Greedy Failures", schedule_md: include_str!("../schedule/DAY_99.md"), blocks: &[], success: &[] },
    StrategyDay { id: 100, phase: "Phase 4", title: "Algorithm Selection Master Checkpoint", schedule_md: include_str!("../schedule/DAY_100.md"), blocks: &[], success: &[] },
    StrategyDay { id: 101, phase: "Phase 5", title: "Advanced Problem Solving — Combining Algorithms", schedule_md: include_str!("../schedule/DAY_101.md"), blocks: &[], success: &[] },
    StrategyDay { id: 102, phase: "Phase 5", title: "Binary Search on Answers", schedule_md: include_str!("../schedule/DAY_102.md"), blocks: &[], success: &[] },
    StrategyDay { id: 103, phase: "Phase 5", title: "Aggressive Cows", schedule_md: include_str!("../schedule/DAY_103.md"), blocks: &[], success: &[] },
    StrategyDay { id: 104, phase: "Phase 5", title: "Binary Search + Greedy Checkpoint", schedule_md: include_str!("../schedule/DAY_104.md"), blocks: &[], success: &[] },
    StrategyDay { id: 105, phase: "Phase 5", title: "Advanced Problem Solving Checkpoint I", schedule_md: include_str!("../schedule/DAY_105.md"), blocks: &[], success: &[] },
    StrategyDay { id: 106, phase: "Phase 5", title: "Multi-Pattern I — Binary Search + Greedy", schedule_md: include_str!("../schedule/DAY_106.md"), blocks: &[], success: &[] },
    StrategyDay { id: 107, phase: "Phase 5", title: "Multi-Pattern II — Graphs + DP", schedule_md: include_str!("../schedule/DAY_107.md"), blocks: &[], success: &[] },
    StrategyDay { id: 108, phase: "Phase 5", title: "Multi-Pattern III — Shortest Paths + Constraints", schedule_md: include_str!("../schedule/DAY_108.md"), blocks: &[], success: &[] },
    StrategyDay { id: 109, phase: "Phase 5", title: "State Space Modelling", schedule_md: include_str!("../schedule/DAY_109.md"), blocks: &[], success: &[] },
    StrategyDay { id: 110, phase: "Phase 5", title: "State Space Search — BFS/Dijkstra/DP Unified", schedule_md: include_str!("../schedule/DAY_110.md"), blocks: &[], success: &[] },
    StrategyDay { id: 111, phase: "Phase 5", title: "DP as Shortest Path in a DAG", schedule_md: include_str!("../schedule/DAY_111.md"), blocks: &[], success: &[] },
    StrategyDay { id: 112, phase: "Phase 5", title: "Unified Optimization — Greedy / DP / Shortest Paths", schedule_md: include_str!("../schedule/DAY_112.md"), blocks: &[], success: &[] },
    StrategyDay { id: 113, phase: "Phase 5", title: "The Meta-Skill of Algorithm Design", schedule_md: include_str!("../schedule/DAY_113.md"), blocks: &[], success: &[] },
    StrategyDay { id: 114, phase: "Phase 5", title: "Complexity as a Design Tool", schedule_md: include_str!("../schedule/DAY_114.md"), blocks: &[], success: &[] },
    StrategyDay { id: 115, phase: "Phase 5", title: "Space Complexity & Engineering Tradeoffs", schedule_md: include_str!("../schedule/DAY_115.md"), blocks: &[], success: &[] },
    StrategyDay { id: 116, phase: "Phase 5", title: "Caching & Reuse → Bridge to Quant Maths", schedule_md: include_str!("../schedule/DAY_116.md"), blocks: &[], success: &[] },
    StrategyDay { id: 117, phase: "Phase 6", title: "Information Theory — Reducing Uncertainty", schedule_md: include_str!("../schedule/DAY_117.md"), blocks: &[], success: &[] },
    StrategyDay { id: 118, phase: "Phase 6", title: "Probability, Rigorously", schedule_md: include_str!("../schedule/DAY_118.md"), blocks: &[], success: &[] },
    StrategyDay { id: 119, phase: "Phase 6", title: "Expected Value — The Core of Quant", schedule_md: include_str!("../schedule/DAY_119.md"), blocks: &[], success: &[] },
    StrategyDay { id: 120, phase: "Phase 6", title: "Variance, Risk & Why EV Isn't Enough", schedule_md: include_str!("../schedule/DAY_120.md"), blocks: &[], success: &[] },
    StrategyDay { id: 121, phase: "Phase 6", title: "Law of Large Numbers & CLT", schedule_md: include_str!("../schedule/DAY_121.md"), blocks: &[], success: &[] },
    StrategyDay { id: 122, phase: "Phase 6", title: "Bayesian Thinking — Updating Beliefs", schedule_md: include_str!("../schedule/DAY_122.md"), blocks: &[], success: &[] },
    StrategyDay { id: 123, phase: "Phase 6", title: "Statistics — Learning From Data", schedule_md: include_str!("../schedule/DAY_123.md"), blocks: &[], success: &[] },
    StrategyDay { id: 124, phase: "Phase 6", title: "Correlation vs Causation", schedule_md: include_str!("../schedule/DAY_124.md"), blocks: &[], success: &[] },
    StrategyDay { id: 125, phase: "Phase 6", title: "Hypothesis Testing & Multiple-Testing Trap", schedule_md: include_str!("../schedule/DAY_125.md"), blocks: &[], success: &[] },
    StrategyDay { id: 126, phase: "Phase 6", title: "Statistical Power & Sample Size", schedule_md: include_str!("../schedule/DAY_126.md"), blocks: &[], success: &[] },
    StrategyDay { id: 127, phase: "Phase 6", title: "Overfitting — The Silent Killer", schedule_md: include_str!("../schedule/DAY_127.md"), blocks: &[], success: &[] },
    StrategyDay { id: 128, phase: "Phase 6", title: "The Bias–Variance Tradeoff", schedule_md: include_str!("../schedule/DAY_128.md"), blocks: &[], success: &[] },
    StrategyDay { id: 129, phase: "Phase 6", title: "From Statistics to Machine Learning", schedule_md: include_str!("../schedule/DAY_129.md"), blocks: &[], success: &[] },
    StrategyDay { id: 130, phase: "Phase 6", title: "Linear Regression — The Quant Workhorse", schedule_md: include_str!("../schedule/DAY_130.md"), blocks: &[], success: &[] },
    StrategyDay { id: 131, phase: "Phase 6", title: "Gradient Descent — How Machines Learn", schedule_md: include_str!("../schedule/DAY_131.md"), blocks: &[], success: &[] },
    StrategyDay { id: 132, phase: "Phase 6", title: "Loss Functions — Defining 'Good'", schedule_md: include_str!("../schedule/DAY_132.md"), blocks: &[], success: &[] },
    StrategyDay { id: 133, phase: "Phase 6", title: "Neural Networks", schedule_md: include_str!("../schedule/DAY_133.md"), blocks: &[], success: &[] },
    StrategyDay { id: 134, phase: "Phase 6", title: "Deep Learning — Why Depth Creates Power", schedule_md: include_str!("../schedule/DAY_134.md"), blocks: &[], success: &[] },
    StrategyDay { id: 135, phase: "Phase 6", title: "Reinforcement Learning", schedule_md: include_str!("../schedule/DAY_135.md"), blocks: &[], success: &[] },
    StrategyDay { id: 136, phase: "Phase 6", title: "DP & the Bellman Principle", schedule_md: include_str!("../schedule/DAY_136.md"), blocks: &[], success: &[] },
    StrategyDay { id: 137, phase: "Phase 6", title: "Markov Processes & Regimes", schedule_md: include_str!("../schedule/DAY_137.md"), blocks: &[], success: &[] },
    StrategyDay { id: 138, phase: "Phase 6", title: "Markov Decision Processes", schedule_md: include_str!("../schedule/DAY_138.md"), blocks: &[], success: &[] },
    StrategyDay { id: 139, phase: "Phase 6", title: "Monte Carlo Methods", schedule_md: include_str!("../schedule/DAY_139.md"), blocks: &[], success: &[] },
    StrategyDay { id: 140, phase: "Phase 6", title: "Stochastic Processes — Random Walk → GBM", schedule_md: include_str!("../schedule/DAY_140.md"), blocks: &[], success: &[] },
    StrategyDay { id: 141, phase: "Phase 6", title: "Time-Series Thinking", schedule_md: include_str!("../schedule/DAY_141.md"), blocks: &[], success: &[] },
    StrategyDay { id: 142, phase: "Phase 6", title: "Stationarity & the Unit Root", schedule_md: include_str!("../schedule/DAY_142.md"), blocks: &[], success: &[] },
    StrategyDay { id: 143, phase: "Phase 6", title: "Volatility & GARCH", schedule_md: include_str!("../schedule/DAY_143.md"), blocks: &[], success: &[] },
    StrategyDay { id: 144, phase: "Phase 7", title: "Return vs Risk — The Central Trade-off", schedule_md: include_str!("../schedule/DAY_144.md"), blocks: &[], success: &[] },
    StrategyDay { id: 145, phase: "Phase 7", title: "The Sharpe Ratio", schedule_md: include_str!("../schedule/DAY_145.md"), blocks: &[], success: &[] },
    StrategyDay { id: 146, phase: "Phase 7", title: "Correlation & Diversification", schedule_md: include_str!("../schedule/DAY_146.md"), blocks: &[], success: &[] },
    StrategyDay { id: 147, phase: "Phase 7", title: "Modern Portfolio Theory", schedule_md: include_str!("../schedule/DAY_147.md"), blocks: &[], success: &[] },
    StrategyDay { id: 148, phase: "Phase 7", title: "CAPM & Beta", schedule_md: include_str!("../schedule/DAY_148.md"), blocks: &[], success: &[] },
    StrategyDay { id: 149, phase: "Phase 7", title: "Alpha — Skill vs Luck", schedule_md: include_str!("../schedule/DAY_149.md"), blocks: &[], success: &[] },
    StrategyDay { id: 150, phase: "Phase 7", title: "Market Efficiency — EMH & the Grossman–Stiglitz Paradox", schedule_md: include_str!("../schedule/DAY_150.md"), blocks: &[], success: &[] },
    StrategyDay { id: 151, phase: "Phase 8", title: "Options I — Payoffs & Optionality", schedule_md: include_str!("../schedule/DAY_151.md"), blocks: &[], success: &[] },
    StrategyDay { id: 152, phase: "Phase 8", title: "Put–Call Parity", schedule_md: include_str!("../schedule/DAY_152.md"), blocks: &[], success: &[] },
    StrategyDay { id: 153, phase: "Phase 8", title: "Black–Scholes & Risk-Neutral Pricing", schedule_md: include_str!("../schedule/DAY_153.md"), blocks: &[], success: &[] },
    StrategyDay { id: 154, phase: "Phase 8", title: "The Greeks", schedule_md: include_str!("../schedule/DAY_154.md"), blocks: &[], success: &[] },
    StrategyDay { id: 155, phase: "Phase 8", title: "Delta-Hedging & Gamma Scalping", schedule_md: include_str!("../schedule/DAY_155.md"), blocks: &[], success: &[] },
    StrategyDay { id: 156, phase: "Phase 8", title: "Implied Volatility & the Smile", schedule_md: include_str!("../schedule/DAY_156.md"), blocks: &[], success: &[] },
    StrategyDay { id: 157, phase: "Phase 8", title: "Binomial Trees", schedule_md: include_str!("../schedule/DAY_157.md"), blocks: &[], success: &[] },
    StrategyDay { id: 158, phase: "Phase 8", title: "Exotics & Path-Dependence", schedule_md: include_str!("../schedule/DAY_158.md"), blocks: &[], success: &[] },
    StrategyDay { id: 159, phase: "Phase 8", title: "Volatility as an Asset — Variance Swaps & VIX", schedule_md: include_str!("../schedule/DAY_159.md"), blocks: &[], success: &[] },
    StrategyDay { id: 160, phase: "Phase 8", title: "Derivatives & Pricing Capstone", schedule_md: include_str!("../schedule/DAY_160.md"), blocks: &[], success: &[] },
    StrategyDay { id: 161, phase: "Phase 9", title: "Time Value of Money & Bond Pricing", schedule_md: include_str!("../schedule/DAY_161.md"), blocks: &[], success: &[] },
    StrategyDay { id: 162, phase: "Phase 9", title: "Duration & DV01", schedule_md: include_str!("../schedule/DAY_162.md"), blocks: &[], success: &[] },
    StrategyDay { id: 163, phase: "Phase 9", title: "Convexity", schedule_md: include_str!("../schedule/DAY_163.md"), blocks: &[], success: &[] },
    StrategyDay { id: 164, phase: "Phase 9", title: "The Yield Curve — Spot, Forward & Bootstrapping", schedule_md: include_str!("../schedule/DAY_164.md"), blocks: &[], success: &[] },
    StrategyDay { id: 165, phase: "Phase 9", title: "Term-Structure Theories & Carry", schedule_md: include_str!("../schedule/DAY_165.md"), blocks: &[], success: &[] },
    StrategyDay { id: 166, phase: "Phase 9", title: "Interest-Rate Risk Management & Immunization", schedule_md: include_str!("../schedule/DAY_166.md"), blocks: &[], success: &[] },
    StrategyDay { id: 167, phase: "Phase 9", title: "Credit Risk & the Merton Model", schedule_md: include_str!("../schedule/DAY_167.md"), blocks: &[], success: &[] },
    StrategyDay { id: 168, phase: "Phase 9", title: "Short-Rate Models — Vasicek & CIR", schedule_md: include_str!("../schedule/DAY_168.md"), blocks: &[], success: &[] },
    StrategyDay { id: 169, phase: "Phase 9", title: "Interest-Rate Derivatives — Swaps, Caps, Swaptions", schedule_md: include_str!("../schedule/DAY_169.md"), blocks: &[], success: &[] },
    StrategyDay { id: 170, phase: "Phase 9", title: "Fixed Income Capstone", schedule_md: include_str!("../schedule/DAY_170.md"), blocks: &[], success: &[] },
    StrategyDay { id: 171, phase: "Phase 10", title: "Statistical Arbitrage I — Mean Reversion & Pairs", schedule_md: include_str!("../schedule/DAY_171.md"), blocks: &[], success: &[] },
    StrategyDay { id: 172, phase: "Phase 10", title: "Cointegration & the Spread", schedule_md: include_str!("../schedule/DAY_172.md"), blocks: &[], success: &[] },
    StrategyDay { id: 173, phase: "Phase 10", title: "Ornstein–Uhlenbeck & Half-Life", schedule_md: include_str!("../schedule/DAY_173.md"), blocks: &[], success: &[] },
    StrategyDay { id: 174, phase: "Phase 10", title: "Building & Backtesting a Pairs Strategy", schedule_md: include_str!("../schedule/DAY_174.md"), blocks: &[], success: &[] },
    StrategyDay { id: 175, phase: "Phase 10", title: "The Kalman Filter — Dynamic Hedge Ratio", schedule_md: include_str!("../schedule/DAY_175.md"), blocks: &[], success: &[] },
    StrategyDay { id: 176, phase: "Phase 10", title: "Cross-Sectional Statistical Arbitrage", schedule_md: include_str!("../schedule/DAY_176.md"), blocks: &[], success: &[] },
    StrategyDay { id: 177, phase: "Phase 10", title: "Signal Construction & Alpha Combination", schedule_md: include_str!("../schedule/DAY_177.md"), blocks: &[], success: &[] },
    StrategyDay { id: 178, phase: "Phase 10", title: "Risk Management & the Quant Quake", schedule_md: include_str!("../schedule/DAY_178.md"), blocks: &[], success: &[] },
    StrategyDay { id: 179, phase: "Phase 10", title: "Execution & Capacity", schedule_md: include_str!("../schedule/DAY_179.md"), blocks: &[], success: &[] },
    StrategyDay { id: 180, phase: "Phase 10", title: "Statistical Arbitrage Capstone", schedule_md: include_str!("../schedule/DAY_180.md"), blocks: &[], success: &[] },
    StrategyDay { id: 181, phase: "Phase 11", title: "Order Books & the Bid–Ask Spread", schedule_md: include_str!("../schedule/DAY_181.md"), blocks: &[], success: &[] },
    StrategyDay { id: 182, phase: "Phase 11", title: "Market Making & Inventory", schedule_md: include_str!("../schedule/DAY_182.md"), blocks: &[], success: &[] },
    StrategyDay { id: 183, phase: "Phase 11", title: "Adverse Selection — Glosten–Milgrom & Kyle", schedule_md: include_str!("../schedule/DAY_183.md"), blocks: &[], success: &[] },
    StrategyDay { id: 184, phase: "Phase 11", title: "Price Impact & Order-Book Dynamics", schedule_md: include_str!("../schedule/DAY_184.md"), blocks: &[], success: &[] },
    StrategyDay { id: 185, phase: "Phase 11", title: "Optimal Execution — Almgren–Chriss", schedule_md: include_str!("../schedule/DAY_185.md"), blocks: &[], success: &[] },
    StrategyDay { id: 186, phase: "Phase 11", title: "High-Frequency Trading & Latency", schedule_md: include_str!("../schedule/DAY_186.md"), blocks: &[], success: &[] },
    StrategyDay { id: 187, phase: "Phase 11", title: "Order Types, Queues & Order Flow", schedule_md: include_str!("../schedule/DAY_187.md"), blocks: &[], success: &[] },
    StrategyDay { id: 188, phase: "Phase 11", title: "Transaction Cost Analysis", schedule_md: include_str!("../schedule/DAY_188.md"), blocks: &[], success: &[] },
    StrategyDay { id: 189, phase: "Phase 11", title: "Fragmentation, Dark Pools & Regulation", schedule_md: include_str!("../schedule/DAY_189.md"), blocks: &[], success: &[] },
    StrategyDay { id: 190, phase: "Phase 11", title: "Microstructure Capstone", schedule_md: include_str!("../schedule/DAY_190.md"), blocks: &[], success: &[] },
    StrategyDay { id: 191, phase: "Phase 12", title: "Why Finance Breaks Naive ML", schedule_md: include_str!("../schedule/DAY_191.md"), blocks: &[], success: &[] },
    StrategyDay { id: 192, phase: "Phase 12", title: "Features & Labeling — Triple-Barrier & Frac-Diff", schedule_md: include_str!("../schedule/DAY_192.md"), blocks: &[], success: &[] },
    StrategyDay { id: 193, phase: "Phase 12", title: "Cross-Validation Pitfalls — Purging & Embargo", schedule_md: include_str!("../schedule/DAY_193.md"), blocks: &[], success: &[] },
    StrategyDay { id: 194, phase: "Phase 12", title: "Tree Ensembles — Random Forests & Boosting", schedule_md: include_str!("../schedule/DAY_194.md"), blocks: &[], success: &[] },
    StrategyDay { id: 195, phase: "Phase 12", title: "Feature Importance — MDA, MDI & SHAP", schedule_md: include_str!("../schedule/DAY_195.md"), blocks: &[], success: &[] },
    StrategyDay { id: 196, phase: "Phase 12", title: "Deep Learning for Finance", schedule_md: include_str!("../schedule/DAY_196.md"), blocks: &[], success: &[] },
    StrategyDay { id: 197, phase: "Phase 12", title: "NLP & Alternative Data", schedule_md: include_str!("../schedule/DAY_197.md"), blocks: &[], success: &[] },
    StrategyDay { id: 198, phase: "Phase 12", title: "Reinforcement Learning for Trading", schedule_md: include_str!("../schedule/DAY_198.md"), blocks: &[], success: &[] },
    StrategyDay { id: 199, phase: "Phase 12", title: "Backtest Overfitting & the Deflated Sharpe", schedule_md: include_str!("../schedule/DAY_199.md"), blocks: &[], success: &[] },
    StrategyDay { id: 200, phase: "Phase 12", title: "Machine Learning for Finance Capstone", schedule_md: include_str!("../schedule/DAY_200.md"), blocks: &[], success: &[] },
    StrategyDay { id: 201, phase: "Phase 13", title: "ARIMA & Box–Jenkins", schedule_md: include_str!("../schedule/DAY_201.md"), blocks: &[], success: &[] },
    StrategyDay { id: 202, phase: "Phase 13", title: "GARCH & Volatility Models", schedule_md: include_str!("../schedule/DAY_202.md"), blocks: &[], success: &[] },
    StrategyDay { id: 203, phase: "Phase 13", title: "State-Space Models & Kalman", schedule_md: include_str!("../schedule/DAY_203.md"), blocks: &[], success: &[] },
    StrategyDay { id: 204, phase: "Phase 13", title: "Regime-Switching & Hidden Markov Models", schedule_md: include_str!("../schedule/DAY_204.md"), blocks: &[], success: &[] },
    StrategyDay { id: 205, phase: "Phase 13", title: "VAR & Granger Causality", schedule_md: include_str!("../schedule/DAY_205.md"), blocks: &[], success: &[] },
    StrategyDay { id: 206, phase: "Phase 13", title: "Cointegration Systems & VECM", schedule_md: include_str!("../schedule/DAY_206.md"), blocks: &[], success: &[] },
    StrategyDay { id: 207, phase: "Phase 13", title: "Spectral Analysis & Wavelets", schedule_md: include_str!("../schedule/DAY_207.md"), blocks: &[], success: &[] },
    StrategyDay { id: 208, phase: "Phase 13", title: "Forecasting & Evaluation", schedule_md: include_str!("../schedule/DAY_208.md"), blocks: &[], success: &[] },
    StrategyDay { id: 209, phase: "Phase 13", title: "Nowcasting & High-Dimensional Time Series", schedule_md: include_str!("../schedule/DAY_209.md"), blocks: &[], success: &[] },
    StrategyDay { id: 210, phase: "Phase 13", title: "Advanced Time Series Capstone", schedule_md: include_str!("../schedule/DAY_210.md"), blocks: &[], success: &[] },
    StrategyDay { id: 211, phase: "Phase 14", title: "Factor Models & Risk Models", schedule_md: include_str!("../schedule/DAY_211.md"), blocks: &[], success: &[] },
    StrategyDay { id: 212, phase: "Phase 14", title: "Black–Litterman", schedule_md: include_str!("../schedule/DAY_212.md"), blocks: &[], success: &[] },
    StrategyDay { id: 213, phase: "Phase 14", title: "The Kelly Criterion & Bet Sizing", schedule_md: include_str!("../schedule/DAY_213.md"), blocks: &[], success: &[] },
    StrategyDay { id: 214, phase: "Phase 14", title: "Risk Parity", schedule_md: include_str!("../schedule/DAY_214.md"), blocks: &[], success: &[] },
    StrategyDay { id: 215, phase: "Phase 14", title: "VaR, CVaR & Coherent Risk Measures", schedule_md: include_str!("../schedule/DAY_215.md"), blocks: &[], success: &[] },
    StrategyDay { id: 216, phase: "Phase 14", title: "Extreme Value Theory & Tail Risk", schedule_md: include_str!("../schedule/DAY_216.md"), blocks: &[], success: &[] },
    StrategyDay { id: 217, phase: "Phase 14", title: "Drawdown Control", schedule_md: include_str!("../schedule/DAY_217.md"), blocks: &[], success: &[] },
    StrategyDay { id: 218, phase: "Phase 14", title: "Robust Portfolio Optimization", schedule_md: include_str!("../schedule/DAY_218.md"), blocks: &[], success: &[] },
    StrategyDay { id: 219, phase: "Phase 14", title: "Performance Attribution", schedule_md: include_str!("../schedule/DAY_219.md"), blocks: &[], success: &[] },
    StrategyDay { id: 220, phase: "Phase 14", title: "Advanced Portfolio & Risk Capstone", schedule_md: include_str!("../schedule/DAY_220.md"), blocks: &[], success: &[] },
    StrategyDay { id: 221, phase: "Phase 15", title: "C++ for Quants — Why & Performance Fundamentals", schedule_md: include_str!("../schedule/DAY_221.md"), blocks: &[], success: &[] },
    StrategyDay { id: 222, phase: "Phase 15", title: "Memory, Cache & Data-Oriented Design", schedule_md: include_str!("../schedule/DAY_222.md"), blocks: &[], success: &[] },
    StrategyDay { id: 223, phase: "Phase 15", title: "Templates & Generic Programming", schedule_md: include_str!("../schedule/DAY_223.md"), blocks: &[], success: &[] },
    StrategyDay { id: 224, phase: "Phase 15", title: "RAII, Move Semantics & Smart Pointers", schedule_md: include_str!("../schedule/DAY_224.md"), blocks: &[], success: &[] },
    StrategyDay { id: 225, phase: "Phase 15", title: "Concurrency & Lock-Free Programming", schedule_md: include_str!("../schedule/DAY_225.md"), blocks: &[], success: &[] },
    StrategyDay { id: 226, phase: "Phase 15", title: "Low-Latency Techniques", schedule_md: include_str!("../schedule/DAY_226.md"), blocks: &[], success: &[] },
    StrategyDay { id: 227, phase: "Phase 15", title: "Numerical Computing & SIMD", schedule_md: include_str!("../schedule/DAY_227.md"), blocks: &[], success: &[] },
    StrategyDay { id: 228, phase: "Phase 15", title: "Building a Trading System — Architecture", schedule_md: include_str!("../schedule/DAY_228.md"), blocks: &[], success: &[] },
    StrategyDay { id: 229, phase: "Phase 15", title: "Testing, Profiling & Debugging", schedule_md: include_str!("../schedule/DAY_229.md"), blocks: &[], success: &[] },
    StrategyDay { id: 230, phase: "Phase 15", title: "C++ & Systems Capstone", schedule_md: include_str!("../schedule/DAY_230.md"), blocks: &[], success: &[] },
];

/// The strategy day that falls on a given date key, given the configured
/// schedule start. Day 1 == the start date; days before it map to nothing.
/// Mirrors `strategyDayForDate` in spec.html.
pub fn strategy_day_for_date(
    date_key: &str,
    schedule_start: &Option<String>,
) -> Option<&'static StrategyDay> {
    use chrono::NaiveDate;
    let start_s = schedule_start.as_ref()?;
    let start = NaiveDate::parse_from_str(start_s, "%Y-%m-%d").ok()?;
    let target = NaiveDate::parse_from_str(date_key, "%Y-%m-%d").ok()?;
    let day_id = (target - start).num_days() + 1; // Day 1 = start date
    if day_id < 1 {
        return None;
    }
    DAYS.iter().find(|d| d.id == day_id)
}

// ── Progress page data (ported from spec.html: TRACKS / STAGES / WORLDS) ──

pub struct Topic {
    pub id: &'static str,
    pub name: &'static str,
    /// Phase key "p1".."p4" — see [`phase_label`].
    pub phase: &'static str,
}

pub struct Track {
    pub id: &'static str,
    pub name: &'static str,
    pub icon: &'static str,
    pub color: &'static str,
    /// Translucent tint behind the track icon chip.
    pub bg_color: &'static str,
    pub topics: &'static [Topic],
}

pub const TRACKS: &[Track] = &[
    Track {
        id: "cpp",
        name: "C++ + DSA",
        icon: "{ }",
        color: "#7c6fff",
        bg_color: "rgba(124,111,255,0.12)",
        topics: &[
            Topic { id: "cpp_syntax", name: "Syntax, loops, functions", phase: "p1" },
            Topic { id: "cpp_arrays", name: "Arrays & strings", phase: "p1" },
            Topic { id: "cpp_vectors", name: "STL vectors & pairs", phase: "p1" },
            Topic { id: "cpp_stl", name: "Maps, sets, iterators", phase: "p1" },
            Topic { id: "cpp_stack_queue", name: "Stack & queue", phase: "p1" },
            Topic { id: "cpp_recursion", name: "Recursion basics", phase: "p1" },
            Topic { id: "cpp_pointers", name: "Pointers & memory", phase: "p1" },
            Topic { id: "cpp_oops", name: "OOP fundamentals", phase: "p1" },
            Topic { id: "dsa_hashing", name: "Hashing & sliding window", phase: "p2" },
            Topic { id: "dsa_binsearch", name: "Binary search", phase: "p2" },
            Topic { id: "dsa_linked", name: "Linked lists", phase: "p3" },
            Topic { id: "dsa_heaps", name: "Heaps", phase: "p3" },
            Topic { id: "dsa_trees", name: "Trees & traversals", phase: "p3" },
            Topic { id: "dsa_graphs", name: "Graphs, BFS & DFS", phase: "p4" },
            Topic { id: "dsa_shortest", name: "Shortest path", phase: "p4" },
            Topic { id: "dsa_dp", name: "Dynamic programming", phase: "p4" },
            Topic { id: "cf_exposure", name: "Codeforces exposure", phase: "p4" },
        ],
    },
    Track {
        id: "math",
        name: "Mathematics",
        icon: "∑",
        color: "#4ecdc4",
        bg_color: "rgba(78,205,196,0.12)",
        topics: &[
            Topic { id: "math_algebra", name: "Algebra intuition (3B1B)", phase: "p1" },
            Topic { id: "math_prob", name: "Probability fundamentals", phase: "p1" },
            Topic { id: "math_calc", name: "Calculus intuition (3B1B)", phase: "p2" },
            Topic { id: "math_stats", name: "Statistics & distributions", phase: "p2" },
            Topic { id: "math_linalg", name: "Linear algebra (3B1B)", phase: "p3" },
            Topic { id: "math_bayes", name: "Bayesian thinking", phase: "p3" },
            Topic { id: "math_regression", name: "Regression & covariance", phase: "p3" },
        ],
    },
    Track {
        id: "linux",
        name: "Linux + Engineering",
        icon: "$_",
        color: "#8b90a0",
        bg_color: "rgba(139,144,160,0.12)",
        topics: &[
            Topic { id: "linux_basics", name: "Terminal basics", phase: "p1" },
            Topic { id: "linux_cpp", name: "Compile C++ from terminal", phase: "p1" },
            Topic { id: "linux_git", name: "Git & version control", phase: "p2" },
            Topic { id: "linux_bash", name: "Bash scripting", phase: "p3" },
        ],
    },
    Track {
        id: "python",
        name: "Python for Quant",
        icon: "Py",
        color: "#ffd166",
        bg_color: "rgba(255,209,102,0.12)",
        topics: &[
            Topic { id: "py_basics", name: "Variables, loops, functions", phase: "p1" },
            Topic { id: "py_lists", name: "Lists & dictionaries", phase: "p1" },
            Topic { id: "py_numpy", name: "NumPy & vectorized ops", phase: "p2" },
            Topic { id: "py_pandas", name: "pandas & data analysis", phase: "p2" },
            Topic { id: "py_matplotlib", name: "Matplotlib & visualization", phase: "p3" },
            Topic { id: "py_simulations", name: "Monte Carlo simulations", phase: "p3" },
            Topic { id: "py_backtest", name: "Backtesting engine", phase: "p4" },
        ],
    },
    Track {
        id: "projects",
        name: "Projects",
        icon: "▶",
        color: "#06d6a0",
        bg_color: "rgba(6,214,160,0.12)",
        topics: &[
            Topic { id: "proj_calc", name: "Calculator", phase: "p1" },
            Topic { id: "proj_student", name: "Student record system", phase: "p1" },
            Topic { id: "proj_guess", name: "Number guessing game", phase: "p1" },
            Topic { id: "proj_prob_sim", name: "Probability simulator", phase: "p2" },
            Topic { id: "proj_stock", name: "Stock analyzer", phase: "p2" },
            Topic { id: "proj_monte", name: "Monte Carlo simulator", phase: "p3" },
            Topic { id: "proj_backtest", name: "Backtesting engine", phase: "p4" },
            Topic { id: "proj_portfolio", name: "Portfolio optimizer", phase: "p4" },
            Topic { id: "proj_trading", name: "Trading simulator", phase: "p4" },
        ],
    },
    Track {
        id: "quant_think",
        name: "Quantitative Thinking",
        icon: "◈",
        color: "#ff9f43",
        bg_color: "rgba(255,159,67,0.12)",
        topics: &[
            Topic { id: "qt_decompose", name: "Decomposition thinking", phase: "p1" },
            Topic { id: "qt_abstraction", name: "Abstraction thinking", phase: "p1" },
            Topic { id: "qt_recursion_think", name: "Recursion thinking", phase: "p2" },
            Topic { id: "qt_optimization", name: "Optimization thinking", phase: "p2" },
            Topic { id: "qt_probabilistic", name: "Probabilistic thinking", phase: "p2" },
            Topic { id: "qt_systems", name: "Systems thinking", phase: "p3" },
            Topic { id: "qt_state", name: "State thinking", phase: "p3" },
            Topic { id: "qt_puzzles", name: "Puzzle-solving fluency", phase: "p1" },
            Topic { id: "qt_probability_q", name: "Probability problem solving", phase: "p2" },
            Topic { id: "qt_reasoning", name: "Logical reasoning", phase: "p2" },
        ],
    },
];

/// One topic's maximum auto-link contribution from a daily block.
pub struct TopicContrib {
    pub topic_id: &'static str,
    /// Cap this block contributes toward the topic on a fully-completed day.
    /// Summed by [`per_day_contrib`] to derive earned progress.
    pub max_contrib: u32,
}

/// Maps each daily block (by index into [`DAILY_BLOCKS`]) to the topics whose
/// progress it can auto-fill. Ported from `BLOCK_TOPIC_MAP` in spec.html; used
/// to mark auto-linked topics (and, later, to drive auto-progress).
pub const BLOCK_TOPIC_MAP: &[&[TopicContrib]] = &[
    // Block 1 — C++ theory
    &[
        TopicContrib { topic_id: "cpp_syntax", max_contrib: 30 },
        TopicContrib { topic_id: "cpp_arrays", max_contrib: 30 },
        TopicContrib { topic_id: "cpp_vectors", max_contrib: 20 },
        TopicContrib { topic_id: "cpp_stl", max_contrib: 20 },
        TopicContrib { topic_id: "cpp_oops", max_contrib: 15 },
        TopicContrib { topic_id: "cpp_pointers", max_contrib: 15 },
        TopicContrib { topic_id: "cpp_recursion", max_contrib: 10 },
    ],
    // Block 2 — Implementation / DSA
    &[
        TopicContrib { topic_id: "cpp_arrays", max_contrib: 40 },
        TopicContrib { topic_id: "cpp_syntax", max_contrib: 20 },
        TopicContrib { topic_id: "dsa_hashing", max_contrib: 15 },
        TopicContrib { topic_id: "dsa_binsearch", max_contrib: 15 },
        TopicContrib { topic_id: "proj_calc", max_contrib: 25 },
        TopicContrib { topic_id: "proj_student", max_contrib: 25 },
        TopicContrib { topic_id: "proj_guess", max_contrib: 25 },
    ],
    // Block 3 — LeetCode gym
    &[
        TopicContrib { topic_id: "cpp_stack_queue", max_contrib: 30 },
        TopicContrib { topic_id: "dsa_hashing", max_contrib: 25 },
        TopicContrib { topic_id: "dsa_binsearch", max_contrib: 25 },
        TopicContrib { topic_id: "dsa_linked", max_contrib: 10 },
        TopicContrib { topic_id: "dsa_heaps", max_contrib: 10 },
        TopicContrib { topic_id: "dsa_trees", max_contrib: 10 },
        TopicContrib { topic_id: "dsa_graphs", max_contrib: 10 },
        TopicContrib { topic_id: "dsa_dp", max_contrib: 10 },
        TopicContrib { topic_id: "cf_exposure", max_contrib: 20 },
    ],
    // Block 4 — Mathematics
    &[
        TopicContrib { topic_id: "math_algebra", max_contrib: 40 },
        TopicContrib { topic_id: "math_prob", max_contrib: 30 },
        TopicContrib { topic_id: "math_calc", max_contrib: 20 },
        TopicContrib { topic_id: "math_stats", max_contrib: 20 },
        TopicContrib { topic_id: "math_linalg", max_contrib: 15 },
        TopicContrib { topic_id: "math_bayes", max_contrib: 15 },
        TopicContrib { topic_id: "math_regression", max_contrib: 10 },
    ],
    // Block 5 — Python
    &[
        TopicContrib { topic_id: "py_basics", max_contrib: 40 },
        TopicContrib { topic_id: "py_lists", max_contrib: 35 },
        TopicContrib { topic_id: "py_numpy", max_contrib: 20 },
        TopicContrib { topic_id: "py_pandas", max_contrib: 15 },
        TopicContrib { topic_id: "py_matplotlib", max_contrib: 10 },
        TopicContrib { topic_id: "py_simulations", max_contrib: 10 },
        TopicContrib { topic_id: "py_backtest", max_contrib: 5 },
        TopicContrib { topic_id: "proj_prob_sim", max_contrib: 15 },
        TopicContrib { topic_id: "proj_stock", max_contrib: 10 },
    ],
    // Block 6 — Linux
    &[
        TopicContrib { topic_id: "linux_basics", max_contrib: 50 },
        TopicContrib { topic_id: "linux_cpp", max_contrib: 50 },
        TopicContrib { topic_id: "linux_git", max_contrib: 20 },
        TopicContrib { topic_id: "linux_bash", max_contrib: 15 },
    ],
    // Block 7 — Quant thinking
    &[
        TopicContrib { topic_id: "qt_decompose", max_contrib: 25 },
        TopicContrib { topic_id: "qt_abstraction", max_contrib: 25 },
        TopicContrib { topic_id: "qt_recursion_think", max_contrib: 20 },
        TopicContrib { topic_id: "qt_optimization", max_contrib: 20 },
        TopicContrib { topic_id: "qt_probabilistic", max_contrib: 20 },
        TopicContrib { topic_id: "qt_systems", max_contrib: 15 },
        TopicContrib { topic_id: "qt_state", max_contrib: 15 },
        TopicContrib { topic_id: "qt_puzzles", max_contrib: 30 },
        TopicContrib { topic_id: "qt_probability_q", max_contrib: 25 },
        TopicContrib { topic_id: "qt_reasoning", max_contrib: 25 },
    ],
    // Block 8 — Journal (soft credit)
    &[
        TopicContrib { topic_id: "qt_decompose", max_contrib: 5 },
        TopicContrib { topic_id: "qt_abstraction", max_contrib: 5 },
    ],
];

/// Whether a topic receives any auto-link credit from the daily blocks.
pub fn topic_has_autolink(topic_id: &str) -> bool {
    BLOCK_TOPIC_MAP
        .iter()
        .any(|block| block.iter().any(|c| c.topic_id == topic_id))
}

/// Every topic across all tracks, in display order.
pub fn all_topics() -> impl Iterator<Item = &'static Topic> {
    TRACKS.iter().flat_map(|t| t.topics.iter())
}

/// Full auto-link contribution a single completed day grants `topic_id` — the
/// sum of its `max_contrib` across every daily block (a completed day means
/// every block is 100% done, so `ratio = 1`).
pub fn per_day_contrib(topic_id: &str) -> u32 {
    BLOCK_TOPIC_MAP
        .iter()
        .flat_map(|block| block.iter())
        .filter(|c| c.topic_id == topic_id)
        .map(|c| c.max_contrib)
        .sum()
}

/// Earned percent for a topic: each completed day banks `per_day_contrib`,
/// capped at 100. Progress is therefore a pure function of `completed_days`
/// (`current_day - 1`) — there is no manually-set value.
pub fn topic_progress(topic_id: &str, completed_days: u32) -> u32 {
    (per_day_contrib(topic_id) * completed_days).min(100)
}

/// Average earned percent across a track's topics (rounded). Mirrors `trackPct`.
pub fn track_pct(track: &Track, completed_days: u32) -> u32 {
    if track.topics.is_empty() {
        return 0;
    }
    let sum: u32 = track
        .topics
        .iter()
        .map(|t| topic_progress(t.id, completed_days))
        .sum();
    (sum as f64 / track.topics.len() as f64).round() as u32
}

/// Average earned percent across every topic (rounded). Mirrors `overallPct`.
pub fn overall_pct(completed_days: u32) -> u32 {
    let topics: Vec<&Topic> = all_topics().collect();
    if topics.is_empty() {
        return 0;
    }
    let sum: u32 = topics
        .iter()
        .map(|t| topic_progress(t.id, completed_days))
        .sum();
    (sum as f64 / topics.len() as f64).round() as u32
}

/// Number of topics at 100%. Mirrors `topicsDone`.
pub fn topics_done(completed_days: u32) -> usize {
    all_topics()
        .filter(|t| topic_progress(t.id, completed_days) >= 100)
        .count()
}

/// Number of tracks with any progress. Mirrors `TRACKS.filter(trackPct>0)`.
pub fn active_tracks(completed_days: u32) -> usize {
    TRACKS.iter().filter(|t| track_pct(t, completed_days) > 0).count()
}

/// Human label for a phase key ("p1".."p4").
pub fn phase_label(phase: &str) -> &'static str {
    match phase {
        "p1" => "Phase 1",
        "p2" => "Phase 2",
        "p3" => "Phase 3",
        "p4" => "Phase 4",
        _ => "Phase",
    }
}

pub struct Stage {
    pub min: u32,
    pub max: u32,
    pub emoji: &'static str,
    pub label: &'static str,
    pub reward: Option<&'static str>,
}

pub const STAGES: &[Stage] = &[
    Stage { min: 0,   max: 24,  emoji: "🌱", label: "🌱 Seed",    reward: None },
    Stage { min: 25,  max: 49,  emoji: "🌿", label: "🌿 Plant",   reward: Some("🏅 C++ Apprentice — Phase 1 complete!") },
    Stage { min: 50,  max: 74,  emoji: "🌳", label: "🌳 Tree",    reward: Some("⚔️ Problem Solver — Phase 2 complete!") },
    Stage { min: 75,  max: 99,  emoji: "🧠", label: "🧠 Thinker", reward: Some("🧠 Quant Thinker — Phase 3 complete!") },
    Stage { min: 100, max: 100, emoji: "🚀", label: "🚀 Quant",   reward: Some("🚀 Quant Candidate — You made it!") },
];

/// The stage that a given overall percent falls into (clamped to the last).
pub fn stage_for(pct: u32) -> &'static Stage {
    STAGES
        .iter()
        .find(|s| pct >= s.min && pct <= s.max)
        .unwrap_or(&STAGES[0])
}

pub struct World {
    pub name: &'static str,
    pub emoji: &'static str,
    pub color: &'static str,
    pub lo: u32,
    pub hi: u32,
    pub desc: &'static str,
}

pub const WORLDS: &[World] = &[
    World { name: "Foundation World", emoji: "🌱", color: "#7c6fff", lo: 0,  hi: 25,  desc: "C++ · Basics · Algebra" },
    World { name: "Builder World",    emoji: "⚙️", color: "#4ecdc4", lo: 25, hi: 50,  desc: "DSA · Stats · NumPy" },
    World { name: "Quant Mind",       emoji: "🧠", color: "#ffd166", lo: 50, hi: 75,  desc: "Graphs · ML · Systems" },
    World { name: "Trading World",    emoji: "🚀", color: "#06d6a0", lo: 75, hi: 100, desc: "DP · Backtest · Deploy" },
];

// ── World Map (CLAUDE.md Section 5): the 9 worlds keyed by DAY range (mirrors
// the roadmap phases), each with its Section-5 accent color. Distinct from the
// 4 percentage-based `WORLDS` above, which drive the Progress page. ──

pub struct MapWorld {
    pub n: u32,
    pub name: &'static str,
    pub emoji: &'static str,
    pub color: &'static str,
    pub lo: u32, // first day, inclusive
    pub hi: u32, // last day, inclusive
    pub desc: &'static str,
}

pub const MAP_WORLDS: &[MapWorld] = &[
    MapWorld { n: 1, name: "Foundation",      emoji: "🌱", color: "#A855F7", lo: 1,   hi: 60,  desc: "C++ · Basics · Algebra" },
    MapWorld { n: 2, name: "DSA Core",        emoji: "⚙️", color: "#2DD4BF", lo: 61,  hi: 120, desc: "Arrays · Lists · Stacks" },
    MapWorld { n: 3, name: "Data Structures", emoji: "🧱", color: "#84CC16", lo: 121, hi: 180, desc: "Trees · Graphs · Heaps" },
    MapWorld { n: 4, name: "Algorithms",      emoji: "🧠", color: "#F59E0B", lo: 181, hi: 240, desc: "DP · Greedy · Search" },
    MapWorld { n: 5, name: "Quant Math",      emoji: "📊", color: "#3B82F6", lo: 241, hi: 300, desc: "Probability · Stats" },
    MapWorld { n: 6, name: "Python & Data",   emoji: "🐍", color: "#8B5CF6", lo: 301, hi: 360, desc: "NumPy · Pandas · ML" },
    MapWorld { n: 7, name: "Linux & Tools",   emoji: "🐧", color: "#6366F1", lo: 361, hi: 400, desc: "Shell · Git · Build" },
    MapWorld { n: 8, name: "Projects",        emoji: "🚀", color: "#EC4899", lo: 401, hi: 480, desc: "Backtest · Systems" },
    MapWorld { n: 9, name: "Interview Prep",  emoji: "🎯", color: "#F97316", lo: 481, hi: 548, desc: "Mock · Review · Polish" },
];

/// Node state for a world given the user's current day (1-based).
#[derive(Clone, Copy, PartialEq)]
pub enum WorldState {
    Completed,
    Current,
    Locked,
}

impl MapWorld {
    /// Number of days in this world.
    pub fn days(&self) -> u32 {
        self.hi - self.lo + 1
    }

    /// Days completed within this world, given total completed days (current-1).
    pub fn done(&self, completed_days: u32) -> u32 {
        completed_days.saturating_sub(self.lo - 1).min(self.days())
    }

    /// World state from the current day (= completed_days + 1).
    pub fn state(&self, current_day: u32) -> WorldState {
        if current_day > self.hi {
            WorldState::Completed
        } else if current_day >= self.lo {
            WorldState::Current
        } else {
            WorldState::Locked
        }
    }
}

// ── Practice problems + daily-loop gates ──
//
// A minimal port of spec.html's QUESTION_BANK / MATH_BANK / QUANT_BANK /
// LINUX_BANK plus PRACTICE_DAY (the day each item belongs to, folded into the
// `day` field). Solving is self-marked, so starter code, sample IO and
// answer-checking are intentionally omitted. `solution` is empty for code
// problems (no reveal).

#[derive(Clone, Copy, PartialEq)]
pub enum PracticeCategory {
    Code,
    Math,
    Quant,
    Linux,
}

impl PracticeCategory {
    pub fn label(self) -> &'static str {
        match self {
            PracticeCategory::Code => "Code",
            PracticeCategory::Math => "Math",
            PracticeCategory::Quant => "Quant",
            PracticeCategory::Linux => "Linux",
        }
    }
}

pub struct PracticeProblem {
    pub id: &'static str,
    pub category: PracticeCategory,
    pub topic: &'static str,
    pub diff: &'static str,
    pub title: &'static str,
    pub prompt: &'static str,
    /// Worked solution as raw HTML; empty for code problems (no reveal).
    pub solution: &'static str,
    pub day: u32,
}

use PracticeCategory::{Code, Linux, Math, Quant};

pub const PROBLEMS: &[PracticeProblem] = &[
    // ── Code (QUESTION_BANK) ──
    PracticeProblem { id: "q_sum", category: Code, topic: "cpp_arrays", diff: "Easy", title: "Sum of array elements", prompt: r#"Read N, then N integers. Print their sum."#, solution: "", day: 1 },
    PracticeProblem { id: "q_max", category: Code, topic: "cpp_arrays", diff: "Easy", title: "Largest element", prompt: r#"Read N, then N integers. Print the largest."#, solution: "", day: 1 },
    PracticeProblem { id: "q_min", category: Code, topic: "cpp_arrays", diff: "Easy", title: "Smallest element", prompt: r#"Read N, then N integers. Print the smallest."#, solution: "", day: 1 },
    PracticeProblem { id: "q_rev", category: Code, topic: "cpp_arrays", diff: "Easy", title: "Reverse an array", prompt: r#"Read N, then N integers. Print them in reverse order, space-separated."#, solution: "", day: 1 },
    PracticeProblem { id: "q_evenodd", category: Code, topic: "cpp_syntax", diff: "Easy", title: "Count even and odd", prompt: r#"Read N, then N integers. Print "<evens> <odds>"."#, solution: "", day: 1 },
    PracticeProblem { id: "q_fib", category: Code, topic: "cpp_syntax", diff: "Easy", title: "Fibonacci up to N terms", prompt: r#"Read N. Print the first N Fibonacci numbers (starting 0 1), space-separated."#, solution: "", day: 1 },
    PracticeProblem { id: "q_fact", category: Code, topic: "cpp_recursion", diff: "Easy", title: "Factorial", prompt: r#"Read N. Print N! (factorial)."#, solution: "", day: 1 },
    PracticeProblem { id: "q_digsum", category: Code, topic: "cpp_syntax", diff: "Easy", title: "Sum of digits", prompt: r#"Read an integer N. Print the sum of its digits."#, solution: "", day: 1 },
    PracticeProblem { id: "q_revstr", category: Code, topic: "cpp_arrays", diff: "Easy", title: "Reverse a string", prompt: r#"Read a string (one line). Print it reversed."#, solution: "", day: 1 },
    PracticeProblem { id: "q_search", category: Code, topic: "cpp_arrays", diff: "Easy", title: "Linear search", prompt: r#"Read N, then N integers, then a target T. Print the 0-based index of T, or -1."#, solution: "", day: 2 },
    PracticeProblem { id: "q_2nd", category: Code, topic: "cpp_arrays", diff: "Medium", title: "Second largest", prompt: r#"Read N, then N integers. Print the second-largest distinct value."#, solution: "", day: 2 },
    PracticeProblem { id: "q_freq", category: Code, topic: "cpp_arrays", diff: "Easy", title: "Frequency of a number", prompt: r#"Read N, then N integers, then a target T. Print how many times T appears."#, solution: "", day: 2 },
    PracticeProblem { id: "q_sumeven", category: Code, topic: "cpp_arrays", diff: "Easy", title: "Sum of even elements", prompt: r#"Read N, then N integers. Print the sum of only the even numbers."#, solution: "", day: 2 },
    PracticeProblem { id: "q_dedup", category: Code, topic: "cpp_arrays", diff: "Medium", title: "Remove duplicates from sorted array", prompt: r#"Read N, then N integers in non-decreasing order. Print the unique values in order, space-separated."#, solution: "", day: 2 },
    PracticeProblem { id: "q_prime", category: Code, topic: "cpp_syntax", diff: "Easy", title: "Check prime", prompt: r#"Read N. Print "yes" if prime, else "no"."#, solution: "", day: 2 },
    PracticeProblem { id: "q_table", category: Code, topic: "cpp_syntax", diff: "Easy", title: "Multiplication table", prompt: r#"Read N. Print N x 1 through N x 10, one per line as "N x i = result"."#, solution: "", day: 2 },
    PracticeProblem { id: "q_gcd", category: Code, topic: "cpp_recursion", diff: "Easy", title: "GCD of two numbers", prompt: r#"Read two integers A and B. Print their GCD."#, solution: "", day: 2 },
    PracticeProblem { id: "q_palin", category: Code, topic: "cpp_arrays", diff: "Easy", title: "Check palindrome", prompt: r#"Read a string. Print "yes" if it reads the same backwards, else "no"."#, solution: "", day: 2 },
    PracticeProblem { id: "q_vowels", category: Code, topic: "cpp_arrays", diff: "Easy", title: "Count vowels", prompt: r#"Read a string. Print the number of vowels (a,e,i,o,u)."#, solution: "", day: 2 },
    PracticeProblem { id: "q_vec_print", category: Code, topic: "cpp_vectors", diff: "Easy", title: "Read into a vector and print", prompt: r#"Read N, then N integers into a vector. Print them back, space-separated."#, solution: "", day: 3 },
    PracticeProblem { id: "q_vec_max", category: Code, topic: "cpp_vectors", diff: "Easy", title: "Largest element in a vector", prompt: r#"Read N, then N integers into a vector. Print the largest element."#, solution: "", day: 3 },
    PracticeProblem { id: "q_vec_sum", category: Code, topic: "cpp_vectors", diff: "Easy", title: "Sum of vector elements", prompt: r#"Read N, then N integers into a vector. Print the sum of all elements."#, solution: "", day: 3 },
    PracticeProblem { id: "q_movezeros", category: Code, topic: "cpp_arrays", diff: "Medium", title: "Move all zeros to the end", prompt: r#"Read N, then N integers. Move all zeros to the end (keeping the order of non-zeros). Print the result space-separated."#, solution: "", day: 3 },
    PracticeProblem { id: "q_rotate1", category: Code, topic: "cpp_arrays", diff: "Easy", title: "Left rotate array by one", prompt: r#"Read N, then N integers. Rotate the array left by one position. Print the result space-separated."#, solution: "", day: 3 },
    PracticeProblem { id: "q_missing", category: Code, topic: "cpp_arrays", diff: "Medium", title: "Find the missing number (1 to N)", prompt: r#"You are given N-1 distinct integers from 1..N (one is missing). Read N, then the N-1 integers. Print the missing number."#, solution: "", day: 3 },

    // ── Math (MATH_BANK) ──
    PracticeProblem { id: "m_dice8", category: Math, topic: "math_prob", diff: "Easy", title: "Two dice sum = 8", prompt: r#"Roll two fair dice. What is the probability the sum equals 8?"#, solution: r#"Favourable pairs: (2,6)(3,5)(4,4)(5,3)(6,2) = 5. Total outcomes = 36. So <code>5/36 ≈ 0.139</code>."#, day: 1 },
    PracticeProblem { id: "m_atleast4", category: Math, topic: "math_prob", diff: "Medium", title: "At least one 4", prompt: r#"Roll two fair dice. Probability that at least one die shows a 4?"#, solution: r#"P(no 4) = (5/6)² = 25/36. So P(at least one 4) = 1 − 25/36 = <code>11/36 ≈ 0.306</code>."#, day: 1 },
    PracticeProblem { id: "m_ace", category: Math, topic: "math_prob", diff: "Easy", title: "Draw an Ace", prompt: r#"Probability of drawing an Ace from a standard 52-card deck?"#, solution: r#"4 aces / 52 cards = <code>1/13 ≈ 0.077</code>."#, day: 1 },
    PracticeProblem { id: "m_comb", category: Math, topic: "math_prob", diff: "Medium", title: "Choose 2 of 5", prompt: r#"How many ways to choose 2 items from 5 distinct items? (C(5,2))"#, solution: r#"C(5,2) = 5! / (2!·3!) = 120 / (2·6) = <code>10</code>."#, day: 1 },
    PracticeProblem { id: "m_linear", category: Math, topic: "math_algebra", diff: "Easy", title: "Solve 2x+3=11", prompt: r#"Solve for x:  2x + 3 = 11.  x = ?"#, solution: r#"2x = 11 − 3 = 8, so x = <code>4</code>."#, day: 1 },
    PracticeProblem { id: "m_log", category: Math, topic: "math_algebra", diff: "Easy", title: "log₂(8)", prompt: r#"Evaluate:  log₂(8) = ?"#, solution: r#"2³ = 8, so log₂(8) = <code>3</code>."#, day: 1 },
    PracticeProblem { id: "m_func", category: Math, topic: "math_algebra", diff: "Easy", title: "f(x)=x², f(5)", prompt: r#"If f(x) = x², what is f(5)?"#, solution: r#"f(5) = 5² = <code>25</code>."#, day: 1 },
    PracticeProblem { id: "m_seq", category: Math, topic: "math_algebra", diff: "Easy", title: "Next: 2,4,8,16…", prompt: r#"Next term in the sequence: 2, 4, 8, 16, … ?"#, solution: r#"Each term doubles (×2), so next is 16 × 2 = <code>32</code>."#, day: 1 },
    PracticeProblem { id: "m_ev_die", category: Math, topic: "math_stats", diff: "Easy", title: "Expected value of a die", prompt: r#"Expected value of a single roll of a fair six-sided die?"#, solution: r#"(1+2+3+4+5+6)/6 = 21/6 = <code>3.5</code>."#, day: 1 },
    PracticeProblem { id: "m_2heads", category: Math, topic: "math_prob", diff: "Easy", title: "Exactly 2 heads in 3 flips", prompt: r#"Flip a fair coin 3 times. Probability of exactly 2 heads?"#, solution: r#"Ways to get exactly 2 heads = C(3,2) = 3, out of 2³ = 8 outcomes. So <code>3/8 = 0.375</code>."#, day: 2 },

    // ── Quant (QUANT_BANK) ──
    PracticeProblem { id: "q_batball", category: Quant, topic: "qt_reasoning", diff: "Easy", title: "Bat and ball", prompt: r#"A bat and a ball cost $1.10 together. The bat costs $1.00 more than the ball. How much is the ball, in cents?"#, solution: r#"Let ball = b. Bat = b + 1.00. Then b + (b+1.00) = 1.10 → 2b = 0.10 → b = 0.05 = <code>5 cents</code>."#, day: 1 },
    PracticeProblem { id: "q_widgets", category: Quant, topic: "qt_reasoning", diff: "Easy", title: "5 machines, 5 widgets", prompt: r#"If 5 machines make 5 widgets in 5 minutes, how many minutes for 100 machines to make 100 widgets?"#, solution: r#"Each machine makes 1 widget in 5 minutes. 100 machines work in parallel, so 100 widgets still take <code>5 minutes</code>."#, day: 1 },
    PracticeProblem { id: "q_squares", category: Quant, topic: "qt_decompose", diff: "Medium", title: "Squares on a chessboard", prompt: r#"How many squares (of all sizes) are on a standard 8×8 chessboard?"#, solution: r#"Count k×k squares for k=1..8: 8²+7²+…+1² = 64+49+36+25+16+9+4+1 = <code>204</code>."#, day: 1 },
    PracticeProblem { id: "q_eggs", category: Quant, topic: "qt_optimization", diff: "Hard", title: "Two eggs, 100 floors", prompt: r#"Two identical eggs, a 100-floor building. Minimum number of drops to guarantee finding the highest safe floor (worst case)?"#, solution: r#"Find smallest x with x(x+1)/2 ≥ 100. x=14 gives 105 ≥ 100. So <code>14 drops</code>."#, day: 1 },
    PracticeProblem { id: "q_lookandsay", category: Quant, topic: "qt_reasoning", diff: "Medium", title: "Look-and-say sequence", prompt: r#"Next term: 1, 11, 21, 1211, 111221, … ?"#, solution: r#"Each term describes the previous: 111221 is "three 1s, two 2s, one 1" → <code>312211</code>."#, day: 1 },
    PracticeProblem { id: "q_socks", category: Quant, topic: "qt_decompose", diff: "Easy", title: "Matching socks in the dark", prompt: r#"A drawer has 10 black and 10 white socks (unsorted, in the dark). How many socks must you pull to guarantee a matching pair?"#, solution: r#"Only 2 colours exist. By the pigeonhole principle, <code>3 socks</code> guarantees two of the same colour."#, day: 1 },
    PracticeProblem { id: "q_clock", category: Quant, topic: "qt_reasoning", diff: "Medium", title: "Clock hand overlaps", prompt: r#"How many times do the hour and minute hands of a clock overlap in 12 hours?"#, solution: r#"The hands align every 12/11 hours, giving <code>11 overlaps</code> in 12 hours (not 12)."#, day: 1 },
    PracticeProblem { id: "q_monty", category: Quant, topic: "qt_probabilistic", diff: "Medium", title: "Monty Hall", prompt: r#"Monty Hall: 3 doors, you pick one, host opens a goat door, you switch. Probability you win the car?"#, solution: r#"Your initial pick wins 1/3. Switching wins whenever your first pick was wrong = <code>2/3 ≈ 0.667</code>."#, day: 1 },
    PracticeProblem { id: "q_die6even", category: Quant, topic: "qt_probabilistic", diff: "Easy", title: "Even vs six", prompt: r#"Roll one fair die. What is the probability the number is even?"#, solution: r#"Even faces {2,4,6} → 3/6 = <code>1/2</code>. Since P(=6) is only 1/6, betting on "even" (1/2) is the better choice."#, day: 2 },
    PracticeProblem { id: "q_birthday", category: Quant, topic: "qt_probabilistic", diff: "Medium", title: "Birthday paradox", prompt: r#"In a room of 23 people, is the probability that two share a birthday more than 50%?"#, solution: r#"Yes — the famous birthday paradox. With 23 people the probability is about 50.7%, so just over <code>50%</code>."#, day: 2 },
    PracticeProblem { id: "q_o1on", category: Quant, topic: "qt_decompose", diff: "Medium", title: "O(1) vs O(n)", prompt: r#"Why is accessing arr[n] O(1), but searching for an element O(n)? Think it through, then reveal."#, solution: r#"An array is a contiguous block in memory. <code>arr[n]</code> is computed directly as <code>base_address + n × element_size</code> — one arithmetic step, independent of array length, so <strong>O(1)</strong>.<br/><br/>Searching for a <em>value</em> (not an index) means you don't know where it lives, so in the worst case you must inspect every element until you find it (or reach the end) — that's up to N comparisons, so <strong>O(n)</strong>. Indexing uses position; searching must examine contents."#, day: 2 },
    PracticeProblem { id: "q_3digit", category: Quant, topic: "qt_reasoning", diff: "Easy", title: "3-digit numbers", prompt: r#"How many 3-digit numbers can be formed using the digits 1, 2, 3, 4 without repetition?"#, solution: r#"First digit: 4 choices, second: 3 left, third: 2 left → 4 × 3 × 2 = <code>24</code>."#, day: 3 },
    PracticeProblem { id: "q_handshake", category: Quant, topic: "qt_reasoning", diff: "Easy", title: "Handshakes", prompt: r#"10 people in a room. Everyone shakes hands once with everyone else. How many handshakes total?"#, solution: r#"C(10,2) = 10·9/2 = <code>45</code>."#, day: 3 },
    PracticeProblem { id: "q_password", category: Quant, topic: "qt_reasoning", diff: "Medium", title: "Password count", prompt: r#"A password is 2 letters (a–z) followed by 2 digits (0–9), e.g. ab12. How many possible passwords are there?"#, solution: r#"26 choices per letter and 10 per digit, in fixed positions: 26 × 26 × 10 × 10 = <code>67,600</code>."#, day: 3 },
    PracticeProblem { id: "q_vec_concept", category: Quant, topic: "cpp_vectors", diff: "Medium", title: "Why vectors?", prompt: r#"Why does vector.push_back(x) feel easier than int arr[100]? What problem is a vector solving?"#, solution: r#"A raw array like <code>int arr[100]</code> has a <strong>fixed size you must decide up front</strong> — too small and you overflow, too big and you waste memory, and you must track how many slots you've actually used.<br/><br/>A <code>std::vector</code> is a <strong>dynamic array</strong>: <code>push_back</code> grows it automatically as you add elements, it remembers its own <code>size()</code>, and it manages (allocates/frees) memory for you. So it solves the "I don't know how many items I'll have" problem and removes manual memory/size bookkeeping."#, day: 3 },

    // ── Quant · Brainteasers (BRAINTEASER_BANK) ──
    PracticeProblem { id: "q_bt_ropes", category: Quant, topic: "qt_reasoning", diff: "Hard", title: "Burning ropes — measure 45 minutes", prompt: r#"You have two ropes. Each takes exactly 60 minutes to burn end-to-end, but they burn unevenly (half the rope may not take 30 minutes). With only a lighter, how do you measure exactly 45 minutes?"#, solution: r#"Light rope A at <strong>both ends</strong> and rope B at <strong>one end</strong> at the same time. Rope A burns from both ends, so it is consumed in 30 minutes. The instant rope A finishes (t = 30), light the <strong>other end</strong> of rope B. Rope B had 30 minutes of burn left as a single flame; with two flames it now finishes in 15 minutes. Total = 30 + 15 = <code>45 minutes</code>."#, day: 1 },
    PracticeProblem { id: "q_bt_switches", category: Quant, topic: "qt_decompose", diff: "Medium", title: "Three switches, one bulb", prompt: r#"Three light switches outside a room each may control a single bulb inside. The door is closed and you can't see the bulb. You may flip switches as much as you like, but you can enter the room only once. How do you determine which switch controls the bulb?"#, solution: r#"Turn switch 1 ON and leave it for a few minutes, then turn it OFF. Turn switch 2 ON and immediately enter the room.<br/><br/>• Bulb <strong>on</strong> → switch 2.<br/>• Bulb <strong>off but warm</strong> → switch 1 (it was on long enough to heat up).<br/>• Bulb <strong>off and cold</strong> → switch 3.<br/><br/>The trick is using <em>heat</em> as a second bit of information."#, day: 1 },
    PracticeProblem { id: "q_bt_bridge", category: Quant, topic: "qt_optimization", diff: "Medium", title: "Bridge crossing at night", prompt: r#"Four people must cross a bridge at night with one flashlight. At most two cross at a time, and a crossing pair moves at the slower person's pace. Their times are 1, 2, 7, and 10 minutes. What is the minimum total time for all four to cross?"#, solution: r#"<code>17 minutes</code>. The key idea is to send the two slowest <em>together</em> so 10 doesn't get added to 7.<br/><br/>1 &amp; 2 cross (2), 1 returns (1), 7 &amp; 10 cross (10), 2 returns (2), 1 &amp; 2 cross (2). Total = 2 + 1 + 10 + 2 + 2 = <strong>17</strong>."#, day: 1 },
    PracticeProblem { id: "q_bt_doors", category: Quant, topic: "qt_decompose", diff: "Medium", title: "100 doors toggled", prompt: r#"100 closed doors in a row. You make 100 passes. On pass k you toggle every k-th door (open↔closed). After 100 passes, how many doors are open?"#, solution: r#"<code>10</code> — the perfect squares (1, 4, 9, 16, 25, 36, 49, 64, 81, 100).<br/><br/>Door n is toggled once for each divisor of n. Divisors come in pairs (d, n/d), so the count is even — leaving the door closed — <em>unless</em> n is a perfect square, where one divisor (√n) is unpaired, giving an odd count and an open door."#, day: 1 },
    PracticeProblem { id: "q_bt_jugs", category: Quant, topic: "qt_reasoning", diff: "Medium", title: "Water jugs — measure 4 litres", prompt: r#"You have a 3-litre jug and a 5-litre jug and an unlimited water supply, but no markings. How do you measure exactly 4 litres?"#, solution: r#"Fill the 5L jug. Pour from it into the 3L jug until full → 2L remain in the 5L jug. Empty the 3L jug, pour the 2L into it. Refill the 5L jug, then top up the 3L jug (which needs 1L more) → exactly <code>4 litres</code> remain in the 5L jug."#, day: 1 },

    PracticeProblem { id: "q_bt_horses", category: Quant, topic: "qt_optimization", diff: "Hard", title: "25 horses, 5 tracks", prompt: r#"You have 25 horses and a track that races 5 at a time. You have no timer — you only see finishing order within a race. What is the minimum number of races to find the 3 fastest horses?"#, solution: r#"<code>7 races</code>. Race the 25 in 5 groups of 5 (5 races) and rank within each group. Race the 5 group-winners (race 6); call the fastest A (overall #1). Only horses that could still be 2nd or 3rd are: 2nd and 3rd from A's group, 1st and 2nd from the runner-up's group, and 1st from the third group — 5 horses. Race those (race 7); the top 2 are the overall 2nd and 3rd."#, day: 2 },
    PracticeProblem { id: "q_bt_poison", category: Quant, topic: "qt_decompose", diff: "Hard", title: "1000 bottles, 1 poisoned", prompt: r#"You have 1000 bottles of wine, exactly one of which is poisoned. The poison kills within 24 hours. With test strips that report positive after 24 hours, what is the minimum number of strips needed to find the poisoned bottle in a single 24-hour round?"#, solution: r#"<code>10</code> strips. Number the bottles 0–999 in 10-bit binary. Assign strip i to bit i, and have it taste every bottle whose i-th bit is 1. After 24 hours, read the strips as a binary number: a positive strip = 1, negative = 0. That number is the poisoned bottle's index. Since 2¹⁰ = 1024 ≥ 1000, ten strips suffice."#, day: 2 },
    PracticeProblem { id: "q_bt_ants", category: Quant, topic: "qt_probabilistic", diff: "Medium", title: "Ants on a triangle", prompt: r#"Three ants sit at the corners of a triangle. Each independently picks a direction (clockwise or counter-clockwise) at random and starts walking along an edge. What is the probability that none of them collide?"#, solution: r#"<code>1/4</code>. No collision happens only if all three ants walk the same way around the triangle — all clockwise or all counter-clockwise. That's 2 favourable outcomes out of 2³ = 8 total: 2/8 = <strong>1/4</strong>."#, day: 2 },
    PracticeProblem { id: "q_bt_goldbar", category: Quant, topic: "qt_decompose", diff: "Medium", title: "Gold bar in 7 pieces", prompt: r#"You hire a worker for 7 days and pay one-seventh of a gold bar per day. You may make only 2 cuts in the bar. How do you pay them exactly 1/7 each day?"#, solution: r#"Two cuts make three pieces of length <strong>1, 2, and 4</strong> sevenths. Pay by making change each day:<br/>Day 1: give 1. Day 2: give 2, take back 1. Day 3: give 1. Day 4: give 4, take back 1 and 2. Day 5: give 1. Day 6: give 2, take back 1. Day 7: give 1. Each day the worker holds exactly the right cumulative total (1,2,3,4,5,6,7 sevenths)."#, day: 2 },

    PracticeProblem { id: "q_bt_pirates", category: Quant, topic: "qt_reasoning", diff: "Hard", title: "Pirates split the gold", prompt: r#"5 rational, greedy pirates (ranked 1–5) split 100 gold coins. The top-ranked pirate proposes a split; all pirates including the proposer vote. If at least half accept, it passes; otherwise the proposer is thrown overboard and the next-ranked pirate proposes. Each pirate prefers gold, but prefers staying alive above all, and prefers throwing others overboard when otherwise indifferent. What does pirate 5 (the top) propose?"#, solution: r#"Pirate 5 keeps <code>98</code>, gives <strong>1 each to pirates 3 and 1</strong>, and 0 to pirates 4 and 2. Reasoning backward: with 2 pirates left, pirate 2 takes all (his own vote is half). So pirate 3 only needs to buy pirate 1 with 1 coin. Pirate 4 needs to buy pirate 2 with 1 coin. Pirate 5 needs two extra votes and buys the cheapest — pirates 1 and 3 (who'd otherwise get 1 and 0 under pirate 4's plan) — with 1 coin each. 98 + 1 + 1 passes 3-of-5."#, day: 3 },
    PracticeProblem { id: "q_bt_trains", category: Quant, topic: "qt_reasoning", diff: "Medium", title: "The fly between two trains", prompt: r#"Two trains are 100 km apart on the same track, heading toward each other, each at 50 km/h. A fly starts at one train and flies back and forth between them at 75 km/h until they collide. How far does the fly travel in total?"#, solution: r#"Don't sum the infinite zig-zag — find the <em>time</em>. The trains close a 100 km gap at a combined 100 km/h, so they meet in 1 hour. The fly flies for that whole hour at 75 km/h, covering <code>75 km</code>."#, day: 3 },
    PracticeProblem { id: "q_bt_balls", category: Quant, topic: "qt_optimization", diff: "Medium", title: "8 balls, one heavier", prompt: r#"You have 8 identical-looking balls; one is slightly heavier. Using a balance scale, what is the minimum number of weighings guaranteed to find the heavy ball?"#, solution: r#"<code>2 weighings</code>. Split into groups of 3, 3, and 2. Weigh the two groups of 3. If they balance, the heavy ball is in the leftover 2 — weigh those two against each other (weighing 2). If one group of 3 is heavier, take any 2 of its 3 balls and weigh them; if they balance the third is heavy, otherwise the heavier pan wins."#, day: 3 },

    // ── Linux (LINUX_BANK) ──
    PracticeProblem { id: "l_pwd", category: Linux, topic: "linux_basics", diff: "Easy", title: "Working directory", prompt: r#"Print the current (working) directory."#, solution: r#"<code>pwd</code> — "print working directory"."#, day: 1 },
    PracticeProblem { id: "l_ls", category: Linux, topic: "linux_basics", diff: "Easy", title: "List files", prompt: r#"List the files in the current directory."#, solution: r#"<code>ls</code> lists directory contents."#, day: 1 },
    PracticeProblem { id: "l_lsla", category: Linux, topic: "linux_basics", diff: "Easy", title: "List all, long", prompt: r#"List ALL files (including hidden) in long format."#, solution: r#"<code>ls -la</code> — -l for long format, -a to include hidden (dot) files."#, day: 1 },
    PracticeProblem { id: "l_mkdir", category: Linux, topic: "linux_basics", diff: "Easy", title: "Make a directory", prompt: r#"Create a new directory called test."#, solution: r#"<code>mkdir test</code> makes a new directory."#, day: 1 },
    PracticeProblem { id: "l_cd", category: Linux, topic: "linux_basics", diff: "Easy", title: "Change directory", prompt: r#"Change into the test directory."#, solution: r#"<code>cd test</code> changes directory."#, day: 1 },
    PracticeProblem { id: "l_cdup", category: Linux, topic: "linux_basics", diff: "Easy", title: "Go up a level", prompt: r#"Move up one directory level (to the parent)."#, solution: r#"<code>cd ..</code> — ".." refers to the parent directory."#, day: 1 },
    PracticeProblem { id: "l_gpp", category: Linux, topic: "linux_cpp", diff: "Easy", title: "Compile with g++", prompt: r#"Compile a C++ file called main.cpp with g++ (default output)."#, solution: r#"<code>g++ main.cpp</code> compiles to the default executable <code>a.out</code>."#, day: 1 },
    PracticeProblem { id: "l_run", category: Linux, topic: "linux_cpp", diff: "Easy", title: "Run a.out", prompt: r#"Run the compiled program a.out from the current directory."#, solution: r#"<code>./a.out</code> — the "./" tells the shell to run from the current folder."#, day: 1 },
    PracticeProblem { id: "l_gppo", category: Linux, topic: "linux_cpp", diff: "Medium", title: "Name the output", prompt: r#"Compile main.cpp into an executable named app."#, solution: r#"<code>g++ main.cpp -o app</code> — -o sets the output filename."#, day: 1 },
    PracticeProblem { id: "l_touch", category: Linux, topic: "linux_basics", diff: "Easy", title: "Create empty file", prompt: r#"Create a new empty file called notes.txt."#, solution: r#"<code>touch notes.txt</code> creates an empty file (or updates its timestamp if it exists)."#, day: 2 },
    PracticeProblem { id: "l_rm", category: Linux, topic: "linux_basics", diff: "Easy", title: "Delete a file", prompt: r#"Delete a file called old.txt."#, solution: r#"<code>rm old.txt</code> removes a file. (Be careful — there is no trash bin!)"#, day: 2 },
    PracticeProblem { id: "l_cp", category: Linux, topic: "linux_basics", diff: "Easy", title: "Copy a file", prompt: r#"Copy a.txt to b.txt."#, solution: r#"<code>cp a.txt b.txt</code> copies a file."#, day: 2 },
    PracticeProblem { id: "l_mv", category: Linux, topic: "linux_basics", diff: "Easy", title: "Rename a file", prompt: r#"Rename (move) a.txt to b.txt."#, solution: r#"<code>mv a.txt b.txt</code> moves or renames a file."#, day: 2 },
    PracticeProblem { id: "l_cat", category: Linux, topic: "linux_basics", diff: "Easy", title: "Show file contents", prompt: r#"Show the contents of a file called notes.txt."#, solution: r#"<code>cat notes.txt</code> prints a file to the terminal."#, day: 3 },
    PracticeProblem { id: "l_echo", category: Linux, topic: "linux_basics", diff: "Easy", title: "Print text", prompt: r#"Print the text hello to the terminal."#, solution: r#"<code>echo hello</code> prints text to standard output."#, day: 3 },
    PracticeProblem { id: "l_echo_file", category: Linux, topic: "linux_basics", diff: "Medium", title: "Write to a file", prompt: r#"Write the text hi into a file called notes.txt (overwriting it), using echo."#, solution: r#"<code>echo "hi" > notes.txt</code> — the <code>></code> operator redirects output into a file, replacing its contents. (Use <code>>></code> to append instead.)"#, day: 3 },
    PracticeProblem { id: "l_nano", category: Linux, topic: "linux_basics", diff: "Easy", title: "Edit in nano", prompt: r#"Open the file notes.txt for editing in the nano text editor."#, solution: r#"<code>nano notes.txt</code> opens the file in nano. Save with Ctrl+O, exit with Ctrl+X."#, day: 3 },
];

/// Practice problems mapped to a strategy day, in display order.
pub fn practice_problems_for_day(day: u32) -> impl Iterator<Item = &'static PracticeProblem> {
    PROBLEMS.iter().filter(move |p| p.day == day)
}

/// Practice gate: every problem for `day` is marked solved (vacuously true for
/// a day with no problems).
pub fn practice_complete(day: u32, solved: &HashMap<String, bool>) -> bool {
    practice_problems_for_day(day).all(|p| solved.get(p.id).copied().unwrap_or(false))
}

/// Strategy gate: every daily-routine item is checked for this day.
pub fn routine_complete(day_checks: &HashMap<String, bool>) -> bool {
    day_checks.values().filter(|v| **v).count() >= total_daily_items()
}

/// Journal gate: all five prompts are non-empty for this day.
pub fn journal_complete(day_journal: &HashMap<String, String>) -> bool {
    ["j1", "j2", "j3", "j4", "j5"]
        .iter()
        .all(|k| day_journal.get(*k).is_some_and(|v| !v.trim().is_empty()))
}

// ── Code problem starters + samples (for the in-browser runner) ──
//
// Ported from spec.html's QUESTION_BANK. Keyed by problem id (Code-category
// problems only). `sample_out` containing "..." is a placeholder that can't be
// auto-checked — those stay manually marked.

pub struct CodeMeta {
    pub id: &'static str,
    pub starter_cpp: &'static str,
    pub starter_py: &'static str,
    pub sample_in: &'static str,
    pub sample_out: &'static str,
}

/// Starter/sample metadata for a code problem, if it has any.
pub fn code_meta(id: &str) -> Option<&'static CodeMeta> {
    CODE_META.iter().find(|m| m.id == id)
}

pub const CODE_META: &[CodeMeta] = &[
    CodeMeta {
        id: "q_sum",
        sample_in: "5\n1 2 3 4 5",
        sample_out: "15",
        starter_cpp: r#"#include <iostream>
using namespace std;
int main(){
  int n; cin>>n;
  int sum=0;
  for(int i=0;i<n;i++){
    int x; cin>>x;
    sum+=x;
  }
  cout<<sum<<endl;
  return 0;
}"#,
        starter_py: r#"n=int(input())
nums=list(map(int,input().split()))
print(sum(nums))"#,
    },
    CodeMeta {
        id: "q_max",
        sample_in: "4\n3 9 2 7",
        sample_out: "9",
        starter_cpp: r#"#include <iostream>
using namespace std;
int main(){
  int n; cin>>n;
  int mx;
  for(int i=0;i<n;i++){
    int x; cin>>x;
    if(i==0||x>mx) mx=x;
  }
  cout<<mx<<endl;
  return 0;
}"#,
        starter_py: r#"n=int(input())
nums=list(map(int,input().split()))
print(max(nums))"#,
    },
    CodeMeta {
        id: "q_min",
        sample_in: "4\n3 9 2 7",
        sample_out: "2",
        starter_cpp: r#"#include <iostream>
using namespace std;
int main(){
  int n; cin>>n;
  int mn;
  for(int i=0;i<n;i++){
    int x; cin>>x;
    if(i==0||x<mn) mn=x;
  }
  cout<<mn<<endl;
  return 0;
}"#,
        starter_py: r#"n=int(input())
nums=list(map(int,input().split()))
print(min(nums))"#,
    },
    CodeMeta {
        id: "q_rev",
        sample_in: "5\n1 2 3 4 5",
        sample_out: "5 4 3 2 1",
        starter_cpp: r#"#include <iostream>
using namespace std;
int main(){
  int n; cin>>n;
  int a[100];
  for(int i=0;i<n;i++) cin>>a[i];
  for(int i=n-1;i>=0;i--){
    cout<<a[i];
    if(i>0) cout<<" ";
  }
  cout<<endl;
  return 0;
}"#,
        starter_py: r#"n=int(input())
nums=list(map(int,input().split()))
print(*nums[::-1])"#,
    },
    CodeMeta {
        id: "q_2nd",
        sample_in: "5\n4 9 9 2 7",
        sample_out: "7",
        starter_cpp: r#"#include <iostream>
#include <set>
using namespace std;
int main(){
  int n; cin>>n;
  set<int> s;
  for(int i=0;i<n;i++){ int x; cin>>x; s.insert(x); }
  auto it=s.rbegin(); ++it;
  cout<<*it<<endl;
  return 0;
}"#,
        starter_py: r#"n=int(input())
nums=set(map(int,input().split()))
print(sorted(nums)[-2])"#,
    },
    CodeMeta {
        id: "q_search",
        sample_in: "5\n10 20 30 40 50\n30",
        sample_out: "2",
        starter_cpp: r#"#include <iostream>
using namespace std;
int main(){
  int n; cin>>n;
  int a[100];
  for(int i=0;i<n;i++) cin>>a[i];
  int t; cin>>t;
  int idx=-1;
  for(int i=0;i<n;i++) if(a[i]==t){ idx=i; break; }
  cout<<idx<<endl;
  return 0;
}"#,
        starter_py: r#"n=int(input())
nums=list(map(int,input().split()))
t=int(input())
print(nums.index(t) if t in nums else -1)"#,
    },
    CodeMeta {
        id: "q_evenodd",
        sample_in: "5\n1 2 3 4 5",
        sample_out: "2 3",
        starter_cpp: r#"#include <iostream>
using namespace std;
int main(){
  int n; cin>>n;
  int e=0,o=0;
  for(int i=0;i<n;i++){
    int x; cin>>x;
    if(x%2==0) e++; else o++;
  }
  cout<<e<<" "<<o<<endl;
  return 0;
}"#,
        starter_py: r#"n=int(input())
nums=list(map(int,input().split()))
e=sum(1 for x in nums if x%2==0)
print(e, n-e)"#,
    },
    CodeMeta {
        id: "q_fact",
        sample_in: "5",
        sample_out: "120",
        starter_cpp: r#"#include <iostream>
using namespace std;
long long fact(int n){
  if(n<=1) return 1;
  return n*fact(n-1);
}
int main(){
  int n; cin>>n;
  cout<<fact(n)<<endl;
  return 0;
}"#,
        starter_py: r#"n=int(input())
f=1
for i in range(2,n+1): f*=i
print(f)"#,
    },
    CodeMeta {
        id: "q_fib",
        sample_in: "7",
        sample_out: "0 1 1 2 3 5 8",
        starter_cpp: r#"#include <iostream>
using namespace std;
int main(){
  int n; cin>>n;
  long long a=0,b=1;
  for(int i=0;i<n;i++){
    cout<<a;
    if(i<n-1) cout<<" ";
    long long c=a+b; a=b; b=c;
  }
  cout<<endl;
  return 0;
}"#,
        starter_py: r#"n=int(input())
a,b=0,1
res=[]
for _ in range(n):
    res.append(a)
    a,b=b,a+b
print(*res)"#,
    },
    CodeMeta {
        id: "q_prime",
        sample_in: "13",
        sample_out: "yes",
        starter_cpp: r#"#include <iostream>
using namespace std;
int main(){
  int n; cin>>n;
  bool prime = n>1;
  for(int i=2;i*i<=n;i++) if(n%i==0){ prime=false; break; }
  cout<<(prime?"yes":"no")<<endl;
  return 0;
}"#,
        starter_py: r#"n=int(input())
prime = n>1 and all(n%i for i in range(2,int(n**0.5)+1))
print("yes" if prime else "no")"#,
    },
    CodeMeta {
        id: "q_gcd",
        sample_in: "48 36",
        sample_out: "12",
        starter_cpp: r#"#include <iostream>
using namespace std;
int gcd(int a,int b){ return b==0?a:gcd(b,a%b); }
int main(){
  int a,b; cin>>a>>b;
  cout<<gcd(a,b)<<endl;
  return 0;
}"#,
        starter_py: r#"import math
a,b=map(int,input().split())
print(math.gcd(a,b))"#,
    },
    CodeMeta {
        id: "q_digsum",
        sample_in: "9305",
        sample_out: "17",
        starter_cpp: r#"#include <iostream>
using namespace std;
int main(){
  int n; cin>>n;
  int s=0;
  while(n>0){ s+=n%10; n/=10; }
  cout<<s<<endl;
  return 0;
}"#,
        starter_py: r#"n=input().strip()
print(sum(int(c) for c in n))"#,
    },
    CodeMeta {
        id: "q_revstr",
        sample_in: "quant",
        sample_out: "tnauq",
        starter_cpp: r#"#include <iostream>
#include <string>
#include <algorithm>
using namespace std;
int main(){
  string s; getline(cin,s);
  reverse(s.begin(),s.end());
  cout<<s<<endl;
  return 0;
}"#,
        starter_py: r#"s=input()
print(s[::-1])"#,
    },
    CodeMeta {
        id: "q_palin",
        sample_in: "level",
        sample_out: "yes",
        starter_cpp: r#"#include <iostream>
#include <string>
using namespace std;
int main(){
  string s; getline(cin,s);
  string r(s.rbegin(),s.rend());
  cout<<(s==r?"yes":"no")<<endl;
  return 0;
}"#,
        starter_py: r#"s=input()
print("yes" if s==s[::-1] else "no")"#,
    },
    CodeMeta {
        id: "q_vowels",
        sample_in: "education",
        sample_out: "5",
        starter_cpp: r#"#include <iostream>
#include <string>
using namespace std;
int main(){
  string s; getline(cin,s);
  int c=0;
  for(char ch:s){
    char l=tolower(ch);
    if(l=='a'||l=='e'||l=='i'||l=='o'||l=='u') c++;
  }
  cout<<c<<endl;
  return 0;
}"#,
        starter_py: r#"s=input().lower()
print(sum(s.count(v) for v in "aeiou"))"#,
    },
    CodeMeta {
        id: "q_table",
        sample_in: "3",
        sample_out: "3 x 1 = 3\n3 x 2 = 6\n... (through x 10)",
        starter_cpp: r#"#include <iostream>
using namespace std;
int main(){
  int n; cin>>n;
  for(int i=1;i<=10;i++)
    cout<<n<<" x "<<i<<" = "<<n*i<<"\n";
  return 0;
}"#,
        starter_py: r#"n=int(input())
for i in range(1,11):
    print(f"{n} x {i} = {n*i}")"#,
    },
    CodeMeta {
        id: "q_freq",
        sample_in: "6\n1 2 3 2 5 2\n2",
        sample_out: "3",
        starter_cpp: r#"#include <iostream>
using namespace std;
int main(){
  int n; cin>>n;
  int a[100];
  for(int i=0;i<n;i++) cin>>a[i];
  int t; cin>>t;
  int cnt=0;
  for(int i=0;i<n;i++) if(a[i]==t) cnt++;
  cout<<cnt<<endl;
  return 0;
}"#,
        starter_py: r#"n=int(input())
nums=list(map(int,input().split()))
t=int(input())
print(nums.count(t))"#,
    },
    CodeMeta {
        id: "q_sumeven",
        sample_in: "6\n1 2 3 4 5 6",
        sample_out: "12",
        starter_cpp: r#"#include <iostream>
using namespace std;
int main(){
  int n; cin>>n;
  int sum=0;
  for(int i=0;i<n;i++){
    int x; cin>>x;
    if(x%2==0) sum+=x;
  }
  cout<<sum<<endl;
  return 0;
}"#,
        starter_py: r#"n=int(input())
nums=list(map(int,input().split()))
print(sum(x for x in nums if x%2==0))"#,
    },
    CodeMeta {
        id: "q_dedup",
        sample_in: "7\n1 1 2 2 2 3 4",
        sample_out: "1 2 3 4",
        starter_cpp: r#"#include <iostream>
using namespace std;
int main(){
  int n; cin>>n;
  int prev; bool first=true;
  for(int i=0;i<n;i++){
    int x; cin>>x;
    if(first||x!=prev){
      if(!first) cout<<" ";
      cout<<x; prev=x; first=false;
    }
  }
  cout<<endl;
  return 0;
}"#,
        starter_py: r#"n=int(input())
nums=list(map(int,input().split()))
out=[]
for x in nums:
    if not out or out[-1]!=x: out.append(x)
print(*out)"#,
    },
    CodeMeta {
        id: "q_vec_print",
        sample_in: "4\n10 20 30 40",
        sample_out: "10 20 30 40",
        starter_cpp: r#"#include <iostream>
#include <vector>
using namespace std;
int main(){
  int n; cin>>n;
  vector<int> v;
  for(int i=0;i<n;i++){ int x; cin>>x; v.push_back(x); }
  for(int i=0;i<(int)v.size();i++){
    cout<<v[i];
    if(i<(int)v.size()-1) cout<<" ";
  }
  cout<<endl;
  return 0;
}"#,
        starter_py: r#"n=int(input())
v=list(map(int,input().split()))
print(*v)"#,
    },
    CodeMeta {
        id: "q_vec_max",
        sample_in: "5\n3 9 2 7 5",
        sample_out: "9",
        starter_cpp: r#"#include <iostream>
#include <vector>
using namespace std;
int main(){
  int n; cin>>n;
  vector<int> v;
  for(int i=0;i<n;i++){ int x; cin>>x; v.push_back(x); }
  int mx=v[0];
  for(int x:v) if(x>mx) mx=x;
  cout<<mx<<endl;
  return 0;
}"#,
        starter_py: r#"n=int(input())
v=list(map(int,input().split()))
print(max(v))"#,
    },
    CodeMeta {
        id: "q_vec_sum",
        sample_in: "5\n1 2 3 4 5",
        sample_out: "15",
        starter_cpp: r#"#include <iostream>
#include <vector>
using namespace std;
int main(){
  int n; cin>>n;
  vector<int> v;
  for(int i=0;i<n;i++){ int x; cin>>x; v.push_back(x); }
  int sum=0;
  for(int x:v) sum+=x;
  cout<<sum<<endl;
  return 0;
}"#,
        starter_py: r#"n=int(input())
v=list(map(int,input().split()))
print(sum(v))"#,
    },
    CodeMeta {
        id: "q_movezeros",
        sample_in: "7\n0 1 0 3 12 0 5",
        sample_out: "1 3 12 5 0 0 0",
        starter_cpp: r#"#include <iostream>
#include <vector>
using namespace std;
int main(){
  int n; cin>>n;
  vector<int> v;
  for(int i=0;i<n;i++){ int x; cin>>x; v.push_back(x); }
  vector<int> res;
  for(int x:v) if(x!=0) res.push_back(x);
  while((int)res.size()<n) res.push_back(0);
  for(int i=0;i<n;i++){ cout<<res[i]; if(i<n-1) cout<<" "; }
  cout<<endl;
  return 0;
}"#,
        starter_py: r#"n=int(input())
v=list(map(int,input().split()))
nz=[x for x in v if x!=0]
res=nz+[0]*(n-len(nz))
print(*res)"#,
    },
    CodeMeta {
        id: "q_rotate1",
        sample_in: "5\n1 2 3 4 5",
        sample_out: "2 3 4 5 1",
        starter_cpp: r#"#include <iostream>
#include <vector>
using namespace std;
int main(){
  int n; cin>>n;
  vector<int> v;
  for(int i=0;i<n;i++){ int x; cin>>x; v.push_back(x); }
  int first=v[0];
  for(int i=0;i<n-1;i++) v[i]=v[i+1];
  v[n-1]=first;
  for(int i=0;i<n;i++){ cout<<v[i]; if(i<n-1) cout<<" "; }
  cout<<endl;
  return 0;
}"#,
        starter_py: r#"n=int(input())
v=list(map(int,input().split()))
v=v[1:]+v[:1]
print(*v)"#,
    },
    CodeMeta {
        id: "q_missing",
        sample_in: "5\n1 2 4 5",
        sample_out: "3",
        starter_cpp: r#"#include <iostream>
using namespace std;
int main(){
  int n; cin>>n;
  long long total=(long long)n*(n+1)/2;
  for(int i=0;i<n-1;i++){ int x; cin>>x; total-=x; }
  cout<<total<<endl;
  return 0;
}"#,
        starter_py: r#"n=int(input())
nums=list(map(int,input().split()))
print(n*(n+1)//2 - sum(nums))"#,
    },
];
