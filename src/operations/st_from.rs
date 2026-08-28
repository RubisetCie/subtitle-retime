use subtitler::Subtitle;

use crate::operations::Operation;
use crate::operations::Selector;
use crate::options::SharedOptions;
use crate::state::SharedState;

/*
 * Select from structure
 */
pub struct StFrom {
    from: u64
}

/*
 * Argument identifier
 */
impl StFrom {
    pub fn new(from: u64) -> Self {
        Self { from }
    }

    pub fn clone(&self) -> StFrom {
        StFrom { from: self.from }
    }

    pub const ARG_ID: u8 = 200;
}

/*
 * Select from implementation
 */
impl Selector for StFrom {
    fn select(&self, sub: &Subtitle) -> bool {
        sub.istart >= self.from
    }

    fn begin(&self) -> u64 {
        self.from
    }

    fn end(&self) -> u64 {
        u64::MAX
    }
}

impl Operation for StFrom {
    fn call(&self, opts: &SharedOptions, st: &mut SharedState) {
        if opts.verbose { println!("-- Selecting subtitles from: {}", subtitler::utils::format_timestamp(self.from, "WebVTT")); }
        st.selector = Box::new(self.clone());
    }
}
