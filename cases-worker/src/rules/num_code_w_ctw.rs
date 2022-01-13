use std::path::PathBuf;

use itertools::Itertools;

use crate::lib::{db, regex, util, Document, Error};

use super::rules::{self, Rule};

pub struct NumCodeWithCtxRule {}

impl Rule for NumCodeWithCtxRule {
    fn get_name(&self) -> &'static str {
        "num_code_w_context_not_T"
    }

    fn check(
        &self,
        document: &Document,
        path: &PathBuf,
        data: &db::EsdCasesData,
    ) -> Result<rules::RuleCheckResult, Error> {
        let match_found = regex::CODE.is_match(&document.full_text);

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

        let cases = regex::CODE
            .captures_iter(&document.full_text)
            .filter(|c| !regex::T_CODE.is_match(&c[0]))
            .map(|c| util::normalize_code(&c[1]))
            .unique()
            .map(|c| db::Match {
                source_case: util::normalize_filename(path),
                matched_case_table: None,
                matched_case_id: None,
                matched_value: Some(c.to_owned()),
                m_type: self.get_name().to_string(),
            })
            .collect_vec();

        Ok(rules::RuleCheckResult {
            is_match: true,
            cases,
            message: None,
        })
    }
}
