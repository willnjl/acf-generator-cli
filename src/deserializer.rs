use crate::acf::{Field, FieldGroup};
use crate::cli_output::{self};
use colored::Colorize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/**
 * open src file and turn it into a string
 */
pub fn read_file(path: &str) -> FieldGroup {
    cli_output::running_task_feedback("Opening file...");
    let file_result = File::open(Path::new(path));

    let mut file = match file_result {
        Ok(file) => file,
        Err(_) => {
            cli_output::exit_with_error(&format!("Unable to open file '{}'", path.yellow()));
            unreachable!();
        }
    };

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let data: FieldGroup = serde_json::from_str(&buffer).unwrap();

    data
}
