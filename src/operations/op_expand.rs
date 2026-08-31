use subtitler::SubtitleFormat;

use crate::operations::Operation;
use crate::options::SharedOptions;
use crate::state::SharedState;

/*
 * Expand/Contract operation structure
 */
pub struct OpExpand {
    amount: i64
}

/*
 * Argument identifier
 */
impl OpExpand {
    pub fn new(amount: i64) -> Self {
        Self { amount }
    }

    pub const ARG_ID_EXPAND: u8 = 105;
    pub const ARG_ID_CONTRACT: u8 = 106;
}

/*
 * Expand/Contract operation implementation
 */
impl Operation for OpExpand {
    fn call(&self, opts: &SharedOptions, st: &mut SharedState) {
        if opts.verbose {
            if self.amount >= 0 {
                println!("-- Expanding by: {} ms", self.amount);
            } else {
                println!("-- Contracting by: {} ms", -self.amount);
            }
        }
        let subtitles = st.subtitle.as_mut().unwrap().subtitles_mut();
        for s in &mut *subtitles {
            if !st.selector.select(s) { continue; }
            s.start = s.start.checked_sub_signed(self.amount).unwrap_or(0);
            s.end = s.end.checked_add_signed(self.amount).unwrap_or(0);

            // Ensure we're not getting reversed timings
            if s.start > s.end {
                s.start -= s.start - s.end;
                s.end = s.start;
            }
        }

        // Prevent overlaps
        for i in 1 .. subtitles.len() {
            if subtitles[i-1].end < st.selector.begin() || subtitles[i].start > st.selector.end() { continue; }
            if subtitles[i].start < subtitles[i-1].end {
                subtitles[i].start += subtitles[i-1].end - subtitles[i].start;
                subtitles[i-1].end = subtitles[i].start;
            }
        }
    }
}
