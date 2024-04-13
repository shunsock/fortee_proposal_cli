use std::path::PathBuf;

pub fn read_file_as_string(file_path: PathBuf) -> Result<String, String> {
    let res = std::fs::read_to_string(file_path);
    match res {
        Ok(html) => Ok(html),
        Err(e) => Err(format!("Error: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    #[test]
    fn test_success_for_read_file_as_string() {
        let current_dir: PathBuf = env::current_dir().expect("HOME directory not found");
        
        let file_path = current_dir
            .join("test_data")
            .join("infrastructure")
            .join("reader")
            .join("test.txt");
        
        let res: Result<String, String> = read_file_as_string(file_path);
        
        assert_eq!(res, Ok("test".to_string()));
    }

    #[test]
    fn test_failed_for_read_file_as_string() {
        let file_path: PathBuf = PathBuf::from("not_found.txt");
        let res: Result<String, String> = read_file_as_string(file_path);
        assert_eq!(res, Err("Error: No such file or directory (os error 2)".to_string()));
    }
}