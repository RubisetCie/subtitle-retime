use subtitler::SubtitleFile;

use crate::operations::Selector;
use crate::StAll;

/*
 * Default values for validation parameters
 */
const DEFAULT_WPM: f64 = f64::INFINITY;
const DEFAULT_CPS: f64 = 16.0;
const DEFAULT_CPL: usize = 42;
const DEFAULT_GAP: u64 = 83;
const DEFAULT_LINES: usize = 2;
const DEFAULT_MAX_DUR: u64 = u64::MAX;
const DEFAULT_MIN_DUR: u64 = u64::MIN;

/*
 * The structure containing the shared state between operations
 */
pub struct SharedState {
    pub subtitle: Option<SubtitleFile>,
    pub selector: Box<dyn Selector>,

    // Validation related
    pub max_wpm: f64,
    pub max_cps: f64,
    pub max_cpl: usize,
    pub max_lines: usize,
    pub max_dur: u64,
    pub min_dur: u64,
    pub min_gap: u64
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            subtitle: None,
            selector: Box::new(StAll::new()), // Select everything by default
            max_wpm: DEFAULT_WPM,
            max_cps: DEFAULT_CPS,
            max_cpl: DEFAULT_CPL,
            max_lines: DEFAULT_LINES,
            max_dur: DEFAULT_MAX_DUR,
            min_dur: DEFAULT_MIN_DUR,
            min_gap: DEFAULT_GAP
        }
    }

    // Reset to its initial state
    pub fn reset(&mut self, subtitles: SubtitleFile) {
        self.subtitle = Some(subtitles);
        self.selector = Box::new(StAll::new());
        self.max_wpm = DEFAULT_WPM;
        self.max_cps = DEFAULT_CPS;
        self.max_cpl = DEFAULT_CPL;
        self.max_lines = DEFAULT_LINES;
        self.max_dur = DEFAULT_MAX_DUR;
        self.min_dur = DEFAULT_MIN_DUR;
        self.min_gap = DEFAULT_GAP;
    }
}
