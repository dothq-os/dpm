use super::deb_control::{Field, Paragraph};

pub fn paragraph_contains(paragraph: Paragraph, query: String) -> Option<Field> {
    let mut field = None;

    let results: Vec<Field> = paragraph
        .fields
        .into_iter()
        .filter(|f| f.name == query)
        .collect();

    if !results.is_empty() {
        field = Some(results[0].clone());
    }

    field
}
