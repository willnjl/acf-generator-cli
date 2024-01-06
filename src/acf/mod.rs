pub mod field;
pub mod field_group;
pub mod layouts;
pub mod post_type;

use crate::acf;
use crate::cli;
use crate::error::ALGError;
use crate::utils;

use serde::Deserialize;

use glob::glob;

use self::field_group::FieldGroup;
use self::post_type::PostType;

pub fn generate(args: &cli::args::Args) -> Result<(), ALGError> {
    for entry in glob(&args.src).map_err(|e| ALGError::GlobError(e))? {
        match entry {
            Ok(pathbuf) => {
                if let Some(path) = pathbuf.to_str() {
                    let json = utils::deserializer::read_file(&path);
                    match_to_type(args, json)
                }
            }
            Err(_) => return Err(ALGError::FileNotFound(args.src.to_string())),
        };
    }
    Ok(())
}

fn match_to_type(args: &cli::args::Args, json: Result<AcfJsonKind, ALGError>) {
    return match json {
        Ok(json) => match json {
            AcfJsonKind::FieldGroup(field_group) => {
                acf::field_group::generate(&field_group, &args.dest, args.overwrite);
            }
            AcfJsonKind::PostType(post_type) => {
                cli::output::warn("post type file");
                acf::post_type::generate(&post_type, &args.dest, args.overwrite);
            }
        },
        Err(e) => {
            cli::output::exit_with_error(e);
            unreachable!()
        }
    };
}
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AcfJsonKind {
    FieldGroup(FieldGroup),
    PostType(PostType),
}
