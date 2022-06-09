use std::{fs, path::PathBuf};

use log::error;

use crate::{
    lib::{db, Document, Error},
    WorkerData,
};

pub fn process_doc(path: &PathBuf, worker_data: &WorkerData) -> Result<Vec<db::Match>, Error> {
    let file_content =
        fs::read_to_string(path).expect(format!("error reading file {}", path.display()).as_str());

    let document = Document::create(&file_content);
    let mut matches = vec![];

    for rule in &worker_data.rules {
        match rule.check(&document, path, worker_data) {
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

    Ok(matches)
}
