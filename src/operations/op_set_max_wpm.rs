use crate::operations::Operation;
use crate::options::SharedOptions;
use crate::state::SharedState;

/*
 * Setting maximum words per minute structure
 */
pub struct OpSetMaxWpm {
    wpm: f64,
}

/*
 * Argument identifier
 */
impl OpSetMaxWpm {
    pub fn new(wpm: f64) -> Self {
        Self { wpm }
    }

    pub const ARG_ID: u8 = 192;
}

/*
 * Setting maximum words per minute implementation
 */
impl Operation for OpSetMaxWpm {
    fn call(&self, opts: &SharedOptions, st: &mut SharedState) {
        if opts.verbose { println!("-- Setting maximum words per minute to: {}", self.wpm); }
        st.max_wpm = self.wpm;
    }
}
