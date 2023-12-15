use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;
#[derive(Debug, Deserialize)]
pub struct FieldGroup {
    key: String,
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

#[derive(Debug, Deserialize)]
pub struct Layouts(pub HashMap<String, Layout>);
#[derive(Debug, Deserialize)]
pub struct Layout {
    pub key: String,
    pub name: String,
    label: String,
    pub sub_fields: Vec<Field>,
}

impl Layout {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn sub_fields(&self) -> &Vec<Field> {
        &self.sub_fields
    }
}

#[derive(Debug, Deserialize)]
pub struct Field {
    #[serde(default)]
    key: String,
    pub name: String,
    pub label: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub sub_fields: Option<Vec<Field>>,
    pub layouts: Option<Layouts>,
}

impl Field {
    pub fn get_kind(&self) -> FieldKind {
        FieldKind::from_str(&self.type_name)
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
