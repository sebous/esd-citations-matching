use std::path::PathBuf;

use itertools::Itertools;
use log::{info, warn};

use crate::lib::{
    self,
    db::{EsdCasesData, Match, ESD_CASE_CODE_TABLE_NAME},
    util,
};

use super::rules::{self, Rule};

pub struct FullCodeRule {}

impl Rule for FullCodeRule {
    fn check(
        &self,
        document: &lib::document::Document,
        path: &PathBuf,
        data: &EsdCasesData,
    ) -> Result<rules::RuleCheckResult, lib::error::Error> {
        let filename = util::normalize_filename(path);
        let match_found = lib::regex::C_CODE.is_match(document.full_text.as_str());

        if !match_found {
            return Ok(rules::RuleCheckResult {
                is_match: false,
                cases: vec![],
                message: String::new(),
            });
        }

        let codes = lib::regex::C_CODE
            .captures_iter(document.full_text.as_str())
            .map(|c| util::normalize_code(&c[1]))
            .collect_vec();

        info!("match found, file: {}, codes: {:?}", &filename, &codes);

        let (code_cases, _) = data;

        let cases = codes
            .iter()
            .unique()
            .map(|c| {
                let case = code_cases.iter().find(|case| &case.code == c);
                if case.is_none() {
                    warn!(
                        "found esd code not exists in db, file: {}, code: {}",
                        filename, c
                    );
                }
                Match {
                    source_case: filename.to_owned(),
                    matched_case_table: ESD_CASE_CODE_TABLE_NAME.to_owned(),
                    matched_case_id: if case.is_some() { case.unwrap().id } else { 0 },
                    m_type: "C-XXX/XX".to_string(),
                }
            })
            .collect_vec();

        Ok(rules::RuleCheckResult {
            is_match: true,
            cases,
            message: String::new(),
        })
    }
}
