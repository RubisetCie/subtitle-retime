use crate::operations::Operation;
use crate::options::SharedOptions;
use crate::state::SharedState;

/*
 * Setting minimum gap structure
 */
pub struct OpSetMinGap {
    gap: u64,
}

/*
 * Argument identifier
 */
impl OpSetMinGap {
    pub fn new(gap: u64) -> Self {
        Self { gap }
    }

    pub const ARG_ID: u8 = 193;
}

/*
 * Setting minimum gap implementation
 */
impl Operation for OpSetMinGap {
    fn call(&self, opts: &SharedOptions, st: &mut SharedState) {
        if opts.verbose { println!("-- Setting minimum gap between subtitles to: {} ms", self.gap); }
        st.min_gap = self.gap;
    }
}
