
use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use std::vec::Vec;


/// Find all files recursively in a directory
/// return a vec of Path
pub fn find_in_directory(dir: &Path, vec: &mut Vec<PathBuf>) -> io::Result<Vec<PathBuf>> {

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                find_in_directory(&path, vec)?;
            } else {
                vec.push(path);
            }
        }
    }
    Ok(vec.to_vec())
}
