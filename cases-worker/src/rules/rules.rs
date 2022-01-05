use std::path::PathBuf;

use crate::{
    lib::{
        db::{EsdCasesData, Match},
        document::Document,
        error::Error,
    },
    rules::code_rules::FullCodeRule,
};

pub struct RuleCheckResult {
    pub message: String,
    pub is_match: bool,
    // TODO: maybe cases could be Vec<Match>?
    pub cases: Vec<Match>,
}
pub trait Rule {
    fn check(
        &self,
        document: &Document,
        path: &PathBuf,
        data: &EsdCasesData,
    ) -> Result<RuleCheckResult, Error>;
}

pub fn get_rules() -> Vec<impl Rule> {
    // rules have to be ordered by speed
    vec![FullCodeRule {}]
}
