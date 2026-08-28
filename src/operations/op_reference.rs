use std::fs;
use std::process;
use subtitler::SubtitleFormat;
use subtitler::SubtitleFile;

use crate::operations::Operation;
use crate::options::SharedOptions;
use crate::state::SharedState;

/*
 * Reference operation structure
 */
pub struct OpReference {
    path: String,
    file: SubtitleFile
}

/*
 * Argument identifier
 */
impl OpReference {
    pub fn new(filename: String) -> Self {
        // Load the reference file
        let data = fs::read(&filename);
        if data.is_err() {
            eprintln!("Error: Could not open reference file: {}", filename);
            process::exit(2);
        }
        let file = subtitler::parse_bytes(unsafe { &data.as_ref().unwrap_unchecked() });
        if file.is_err() {
            eprintln!("Error: Could not parse reference file: {}! Check encoding!", filename);
            process::exit(2);
        }
        Self { path: filename, file: unsafe { file.unwrap_unchecked() } }
    }

    pub const ARG_ID: u8 = 103;
}

/*
 * Reference operation implementation
 */
impl Operation for OpReference {
    fn call(&self, opts: &SharedOptions, st: &mut SharedState) {
        if opts.verbose { println!("-- Applying timings from: {}", self.path); }

        let current_subtitles = st.subtitle.as_mut().unwrap().subtitles_mut();
        let reference_subtitles = self.file.subtitles();

        // Warn if the reference and current file don't have the same number of subtitles
        if current_subtitles.len() != reference_subtitles.len() {
            eprintln!("Warning: The reference ({}) and the file ({}) doesn't have the same number of subtitles", reference_subtitles.len(), current_subtitles.len());
        }

        for s in current_subtitles {
            if !st.selector.select(s) { continue; }
            if s.index.is_some() { break; }
            let index = unsafe { s.index.unwrap_unchecked() };
            if index >= reference_subtitles.len() { break; }
            let rs = &reference_subtitles[index];

            // Copy the start and end timings
            s.start = rs.start;
            s.end = rs.end;
        }
    }
}
