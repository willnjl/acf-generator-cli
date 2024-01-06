use serde::Deserialize;

use crate::acf::layouts::Layouts;

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

    pub fn generate(field: &Field, mut indent: isize) -> String {
        let mut buffer = String::new();

        let start = field.template_start(indent);

        buffer.push_str(&start);

        if let Some(fields) = &field.sub_fields() {
            indent += 2;

            for field in fields {
                buffer.push_str(&field.add_field(indent + 1));
            }
        }

        let end = field.template_end(indent);
        buffer.push_str(&end);

        buffer
    }

    fn template_start(&self, indentation: isize) -> String {
        let inner = Field::get_indent(indentation, 1);
        let outer = Field::get_indent(indentation, 0);
        match self.get_kind() {
            FieldKind::Relationship => String::new(),
            FieldKind::Repeater => {
                format!(
                    "{}?> \n\n{}<?\n{}// {}\n{}if (have_rows('{}')) : ?> \n{} <? while (have_rows('{}')) :  the_row(); \n",
                    outer,outer,outer, self.label(),outer ,self.name(),inner,self.name(),
                )
            }
            _ => String::new(),
        }
    }

    fn template_end(&self, indentation: isize) -> String {
        let inner = Field::get_indent(indentation, 0);
        let outer = Field::get_indent(indentation, -1);
        match self.get_kind() {
            FieldKind::Relationship => String::new(),
            FieldKind::Repeater => {
                format!(
                    "{}?>\n\n{}<? endwhile;\n{}wp_reset_query(); ?> \n{} <? endif;",
                    inner, inner, inner, outer
                )
            }
            FieldKind::Generic => Field::add_field(self, 0),
            _ => String::new(),
        }
    }

    fn add_field(&self, indentation: isize) -> String {
        let indent = Field::get_indent(indentation, 0);
        return match self.get_kind() {
            FieldKind::Generic => {
                format!(
                    "{}${} = get_sub_field(\"{}\"); // {} -- {}\n",
                    indent,
                    self.name(),
                    self.name(),
                    self.label(),
                    self.type_name(),
                )
            }
            _ => String::new(),
        };
    }

    fn get_indent(indent: isize, offset: isize) -> String {
        let n = match indent.wrapping_add(offset) {
            x if x < 0 => 0,
            x => x,
        } as usize;

        "\t".to_string().repeat(n)
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
