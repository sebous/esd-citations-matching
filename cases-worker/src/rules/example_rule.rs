use crate::lib::{document::Document, error::Error};

use super::rules::{Rule, RuleCheckResult};

/// counting words only, always matches
pub struct ExampleRule {}

impl Rule for ExampleRule {
    fn check(document: &Document) -> Result<RuleCheckResult, Error> {
        Ok(RuleCheckResult {
            is_match: true,
            message: document.words.iter().count().to_string(),
        })
    }
}
