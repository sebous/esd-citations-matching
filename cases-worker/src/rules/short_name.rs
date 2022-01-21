use itertools::Itertools;
use regex::Regex;

use super::Rule;
use super::RuleCheckResult;
use crate::lib::{db, util, Document, Error};
use crate::WorkerData;

pub struct ShortNameRule {}
impl Rule for ShortNameRule {
    fn get_name(&self) -> &'static str {
        "short_name"
    }

    fn check(
        &self,
        document: &Document,
        path: &std::path::PathBuf,
        worker_data: &WorkerData,
    ) -> Result<RuleCheckResult, Error> {
        let mut matches: Vec<db::Match> = vec![];

        for (id, re) in &worker_data.short_name_re {
            if re.is_match(document.full_text_l.as_str()) {
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
                });
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
