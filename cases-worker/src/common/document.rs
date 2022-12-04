use unidecode::unidecode;

use crate::csv_source::Case;

#[derive(Debug)]
pub struct Document {
    pub id: String,
    pub full_text: String,
    pub full_text_l: String,
}

impl Document {
    pub fn create(case: &Case) -> Document {
        let normalized = unidecode(&case.text);
        Document {
            id: case.id.clone(),
            full_text: normalized.clone(),
            full_text_l: normalized.to_lowercase(),
        }
    }
}
