
use std::path::PathBuf;
use std::string::String;

#[derive(Debug, Clone)]
pub struct PictureFile {
    pub path: Option<PathBuf>,
    title: Option<String>,
    width: Option<u16>,
    height: Option<u16>,
}

impl PictureFile {
    /// Default empty picture file
    pub fn new() -> PictureFile {
        PictureFile {
            path: None,
            title: None,
            width: None,
            height: None,
        }
    }

    /// Add source path to picture
    pub fn with_path(mut self, p: PathBuf) -> PictureFile {
        self.path = Some(p);
        self
    }

    /// Add dimensions to picture file
    pub fn with_dimensions(mut self, w: u16, h: u16) -> PictureFile {
        self.width = Some(w);
        self.height = Some(h);
        self
    }

    /// Add title to picture
    pub fn with_title(mut self, t: &str) -> PictureFile {
        self.title = Some(String::from(t));
        self
    }

    /// Return a string of the dimension of the file
    /// If dimension are not defined, return "default"
    //
    // This is to be used to copy file in appropriate folder
    // depending on their dimensions
    pub fn get_dimension_string(&self) -> String {

        match (self.width, self.height) {
            (Some(w), Some(h)) => format!("{}x{}", w, h),
            _ => String::from("default"),
        }

    }

    /// Return a truncated name from the initial path
    ///
    /// Truncate the name to 10 characters
    /// Append ".jpg" at the end
    pub fn get_name(&self) -> Option<String> {
        if let Some(ref p) = self.path {
            let name = p.file_name().unwrap().to_os_string().into_string();
            if name.is_ok() {
                let mut name_string = name.ok().unwrap();
                name_string.truncate(10);
                name_string.push_str(".jpg");
                return Some(name_string);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {

    use crate::file::PictureFile;

    #[test]
    fn test_dimension_string() {
        let pic1 = PictureFile::new();
        assert_eq!(pic1.get_dimension_string(), "default");

        let pic2 = PictureFile::new().with_dimensions(1280, 1024);
        assert_eq!(pic2.get_dimension_string(), "1280x1024");
        assert_ne!(pic2.get_dimension_string(), "128x102");

        let pic3 = PictureFile::new().with_dimensions(640, 480);
        assert_eq!(pic3.get_dimension_string(), "640x480");
    }
}
