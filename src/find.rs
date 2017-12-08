
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

/// copy source file to target directory
// TODO should probably return something..
pub fn copy_to_dir(source_file: &Path, target_dir: &Path, target_name: &str) {

    if source_file.is_file() {
        if target_dir.is_dir() {
            fs::copy(source_file, target_dir.join(target_name)).expect("source file not written");
        } else {
            // !("Creating directory {:?}", target_dir);
            fs::create_dir_all(target_dir).expect("dir not created");
            fs::copy(source_file, target_dir.join(target_name)).expect("source file not written");
        }
    } else {
        error!(" source file {} does not exist", source_file.display());
    }
}
