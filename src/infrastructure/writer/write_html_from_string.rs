use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

pub fn write_html_from_string(content: &str, file_save_path: PathBuf) -> Result<(), io::Error> {
    let mut file = File::create(file_save_path)?;
    write!(file, "{}", content)?;
    Ok(())
}
