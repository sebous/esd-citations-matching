use std::{fs, path::PathBuf};

use itertools::Itertools;
use log::error;
use rusqlite::Connection;

use crate::{
    lib::{db, Document, Error},
    rules::Rule,
};

pub fn process_doc(
    path: &PathBuf,
    rules: &Vec<Box<dyn Rule>>,
    data: &Vec<db::EsdCase>,
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

    for rule in rules {
        match rule.check(&document, path, data) {
            Ok(result) => {
                if result.is_match {
                    for m in result.cases {
                        db::save_match(m, db_conn).unwrap();
                    }
                    return Ok(());
                }
            }
            Err(error) => {
                error!("{}", error);
            }
        }
    }

    Ok(())
}
