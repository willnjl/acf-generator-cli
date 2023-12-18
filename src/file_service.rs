use crate::cli_output;
use crate::error::ALGError;
use colored::Colorize;
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
            writeln!(file, "{}", content).expect("Write Fail");
        }
    }

    fn open(path: &str, ow: bool) -> Result<FileService, ALGError> {
        if ow {
            return match OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&path)
            {
                Ok(file) => Ok(FileService { file: Some(file) }),
                Err(e) => Err(ALGError::IoError(e)),
            };
        };

        return match OpenOptions::new().create_new(true).write(true).open(path) {
            Ok(file) => {
                cli_output::file_created_feedback(&format!("{}", path.yellow(),));
                Ok(FileService { file: Some(file) })
            }
            Err(e) => Err(ALGError::IoError(e)),
        };
    }
}
