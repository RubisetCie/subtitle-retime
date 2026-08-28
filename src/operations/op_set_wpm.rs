use crate::operations::Operation;
use crate::options::SharedOptions;
use crate::state::SharedState;

/*
 * Setting maximum words per minute structure
 */
pub struct OpSetWpm {
    wpm: f64,
}

/*
 * Argument identifier
 */
impl OpSetWpm {
    pub fn new(wpm: f64) -> Self {
        Self { wpm }
    }

    pub const ARG_ID: u8 = 192;
}

/*
 * Setting maximum words per minute implementation
 */
impl Operation for OpSetWpm {
    fn call(&self, opts: &SharedOptions, st: &mut SharedState) {
        if opts.verbose { println!("-- Setting maximum words per minute to: {}", self.wpm); }
        st.wpm = self.wpm;
    }
}
