use std::path::PathBuf;

use crate::{
    lib::{
        db::{EsdCasesData, Match},
        Document, Error,
    },
    rules::{full_code::FullCodeRule, num_code_w_ctw::NumCodeWithCtxRule},
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

pub fn get_rules() -> Vec<Box<dyn Rule>> {
    // rules have to be ordered by speed
    vec![Box::new(FullCodeRule {}), Box::new(NumCodeWithCtxRule {})]
}
