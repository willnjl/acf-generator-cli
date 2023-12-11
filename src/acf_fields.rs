use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct FieldGroup {
    key: String,
    title: String,
    pub fields: Vec<FlexibleContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlexibleContent {
    key: String,
    pub label: String,
    name: String,
    layouts: Option<HashMap<String, Layout>>,
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
    sub_fields: Option<Vec<Field>>,
}
