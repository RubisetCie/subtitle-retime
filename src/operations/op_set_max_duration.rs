use crate::operations::Operation;
use crate::options::SharedOptions;
use crate::state::SharedState;

/*
 * Setting maximum duration structure
 */
pub struct OpSetMaxDur {
    duration: u64,
}

/*
 * Argument identifier
 */
impl OpSetMaxDur {
    pub fn new(duration: u64) -> Self {
        Self { duration }
    }

    pub const ARG_ID: u8 = 194;
}

/*
 * Setting maximum duration implementation
 */
impl Operation for OpSetMaxDur {
    fn call(&self, opts: &SharedOptions, st: &mut SharedState) {
        if opts.verbose { println!("-- Setting maximum duration for subtitles to: {} ms", self.duration); }
        st.max_dur = self.duration;
    }
}
