use subtitler::SubtitleFormat;

use crate::operations::Operation;
use crate::options::SharedOptions;
use crate::state::SharedState;

/*
 * Shift operation structure
 */
pub struct OpShift {
    delay: i64
}

/*
 * Argument identifier
 */
impl OpShift {
    pub fn new(delay: i64) -> Self {
        Self { delay }
    }

    pub const ARG_ID: u8 = 100;
}

/*
 * Shift operation implementation
 */
impl Operation for OpShift {
    fn call(&self, opts: &SharedOptions, st: &mut SharedState) {
        if opts.verbose { println!("-- Shifting by: {} ms", self.delay); }
        for s in st.subtitle.as_mut().unwrap().subtitles_mut() {
            if !st.selector.select(s) { continue; }
            s.shift(self.delay);
        }
    }
}
