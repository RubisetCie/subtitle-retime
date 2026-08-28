use subtitler::Subtitle;

use crate::operations::Operation;
use crate::operations::Selector;
use crate::options::SharedOptions;
use crate::state::SharedState;

/*
 * Select all structure
 */
pub struct StAll {}

impl StAll {
    pub fn new() -> Self {
        Self {}
    }
}

/*
 * Select all implementations
 */
impl Selector for StAll {
    fn select(&self, _: &Subtitle) -> bool {
        true
    }

    fn begin(&self) -> u64 {
        0
    }

    fn end(&self) -> u64 {
        u64::MAX
    }
}

impl Operation for StAll {
    fn call(&self, opts: &SharedOptions, st: &mut SharedState) {
        if opts.verbose { println!("-- Selecting all subtitles"); }
        st.selector = Box::new(StAll::new());
    }
}
