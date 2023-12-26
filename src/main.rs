mod acf;
mod cli_args;
mod cli_output;
mod deserializer;
mod error;
mod file_service;
mod template_gen;

use clap::Parser;
use cli_output::cli_output::exit_with_error;

fn main() {
    let args = cli_args::Args::parse();

    match deserializer::read_file(&args.src) {
        Ok(json) => {
            acf::process_group(&json, &args.dest, args.overwrite);
        }
        Err(e) => {
            exit_with_error(e);
        }
    }
}
