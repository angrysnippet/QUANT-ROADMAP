//! Static roadmap data ported verbatim from `spec.html`:
//! - `DAILY_BLOCKS`: the eight daily routine blocks and their checkable items
//!   (21 items total). Daily-check state is keyed `"{block_index}_{item_index}"`
//!   per date.
//! - `DAYS`: the day-by-day strategy schedule. `schedule_start` maps "Day 1"
//!   onto a calendar date; each subsequent day follows one calendar day later.

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

pub struct StrategyDay {
    pub id: i64,
    pub title: &'static str,
    pub phase: &'static str,
    /// Number of blocks in this strategy day (used for the "N blocks" meta).
    pub blocks: usize,
}

pub const DAYS: &[StrategyDay] = &[
    StrategyDay { id: 1, phase: "Phase 1", blocks: 7, title: "Arrays Fundamentals" },
    StrategyDay { id: 2, phase: "Phase 1", blocks: 7, title: "Array Traversal, Frequency & Search" },
    StrategyDay { id: 3, phase: "Phase 1", blocks: 7, title: "STL Vectors, Counting & Patterns" },
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
