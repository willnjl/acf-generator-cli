use crate::cli;
use crate::error::ALGError;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::acf::AcfJsonKind;

/**
 * open src file and turn it into a string
 */
pub fn read_file(path: &str) -> Result<AcfJsonKind, ALGError> {
    cli::output::info("Opening file...");

    let mut f = match File::open(Path::new(path)) {
        Ok(file) => file,
        Err(_) => return Err(ALGError::FileNotFound(path.to_string())),
    };

    let mut s = String::new();

    if let Err(err) = f.read_to_string(&mut s) {
        return Err(ALGError::IoError(err));
    }

    let data: Result<AcfJsonKind, ALGError> =
        serde_json::from_str(&s).map_err(ALGError::InvalidJson);

    data
}
