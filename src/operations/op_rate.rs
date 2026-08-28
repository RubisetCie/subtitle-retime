use subtitler::SubtitleFormat;

use crate::operations::Operation;
use crate::options::SharedOptions;
use crate::state::SharedState;

/*
 * Rate operation structure
 */
pub struct OpRate {
    from: f64,
    to: f64,
    ratio: f64
}

/*
 * Argument identifier
 */
impl OpRate {
    pub fn new(from: f64, to: f64) -> Self {
        Self { from, to, ratio: to / from }
    }

    pub const ARG_ID: u8 = 102;
}

/*
 * Rate operation implementation
 */
impl Operation for OpRate {
    fn call(&self, opts: &SharedOptions, st: &mut SharedState) {
        if opts.verbose { println!("-- Changing rate from: {} fps to: {} fps", self.from, self.to); }
        for s in st.subtitle.as_mut().unwrap().subtitles_mut() {
            if !st.selector.select(s) { continue; }
            let begin = st.selector.begin();
            s.start = ((((s.start - begin) as f64) * self.ratio).round() as u64) + begin;
            s.end = ((((s.end - begin) as f64) * self.ratio).round() as u64) + begin;
        }
    }
}
