use itertools::Itertools;

use crate::{
    common::{self, logger, util, Error},
    WorkerData,
};

use super::rules::{self, Rule};

pub struct FullCodeRule {}

impl Rule for FullCodeRule {
    fn get_name(&self) -> &'static str {
        "full_code"
    }

    fn check(
        &self,
        document: &common::Document,
        worker_data: &WorkerData,
    ) -> Result<rules::RuleCheckResult, Error> {
        let match_found = common::regex::C_CODE.is_match(document.full_text.as_str());

        if !match_found {
            return Ok(rules::RuleCheckResult {
                is_match: false,
                matches: vec![],
                message: None,
            });
        }

        let codes = common::regex::C_CODE
            .captures_iter(document.full_text.as_str())
            .filter_map(|c| {
                c.get(1).and_then(|m| {
                    Some((
                        util::normalize_code(m.as_str()),
                        util::extract_match_context(&m, document),
                    ))
                })
            })
            .unique()
            .collect_vec();

        let matches = codes
            .iter()
            .map(|(code, str_ctx)| {
                let matched_case = worker_data
                    .source_data
                    .iter()
                    .find(|&case| case.codes_list.contains(code));
                match matched_case {
                    None => {
                        logger::rule_warning(
                            self.get_name(),
                            "found matching code that doesn't exist in db",
                            &document.id,
                            code,
                        );
                        None
                    }
                    Some(case) => Some(rules::Match {
                        source_case_id: document.id.clone(),
                        matched_case_code: case.code.clone(),
                        m_type: self.get_name().to_string(),
                        match_context: Some(str_ctx.to_string()),
                    }),
                }
            })
            .filter_map(|x| x)
            .unique_by(|m| m.matched_case_code.clone())
            .collect_vec();

        Ok(rules::RuleCheckResult {
            is_match: true,
            matches,
            message: None,
        })
    }
}
