use std::path::PathBuf;

use itertools::Itertools;
use log::info;

use crate::lib::{db, document, error, logger, regex, util};

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
        let match_found = regex::T_CODE.is_match(&document.full_text);

        if !match_found {
            return Ok(rules::RuleCheckResult {
                is_match: false,
                message: None,
                cases: vec![],
            });
        }

        let dvur_keyword_present = util::check_dvur_existence(document);

        if dvur_keyword_present.is_none() {
            return Ok(rules::RuleCheckResult {
                is_match: false,
                message: None,
                cases: vec![],
            });
        }

        let codes = regex::T_CODE
            .captures_iter(&document.full_text)
            .map(|c| util::normalize_code(&c[1]))
            .unique()
            .collect_vec();

        let cases = codes
            .iter()
            .map(|c| {
                // logger::rule_info(self.get_name(), "match found", path, c);
                db::Match {
                    source_case: util::normalize_filename(path),
                    matched_case_table: String::new(),
                    matched_case_id: 0,
                    m_type: self.get_name().to_string(),
                }
            })
            .collect_vec();

        Ok(rules::RuleCheckResult {
            is_match: true,
            cases,
            message: None,
        })
    }
}
