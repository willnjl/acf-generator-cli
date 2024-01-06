use serde::Deserialize;

use crate::acf::field::{Field, FieldKind};
use crate::error::ALGError;
use crate::utils::file_service::FileService;
#[derive(Debug, Deserialize)]
pub struct PostType {
    post_type: String,
    title: String,
}

impl PostType {
    pub fn post_type(&self) -> &str {
        &self.post_type
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn files(&self, dest: &str) -> Vec<String> {
        let archive = format!("{}/archive-{}.php", dest, self.post_type);
        let card = format!("{}/template-parts/{}-card.php", dest, self.post_type);

        vec![archive, card]
    }
}

pub fn generate(post_type: &PostType, dest: &str, ow: bool) -> Result<(), ALGError> {
    for path in post_type.files(dest) {
        let mut writer: FileService = FileService::new(&path, ow)?;
        let mut buffer = "<?php \n".to_string();

        writer.write(&buffer);
    }

    Ok(())
}
