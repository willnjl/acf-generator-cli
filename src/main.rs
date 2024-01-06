mod acf;
mod cli;
mod error;
mod utils;

use clap::Parser;
use error::ALGError;
use glob::glob;

fn main() {
    let args = cli::args::Args::parse();

    if let Err(e) = process_files(&args) {
        cli::output::exit_with_error(e);
    }
}

fn process_files(args: &cli::args::Args) -> Result<(), ALGError> {
    for entry in glob(&args.src).map_err(|e| ALGError::GlobError(e))? {
        match entry {
            Ok(pathbuf) => {
                if let Some(path) = pathbuf.to_str() {
                    match utils::deserializer::read_file(&path) {
                        Ok(json) => {
                            acf::field_group::process(&json, &args.dest, args.overwrite)?;
                        }
                        Err(e) => {
                            cli::output::exit_with_error(e);
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
