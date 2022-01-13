use std::path::PathBuf;

use itertools::Itertools;

use crate::lib::{self, db, logger, util};

use super::rules::{self, Rule};

pub struct FullCodeRule {}

impl Rule for FullCodeRule {
    fn get_name(&self) -> &'static str {
        "full_code_C"
    }

    fn check(
        &self,
        document: &lib::Document,
        path: &PathBuf,
        data: &db::EsdCasesData,
    ) -> Result<rules::RuleCheckResult, lib::Error> {
        let match_found = lib::regex::C_CODE.is_match(document.full_text.as_str());

        if !match_found {
            return Ok(rules::RuleCheckResult {
                is_match: false,
                cases: vec![],
                message: None,
            });
        }

        let codes = lib::regex::C_CODE
            .captures_iter(document.full_text.as_str())
            .map(|c| util::normalize_code(&c[1]))
            .unique()
            .collect_vec();

        let (code_cases, _) = data;

        let cases = codes
            .iter()
            .map(|c| {
                let case = code_cases.iter().find(|case| &case.code == c);
                if case.is_none() {
                    logger::rule_warning(
                        self.get_name(),
                        "found esd code not exists in db",
                        path,
                        c,
                    );
                    None
                } else {
                    Some(db::Match {
                        source_case: util::normalize_filename(path),
                        matched_case_table: Some(db::EsdCaseCode::TABLE_NAME.to_string()),
                        matched_case_id: case.map(|c| c.id),
                        matched_value: case.map(|c| c.code.to_owned()),
                        m_type: self.get_name().to_string(),
                    })
                }
            })
            .filter_map(|x| x)
            .collect_vec();

        Ok(rules::RuleCheckResult {
            is_match: true,
            cases,
            message: None,
        })
    }
}
