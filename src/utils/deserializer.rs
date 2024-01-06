use crate::acf::field_group::FieldGroup;
use crate::cli;
use crate::error::ALGError;

use std::fs::File;
use std::io::Read;
use std::path::Path;

/**
 * open src file and turn it into a string
 */
pub fn read_file(path: &str) -> Result<FieldGroup, ALGError> {
    cli::output::info("Opening file...");

    let mut f = match File::open(Path::new(path)) {
        Ok(file) => file,
        Err(_) => return Err(ALGError::FileNotFound(path.to_string())),
    };

    let mut s = String::new();

    if let Err(err) = f.read_to_string(&mut s) {
        return Err(ALGError::IoError(err));
    }

    let data: Result<FieldGroup, ALGError> =
        serde_json::from_str(&s).map_err(ALGError::InvalidJson);

    data
}
