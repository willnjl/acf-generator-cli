use serde::{Deserialize, Serialize};
use std::{collections::HashMap, vec};

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Field {
    key: String,
    name: String,
    label: String,
    r#type: String,
    layouts: Option<HashMap<String, Layout>>,
    sub_fields: Option<Vec<Field>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Layout {
    key: String,
    name: String,
    label: String,
    sub_fields: Vec<Field>,
}

impl Layout {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn sub_fields(&self) -> &Vec<Field> {
        &self.sub_fields
    }
}

pub enum FieldTypes {
    FlexibleContent,
    Generic,
}

impl FieldTypes {
    pub fn to_str(&self) -> &'static str {
        match *self {
            FieldTypes::FlexibleContent => "flexible_content",
            _ => "generic",
        }
    }
}

impl Field {
    pub fn label(&self) -> &str {
        &self.label
    }
    pub fn field_type(&self) -> &str {
        &self.r#type
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn layouts(&self) -> &Option<HashMap<String, Layout>> {
        &self.layouts
    }

    pub fn sub_fields(&self) -> &Option<Vec<Field>> {
        &self.sub_fields
    }
}
