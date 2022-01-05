use std::{
    fs::{self},
    path::PathBuf,
};

use itertools::Itertools;
use log::{error, info};
use rusqlite::Connection;

use crate::{
    lib::{
        db::{save_match, EsdCasesData, Match},
        document::Document,
        error::Error,
    },
    rules::rules::Rule,
};

pub fn process_doc(
    path: &PathBuf,
    rules: &Vec<impl Rule>,
    data: &EsdCasesData,
    db_conn: &Connection,
) -> Result<(), Error> {
    // println!("{}", path.display());

    let file_content =
        fs::read_to_string(path).expect(format!("error reading file {}", path.display()).as_str());
    let words = file_content
        .lines()
        .flat_map(|l| {
            l.split_whitespace()
                .map(|spl| spl.to_string())
                .collect_vec()
        })
        .collect_vec();

    let document = Document {
        full_text: file_content,
        words,
    };

    format!("{:?}", path.file_name().unwrap());

    for rule in rules {
        match rule.check(&document) {
            Ok(result) => {
                if result.is_match {
                    save_match(
                        Match {
                            source_case: format!("{:?}", path.file_name().unwrap()),
                            matched_case_id: result.case_id,
                            matched_case_table: result.case_table,
                        },
                        db_conn,
                    )
                    .unwrap();
                }
            }
            Err(error) => {
                error!("{}", error);
            }
        }
    }

    Ok(())
}
