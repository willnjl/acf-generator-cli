mod acf;
mod cli_args;
mod cli_output;
mod deserializer;
mod error;
mod file_gen;
mod template_gen;

use clap::Parser;

fn main() {
    let args = cli_args::Args::parse();
    let json_string = deserializer::read_file(&args.src);

    acf::process_group(&json_string, &args.dest, args.overwrite);
}
