use serde::Deserialize;

use crate::acf::layouts::Layouts;
use crate::utils::template_gen;

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

pub fn proccess(field: &Field, mut indent: isize) -> String {
    let mut buffer = String::new();

    let start = template_gen::template_start(field, indent);

    buffer.push_str(&start);

    if let Some(fields) = &field.sub_fields() {
        indent += 2;

        for field in fields {
            buffer.push_str(&template_gen::add_field(field, indent + 1));
        }
    }

    let end = template_gen::template_end(field, indent);
    buffer.push_str(&end);

    buffer
}
