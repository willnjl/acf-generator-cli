use colored::Colorize;

use crate::acf_fields::{Field, FieldKind};
use crate::cli_output;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{self, Write};

pub struct PhpFileGenerator {
    file: Option<File>,
}

impl PhpFileGenerator {
    pub fn new(path: &str, overwrite: bool) -> PhpFileGenerator {
        let _ = create_dir_all(std::path::Path::new(path).parent().unwrap()); // Create the directory structure if it doesn't exist

        if overwrite {
            return match OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&path)
            {
                Ok(file) => {
                    cli_output::file_created_feedback(&format!("{}", path.yellow(),));
                    PhpFileGenerator { file: Some(file) }
                }
                Err(e) => {
                    cli_output::exit_with_error(&format!(
                        "{} {}",
                        "Error creating ",
                        &path.yellow()
                    ));
                    PhpFileGenerator { file: None }
                }
            };
        };

        return match OpenOptions::new().create_new(true).write(true).open(path) {
            Ok(file) => {
                cli_output::file_created_feedback(&format!("{}", path.yellow(),));
                PhpFileGenerator { file: Some(file) }
            }
            Err(e) => {
                Self::file_creation_error_handler(&path, e);
                PhpFileGenerator { file: None }
            }
        };
    }

    pub fn template_start(field: &Field, indentation: isize) -> String {
        let inner = Self::get_indent(indentation, 1);
        let outer = Self::get_indent(indentation, 0);
        match field.get_kind() {
            FieldKind::Relationship => String::new(),
            FieldKind::Repeater => {
                format!(
                    "{}?> \n\n{}<?\n{}// {}\n{}if (have_rows('{}')) : ?> \n{} <? while (have_rows('{}')) :  the_row(); \n",
                    outer,outer,outer, field.label(),outer ,field.name(),inner,field.name(),
                )
            }
            _ => String::new(),
        }
    }
    pub fn template_end(field: &Field, indentation: isize) -> String {
        let inner = Self::get_indent(indentation, 0);
        let outer = Self::get_indent(indentation, -1);
        match field.get_kind() {
            FieldKind::Relationship => String::new(),
            FieldKind::Repeater => {
                format!(
                    "{}?>\n\n{}<? endwhile;\n{}wp_reset_query(); ?> \n{} <? endif;",
                    inner, inner, inner, outer
                )
            }
            FieldKind::Generic => Self::add_field(field, 0),
            _ => String::new(),
        }
    }

    pub fn add_field(field: &Field, indentation: isize) -> String {
        let indent = Self::get_indent(indentation, 0);
        return match field.get_kind() {
            FieldKind::Generic => {
                format!(
                    "{}${} = get_sub_field(\"{}\"); // {} -- {}\n",
                    indent,
                    field.name(),
                    field.name(),
                    field.label(),
                    field.type_name(),
                )
            }
            _ => String::new(),
        };
    }

    fn get_indent(indent: isize, offset: isize) -> String {
        let n = match indent.wrapping_add(offset) {
            x if x < 0 => 0,
            x => x,
        } as usize;

        "\t".to_string().repeat(n)
    }

    pub fn write_to_file(&mut self, content: &str) {
        if let Some(file) = &mut self.file {
            writeln!(file, "{}", content).expect("Write Fail");
        }
    }

    fn file_creation_error_handler(path: &str, e: io::Error) {
        match e.kind() {
            io::ErrorKind::AlreadyExists => {
                cli_output::file_exists_feedback(&format!("{} ...", path.yellow()));
            }
            _ => {}
        };
    }
}
