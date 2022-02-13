use unidecode::unidecode;

#[derive(Debug)]
pub struct Document {
    pub full_text: String,
    pub full_text_l: String,
}

impl Document {
    pub fn create(full_text: &str) -> Document {
        let normalized = unidecode(full_text);
        Document {
            full_text: normalized.clone(),
            full_text_l: normalized.to_lowercase(),
        }
    }
}
