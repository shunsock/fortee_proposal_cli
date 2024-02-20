use std::fs::File;
use std::io;
use std::io::Read;

pub fn read_html() -> io::Result<String> {
    let file_path = "data/html/output.html";
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
