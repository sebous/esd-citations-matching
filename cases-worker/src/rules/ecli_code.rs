use super::rules::{self, Rule};
use std::path::PathBuf;

use itertools::Itertools;

use crate::{
    lib::{
        self,
        db::{self},
        logger, util, Error,
    },
    WorkerData,
};

pub struct EcliCodeRule {}

impl Rule for EcliCodeRule {
    fn get_name(&self) -> &'static str {
        "ecli_code"
    }

    fn check(
        &self,
        document: &lib::Document,
        path: &PathBuf,
        worker_data: &WorkerData,
    ) -> Result<super::RuleCheckResult, Error> {
        let match_found = lib::regex::ECLI_CODE.is_match(document.full_text_l.as_str());

        if !match_found {
            return Ok(rules::RuleCheckResult {
                is_match: false,
                matches: vec![],
                message: None,
            });
        }

        let codes = lib::regex::ECLI_CODE
            .captures_iter(document.full_text_l.as_str())
            .map(|c| c[0].to_owned())
            .unique()
            .collect_vec();

        let matches = codes
            .iter()
            .map(|ecli| {
                let ecli_upper = ecli.to_uppercase();
                let matched_case = worker_data.data.iter().find(|x| x.ecli == ecli_upper);

                match matched_case {
                    None => {
                        logger::rule_warning(
                            self.get_name(),
                            "found matching ecli that doesn't exist in db",
                            path,
                            &ecli_upper,
                        );
                        None
                    }
                    Some(case) => Some(db::Match {
                        source_case_id: worker_data
                            .source_data
                            .iter()
                            .find(|x| x.file_name == util::normalize_filename(path))
                            .unwrap()
                            .id,
                        matched_case_id: case.id,
                        matched_value: ecli_upper.to_owned(),
                        m_type: self.get_name().to_string(),
                    }),
                }
            })
            .filter_map(|x| x)
            .unique_by(|x| x.matched_case_id)
            .collect_vec();

        Ok(rules::RuleCheckResult {
            is_match: true,
            matches,
            message: None,
        })
    }
}
