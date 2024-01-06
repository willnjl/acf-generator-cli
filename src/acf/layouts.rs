use serde::Deserialize;
use std::collections::HashMap;

use crate::acf::Field;

#[derive(Debug, Deserialize)]
pub struct Layouts(pub HashMap<String, Layout>);
#[derive(Debug, Deserialize)]
pub struct Layout {
    name: String,
    sub_fields: Vec<Field>,
}

impl Layout {
    pub fn sub_fields(&self) -> &Vec<Field> {
        &self.sub_fields
    }

    pub fn get_path(&self, field: &Field, dest: &str) -> String {
        format!("{}/{}/{}.php", dest, field.name(), self.name)
    }
}
