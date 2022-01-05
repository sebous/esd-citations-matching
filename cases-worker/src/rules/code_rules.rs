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
        document: &lib::document::Document,
        path: &PathBuf,
        data: &db::EsdCasesData,
    ) -> Result<rules::RuleCheckResult, lib::error::Error> {
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

        // info!("match found, file: {}, codes: {:?}", &filename, &codes);

        let (code_cases, _) = data;

        let cases = codes
            .iter()
            .unique()
            .map(|c| {
                let case = code_cases.iter().find(|case| &case.code == c);
                if case.is_none() {
                    logger::rule_warning(
                        self.get_name(),
                        "found esd code not exists in db",
                        path,
                        c,
                    );
                }
                db::Match {
                    source_case: util::normalize_filename(path),
                    matched_case_table: db::EsdCaseCode::TABLE_NAME.to_string(),
                    matched_case_id: if case.is_some() { case.unwrap().id } else { 0 },
                    m_type: self.get_name().to_string(),
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
