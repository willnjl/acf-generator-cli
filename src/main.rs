use crate::acf_fields::FieldKind;
use acf_fields::{Field, FieldGroup};
use clap::Parser;
use colored::Colorize;
use php_generator::PhpFileGenerator;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod acf_fields;
mod cli_output;
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

    #[arg(short, long)]
    overwrite: bool,
}

/**
 * open src file and turn it into a string
 */
fn read_file(path: &str) -> FieldGroup {
    cli_output::running_task_feedback("Opening file...");
    let file_result = File::open(Path::new(path));

    let mut file = match file_result {
        Ok(file) => file,
        Err(_) => {
            cli_output::exit_with_error(&format!("Unable to open file '{}'", path.yellow()));
            unreachable!();
        }
    };

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let data: FieldGroup = serde_json::from_str(&buffer).unwrap();

    data
}

fn process_field_group(group: &FieldGroup, dest: &str, ow: bool) {
    for field in group.fields() {
        if field.get_kind() == FieldKind::FlexibleContent {
            if let Some(layouts) = &field.layouts() {
                for (_, layout) in &layouts.0 {
                    let mut buffer = "<?php \n".to_string();
                    let path = layout.get_path(field, dest);
                    let mut writer = PhpFileGenerator::new(&path, ow);
                    let indent: isize = 0;
                    for field in layout.sub_fields() {
                        buffer.push_str(&proccess_field(&field, indent));
                    }
                    writer.write_to_file(&buffer);
                }
            }
        }
    }
}

fn proccess_field(field: &Field, mut indent: isize) -> String {
    let mut buffer = String::new();

    let start = PhpFileGenerator::template_start(field, indent);

    buffer.push_str(&start);

    if let Some(fields) = &field.sub_fields() {
        indent += 2;

        for field in fields {
            buffer.push_str(&PhpFileGenerator::add_field(field, indent + 1));
        }
    }

    let end = PhpFileGenerator::template_end(field, indent);
    buffer.push_str(&end);
    buffer
}

fn main() {
    let args = Args::parse();

    let json_string = read_file(&args.src);

    process_field_group(&json_string, &args.dest, args.overwrite);
}
