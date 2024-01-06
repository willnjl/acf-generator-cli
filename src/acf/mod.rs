pub mod field;
pub mod field_group;
pub mod layouts;
pub mod post_type;

use field::{Field, FieldKind};
use field_group::FieldGroup;

use crate::error::ALGError;
use crate::utils::file_service::FileService;
use crate::utils::template_gen;

pub fn process_group(group: &FieldGroup, dest: &str, ow: bool) -> Result<(), ALGError> {
    for field in group.fields() {
        if field.get_kind() == FieldKind::FlexibleContent {
            if let Some(layouts) = &field.layouts() {
                for (_, layout) in &layouts.0 {
                    let mut buffer = "<?php \n".to_string();
                    let path = layout.get_path(field, dest);
                    let mut writer: FileService = FileService::new(&path, ow)?;
                    let indent: isize = 0;
                    for field in layout.sub_fields() {
                        buffer.push_str(&proccess_field(&field, indent));
                    }
                    writer.write(&buffer);
                }
            }
        }
    }
    Ok(())
}

pub fn proccess_field(field: &Field, mut indent: isize) -> String {
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
