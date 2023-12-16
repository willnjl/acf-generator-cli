use clap::Parser;

mod acf_fields;
mod cli_args;
mod cli_output;
mod file_gen;
use acf_fields::*;

fn main() {
    let args = cli_args::Args::parse();
    let json_string = read_file(&args.src);

    process_field_group(&json_string, &args.dest, args.overwrite);
}
