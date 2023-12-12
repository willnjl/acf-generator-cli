use acf_fields::{Field, FieldGroup};
use clap::Parser;
use php_generator::PhpFileGenerator;
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

fn process_field_group(group: &FieldGroup, dest: &str) {
    println!("Processing field: {}", group.label());
    for field in group.fields() {
        process_field_recursive(&field, &dest, &mut None);
    }
}

fn process_field_recursive(field: &Field, dest: &str, file: &mut Option<&mut PhpFileGenerator>) {
    if let Some(sub_fields) = &field.sub_fields() {
        for sub_field in sub_fields {
            process_field_recursive(sub_field, dest, file);
        }
    }

    if let Some(file) = file {
        file.add_field(field);
    }

    if let Some(layouts) = &field.layouts() {
        for (key, layout) in layouts {
            println!("Processing layout with key: {}", layout.name());
            if let Some(mut file) = PhpFileGenerator::new(layout.name(), dest) {
                for layout_field in layout.sub_fields() {
                    process_field_recursive(layout_field, dest, &mut Some(&mut file));
                }
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    let json_string = read_file(&args.src);

    process_field_group(&json_string, &args.dest);
}
