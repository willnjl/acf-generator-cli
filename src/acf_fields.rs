use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct FieldGroup {
    key: String,
    title: String,
    pub fields: Option<Vec<Field>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Layout {
    key: String,
    name: String,
    label: String,
    sub_fields: Vec<Field>,
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

impl Field {
    pub fn label(&self) -> &str {
        &self.label
    }
    pub fn r#type(&self) -> &str {
        &self.r#type
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}
