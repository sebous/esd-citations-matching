use std::path::PathBuf;

use crate::{
    lib::{db::Match, Document, Error},
    rules::{full_code::FullCodeRule, short_name::ShortNameRule, NumCodeWithKey},
    WorkerData,
};

pub struct RuleCheckResult {
    pub message: Option<String>,
    pub is_match: bool,
    pub matches: Vec<Match>,
}
pub trait Rule {
    fn check(
        &self,
        document: &Document,
        path: &PathBuf,
        worker_data: &WorkerData,
    ) -> Result<RuleCheckResult, Error>;
    fn get_name(&self) -> &'static str;
}

pub fn get_rules() -> Vec<Box<dyn Rule>> {
    // rules have to be ordered by speed
    vec![
        Box::new(FullCodeRule {}),
        Box::new(ShortNameRule {}),
        Box::new(NumCodeWithKey {}),
    ]
}
