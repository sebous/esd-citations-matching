#![feature(test)]
use std::fs;

use ::regex::Regex;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use initialize::init;
use itertools::Itertools;
use lib::{db, regex};
use log::info;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rules::{get_rules, BoxedRule};
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
    rules: Vec<BoxedRule>,
    data: Vec<db::EsdCase>,
    source_data: Vec<db::SourceCase>,
    short_name_reg: Vec<(usize, Regex)>,
    full_name_reg: Vec<(usize, Regex)>,
}

const SOURCE_DATA_DIR: &str = "../source_data";

fn process() -> Result<()> {
    // get db connection
    let db_conn = Connection::open("../db/db.sqlite").unwrap();
    // get rules
    let rules = get_rules();
    // get db data
    let (data, source_data) = db::fetch_data(&db_conn).unwrap();
    // generate regexes from short_names
    let short_reg = regex::gen_shname_regx(&data);
    let full_reg = regex::gen_fname_regx(&data);

    let worker_data = WorkerData {
        rules,
        data,
        source_data,
        short_name_reg: short_reg,
        full_name_reg: full_reg,
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
    db::clear_matches(&db_conn).unwrap();

    let paths = fs::read_dir(SOURCE_DATA_DIR)
        .unwrap()
        .filter_map(|r| r.ok())
        .collect_vec();

    let matches = paths
        // .iter()
        // .take(1000)
        .par_iter()
        // .par_bridge()
        .progress_with(pb)
        .map(|entry| {
            let path = entry.path();
            process::process_doc(&path, &worker_data)
        })
        .filter_map(|r| r.ok())
        .flatten()
        .collect::<Vec<db::Match>>();

    // save matches to db
    info!("saving {} matches to db..", matches.len());

    db::save_matches(&matches, db_conn)?;

    Ok(())
}
