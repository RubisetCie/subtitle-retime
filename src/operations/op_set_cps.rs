use crate::operations::Operation;
use crate::options::SharedOptions;
use crate::state::SharedState;

/*
 * Setting maximum characters per second structure
 */
pub struct OpSetCps {
    cps: f64,
}

/*
 * Argument identifier
 */
impl OpSetCps {
    pub fn new(cps: f64) -> Self {
        Self { cps }
    }

    pub const ARG_ID: u8 = 191;
}

/*
 * Setting maximum characters per second implementation
 */
impl Operation for OpSetCps {
    fn call(&self, opts: &SharedOptions, st: &mut SharedState) {
        if opts.verbose { println!("-- Setting maximum characters per second to: {}", self.cps); }
        st.cps = self.cps;
    }
}
