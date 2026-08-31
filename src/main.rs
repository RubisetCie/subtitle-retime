use std::env;
use std::fs;
use std::io::Write;
use std::process;
use std::path::Path;
use std::path::PathBuf;

use subtitler::model::format::Format;
use subtitler::SubtitleFormat;

use crate::operations::op_shift::OpShift;
use crate::operations::op_speed::OpSpeed;
use crate::operations::op_rate::OpRate;
use crate::operations::op_reference::OpReference;
use crate::operations::op_expand::OpExpand;
use crate::operations::op_gap::OpGap;
use crate::operations::op_set_gap::OpSetGap;
use crate::operations::op_set_cps::OpSetCps;
use crate::operations::op_set_wpm::OpSetWpm;
use crate::operations::op_set_limit::OpSetLimit;
use crate::operations::op_validate::OpValidate;
use crate::operations::st_all::StAll;
use crate::operations::st_from::StFrom;
use crate::operations::st_to::StTo;
use crate::operations::st_between::StBetween;
use crate::options::{parse_type, parse_time_signed, parse_time_unsigned, parse_time_or_timestamp};
use crate::options::Options;
use crate::options::SharedOptions;
use crate::state::SharedState;

mod temp;
mod state;
mod options;
mod operations;

/*
 * Display the usage / help
 */
