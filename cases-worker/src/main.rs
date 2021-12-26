use initialize::init;

mod initialize;
mod lib;
mod process;
mod rules;
// #[macro_use]
// extern crate lazy_static;

fn main() {
    init();
}

fn process() {
    // get db connection
    // get files
    // get rules
    // process each file
    process::process_doc();
}
