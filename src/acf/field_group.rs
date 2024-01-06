use serde::Deserialize;

use crate::acf::Field;

#[derive(Debug, Deserialize)]
pub struct FieldGroup {
    fields: Vec<Field>,
}

impl FieldGroup {
    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }
}
