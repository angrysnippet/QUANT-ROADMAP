# QUANT ROADMAP

## A 548-Day Journey from Programming Foundations to Job-Ready Quant

A structured, long-term roadmap that takes a beginner all the way from first principles of programming to genuine readiness for **quantitative research, quantitative development, and quant-trading roles** at competitive firms — plus software-engineering internships and technical interviews along the way.

This roadmap is not a collection of random tutorials or courses.

It is a carefully sequenced, day-by-day learning system that builds a coherent skill stack:

* C++ & low-latency systems engineering
* Problem solving & data structures / algorithms
* Mathematics, probability & statistics
* Stochastic processes & quantitative finance theory
* Derivatives pricing, fixed income & rates
* Statistical arbitrage & systematic trading
* Market microstructure & execution
* Machine learning for finance
* Time-series analysis & forecasting
* Portfolio construction & risk management
* Python research workflows (NumPy / pandas / scikit-learn)
* Linux, Git & professional engineering practice
* Systems thinking, research ability & interview readiness

Each day is a self-contained, richly-written lesson with study material, *why-it-matters* context, runnable code (Python for research, C++ for systems), worked interview-style problems, cross-domain connections, and a milestone check.

---

# Vision

Most students consume endless tutorials but never build a coherent skill stack.

Common problems include:

* Learning without direction
* Weak fundamentals
* Poor retention
* Tutorial dependency
* Lack of projects
* No measurable progress
* Theory that never reaches deployable, real-world skill

This roadmap solves those problems by providing a structured day-by-day learning journey that compounds over time — and, crucially, that does not stop at toy examples. After the computer-science foundations, it builds **real quantitative-finance depth** (the kind asked about in quant interviews and used on real desks) and then the **systems engineering** to actually deploy it.

The objective is simple:

> Become someone who can think like an engineer, solve problems like a competitive programmer, reason under uncertainty like a quant, price and hedge like a derivatives desk, and build fast, correct systems like a low-latency developer.

---

# Final Target Profile

By Day 548, you should possess competence across the following domains.

## 1. C++ & Systems Engineering

* Modern C++ (RAII, smart pointers, move semantics, templates / generic programming)
* The performance mindset — memory hierarchy, cache-friendly / data-oriented design
* Concurrency & lock-free programming; the memory model
* Low-latency techniques (kernel bypass, zero-allocation hot paths, tail-latency measurement)
* Numerical computing & SIMD; floating-point correctness and stability
* Trading-system architecture (event-driven design, research-to-production parity)
* Testing, profiling & debugging (sanitizers, Amdahl's law, *verify-don't-hope*)

## 2. Problem Solving, Data Structures & Algorithms

* Arrays, strings, hashing, stacks, queues, linked lists, trees, heaps, graphs
* Recursion, binary search, greedy, dynamic programming, shortest paths
* Pattern recognition, decomposition, optimization, complexity analysis
* CSES / Codeforces / LeetCode-style problem solving

## 3. Mathematics, Probability & Statistics

* Algebra, functions, sequences, calculus and linear-algebra foundations
* Random variables, expectation, variance, conditional probability, Bayes
* Distributions, the Law of Large Numbers and Central Limit Theorem
* Estimation, hypothesis testing, regression — and the multiple-testing / overfitting traps

## 4. Stochastic Processes & Quant Mathematics

* Random walks, martingales, Brownian motion, geometric Brownian motion, Itô intuition
* Markov chains, Markov decision processes, Monte-Carlo methods
* Information theory, fat tails, and the mathematics of risk

## 5. Quantitative Finance

* **Portfolio theory & CAPM** — diversification, the efficient frontier, alpha vs beta, market efficiency
* **Derivatives & pricing** — options, put–call parity, Black–Scholes, the Greeks, delta-hedging, implied vol, binomial trees, exotics, volatility as an asset
* **Fixed income & rates** — bonds, duration / convexity, the yield curve, credit & the Merton model, short-rate models, swaps & swaptions
* **Statistical arbitrage** — mean reversion, cointegration, the Kalman filter, cross-sectional signals, alpha combination, crowding & capacity
* **Market microstructure** — the order book, market making, adverse selection, price impact, optimal execution, HFT, transaction-cost analysis
* **Machine learning for finance** — done *honestly*: labeling, purged cross-validation, feature importance, the deflated Sharpe ratio, and the limits of ML in markets
* **Time-series analysis** — ARIMA, GARCH, state-space / Kalman, regime-switching, VAR / cointegration, forecasting & nowcasting
* **Advanced portfolio & risk** — factor models, Black–Litterman, the Kelly criterion, risk parity, VaR / CVaR, extreme value theory, drawdown control, robust optimization, performance attribution

