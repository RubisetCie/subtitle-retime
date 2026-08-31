use subtitler::Subtitle;

use crate::options::SharedOptions;
use crate::state::SharedState;

pub mod op_shift;
pub mod op_speed;
pub mod op_rate;
pub mod op_reference;
pub mod op_expand;
pub mod op_gap;
pub mod op_set_gap;
pub mod op_set_cps;
pub mod op_set_wpm;
pub mod op_set_limit;
pub mod op_validate;
pub mod st_all;
pub mod st_from;
pub mod st_to;
pub mod st_between;

/*
 * The base trait for operations
 */
pub trait Operation {
    fn call(&self, opts: &SharedOptions, st: &mut SharedState);
}

/*
 * The base trait for selectors
 */
pub trait Selector {
    fn select(&self, sub: &Subtitle) -> bool;

    fn begin(&self) -> u64;
    fn end(&self) -> u64;
}
