use subtitler::Subtitle;
use subtitler::utils::format_timestamp;

use crate::operations::Operation;
use crate::operations::Selector;
use crate::options::SharedOptions;
use crate::state::SharedState;

/*
 * Select between structure
 */
pub struct StBetween {
    from: u64,
    to: u64
}

/*
 * Argument identifier
 */
impl StBetween {
    pub fn new(from: u64, to: u64) -> Self {
        Self { from, to }
    }

    pub fn clone(&self) -> StBetween {
        StBetween { from: self.from, to: self.to }
    }

    pub const ARG_ID: u8 = 202;
}

/*
 * Select between implementation
 */
impl Selector for StBetween {
    fn select(&self, sub: &Subtitle) -> bool {
        sub.istart >= self.from && sub.iend <= self.to
    }

    fn begin(&self) -> u64 {
        self.from
    }

    fn end(&self) -> u64 {
        self.to
    }
}

impl Operation for StBetween {
    fn call(&self, opts: &SharedOptions, st: &mut SharedState) {
        if opts.verbose { println!("-- Selecting subtitles between: {} - {}", format_timestamp(self.from, "WebVTT"), format_timestamp(self.to, "WebVTT")); }
        st.selector = Box::new(self.clone());
    }
}
