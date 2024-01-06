mod acf;
mod cli_args;
mod cli_output;
mod error;
mod utils;

use clap::Parser;
use cli_output::cli_output::exit_with_error;
use error::ALGError;
use glob::glob;

fn main() {
    let args = cli_args::Args::parse();

    if let Err(e) = process_files(&args) {
        exit_with_error(e);
    }
}

fn process_files(args: &cli_args::Args) -> Result<(), ALGError> {
    for entry in glob(&args.src).map_err(|e| ALGError::GlobError(e))? {
        match entry {
            Ok(pathbuf) => {
                if let Some(path) = pathbuf.to_str() {
                    match utils::deserializer::read_file(&path) {
                        Ok(json) => {
                            acf::process_group(&json, &args.dest, args.overwrite)?;
                        }
                        Err(e) => {
                            exit_with_error(e);
                            unreachable!()
                        }
                    };
                }
            }
            Err(_) => return Err(ALGError::FileNotFound(args.src.to_string())),
        };
    }

    Ok(())
}
