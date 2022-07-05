use itertools::Itertools;

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

        for (id, re) in &worker_data.short_name_reg {
            if re.is_match(&document.full_text_l.as_str()) {
                let matched_case = worker_data.data.iter().find(|c| c.id == *id).unwrap();

                let date = matched_case.date.clone();
                if date.is_none() {
                    continue;
                }
                let matched_year = date.unwrap()[..4].to_string();

                let matched_short_name = re
                    .captures_iter(&document.full_text_l)
                    .filter(|c| match &c.get(0) {
                        Some(m) => util::find_keyword_in_radius(
                            &document,
                            m.start(),
                            m.end(),
                            75,
                            vec![matched_year.to_string()],
                        )
                        .is_some(),
                        None => false,
                    })
                    .map(|c| c[0].to_string())
                    .unique()
                    .next();

                if matched_short_name.is_none() {
                    continue;
                }

                matches.push(db::Match {
                    source_case_id: worker_data
                        .source_data
                        .iter()
                        .find(|x| x.file_name == util::normalize_filename(path))
                        .unwrap()
                        .id,
                    matched_case_id: id.to_owned(),
                    matched_value: matched_case.short_name.to_owned().unwrap(),
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
