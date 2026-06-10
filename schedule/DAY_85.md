# Day 85 — 🧠✏️ Quant Mind World · Edit Distance — Measuring How Different Two Strings Are

> Yesterday LCS measured *similarity.* Today Edit Distance measures *difference* — and powers spell-checkers, search engines, autocorrect, DNA analysis, NLP, and fuzzy matching.

**Focus:** Edit Distance · String Transformation · Minimum Operations · 2D DP · Similarity Measurement

---

## Previous Day Review (10 min)
- State what `dp[i][j]` means in LCS.
- Recall the match vs mismatch transition.

---

## The core problem
Convert `CAT` → `CUT`: replace `A` with `U` — **1 operation**, so Edit Distance = 1.

---

## Block 1 — C++ (Allowed operations)
Only three moves: **Insert** (`CAT → CHAT`), **Delete** (`CHAT → CAT`), **Replace** (`CAT → CUT`).

*Why it matters:* "minimum operations to transform A into B" is the canonical *transformation-cost* problem — the same 2D-grid DP as LCS, with min instead of max.

**Code from scratch:** `edit_distance_intro.cpp`; store `a`, `b`; print lengths.

---

## Block 2 — DSA (State design)
```
dp[i][j] = min operations to convert A[0..i-1] into B[0..j-1]
```
`dp[2][2]` converts `CA` → `CU`.

**Thinking question:** why does Edit Distance need two indices?

---

## Block 3 — DSA (Transition)
- **Match** (`A[i-1]==B[j-1]`): no op → `dp[i][j] = dp[i-1][j-1]`.
- **Differ:** `dp[i][j] = 1 + min( dp[i][j-1]  (insert), dp[i-1][j]  (delete), dp[i-1][j-1]  (replace) )`.

**Task:** compute a few cells for `CAT → CUT` by hand.

---

## Block 4 — Mathematics (Base cases)
Empty `A` → `DOG` needs 3 inserts, so `dp[0][j] = j`; likewise `dp[i][0] = i`.

**Exercise:** `"" → HELLO` — how many operations? (5.) **Challenge:** why are base cases crucial in DP? (They seed every other cell.)

---

## Block 5 — Quant Thinking (Similarity vs difference)

`APPLE` vs `APPLY` — very similar (distance 1); `APPLE` vs `TRAIN` — far.

**Problems:** spell checker; search suggestions; document comparison.

**Hard puzzle:** a user types `statisitcs` instead of `statistics` — how does Edit Distance drive autocorrect? (Pick the dictionary word with the smallest distance.)

**Career connection:** Edit Distance underlies NLP, data cleaning, entity matching, and information retrieval — real datasets are full of typos and near-matches.

---

## Block 6 — Python · Student Management System v69
Create `edit_distance.py`: init `dp`, set base cases `dp[0][j]`/`dp[i][0]`, fill via match (`dp[i-1][j-1]`) / mismatch (`1 + min(...)`). Bonus: recover the actual operation sequence (insert/delete/replace).

**Linux:** learn `cmp`; practice `cmp file1.txt file2.txt`. **Question:** how do systems compare files efficiently? Think similarity metrics.

---

## Quant Thinking Track — Transformation Cost
Many problems are *current state → target state*: what's the **minimum cost to transform** one into the other? Editing text, refactoring code, portfolio rebalancing, system migration. Strong solvers think in *transformation cost*, not just same/different.

---

## Portfolio Building
`DP/`:
- `edit_distance.cpp`
- `edit_distance_operations.cpp`

---

## Communication Exercise
In 5 lines, explain: *"Why does Edit Distance take the min of three neighbours on a mismatch?"*

---

## Journal
- Edit distance of `"" → HELLO`?
- Why two indices?
- What real "transformation cost" could I measure this way?

---

## 🚩 Day 85 Milestone
You're done when you can answer **what `dp[i][j]` represents in Edit Distance** — *the minimum insertions, deletions, and replacements to turn the first i characters of one string into the first j of the other.*

---

## Next 🚀
Day 86 — **Advanced Sequence DP:** how LIS, LCS, and Edit Distance are one family, building the pattern-recognition skills for advanced DP.

---

## Tracker Update (after Day 85)
- DSA: sequence DP → **60%**, LCS → **50%**, edit distance → **35%**
- 2D DP states → **65%**
- Similarity analysis → **45%**, transformation modeling → **35%**
- Python string DP → **50%**
