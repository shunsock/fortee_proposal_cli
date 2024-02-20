use std::fs::File;
use std::io;
use std::io::Write;

pub fn write_html_from_string(content: &str) -> Result<(), io::Error> {
    let mut file = File::create("data/html/output.html")?;
    write!(file, "{}", content)?;
    Ok(())
}
