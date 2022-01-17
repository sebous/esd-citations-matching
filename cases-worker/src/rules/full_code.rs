use std::path::PathBuf;

use itertools::Itertools;

use crate::lib::{
    self,
    db::{self, Code},
    logger, util, Error,
};

use super::rules::{self, Rule};

pub struct FullCodeRule {}

impl Rule for FullCodeRule {
    fn get_name(&self) -> &'static str {
        "full_code_c"
    }

    fn check(
        &self,
        document: &lib::Document,
        path: &PathBuf,
        data: &Vec<db::EsdCase>,
    ) -> Result<rules::RuleCheckResult, Error> {
        let match_found = lib::regex::C_CODE.is_match(document.full_text.as_str());

        if !match_found {
            return Ok(rules::RuleCheckResult {
                is_match: false,
                matches: vec![],
                message: None,
            });
        }

        let codes = lib::regex::C_CODE
            .captures_iter(document.full_text.as_str())
            .map(|c| util::normalize_code(&c[1]))
            .unique()
            .collect_vec();

        let matches = codes
            .iter()
            .map(|code| {
                let matched_case = data.iter().find(|&case| case.get_codes().contains(code));
                match matched_case {
                    None => {
                        logger::rule_warning(
                            self.get_name(),
                            "found matching code that doesn't exist in db",
                            path,
                            code,
                        );
                        None
                    }
                    Some(case) => Some(db::Match {
                        source_case: util::normalize_filename(path),
                        matched_case_id: case.id,
                        matched_value: case.code.to_owned(),
                        m_type: self.get_name().to_string(),
                    }),
                }
            })
            .filter_map(|x| x)
            .unique_by(|m| m.matched_case_id)
            .collect_vec();

        Ok(rules::RuleCheckResult {
            is_match: true,
            matches,
            message: None,
        })
    }
}
