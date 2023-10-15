use std::fs;
use std::fs::File;

pub struct FileSystemHandler {
    pub home_directory_software: String
}

impl FileSystemHandler {
    pub(crate) fn new() -> Result<Self,String> {
        let home_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => return Err("Could not get home directory".to_string()),
        };

        let home_dir_str = match home_dir.to_str() {
            Some(dir_str) => dir_str.to_string(),
            None => return Err("Could not convert home directory to string".to_string()),
        };

        let home_directory_software = format!("{}/.InfoPanel", home_dir_str);

        if fs::metadata(&home_directory_software).is_ok() {
            Ok(FileSystemHandler { home_directory_software })
        } else {
            match create_directory(&home_directory_software) {
                Ok(_) => Ok(FileSystemHandler { home_directory_software }),
                Err(e) => Err(e.to_string()),
            }
        }
    }

    pub fn create_directory(&self, path: &str) -> Result<String, String> {
        let absolute_path = format!("{}/{}", self.home_directory_software, path);

        return match create_directory(&absolute_path) {
            Ok(_) => Ok(absolute_path),
            Err(e) => Err(e)
        }
    }

    pub fn create_file(&self, path: &str) -> Result<String, String> {
        let absolute_path = format!("{}/{}", self.home_directory_software, path);

        return match create_file(&absolute_path) {
            Ok(_) => Ok(absolute_path),
            Err(e) => Err(e)
        }
    }

    pub fn is_ok(&self, path: &str) -> bool {
        fs::metadata(&format!("{}/{}", self.home_directory_software, path)).is_ok()
    }
}

fn create_directory(path: &str) -> Result<(), String> {
    if fs::metadata(&path).is_ok() {
        return Ok(());
    }

    match fs::create_dir(&path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

fn create_file(path: &str) -> Result<(), String> {
    if fs::metadata(&path).is_ok() {
        return Ok(());
    }

    match File::create(format!("{}", path)) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}