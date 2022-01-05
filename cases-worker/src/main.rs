use std::fs;

use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use initialize::init;
use lib::{db::fetch_data, error::Error};
use rules::rules::get_rules;
use rusqlite::{Connection, Result};

mod initialize;
mod lib;
mod process;
mod rules;

#[macro_use]
extern crate lazy_static;

fn main() {
    init();
    process().unwrap();
}

const SOURCE_DATA_DIR: &str = "../source_data";

fn process() -> Result<(), Error> {
    // get db connection
    let db_conn = Connection::open("../db/db.sqlite").unwrap();
    // get rules
    let rules = get_rules();
    // get db data
    // TODO: handle db error
    let data = fetch_data(&db_conn).unwrap();

    // setup progress bar
    let total_count = fs::read_dir(SOURCE_DATA_DIR).unwrap().count();
    let pb = ProgressBar::new(total_count as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}")
            .progress_chars("#-"),
    );

    // process each file
    for path in fs::read_dir(SOURCE_DATA_DIR).unwrap().progress_with(pb)
    // .take(1000)
    {
        let pathbuf = path.unwrap().path();
        process::process_doc(&pathbuf, &rules, &data, &db_conn)?;
    }

    Ok(())
}
