use std::{ error, fs, io::{ self, Write }, path::{ Path, PathBuf } };

#[derive(Clone)]
pub struct ImageStorageService {
    loc: String,
}

impl ImageStorageService {
    pub fn new(location: &str) -> Self {
        return ImageStorageService { loc: location.to_owned() };
    }

    pub fn save_image(
        &self,
        filename: &str,
        content: &[u8]
    ) -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
        let full_path = Path::new(&self.loc).join(filename);
        ImageStorageService::write_file_to_storage(full_path, content)?;
        return Ok(());
    }

    pub fn remove_file_image(
        &self,
        filename: &str
    ) -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
        let full_path = Path::new(&self.loc).join(filename);
        fs::remove_file(&full_path)?;
        return Ok(());
    }

    fn write_file_to_storage(location: PathBuf, content: &[u8]) -> io::Result<()> {
        fs::create_dir_all(&location.parent().unwrap())?;
        let mut file = fs::File::create(&location)?;
        file.write_all(content)?;
        Ok(())
    }
}
