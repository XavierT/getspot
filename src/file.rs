
use std::path::{Path, PathBuf};
use std::string::String;

#[derive(Debug)]
pub struct PictureFile {
    path: Option<PathBuf>,
    title: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
}

impl PictureFile {
    pub fn new() -> PictureFile {
        PictureFile {
            path: None,
            title: None,
            width: None,
            height: None,
        }
    }
    // add code here

    
}
