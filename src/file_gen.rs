use crate::cli_output;
use colored::Colorize;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{self, Write};

pub struct FileGen {
    file: Option<File>,
}

impl FileGen {
    pub fn new(path: &str, overwrite: bool) -> FileGen {
        let _ = create_dir_all(std::path::Path::new(path).parent().unwrap()); // Create the directory structure if it doesn't exist

        if overwrite {
            return match OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&path)
            {
                Ok(file) => {
                    cli_output::file_created_feedback(&format!("{}", path.yellow(),));
                    FileGen { file: Some(file) }
                }
                Err(_) => {
                    cli_output::exit_with_error(&format!(
                        "{} {}",
                        "Error creating ",
                        &path.yellow()
                    ));
                    FileGen { file: None }
                }
            };
        };

        return match OpenOptions::new().create_new(true).write(true).open(path) {
            Ok(file) => {
                cli_output::file_created_feedback(&format!("{}", path.yellow(),));
                FileGen { file: Some(file) }
            }
            Err(e) => {
                Self::file_creation_error_handler(&path, e);
                FileGen { file: None }
            }
        };
    }
    pub fn write_to_file(&mut self, content: &str) {
        if let Some(file) = &mut self.file {
            writeln!(file, "{}", content).expect("Write Fail");
        }
    }

    fn file_creation_error_handler(path: &str, e: io::Error) {
        match e.kind() {
            io::ErrorKind::AlreadyExists => {
                cli_output::file_exists_feedback(&format!("{} ...", path.yellow()));
            }
            _ => {}
        };
    }
}
