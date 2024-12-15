use std::{ error, fs, io::{ self, Write }, path::{ Path, PathBuf } };

use rust_commons::rand::{ self, Rng };

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
    ) -> Result<String, Box<dyn error::Error + Send + Sync + 'static>> {
        let new_filename = self.image_name_generator(filename)?;
        let full_path = Path::new(&self.loc).join(new_filename.as_str());
        ImageStorageService::write_file_to_storage(full_path, content)?;
        return Ok(new_filename);
    }

    pub fn remove_file_image(
        &self,
        filename: &str
    ) -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
        let full_path = Path::new(&self.loc).join(filename);
        fs::remove_file(&full_path)?;
        return Ok(());
    }

    fn image_name_generator(
        &self,
        filename: &str
    ) -> Result<String, Box<dyn error::Error + Send + Sync + 'static>> {
        let full_path = Path::new(&self.loc).join(filename);
        if full_path.exists() {
            let num: u64 = rand::thread_rng().gen();
            let num_str: String = num.to_string();
            let parts: Vec<&str> = filename.split('.').collect();
            if parts.len() != 2 {
                return Err(Box::from("Uploaded file has not correct filename"));
            }
            let new_file_name = format!("{}_{}.{}", parts[0], num_str, parts[1]);
            return self.image_name_generator(&new_file_name);
        }
        return Ok(filename.to_owned());
    }

    fn write_file_to_storage(location: PathBuf, content: &[u8]) -> io::Result<()> {
        fs::create_dir_all(&location.parent().unwrap())?;
        let mut file = fs::File::create(&location)?;
        file.write_all(content)?;
        Ok(())
    }
}
