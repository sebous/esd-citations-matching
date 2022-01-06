use std::path::PathBuf;

use crate::{
    lib::{
        db::{EsdCasesData, Match},
        document::Document,
        error::Error,
    },
    rules::full_code::FullCodeRule,
};

pub struct RuleCheckResult {
    pub message: Option<String>,
    pub is_match: bool,
    pub cases: Vec<Match>,
}
pub trait Rule {
    fn check(
        &self,
        document: &Document,
        path: &PathBuf,
        data: &EsdCasesData,
    ) -> Result<RuleCheckResult, Error>;
    fn get_name(&self) -> &'static str;
}

pub fn get_rules() -> Vec<impl Rule> {
    // rules have to be ordered by speed
    vec![FullCodeRule {}]
}
