pub mod cli_output {

    use colored::Colorize;
    use std::fmt;
    use std::io::{self, Write};

    use crate::error::ALGError;

    #[derive(Debug)]
    pub enum CustomError {
        WriteError(io::Error),
    }

    // Implement the Display trait for CustomError
    impl fmt::Display for CustomError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                CustomError::WriteError(e) => {
                    write!(f, "Error writing to the command line: {}", e)
                }
            }
        }
    }

    // Function to write a message to the command line
    pub fn write_to_cli(message: String) -> Result<(), CustomError> {
        match writeln!(io::stdout(), "{}", message) {
            Ok(_) => Ok(()),
            Err(e) => Err(CustomError::WriteError(e)),
        }
    }

    pub fn exit_with_error(e: ALGError) {
        error(&format!("{}", e));
        std::process::exit(1);
    }

    pub fn warn(msg: &str) {
        write_to_cli(format!("{} {}", "[WARN]".yellow(), msg));
    }

    pub fn error(msg: &str) {
        write_to_cli(format!("{} {}", "[ERROR]".red(), msg));
    }

    pub fn info(msg: &str) {
        write_to_cli(format!("{} {}", "[INFO]".blue(), msg));
    }

    pub fn create(msg: &str) {
        write_to_cli(format!("{} {}", "[CREATE]".green(), msg));
    }
}
