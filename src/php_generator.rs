// src/php_generator.rs

use crate::acf_fields::Field;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub struct PhpFileGenerator {
    file: File,
}

impl PhpFileGenerator {
    pub fn new(file_name: &str, dest: &str) -> Option<PhpFileGenerator> {
        let path = format!("{}/{}.php", dest, file_name);

        let file = OpenOptions::new().create_new(true).write(true).open(path);

        return match file {
            Ok(file) => {
                writeln!(&file, "{}", "<?php \n\n").expect("Write Fail");
                Some(PhpFileGenerator { file })
            }
            Err(_e) => None,
        };
    }

    pub fn add_field(&mut self, field: &Field) {
        let content = format!(
            "//{} - {}\n ${} = get_sub_field(\"{}\"); \n\n",
            field.label(),
            field.field_type(),
            field.name(),
            field.name()
        );

        self.write_to_file(&content);
    }

    fn write_to_file(&mut self, content: &str) {
        writeln!(self.file, "{}", content).expect("Write Fail");
    }
}
