#![feature(test)]
use std::error::Error;

use indicatif::ParallelProgressIterator;
use initialize::init;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rules::{get_rules, BoxedRule, Match};
use rusqlite::Result;

use crate::common::{csv_writer, progress_bar};

mod common;
mod csv_source;
mod initialize;
mod process;
mod rules;

#[macro_use]
extern crate lazy_static;

fn main() {
    init();
    process().unwrap();
}

pub struct WorkerData {
    pub rules: Vec<BoxedRule>,
    pub source_data: Vec<csv_source::EsdCase>,
}

fn process() -> Result<(), Box<dyn Error>> {
    // get rules
    let rules = get_rules();

    // get esd data from csv_source
    let data = csv_source::load_target_cases()?;

    let worker_data = WorkerData {
        rules,
        source_data: data,
    };

    let us_data = csv_source::load_us_cases()?;

    // setup progress bar
    let pb = progress_bar::init_progress_bar(us_data.len());

    let matches = us_data
        .par_iter()
        .progress_with(pb)
        .map(|entry| process::process_doc(entry, &worker_data))
        .filter_map(|r| r.ok())
        .flatten()
        .collect::<Vec<Match>>();

    println!("complete, matches: {}", matches.len());
    csv_writer::write_matches_to_file(&matches)?;

    Ok(())
}
