use serde::Deserialize;
use std::collections::HashMap;

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
