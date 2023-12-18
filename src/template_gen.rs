use crate::acf::{Field, FieldKind};
use crate::cli_output;
use colored::Colorize;
use std::io;

pub fn template_start(field: &Field, indentation: isize) -> String {
    let inner = get_indent(indentation, 1);
    let outer = get_indent(indentation, 0);
    match field.get_kind() {
        FieldKind::Relationship => String::new(),
        FieldKind::Repeater => {
            format!(
                    "{}?> \n\n{}<?\n{}// {}\n{}if (have_rows('{}')) : ?> \n{} <? while (have_rows('{}')) :  the_row(); \n",
                    outer,outer,outer, field.label(),outer ,field.name(),inner,field.name(),
                )
        }
        _ => String::new(),
    }
}
pub fn template_end(field: &Field, indentation: isize) -> String {
    let inner = get_indent(indentation, 0);
    let outer = get_indent(indentation, -1);
    match field.get_kind() {
        FieldKind::Relationship => String::new(),
        FieldKind::Repeater => {
            format!(
                "{}?>\n\n{}<? endwhile;\n{}wp_reset_query(); ?> \n{} <? endif;",
                inner, inner, inner, outer
            )
        }
        FieldKind::Generic => add_field(field, 0),
        _ => String::new(),
    }
}

pub fn add_field(field: &Field, indentation: isize) -> String {
    let indent = get_indent(indentation, 0);
    return match field.get_kind() {
        FieldKind::Generic => {
            format!(
                "{}${} = get_sub_field(\"{}\"); // {} -- {}\n",
                indent,
                field.name(),
                field.name(),
                field.label(),
                field.type_name(),
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
