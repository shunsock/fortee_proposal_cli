use std::io;
use std::path::{Path, PathBuf};
use std::{env, fs};

struct FileCopier {
    src: PathBuf,
    dest: PathBuf,
}

impl FileCopier {
    /// Create a new FileCopier instance
    /// # Arguments
    /// src: Source file path
    /// Returns a Result containing the FileCopier instance or an error
    /// # Errors
    /// Returns an error if the source file does not exist
    /// # Example
    /// ```
    /// let copier = FileCopier::new("path/to/your/source/image.png");
    /// ```
    /// # Note
    /// The destination path is set to the current directory
    /// with the same filename as the source file
    /// src: "path/to/your/source/image.png"
    /// dest: "path/to/current_dir/image.png"
    fn new(src: &str) -> io::Result<Self> {
        let src_path: PathBuf = PathBuf::from(src);
        if !src_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Source file does not exist",
            ));
        }

        // Get the filename from the source path
        let filename = match src_path.file_name() {
            Some(name) => name,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid source path",
                ))
            }
        };

        // Construct the destination path
        let mut dest_path: PathBuf = env::current_dir()?;
        dest_path.push(filename);

        Ok(FileCopier {
            src: src_path,
            dest: dest_path,
        })
    }

    /// Copy the source file to the destination path
    /// Returns a Result containing the destination path or an error
    /// # Example
    /// ```
    /// let copier = FileCopier::new("path/to/your/source/image.png");
    /// copier.copy();
    /// ```
    /// # Note
    /// The destination path is set to the current directory
    /// with the same filename as the source file
    /// # Example
    /// src: "path/to/your/source/image.png"
    /// dest: "path/to/current_dir/image.png"
    fn copy(&self) -> io::Result<()> {
        fs::copy(&self.src, &self.dest)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_copier() {
        let copier = FileCopier::new("test_data/infrastructure/copier/logo.webp");
        assert!(copier.is_ok());

        let copier = copier.unwrap();
        let result = copier.copy();

        assert!(result.is_ok());
        assert!(Path::new(&copier.dest).exists());

        // Clean up the copied file
        fs::remove_file(&copier.dest).unwrap();
    }
}
