use std::str::FromStr;
use std::process;
use std::path::Path;

use crate::operations::Operation;

/*
 * The structure containing the options passed as arguments
 */
pub struct Options {
    // The list of files to be processed
    pub files: Vec<Box<Path>>,

    // Operations to be executed in order
    pub operations: Vec<Box<dyn Operation>>,

    pub output: Box<Path>,
    pub format: Option<subtitler::model::format::Format>,
    pub dry: bool,
    pub inplace: bool,
    pub verbose: bool
}

/*
 * The structure containing the options passed to the operations
 */
pub struct SharedOptions {
    pub verbose: bool
}

/*
 * Argument parsing utilities
 */
pub fn parse_type<T: FromStr>(string: &str) -> T {
    let res = string.parse::<T>();
    if res.is_err() {
        eprintln!("Error: {} is not a parseable argument!", string);
        process::exit(2);
    }
    unsafe { res.unwrap_unchecked() }
}

pub fn parse_time_signed(string: &str) -> i64 {
    let res = string.parse::<f32>();
    if res.is_err() {
        eprintln!("Error: {} is not a parseable argument!", string);
        process::exit(2);
    }
    return unsafe { (res.unwrap_unchecked() * 100.0) as i64 }; // Convert to milliseconds
}

pub fn parse_time_unsigned(string: &str) -> u64 {
    let res = string.parse::<f32>();
    if res.is_err() {
        eprintln!("Error: {} is not a parseable argument!", string);
        process::exit(2);
    }
    return unsafe { (res.unwrap_unchecked() * 100.0) as u64 }; // Convert to milliseconds
}

pub fn parse_time_or_timestamp(string: &str) -> u64 {
    let res = string.parse::<f32>();
    if res.is_ok() {
        return unsafe { (res.unwrap_unchecked() * 100.0) as u64 }; // Convert to milliseconds
    }
    let res = subtitler::utils::parse_timestamp(string, subtitler::model::Format::Vtt);
    if res.is_ok() {
        return unsafe { res.unwrap_unchecked() };
    }
    eprintln!("Error: {} is not a parseable argument!", string);
    process::exit(2);
}
