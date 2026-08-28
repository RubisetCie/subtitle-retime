use subtitler::SubtitleFormat;
use subtitler::model::validation::ValidationIssue;

use crate::operations::Operation;
use crate::options::SharedOptions;
use crate::state::SharedState;

/*
 * Validation operation structure
 */
pub struct OpValidate {}

impl OpValidate {
    pub fn new() -> Self {
        Self {}
    }
}

/*
 * Validation operation implementation
 */
impl Operation for OpValidate {
    fn call(&self, _: &SharedOptions, st: &mut SharedState) {
        let subtitles = st.subtitle.as_ref().unwrap().subtitles();
        let mut issues = Vec::<ValidationIssue>::new();

        // Duration validation
        for (i, s) in subtitles.iter().enumerate() {
            if !st.selector.select(s) { continue; }
            if s.end < s.start {
                issues.push(ValidationIssue::NegativeDuration {
                    index: i,
                    start: s.start,
                    end: s.end,
                });
            } else if s.start == s.end {
                issues.push(ValidationIssue::ZeroDuration {
                    index: i,
                    time: s.start,
                });
            }
        }

        // Overlap validation
        let mut order: Vec<usize> = (0 .. subtitles.len()).collect();
        order.sort_by_key(|&i| (subtitles[i].start, subtitles[i].end));
        for w in order.windows(2) {
            let (a, b) = (w[0], w[1]);
            if subtitles[b].start < st.selector.begin() || subtitles[a].end > st.selector.end() { continue; }
            if subtitles[b].start < subtitles[a].end {
                issues.push(ValidationIssue::Overlap {
                    index_a: a,
                    index_b: b,
                    end_a: subtitles[a].end,
                    start_b: subtitles[b].start,
                });
            }
        }

        // Consistent time validation
        for i in 1 .. subtitles.len() {
            if subtitles[i-1].start < st.selector.begin() || subtitles[i].end > st.selector.end() { continue; }
            if subtitles[i].start < subtitles[i-1].start {
                issues.push(ValidationIssue::DecreasingStartTime {
                    index: i,
                    prev_start: subtitles[i-1].start,
                    curr_start: subtitles[i].start,
                });
            }
        }

        // Check number of characters/words
        for (i, s) in subtitles.iter().enumerate() {
            if !st.selector.select(s) { continue; }
            let count = s.text.chars().count();
            if count > st.limit {
                issues.push(ValidationIssue::TextTooLong {
                    index: i,
                    chars: count,
                    max_chars: st.limit,
                });
            }

            let cps = s.chars_per_second();
            if cps > st.cps {
                issues.push(ValidationIssue::CpsTooHigh {
                    index: i,
                    cps,
                    max_cps: st.cps,
                });
            }

            let wpm = s.reading_speed_wpm();
            if wpm > st.wpm {
                issues.push(ValidationIssue::WpmTooHigh {
                    index: i,
                    wpm,
                    max_wpm: st.wpm,
                });
            }
        }

        // Check for minimum gap
        for i in 1 .. subtitles.len() {
            if subtitles[i-1].end < st.selector.begin() || subtitles[i].start > st.selector.end() { continue; }
            let gap = subtitles[i].start.saturating_sub(subtitles[i-1].end);
            if gap < st.gap {
                issues.push(ValidationIssue::TooLongGap {
                    index: i,
                    prev_end: subtitles[i-1].end,
                    curr_start: subtitles[i].start,
                    gap_ms: gap,
                });
            }
        }

        // Display the issues found
        if !issues.is_empty() {
            for i in issues {
                println!("Validation: {}", i.description());
            }
        } else {
            println!("Validation: no issues found!");
        }
    }
}
