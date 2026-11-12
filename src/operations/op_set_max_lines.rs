use crate::operations::Operation;
use crate::options::SharedOptions;
use crate::state::SharedState;

/*
 * Setting lines limit structure
 */
pub struct OpSetMaxLines {
    limit: usize,
}

/*
 * Argument identifier
 */
impl OpSetMaxLines {
    pub fn new(limit: usize) -> Self {
        Self { limit }
    }

    pub const ARG_ID: u8 = 196;
}

/*
 * Setting lines limit implementation
 */
impl Operation for OpSetMaxLines {
    fn call(&self, opts: &SharedOptions, st: &mut SharedState) {
        if opts.verbose { println!("-- Setting number of lines limit to: {}", self.limit); }
        st.max_lines = self.limit;
    }
}
