use crate::{
    lib::{document::Document, error::Error},
    rules::code_rules::FullCodeRule,
};

pub struct RuleCheckResult {
    pub message: String,
    pub is_match: bool,
    pub case_id: Option<usize>,
    pub case_table: Option<String>,
}
pub trait Rule {
    fn check(&self, document: &Document) -> Result<RuleCheckResult, Error>;
}

pub fn get_rules() -> Vec<impl Rule> {
    // rules have to be ordered by speed
    vec![FullCodeRule {}]
}
