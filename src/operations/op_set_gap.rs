use crate::operations::Operation;
use crate::options::SharedOptions;
use crate::state::SharedState;

/*
 * Setting minimum gap structure
 */
pub struct OpSetGap {
    gap: u64,
}

/*
 * Argument identifier
 */
impl OpSetGap {
    pub fn new(gap: u64) -> Self {
        Self { gap }
    }

    pub const ARG_ID: u8 = 193;
}

/*
 * Setting minimum gap implementation
 */
impl Operation for OpSetGap {
    fn call(&self, opts: &SharedOptions, st: &mut SharedState) {
        if opts.verbose { println!("-- Setting minimum gap between subtitles to: {}", self.gap); }
        st.gap = self.gap;
    }
}
