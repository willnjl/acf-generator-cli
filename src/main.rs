use acf_fields::{Field, FieldGroup};
use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod acf_fields;
mod php_generator;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// src file containing acf fields (JSON)
    #[arg(short, long)]
    src: String,

    /// destination folder
    #[arg(short, long, default_value = "./")]
    dest: String,
}

/**
 * open src file and turn it into a string
 */
fn read_file(path: &str) -> FieldGroup {
    let mut file = File::open(Path::new(path)).expect("Unable to load file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let data: FieldGroup = serde_json::from_str(&buffer).unwrap();

    data
}

fn write_files(data: FieldGroup, dest: &str) {
    if let Some(fields) = data.fields {
        for field in fields {
            let php_gen = php_generator::PhpFileGenerator::new(field.name(), dest);
            if let Some(mut i) = php_gen {
                i.add_field(field.name(), field.label(), field.r#type());
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    let json_string = read_file(&args.src);

    write_files(json_string, &args.dest)
}
