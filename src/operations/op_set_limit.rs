use crate::operations::Operation;
use crate::options::SharedOptions;
use crate::state::SharedState;

/*
 * Setting characters limit structure
 */
pub struct OpSetLimit {
    limit: usize,
}

/*
 * Argument identifier
 */
impl OpSetLimit {
    pub fn new(limit: usize) -> Self {
        Self { limit }
    }

    pub const ARG_ID: u8 = 190;
}

/*
 * Setting characters limit implementation
 */
impl Operation for OpSetLimit {
    fn call(&self, opts: &SharedOptions, st: &mut SharedState) {
        if opts.verbose { println!("-- Setting character limit to: {}", self.limit); }
        st.limit = self.limit;
    }
}
