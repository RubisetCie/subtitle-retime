use subtitler::SubtitleFile;

use crate::operations::Selector;
use crate::StAll;

/*
 * Default values for validation parameters
 */
const DEFAULT_WPM: f64 = f64::INFINITY;
const DEFAULT_CPS: f64 = 16.0;
const DEFAULT_GAP: u64 = 33;
const DEFAULT_LIMIT: usize = usize::MAX;

/*
 * The structure containing the shared state between operations
 */
pub struct SharedState {
    pub subtitle: Option<SubtitleFile>,
    pub selector: Box<dyn Selector>,

    // Validation related
    pub wpm: f64,
    pub cps: f64,
    pub gap: u64,
    pub limit: usize
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            subtitle: None,
            selector: Box::new(StAll::new()), // Select everything by default
            wpm: DEFAULT_WPM,
            cps: DEFAULT_CPS,
            gap: DEFAULT_GAP,
            limit: DEFAULT_LIMIT
        }
    }

    // Reset to its initial state
    pub fn reset(&mut self, subtitles: SubtitleFile) {
        self.subtitle = Some(subtitles);
        self.selector = Box::new(StAll::new());
        self.wpm = DEFAULT_WPM;
        self.cps = DEFAULT_CPS;
        self.gap = DEFAULT_GAP;
        self.limit = DEFAULT_LIMIT;
    }
}
