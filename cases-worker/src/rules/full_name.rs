use std::path::PathBuf;

use itertools::Itertools;

use super::{Rule, RuleCheckResult};
use crate::{
    lib::{db, util, Document, Error},
    WorkerData,
};

pub struct FullNameRule {}
impl Rule for FullNameRule {
    fn get_name(&self) -> &'static str {
        "full_name"
    }

    fn check(
        &self,
        document: &Document,
        path: &PathBuf,
        worker_data: &WorkerData,
    ) -> Result<RuleCheckResult, Error> {
        let mut matches: Vec<db::Match> = vec![];
        let text = document.full_text_l.as_str();

        for (id, re) in &worker_data.full_name_reg {
            if re.is_match(text) {
                matches.push(db::Match {
                    source_case: util::normalize_filename(path),
                    matched_case_id: id.to_owned(),
                    matched_value: worker_data
                        .data
                        .iter()
                        .find(|c| c.id == *id)
                        .unwrap()
                        .short_name
                        .to_owned(),
                    m_type: self.get_name().to_string(),
                })
            }
        }

        let matches = matches
            .into_iter()
            .unique_by(|x| x.matched_case_id)
            .collect_vec();

        Ok(RuleCheckResult {
            is_match: matches.len() > 0,
            matches,
            message: None,
        })
    }
}
