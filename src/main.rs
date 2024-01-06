mod acf;
mod cli;
mod error;
mod utils;

use clap::Parser;

fn main() {
    let args = cli::args::Args::parse();

    if let Err(e) = acf::process(&args) {
        cli::output::exit_with_error(e);
    }
}
