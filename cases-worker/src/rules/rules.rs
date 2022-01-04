use crate::{
    lib::{document::Document, error::Error},
    rules::example_rule::ExampleRule,
};

pub struct RuleCheckResult {
    pub message: String,
    pub is_match: bool,
}
pub trait Rule {
    fn check(&self, document: &Document) -> Result<RuleCheckResult, Error>;
}

pub fn get_rules() -> Vec<impl Rule> {
    vec![ExampleRule {}]
}
