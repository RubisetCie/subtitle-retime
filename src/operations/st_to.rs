use subtitler::Subtitle;

use crate::operations::Operation;
use crate::operations::Selector;
use crate::options::SharedOptions;
use crate::state::SharedState;

/*
 * Select to structure
 */
pub struct StTo {
    to: u64
}

/*
 * Argument identifier
 */
impl StTo {
    pub fn new(to: u64) -> Self {
        Self { to }
    }

    pub fn clone(&self) -> StTo {
        StTo { to: self.to }
    }

    pub const ARG_ID: u8 = 201;
}

/*
 * Select to implementation
 */
impl Selector for StTo {
    fn select(&self, sub: &Subtitle) -> bool {
        sub.iend <= self.to
    }

    fn begin(&self) -> u64 {
        0
    }

    fn end(&self) -> u64 {
        self.to
    }
}

impl Operation for StTo {
    fn call(&self, opts: &SharedOptions, st: &mut SharedState) {
        if opts.verbose { println!("-- Selecting subtitles to: {}", subtitler::utils::format_timestamp(self.to, "WebVTT")); }
        st.selector = Box::new(self.clone());
    }
}