## 6. Python for Research

* Core Python, NumPy, pandas, scikit-learn
* Data manipulation, backtesting, and reproducible research workflows
* Prototyping ideas quickly and validating them rigorously

## 7. Linux & Professional Tooling

* Command line, file systems, processes, Bash scripting
* Git / GitHub, documentation, clean code, testing fundamentals
* Industry-level development workflow competence

## 8. Systems & Research Thinking

* Memory / performance awareness and computational trade-offs
* Architecture fundamentals and thinking beyond code
* Research ability — forming, testing, and *honestly evaluating* hypotheses
* Interview readiness — brainteasers, mental math, and clear technical communication

---

# Roadmap Structure

The roadmap progresses through tightly-sequenced **modules** (shown as Phases in the companion website). Each builds on the last, moving from foundations to genuine quant depth to deployable engineering and, finally, to interview readiness. Approximate day ranges:

## Foundations — Programming, Data Structures & Algorithms (Days 1–115)

* **Phase 1 (Days 1–14)** — Foundation World: C++ basics, arrays, vectors, hashing, binary search, recursion, pointers & references
* **Phase 2 (Days 15–41)** — OOP & Data Structures: classes, linked lists, stacks, queues, graphs, BFS/DFS, topological sort
* **Phase 3 (Days 42–66)** — Dynamic Programming & Trees: memoization/tabulation, trees, BSTs, heaps, hashing, DSU, MST
* **Phase 4 (Days 67–100)** — Algorithms: shortest paths, advanced graph & DP techniques, problem-solving patterns
* **Phase 5 (Days 101–115)** — Advanced Problem Solving: combining techniques, state-space modelling, the meta-skill of algorithm design

## Quantitative Foundations (Days 116–143)

* **Phase 6 — Quant Mathematics Core:** probability & statistics with real depth, expected value, variance & risk, the LLN/CLT, Bayes, hypothesis testing, regression, an honest ML intro, dynamic programming & Bellman, Markov processes & MDPs, Monte-Carlo methods, and stochastic processes (random walk → martingale → Brownian motion → GBM → Itô)

## Quantitative Finance (Days 144–220)

* **Phase 7 (Days 144–150)** — Finance & Portfolio Theory: return vs risk, the Sharpe ratio, correlation & diversification, MPT, CAPM, alpha, market efficiency
* **Phase 8 (Days 151–160)** — Derivatives & Pricing: optionality, put–call parity, Black–Scholes, the Greeks, delta-hedging, implied vol & the smile, binomial trees, exotics, volatility as an asset
* **Phase 9 (Days 161–170)** — Fixed Income & Rates: bonds, duration & convexity, the yield curve, carry, immunization, credit & Merton, short-rate models, interest-rate derivatives
* **Phase 10 (Days 171–180)** — Statistical Arbitrage: mean reversion, cointegration, Ornstein–Uhlenbeck, the Kalman filter, cross-sectional signals, alpha combination, crowding & capacity
* **Phase 11 (Days 181–190)** — Market Microstructure: order books, market making, adverse selection, price impact, optimal execution (Almgren–Chriss), HFT, transaction-cost analysis, market structure
* **Phase 12 (Days 191–200)** — Machine Learning for Finance: why finance breaks naive ML, labeling & features, purged cross-validation, tree ensembles, feature importance, deep learning & NLP/alt-data, RL, and backtest overfitting / the deflated Sharpe
* **Phase 13 (Days 201–210)** — Advanced Time Series: ARIMA, GARCH, state-space & Kalman, regime-switching/HMM, VAR & Granger causality, cointegration systems, spectral analysis, forecasting & nowcasting
* **Phase 14 (Days 211–220)** — Advanced Portfolio & Risk: factor models, Black–Litterman, the Kelly criterion, risk parity, VaR/CVaR, extreme value theory, drawdown control, robust optimization, performance attribution

## Engineering & Deployment (Days 221–230)

