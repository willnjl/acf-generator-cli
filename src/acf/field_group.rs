use serde::Deserialize;

use crate::acf::field::*;
use crate::error::ALGError;
use crate::utils::file_service::FileService;
#[derive(Debug, Deserialize)]
pub struct FieldGroup {
    fields: Vec<Field>,
}

impl FieldGroup {
    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }
}

pub fn process(group: &FieldGroup, dest: &str, ow: bool) -> Result<(), ALGError> {
    for field in group.fields() {
        if field.get_kind() == FieldKind::FlexibleContent {
            if let Some(layouts) = &field.layouts() {
                for (_, layout) in &layouts.0 {
                    let mut buffer = "<?php \n".to_string();
                    let path = layout.get_path(field, dest);
                    let mut writer: FileService = FileService::new(&path, ow)?;
                    let indent: isize = 0;
                    for field in layout.sub_fields() {
                        buffer.push_str(&proccess(&field, indent));
                    }
                    writer.write(&buffer);
                }
            }
        }
    }
    Ok(())
}
