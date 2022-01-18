use std::path::PathBuf;

use regex::Regex;

use crate::{
    lib::{
        db::{EsdCase, Match},
        Document, Error,
    },
    rules::{full_code::FullCodeRule, short_name::ShortNameRule, NumCodeWithKey},
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
        data: &Vec<EsdCase>,
        regexes: &Vec<(usize, Regex)>,
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
