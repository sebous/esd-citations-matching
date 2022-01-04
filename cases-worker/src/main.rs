use std::fs;

use initialize::init;
use lib::{db::fetch_data, error::Error};
use rules::rules::get_rules;
use rusqlite::{Connection, Result};

mod initialize;
mod lib;
mod process;
mod rules;
// #[macro_use]
// extern crate lazy_static;

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

    let total_count = fs::read_dir(SOURCE_DATA_DIR).unwrap().count();

    // process each file
    for (i, path) in fs::read_dir(SOURCE_DATA_DIR).unwrap().enumerate() {
        let pathbuf = path.unwrap().path();
        process::process_doc(&pathbuf, &rules, &data, &db_conn)?;
    }

    Ok(())
}
