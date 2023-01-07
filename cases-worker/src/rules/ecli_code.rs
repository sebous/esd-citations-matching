use super::rules::{self, Match, Rule};

use itertools::Itertools;

use crate::{
    common::{self, logger, util, Error},
    WorkerData,
};

pub struct EcliCodeRule {}

impl Rule for EcliCodeRule {
    fn get_name(&self) -> &'static str {
        "ecli"
    }

    fn check(
        &self,
        document: &common::Document,
        worker_data: &WorkerData,
    ) -> Result<super::RuleCheckResult, Error> {
        let match_found = common::regex::ECLI_CODE.is_match(document.full_text_l.as_str());

        if !match_found {
            return Ok(rules::RuleCheckResult {
                is_match: false,
                matches: vec![],
                message: None,
            });
        }

        let codes = common::regex::ECLI_CODE
            .captures_iter(document.full_text_l.as_str())
            .filter_map(|c| {
                c.get(0)
                    .and_then(|m| Some((m.as_str(), util::extract_match_context(&m, document))))
            })
            .unique()
            .collect_vec();

        let matches = codes
            .iter()
            .map(|(ecli, str_ctx)| {
                let ecli_upper = ecli.to_uppercase();
                let matched_case = worker_data
                    .source_data
                    .iter()
                    .find(|x| x.ecli == ecli_upper);

                match matched_case {
                    None => {
                        logger::rule_warning(
                            self.get_name(),
                            "found matching ecli that doesn't exist in db",
                            &document.id,
                            &ecli_upper,
                        );
                        None
                    }
                    Some(case) => Some(Match {
                        matched_case_code: case.code.clone(),
                        source_case_id: document.id.clone(),
                        m_type: self.get_name().to_string(),
                        match_context: Some(str_ctx.to_string()),
                    }),
                }
            })
            .filter_map(|x| x)
            .unique_by(|x| x.matched_case_code.clone())
            .collect_vec();

        Ok(rules::RuleCheckResult {
            is_match: true,
            matches,
            message: None,
        })
    }
}
