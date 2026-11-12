use crate::operations::Operation;
use crate::options::SharedOptions;
use crate::state::SharedState;

/*
 * Setting characters limit structure
 */
pub struct OpSetMaxLength {
    limit: usize,
}

/*
 * Argument identifier
 */
impl OpSetMaxLength {
    pub fn new(limit: usize) -> Self {
        Self { limit }
    }

    pub const ARG_ID: u8 = 190;
}

/*
 * Setting characters limit implementation
 */
impl Operation for OpSetMaxLength {
    fn call(&self, opts: &SharedOptions, st: &mut SharedState) {
        if opts.verbose { println!("-- Setting character limit per line to: {}", self.limit); }
        st.max_cpl = self.limit;
    }
}
