#![feature(test)]
use std::fs;

use ::regex::Regex;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use initialize::init;
use lib::{db, regex, Error};
use rules::get_rules;
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

pub struct WorkerData {
    db_conn: Connection,
    rules: Vec<Box<dyn rules::Rule>>,
    data: Vec<db::EsdCase>,
    short_name_re: Vec<(usize, Regex)>,
}

const SOURCE_DATA_DIR: &str = "../source_data";

fn process() -> Result<(), Error> {
    // get db connection
    let db_conn = Connection::open("../db/db.sqlite").unwrap();
    // get rules
    let rules = get_rules();
    // get db data
    let data = db::fetch_data(&db_conn).unwrap();
    // generate regexes from short_names
    let regexes = regex::generate_short_name_regexes(&data);

    let worker_data = WorkerData {
        db_conn,
        rules,
        data,
        short_name_re: regexes,
    };

    // setup progress bar
    let total_count = fs::read_dir(SOURCE_DATA_DIR).unwrap().count();
    let pb = ProgressBar::new(total_count as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}")
            .progress_chars("#>"),
    );

    // clear matches table
    db::clear_matches(&worker_data.db_conn).unwrap();

    // process each file
    for path in fs::read_dir(SOURCE_DATA_DIR).unwrap().progress_with(pb)
    // .take(1)
    {
        let pathbuf = path.unwrap().path();
        process::process_doc(&pathbuf, &worker_data)?;
    }

    Ok(())
}
