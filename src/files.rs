/*
* File IO module
*
* Provides utilities for working with files within a designated root directory.
* The FileTree struct offers safe file access by constraining operations to a root path.
* 
* This "first" version is primitive, it reads the file at every request.
* It performs syscalls at every request which is not very efficient.
* If we want to trade memory for speed, we can store those buffers in memory
* and write them in the tcp connection at every request.
* 
* Tradeoff: must update the buffers when files are changed on the disk.
*/


use std::fs::File;
use std::io::{BufReader, Error, Read};
use std::path::{Path, PathBuf};

/// A file tree rooted at a specific directory path.
/// 
/// Provides safe file operations by ensuring all file access is relative to the root directory.
/// 
/// # Examples
/// 
/// ```
/// use std::path::PathBuf;
/// use file_shover::files::FileTree;
/// 
/// let tree = FileTree::new(PathBuf::from("test-sites"));
/// let reader = tree.get_reader("one-file/index.html")?;
/// Ok::<(), std::io::Error>(())
/// ```
pub struct FileTree {
    root: PathBuf,
}

impl FileTree {
    /// Creates a new FileTree with the specified root directory.
    /// 
    /// # Arguments
    /// 
    /// * `root` - The root directory path for this file tree
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::path::PathBuf;
    /// use file_shover::files::FileTree;
    /// 
    /// let tree = FileTree::new(PathBuf::from("/home/user/documents"));
    /// ```
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// Opens a file relative to the root directory and returns a buffered reader.
    /// 
    /// # Arguments
    /// 
    /// * `path` - The path to the file relative to the root directory
    /// 
    /// # Returns
    /// 
    /// Returns a `Result` containing a `BufReader<File>` on success, or an `Error` on failure.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::path::PathBuf;
    /// use file_shover::files::FileTree;
    /// 
    /// let tree = FileTree::new(PathBuf::from("."));
    /// match tree.get_reader("example.txt") {
    ///     Ok(reader) => { /* use reader */ },
    ///     Err(e) => eprintln!("Failed to open file: {}", e),
    /// }
    /// ```
    pub fn get_reader<P: AsRef<Path>>(&self, path: P) -> Result<BufReader<File>, Error> {
        let path_str = path.as_ref().to_str()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid UTF-8 in path"))?;
        
        // Strip leading "/" if present (HTTP paths start with /)
        let clean_path = path_str.strip_prefix('/').unwrap_or(path_str);
        
        // Security checks
        if clean_path == "." || clean_path == ".." {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Illegal path"));
        }

        if clean_path.is_empty() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Empty path"));
        }

        // Additional security: prevent path traversal
        if clean_path.contains("..") {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Path traversal not allowed"));
        }

        let file = File::open(self.root.join(clean_path))?;
        Ok(BufReader::new(file))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_works() {
        let tree = FileTree::new(PathBuf::from("."));
        let mut reader = tree
            .get_reader(Path::new("test-sites/one-file/index.html"))
            .expect("Failed to open test file");
        let mut buff = Vec::new();
        reader.read_to_end(&mut buff)
            .expect("Failed to read file content");
        assert_eq!(buff, "<h1>Hello World</h1>".as_bytes().to_vec())
    }

    #[test]
    fn test_file_not_found() {
        let tree = FileTree::new(PathBuf::from("."));
        let result = tree.get_reader("nonexistent-file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_root_directory_handling() {
        let tree = FileTree::new(PathBuf::from("test-sites"));
        let mut reader = tree
            .get_reader("one-file/index.html")
            .expect("Failed to open file with different root");
        let mut buff = Vec::new();
        reader.read_to_end(&mut buff)
            .expect("Failed to read content");
        assert_eq!(buff, "<h1>Hello World</h1>".as_bytes().to_vec())
    }

    #[test]
    fn test_illegal_path_dot() {
        let tree = FileTree::new(PathBuf::from("."));
        let result = tree.get_reader(".");
        assert!(result.is_err());
    }

    #[test]
    fn test_illegal_path_dotdot() {
        let tree = FileTree::new(PathBuf::from("."));
        let result = tree.get_reader("..");
        assert!(result.is_err());
    }
}
