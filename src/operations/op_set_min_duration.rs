use crate::operations::Operation;
use crate::options::SharedOptions;
use crate::state::SharedState;

/*
 * Setting minimum duration structure
 */
pub struct OpSetMinDur {
    duration: u64,
}

/*
 * Argument identifier
 */
impl OpSetMinDur {
    pub fn new(duration: u64) -> Self {
        Self { duration }
    }

    pub const ARG_ID: u8 = 195;
}

/*
 * Setting minimum duration implementation
 */
impl Operation for OpSetMinDur {
    fn call(&self, opts: &SharedOptions, st: &mut SharedState) {
        if opts.verbose { println!("-- Setting minimum duration for subtitles to: {} ms", self.duration); }
        st.min_dur = self.duration;
    }
}
