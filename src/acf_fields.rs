use serde::Deserialize;
use std::collections::HashMap;

use crate::cli_output::{self};
use crate::file_gen::FileGen;
use colored::Colorize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/**


 Field Group


*/
#[derive(Debug, Deserialize)]
pub struct FieldGroup {
    title: String,
    fields: Vec<Field>,
}

impl FieldGroup {
    pub fn label(&self) -> &str {
        &self.title
    }
    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }
}

/**


     Layouts


*/

#[derive(Debug, Deserialize)]
pub struct Layouts(pub HashMap<String, Layout>);
#[derive(Debug, Deserialize)]
pub struct Layout {
    name: String,
    sub_fields: Vec<Field>,
}

impl Layout {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn sub_fields(&self) -> &Vec<Field> {
        &self.sub_fields
    }

    pub fn get_path(&self, field: &Field, dest: &str) -> String {
        format!("{}/{}/{}.php", dest, field.name(), self.name)
    }
}

/**


    Fields


*/
#[derive(Debug, Deserialize)]
pub struct Field {
    #[serde(default)]
    name: String,
    label: String,
    #[serde(rename = "type")]
    type_name: String,
    sub_fields: Option<Vec<Field>>,
    layouts: Option<Layouts>,
}

impl Field {
    pub fn get_kind(&self) -> FieldKind {
        FieldKind::from_str(&self.type_name)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn label(&self) -> &str {
        &self.label
    }
    pub fn sub_fields(&self) -> &Option<Vec<Field>> {
        &self.sub_fields
    }
    pub fn layouts(&self) -> &Option<Layouts> {
        &self.layouts
    }
    pub fn type_name(&self) -> &str {
        &self.type_name
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum FieldKind {
    FlexibleContent,
    Generic,
    Relationship,
    Repeater,
}

impl FieldKind {
    pub fn from_str(field_type: &str) -> FieldKind {
        match field_type {
            "flexible_content" => FieldKind::FlexibleContent,
            "repeater" => FieldKind::Repeater,
            "relationship" => FieldKind::Relationship,
            _ => FieldKind::Generic,
        }
    }
}

pub fn process_field_group(group: &FieldGroup, dest: &str, ow: bool) {
    for field in group.fields() {
        if field.get_kind() == FieldKind::FlexibleContent {
            if let Some(layouts) = &field.layouts() {
                for (_, layout) in &layouts.0 {
                    let mut buffer = "<?php \n".to_string();
                    let path = layout.get_path(field, dest);
                    let mut writer = FileGen::new(&path, ow);
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

pub fn proccess_field(field: &Field, mut indent: isize) -> String {
    let mut buffer = String::new();

    let start = FileGen::template_start(field, indent);

    buffer.push_str(&start);

    if let Some(fields) = &field.sub_fields() {
        indent += 2;

        for field in fields {
            buffer.push_str(&FileGen::add_field(field, indent + 1));
        }
    }

    let end = FileGen::template_end(field, indent);
    buffer.push_str(&end);
    buffer
}

/**
 * open src file and turn it into a string
 */
pub fn read_file(path: &str) -> FieldGroup {
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
