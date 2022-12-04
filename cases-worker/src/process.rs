use itertools::Itertools;
use log::error;

use crate::{
    common::{Document, Error},
    csv_source::Case,
    rules::Match,
    WorkerData,
};

pub fn process_doc(entry: &Case, worker_data: &WorkerData) -> Result<Vec<Match>, Error> {
    let document = Document::create(entry);
    let mut matches = vec![];

    for rule in &worker_data.rules {
        match rule.check(&document, worker_data) {
            Ok(result) => {
                if result.is_match {
                    matches.extend(result.matches);
                    if !cfg!(feature = "allrules") {
                        return Ok(matches);
                    }
                }
            }
            Err(error) => {
                error!("{}", error);
            }
        }
    }

    let unique_matches = matches
        .into_iter()
        .unique_by(|m| m.matched_case_code.clone())
        .collect_vec();

    Ok(unique_matches)
}