* **Phase 15 — C++, Software Engineering & Systems:** why C++ rules low-latency finance, memory/cache & data-oriented design, templates, RAII & smart pointers, concurrency & lock-free programming, low-latency techniques, numerical computing & SIMD, trading-system architecture, and testing/profiling/debugging — *the engineering that turns validated theory into deployable, money-making systems*

## Mastery & Interview Readiness (Days 231–548)

* Quant brainteasers & interview preparation (probability, combinatorics, Bayes, game theory, estimation, mental math, behavioral/fit)
* Portfolio projects that combine the full stack
* Revision, consolidation, and job-application strategy

> The day ranges are approximate and the later phases continue to evolve. The throughline never changes: **each day teaches one substantial concept with depth, code, and an interview-grade problem — and they compound.**

---

# Learning Philosophy

## Consistency Over Intensity

Small progress every day compounds dramatically over time.

---

## Understanding Over Memorization

Always ask:

* Why does this work?
* How does this work?
* When should I use this?

Every lesson is built around *why-it-matters* and cross-domain connections, not rote facts.

---

## Practice Driven Learning

Learning Cycle:

Learn → Implement → Practice → Review → Repeat

Each day includes runnable code and a worked problem so you *build*, not just read.

---

## Revision Is Mandatory

Recommended revision schedule:

* Day 0
* Day 3
* Day 7
* Day 21
* Day 60

Knowledge that is not revised is eventually forgotten. (Each day opens with a short *Previous Day Review* for exactly this reason.)

---

## Intellectual Honesty

A recurring theme across the quant modules: **don't fool yourself.** Validate edges out-of-sample, deflate for the number of trials, respect estimation error and fat tails, and verify code by measurement — not hope. The single most valuable quant skill is judging whether a result is *real*.

---

## Projects Create Real Skill

Watching tutorials creates familiarity.

Building projects creates capability.

---

# Progress Tracking

Suggested status system:

* [ ] Not Started
* [~] In Progress
* [x] Completed
* [R] Revised

Example:

Day 1 [x]

Day 2 [x]

Day 3 [R]

Day 4 [ ]

---

# Daily Audit Template

For every completed day:

Date:

Planned Time:

Actual Time:

Difficulty (1–10):

Understanding (1–10):

What I Learned:

Problems Faced:

Improvements For Tomorrow:

---

# Who Is This Roadmap For?

This roadmap is suitable for:

* Quant aspirants (research, development, trading)
* Computer Science & Engineering students
* Self-learners building a serious, coherent skill stack
* Software Engineering & internship seekers
* Placement & technical-interview candidates

It assumes only basic comfort with a computer and a willingness to show up daily.

---

# Expected Outcome

Upon successful completion, you should have:

✅ Strong C++ fundamentals and low-latency systems awareness

✅ Solid data-structures & algorithms problem-solving

✅ Probability, statistics & stochastic-process fluency

✅ Genuine quantitative-finance knowledge — derivatives, fixed income, stat-arb, microstructure, portfolio & risk

✅ Machine-learning-for-finance done honestly

✅ Time-series & forecasting skills

✅ Python research and Linux / Git engineering competence

✅ The ability to take a strategy from research idea to deployed, tested system

✅ Multiple portfolio projects

✅ Interview readiness — brainteasers, mental math, and clear communication

---

# Website

Roadmap Website:

https://angrysnippet.github.io/QUANT-ROADMAP/

---

# Disclaimer

This roadmap is a long-term skill development system.

## AI study planner

The `/planner` route embeds the separately deployed AI Study Platform. During
local development it defaults to `http://localhost:5173`. For production, build
the Quant Roadmap frontend with the planner client URL set at compile time:

```powershell
$env:AI_STUDY_PLATFORM_URL='https://your-planner.vercel.app'
dx build --release
```

The planner API remains a separate Railway or Render service. Configure its
`CLIENT_URL` and the planner client's `VITE_API_URL` as described in the AI
Study Platform deployment guide.

The goal is not speed.

The goal is depth.

The goal is not completing days.

The goal is becoming the type of person capable of solving difficult problems, building useful systems, conducting quantitative analysis, and succeeding in highly competitive technical environments.

---

# Day 1 → Day 548

One day.

One concept.

One problem.

One improvement.

Repeated consistently for 548 days.

Trust the process.
