use std::process;
use subtitler::SubtitleFormat;

use crate::operations::Operation;
use crate::options::SharedOptions;
use crate::state::SharedState;

/*
 * Gap modes enumeration
 */
#[derive(PartialEq)]
enum OpGapMode {
    Start,
    End,
    Dual,
    Smart,
    Invalid
}

fn gap_mode_to_string(mode: &OpGapMode) -> &str {
    match mode {
        OpGapMode::Start   => "start",
        OpGapMode::End     => "end",
        OpGapMode::Dual    => "dual",
        OpGapMode::Smart   => "smart",
        OpGapMode::Invalid => "invalid"
    }
}

fn gap_mode_from_string(mode: &str) -> OpGapMode {
    match mode.to_lowercase().as_str() {
        "start" => OpGapMode::Start,
        "end"   => OpGapMode::End,
        "dual"  => OpGapMode::Dual,
        "smart" => OpGapMode::Smart,
        _       => OpGapMode::Invalid
    }
}

/*
 * Gap operation structure
 */
pub struct OpGap {
    mode: OpGapMode
}

/*
 * Argument identifier
 */
impl OpGap {
    pub fn new(mode: String) -> Self {
        let gap_mode = gap_mode_from_string(&mode);
        if gap_mode == OpGapMode::Invalid {
            eprintln!("Error: Could not parse gap mode: {}", mode);
            process::exit(2);
        }
        Self { mode: gap_mode }
    }

    pub const ARG_ID: u8 = 104;
}

/*
 * Gap operation implementation
 */
impl Operation for OpGap {
    fn call(&self, opts: &SharedOptions, st: &mut SharedState) {
        if opts.verbose { println!("-- Applying gap of: {} ms with mode: {}", st.gap, gap_mode_to_string(&self.mode)); }
        let subtitles = st.subtitle.as_mut().unwrap().subtitles_mut();
        for i in 1 .. subtitles.len() {
            if subtitles[i-1].end < st.selector.begin() || subtitles[i].start > st.selector.end() { continue; }

            // Dig the gap
            let gap = subtitles[i].start.saturating_sub(subtitles[i-1].end);
            if gap < st.gap {
                match self.mode {
                    OpGapMode::Start => {
                        subtitles[i].start += st.gap - gap;
                    },
                    OpGapMode::End => {
                        subtitles[i-1].end -= st.gap - gap;
                    },
                    OpGapMode::Dual => {
                        let delta = st.gap - gap;
                        let dt_start = delta / 2;
                        let dt_end = delta - dt_start;
                        subtitles[i].start += dt_start;
                        subtitles[i-1].end -= dt_end;
                    },
                    OpGapMode::Smart => {
                        // Check the CPS of both subtitles
                        let cps_a = subtitles[i].chars_per_second();
                        let cps_b = subtitles[i-1].chars_per_second();
                        if cps_a < cps_b {
                            subtitles[i].start += st.gap - gap;
                        } else {
                            subtitles[i-1].end -= st.gap - gap;
                        }
                    },
                    OpGapMode::Invalid => break
                }
            }
        }
    }
}
