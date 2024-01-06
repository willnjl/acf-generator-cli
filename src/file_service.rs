use crate::cli_output::cli_output;
use crate::error::ALGError;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;

pub struct FileService {
    file: Option<File>,
}

impl FileService {
    pub fn new(path: &str, overwrite: bool) -> Result<FileService, ALGError> {
        let _ = create_dir_all(std::path::Path::new(path).parent().unwrap()); // Create the directory structure if it doesn't exist
        FileService::open(path, overwrite)
    }

    pub fn write(&mut self, content: &str) {
        if let Some(file) = &mut self.file {
            // writeln!(file, "{}", content).expect("Write Fail");
        }
    }

    fn open(path: &str, ow: bool) -> Result<FileService, ALGError> {
        if ow {
            // overwite
            return match OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&path)
            {
                Ok(file) => {
                    cli_output::create(&format!("layout {} created!", path));
                    Ok(FileService { file: Some(file) })
                }
                Err(e) => Err(ALGError::IoError(e)),
            };
        };

        // skip existing files
        return match OpenOptions::new().create_new(true).write(true).open(path) {
            Ok(file) => {
                cli_output::create(&format!("layout {}", path));
                Ok(FileService { file: Some(file) })
            }
            Err(_) => {
                cli_output::info(&ALGError::FileAlreadyExists(path.to_string()).to_string());
                Ok(FileService { file: None })
            }
        };
    }
}
