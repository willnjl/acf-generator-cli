use crate::acf_fields::{Field, FieldKind};
use std::fs::{File, OpenOptions};
use std::io::Write;

pub struct PhpFileGenerator {
    file: Option<File>,
}

impl PhpFileGenerator {
    pub fn new(file_name: &str, dest: &str, overwrite: bool) -> PhpFileGenerator {
        let path = format!("{}/{}.php", dest, file_name);

        if overwrite {
            return match OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)
            {
                Ok(file) => PhpFileGenerator { file: Some(file) },
                Err(_) => PhpFileGenerator { file: None },
            };
        };

        return match OpenOptions::new().create_new(true).write(true).open(path) {
            Ok(file) => PhpFileGenerator { file: Some(file) },
            Err(_) => PhpFileGenerator { file: None },
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
                    outer,outer,outer, field.label,outer ,field.name,inner,field.name,
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
                    indent, field.name, field.name, field.label, field.type_name,
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
}
