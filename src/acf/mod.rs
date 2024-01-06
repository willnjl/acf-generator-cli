pub mod field;
pub mod field_group;
pub mod layouts;
pub mod post_type;

use crate::acf;
use crate::cli;
use crate::error::ALGError;
use crate::utils;

use glob::glob;

pub fn process(args: &cli::args::Args) -> Result<(), ALGError> {
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
