use std::path::PathBuf;

use itertools::Itertools;

use crate::lib::{db, document, error, regex, util};

use super::rules::{self, Rule};

pub struct TCodeWithCtxRule {}

impl Rule for TCodeWithCtxRule {
    fn get_name(&self) -> &'static str {
        "T_code_with_context"
    }

    fn check(
        &self,
        document: &document::Document,
        path: &PathBuf,
        data: &db::EsdCasesData,
    ) -> Result<rules::RuleCheckResult, error::Error> {
        let (cases_code, cases_fulltext) = data;

        let match_found = regex::T_CODE.is_match(&document.full_text);
        if !match_found {
            return Ok(rules::RuleCheckResult {
                is_match: false,
                message: None,
                cases: vec![],
            });
        }

        let matches = regex::T_CODE
            .captures_iter(&document.full_text)
            .map(|c| util::normalize_code(&c[1]))
            .collect_vec();

        unimplemented!()
    }
}
