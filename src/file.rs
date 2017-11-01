
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
    pub fn with_path( mut self, p: PathBuf) -> PictureFile{
        self.path = Some(p);
        self
    }

    /// Add dimensions to picture file
    pub fn with_dimensions(mut self, w:u32, h:u32) ->  PictureFile{
        self.width = Some(w);
        self.height = Some(h);
        self
    }

    /// Add title to picture
    pub fn with_title( mut self, t: &str) -> PictureFile{
        self.title = Some(String::from(t));
        self
    }

    /// Return a string of the dimension of the file
    /// If dimension are not defined, return "default"
    //
    // This is to be used to copy file in appropriate folder
    // depending on their dimensions
    pub fn get_dimension_string(&self) -> String{

        match (self.width, self.height) {
            (Some(w), Some(h)) => format!("{}x{}",w, h),
            _ => String::from("default"),
        }
        
    }

}

#[cfg(test)]
mod tests {

    use ::file::{PictureFile};

    #[test]
    fn test_dimension_string() {
        let pic1 = PictureFile::new();
        assert_eq!(pic1.get_dimension_string(), "default" );

        let pic2 = PictureFile::new().with_dimensions(1280,1024);
        assert_eq!(pic2.get_dimension_string(),"1280x1024");
        assert_ne!(pic2.get_dimension_string(),"128x102");

        let pic3 = PictureFile::new().with_dimensions(640,480);
        assert_eq!(pic3.get_dimension_string(),"640x480");
    }
}
    