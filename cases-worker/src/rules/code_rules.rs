use log::info;

use crate::lib;

use super::rules::{self, Rule};

pub struct FullCodeRule {}

impl Rule for FullCodeRule {
    fn check(
        &self,
        document: &lib::document::Document,
    ) -> Result<rules::RuleCheckResult, lib::error::Error> {
        let match_found = lib::regex::C_CODE.is_match(document.full_text.as_str());

        info!("match found");

        Ok(rules::RuleCheckResult {
            is_match: false,
            case_id: Some(0),
            case_table: Some("".to_string()),
            message: "".to_string(),
        })
    }
}
