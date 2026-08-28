use std::fs;
use std::io;
use std::path::PathBuf;

/*
 * Create a temporary file and opens it
 */
pub fn open(filename: &PathBuf) -> Result<fs::File, io::Error> {
    let mut temp_builder = fs::OpenOptions::new();
    temp_builder.write(true).create_new(true);

    #[cfg(unix)]
    temp_builder.mode(0o644);

    return temp_builder.open(filename);
}