fn help(prog: &str) {
    let name: &str = env!("CARGO_PKG_NAME");
    let desc: &str = env!("CARGO_PKG_DESCRIPTION");
    eprint!("{name} - {desc}
Usage: {prog} <options/functions> [--] <file-1> [<file-2> [<...>]]

Options:
  -o, --output <o>: Specify output (filename when single, directory when multiple).
  -f, --format <f>: Specify out format (see supported formats below).
  -n, --dry-run   : Don't output anything, just display the operations.
  -i, --inplace   : Overwrite the input files.
  -h, --help      : Display usage help.
  -v, --verbose   : Increase verbosity of processing.

Selectors:
  -all, -whole    : Select all subtitles in the file (active by default).
  -from <f>       : Select all subtitles after a time point.
  -to   <t>       : Select all subtitles before a time point.
  -between <f> <t>: Select all subtitles between two time points.

Functions:
  -shift <s>      : Shift selected subtitles by a time amount (seconds).
  -speed <s>      : Change speed of selected subtitles (factor).
  -rate <a> <b>   : Change speed from one frames per seconds to another.
  -reference <f>  : Apply subtitle timings from a reference file.
  -contract <v>   : Contract subtitles by a time amount (seconds).
  -expand <v>     : Expand subtitles by a time amount (seconds).
  -gap <m>        : Enarge the gap between subtitles if lower than threshold (set-gap).
                    Multiple trimming modes are available:
                      start: trim the start of the subtitle.
                      end  : trim the end of the subtitle.
                      dual : trim both the start and the end.
                      smart: trim the subtitle that has the least of CPS.
  -delay <s>      : Synonym for \"-shift\".
  -set-gap <g>    : Set the minimum gap in seconds between subtitles.
  -set-cps <c>    : Set the maximum characters per second, for next validation.
  -set-wpm <w>    : Set the maximum words per minute, for next validation.
  -set-limit <m>  : Set the maximum number of characters in a subtitle, for next validation.
  -validate       : Check for timing issues.

Examples:
  {prog} -all -shift -0.25
  {prog} -from 0:00:30.0 -speed 0.96

Supported formats:
  SRT, VTT, ASS, SSA, MICRODVD, SUBVIEWER, TTML
  SBV, SCC, SAMI, LRC, MPL2, EBUSTL, DFXP, WHISPER
", name=name, desc=desc, prog=prog);
    process::exit(2);
}

/*
 * Base arguments identifiers
 */
const ARG_OUTPUT: u8 = 1;
const ARG_FORMAT: u8 = 2;

/*
 * Parse the arguments and separate them into two categories: operations and files on which the later will be executed onto
 */
fn parse_args(options: &mut Options) {
    // Whether the options have finished parsing
    let mut end: bool = false;

    // Retrieve the path of the executable as the first argument
    let mut prog: String = "".to_string();
    let mut output: String = "".to_string();

    // Allow to retrieve arguments for options that takes value(s)
    let mut argstack: Vec<String> = Vec::new();
    let mut argstackid: u8 = 0;

    for arg in env::args() {
        if prog.is_empty() {
            prog = arg;
            continue;
        }
        if end {
            options.files.push(Path::new(&arg).into());
            continue;
        }

        // Handle the arguments with options
        if argstackid != 0 {
            argstack.push(arg);
            match argstackid {
                // Output directory
                ARG_OUTPUT => {
                    if argstack.len() >= 1 {
                        output = unsafe { argstack.pop().unwrap_unchecked() };
                        argstackid = 0;
                        argstack.clear();
                    }
                },
                // Format output specifier
                ARG_FORMAT => {
                    if argstack.len() >= 1 {
                        let format = unsafe { argstack.pop().unwrap_unchecked() };
                        options.format = match format.to_uppercase().as_str() {
                            "SRT" => Some(Format::Srt),
                            "VTT" => Some(Format::Vtt),
                            "ASS" => Some(Format::Ass),
                            "SSA" => Some(Format::Ssa),
                            "MICRODVD" => Some(Format::MicroDvd),
                            "SUBVIEWER" => Some(Format::SubViewer),
                            "SBV" => Some(Format::Sbv),
                            "SCC" => Some(Format::Scc),
                            "SAMI" => Some(Format::Sami),
                            "TTML" => Some(Format::Ttml),
                            "LRC" => Some(Format::Lrc),
                            "MPL2" => Some(Format::Mpl2),
                            "EBUSTL" => Some(Format::EbuStl),
                            "DFXP" => Some(Format::Dfxp),
                            "WHISPER" => Some(Format::Whisper),
                            _ => None
                        };
                        if options.format.is_none() {
                            eprintln!("Warning: Unrecognized or unsupported format for option: {}", format);
                        }
                        argstackid = 0;
                        argstack.clear();
                    }
                },
                // Shift operation
                OpShift::ARG_ID => {
                    if argstack.len() >= 1 {
                        options.operations.push(Box::new(OpShift::new(
                            unsafe { parse_time_signed(&argstack.pop().unwrap_unchecked()) }
                        )));
                        argstackid = 0;
                        argstack.clear();
                    }
                },
                // Speed operation
                OpSpeed::ARG_ID => {
                    if argstack.len() >= 1 {
                        options.operations.push(Box::new(OpSpeed::new(
                            unsafe { parse_type(&argstack.pop().unwrap_unchecked()) }
                        )));
                        argstackid = 0;
                        argstack.clear();
                    }
                },
                // Rate operation
                OpRate::ARG_ID => {
                    if argstack.len() >= 2 {
                        options.operations.push(Box::new(OpRate::new(
                            unsafe { parse_type(&argstack.pop().unwrap_unchecked()) },
                            unsafe { parse_type(&argstack.pop().unwrap_unchecked()) }
                        )));
                        argstackid = 0;
                        argstack.clear();
                    }
                },
                // Reference operation
                OpReference::ARG_ID => {
                    if argstack.len() >= 1 {
                        options.operations.push(Box::new(OpReference::new(
                            unsafe { argstack.pop().unwrap_unchecked() },
                        )));
                        argstackid = 0;
                        argstack.clear();
                    }
                },
                // Contract operation
                OpExpand::ARG_ID_CONTRACT => {
                    if argstack.len() >= 1 {
                        options.operations.push(Box::new(OpExpand::new(
                            unsafe { -parse_time_signed(&argstack.pop().unwrap_unchecked()) },
                        )));
                        argstackid = 0;
                        argstack.clear();
                    }
                },
                // Expand operation
                OpExpand::ARG_ID_EXPAND => {
                    if argstack.len() >= 1 {
                        options.operations.push(Box::new(OpExpand::new(
                            unsafe { parse_time_signed(&argstack.pop().unwrap_unchecked()) },
                        )));
                        argstackid = 0;
                        argstack.clear();
                    }
                },
                // Gap operation
                OpGap::ARG_ID => {
                    if argstack.len() >= 1 {
                        options.operations.push(Box::new(OpGap::new(
                            unsafe { argstack.pop().unwrap_unchecked() },
                        )));
                        argstackid = 0;
                        argstack.clear();
                    }
                },
                // Set minimum gap operation
                OpSetGap::ARG_ID => {
                    if argstack.len() >= 1 {
                        options.operations.push(Box::new(OpSetGap::new(
                            unsafe { parse_time_unsigned(&argstack.pop().unwrap_unchecked()) },
                        )));
                        argstackid = 0;
                        argstack.clear();
                    }
                },
                // Set maximum characters per second operation
                OpSetCps::ARG_ID => {
                    if argstack.len() >= 1 {
                        options.operations.push(Box::new(OpSetCps::new(
                            unsafe { parse_type(&argstack.pop().unwrap_unchecked()) },
                        )));
                        argstackid = 0;
                        argstack.clear();
                    }
                },
                // Set maximum words per minute operation
                OpSetWpm::ARG_ID => {
                    if argstack.len() >= 1 {
                        options.operations.push(Box::new(OpSetWpm::new(
                            unsafe { parse_type(&argstack.pop().unwrap_unchecked()) },
                        )));
                        argstackid = 0;
                        argstack.clear();
                    }
                },
                // Set maximum number of characters operation
                OpSetLimit::ARG_ID => {
                    if argstack.len() >= 1 {
                        options.operations.push(Box::new(OpSetLimit::new(
                            unsafe { parse_type(&argstack.pop().unwrap_unchecked()) },
                        )));
                        argstackid = 0;
                        argstack.clear();
                    }
                },
                // From selector
                StFrom::ARG_ID => {
                    if argstack.len() >= 1 {
                        options.operations.push(Box::new(StFrom::new(
                            unsafe { parse_time_or_timestamp(&argstack.pop().unwrap_unchecked()) }
                        )));
                        argstackid = 0;
                        argstack.clear();
                    }
                },
                // To selector
                StTo::ARG_ID => {
                    if argstack.len() >= 1 {
                        options.operations.push(Box::new(StTo::new(
                            unsafe { parse_time_or_timestamp(&argstack.pop().unwrap_unchecked()) }
                        )));
                        argstackid = 0;
                        argstack.clear();
                    }
                },
                // Between selector
                StBetween::ARG_ID => {
                    if argstack.len() >= 2 {
                        options.operations.push(Box::new(StBetween::new(
                            unsafe { parse_time_or_timestamp(&argstack.pop().unwrap_unchecked()) },
                            unsafe { parse_time_or_timestamp(&argstack.pop().unwrap_unchecked()) }
                        )));
                        argstackid = 0;
                        argstack.clear();
                    }
                }
                _ => { argstackid = 0; }
            }
            continue;
        }

        // Check if it's an option (starting with dash)
        if arg.starts_with('-') {
            match arg.as_str() {
                // Base options
                "-?" | "-h" | "--help" => help(&prog),
                "-v" | "--verbose"     => options.verbose = true,
                "-i" | "--inplace"     => options.inplace = true,
                "-n" | "--dry-run"     => options.dry = true,
                "-f" | "--format"      => argstackid = ARG_FORMAT,
                "-o" | "--output"      => argstackid = ARG_OUTPUT,
                // Selectors
                "-all" | "-whole"      => options.operations.push(Box::new(StAll::new())),
                "-from"                => argstackid = StFrom::ARG_ID,
                "-to"                  => argstackid = StTo::ARG_ID,
                "-between"             => argstackid = StBetween::ARG_ID,
                // Functions
                "-shift" | "-delay"    => argstackid = OpShift::ARG_ID,
                "-speed"               => argstackid = OpSpeed::ARG_ID,
                "-rate"                => argstackid = OpRate::ARG_ID,
                "-reference"           => argstackid = OpReference::ARG_ID,
                "-contract"            => argstackid = OpExpand::ARG_ID_CONTRACT,
                "-expand"              => argstackid = OpExpand::ARG_ID_EXPAND,
                "-gap"                 => argstackid = OpGap::ARG_ID,
                "-set-gap"             => argstackid = OpSetGap::ARG_ID,
                "-set-cps"             => argstackid = OpSetCps::ARG_ID,
                "-set-wpm"             => argstackid = OpSetWpm::ARG_ID,
                "-set-limit"           => argstackid = OpSetLimit::ARG_ID,
                "-validate"            => options.operations.push(Box::new(OpValidate::new())),
                "--" => { end = true; continue; },
                _ => {}
            }
        // Otherwise, assume it's a file
        } else {
            options.files.push(Path::new(&arg).into())
        }
    }

    // If no file nor operations has been supplied, display help
    if argstackid != 0 {
        eprintln!("Error: Unexpected ending in a middle of argument parsing!");
        help(&prog);
    }
    if options.files.is_empty() {
        eprintln!("Error: No files have been supplied!");
        help(&prog);
    }

    // Deal with the output option
    if !output.is_empty() && !options.dry {
        options.output = Path::new(&output).into();
    }
}

#[tokio::main(flavor = "local")]
async fn main() {
    let mut options = Options {
        files: Vec::new(),
        operations: Vec::new(),
        output: Path::new(".").into(),
        format: None,
        dry: false,
        inplace: false,
        verbose: false
    };

    // Parse the arguments
    parse_args(&mut options);

    let shared_options = SharedOptions {
        verbose: options.verbose
    };
    let mut shared_state = SharedState::new();

    let dir: bool = options.output.is_dir() || options.dry;

    if options.files.len() > 1 && !dir {
        eprintln!("Error: Output must be a directory when multiple inputs!");
        process::exit(3);
    }

    // Initialize the temporary file used for in-place modifications
    let mut temp_path;
    if options.inplace && !options.dry {
        temp_path = env::temp_dir();
        temp_path.push("subtitle-toolbox-temporary");
    } else {
        temp_path = PathBuf::new();
    }

    // Process each file
    for filename in options.files.iter() {
        // Compute the destination
        let mut output = options.output.clone();
        if !options.inplace && !options.dry {
            if dir {
                // Compute the extension of the new file
                let current_ext = filename.extension().unwrap_or_default().to_str().unwrap();
                let ext = match options.format {
                    Some(Format::Srt) => "srt",
                    Some(Format::Vtt) => "vtt",
                    Some(Format::Ass) => "ass",
                    Some(Format::Ssa) => "ssa",
                    Some(Format::MicroDvd) => "sub",
                    Some(Format::SubViewer) => "sub",
                    Some(Format::Sbv) => "sbv",
                    Some(Format::Scc) => "scc",
                    Some(Format::Sami) => "smi",
                    Some(Format::Ttml) => "ttml",
                    Some(Format::Lrc) => "lrc",
                    Some(Format::Mpl2) => "mpl",
                    Some(Format::EbuStl) => "stl",
                    Some(Format::Dfxp) => "dfxp",
                    Some(Format::Whisper) => "whisper",
                    _ => current_ext,
                };
                // Check if the output directory is the same as the file's
                if filename.parent().is_some() && unsafe { options.output != filename.parent().unwrap_unchecked().into() } {
                    if current_ext.to_lowercase() == ext {
                        output = options.output.join(filename.file_name().unwrap_or_default()).into();
                    } else {
                        let mut path: String = "".to_string();
                        path.push_str(filename.file_stem().unwrap_or_default().to_str().unwrap());
                        path.push_str(".");
                        path.push_str(ext);
                        output = options.output.join(path).into();
                    }
                } else {
                    let mut path: String = "".to_string();
                    path.push_str(filename.file_stem().unwrap_or_default().to_str().unwrap());
                    if current_ext.to_lowercase() == ext { path.push_str("-modified"); }
                    path.push_str(".");
                    path.push_str(ext);
                    output = options.output.join(path).into();
                }
            }
        }
        if options.verbose { println!("- Processing file... {}", filename.display()); }

        // Load the file
        let data = fs::read(filename);
        if data.is_err() {
            eprintln!("Error: Could not open file: {}", filename.display());
            continue;
        }
        let file = subtitler::parse_bytes(unsafe { &data.as_ref().unwrap_unchecked() });
        if file.is_err() {
            eprintln!("Error: Could not parse file: {}! Check encoding!", filename.display());
            continue;
        }

        // Reset the shared state
        shared_state.reset(unsafe { file.unwrap_unchecked() });

        for op in options.operations.iter() {
            op.call(&shared_options, &mut shared_state);
        }

        if !options.dry {
            // Convert to a specific format
            let output_str;
            if options.format.is_some() {
                let format = unsafe { options.format.unwrap_unchecked() };
                output_str = shared_state.subtitle.as_ref().unwrap().to_string_with_format(&format);
            } else {
                output_str = shared_state.subtitle.as_ref().unwrap().to_string();
            }

            // Write the output file
            if !options.inplace {
                if fs::write(&output, output_str).is_ok() {
                    if options.verbose { println!("- File processing finished: {}", output.display()); }
                } else {
                    eprintln!("Error: could not write the file: {}", output.display());
                }
            } else {
                // Write to a temporary file before overwriting the input
                let file_res = temp::open(&temp_path);
                if file_res.is_err() {
                    eprintln!("Error: could not write the temporary file!");
                    continue;
                }
                let mut file = unsafe { file_res.unwrap_unchecked() };
                if file.write_all(output_str.as_bytes()).is_err() {
                    eprintln!("Error: could not write the temporary file!");
                    drop(file);
                    continue;
                }
                drop(file); // Close the file
                if fs::copy(&temp_path, filename).is_ok() {
                    if options.verbose { println!("- File processing finished: {}", filename.display()); }
                } else {
                    eprintln!("Error: could not write the temporary file!");
                }
            }
        } else if options.verbose {
            println!("- Dry-run finished: {}", filename.display());
        }
    }
}
