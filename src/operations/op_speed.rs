use subtitler::SubtitleFormat;

use crate::operations::Operation;
use crate::options::SharedOptions;
use crate::state::SharedState;

/*
 * Speed operation structure
 */
pub struct OpSpeed {
    factor: f64
}

/*
 * Argument identifier
 */
impl OpSpeed {
    pub fn new(factor: f64) -> Self {
        Self { factor }
    }

    pub const ARG_ID: u8 = 101;
}

/*
 * Speed operation implementation
 */
impl Operation for OpSpeed {
    fn call(&self, opts: &SharedOptions, st: &mut SharedState) {
        if opts.verbose { println!("-- Changing speed by factor: {}", self.factor); }
        for s in st.subtitle.as_mut().unwrap().subtitles_mut() {
            if !st.selector.select(s) { continue; }
            let begin = st.selector.begin();
            s.start = ((((s.start - begin) as f64) * self.factor).round() as u64) + begin;
            s.end = ((((s.end - begin) as f64) * self.factor).round() as u64) + begin;
        }
    }
}
