use std::fs;

pub struct FileSystemHandler {
    pub home_directory_software: String
}

impl FileSystemHandler {
    pub(crate) fn new() -> Result<Self,String> {
        let option_home_dir = dirs::home_dir();

        return match option_home_dir {
            Some(home_dir_buf) => {
                return match home_dir_buf.to_str() {
                    Some(home_dir) => {
                        let home_directory_software = format!("{}/.InfoPanel", home_dir);
                        match create_directory(&home_directory_software) {
                            Ok(_) => Ok(FileSystemHandler { home_directory_software }),
                            Err(e) => Err(e)
                        }
                    },
                    None => Err(String::from("Could not get home directory"))
                }
            },
            None => Err(String::from("Could not get home directory"))
        }
    }

    pub fn create_directory(&self, path: &str) -> Result<String, String> {
        let absolute_path = format!("{}/{}", self.home_directory_software, path);

        return match create_directory(&absolute_path) {
            Ok(_) => Ok(absolute_path),
            Err(e) => Err(e)
        }
    }
}

fn create_directory(path: &str) -> Result<(), String> {
    if fs::metadata(&path).is_ok() {
        return Ok(());
    }

    match fs::create_dir(&path) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{}", e.to_string());
            Err(e.to_string())},
    }
}