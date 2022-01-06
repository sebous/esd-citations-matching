use std::path::PathBuf;

use log::{info, warn};

use super::util;

pub fn rule_warning(rule_name: &str, message: &str, file_path: &PathBuf, value: &str) {
    let filename = util::normalize_filename(file_path);
    warn!(
        "rule: '{}', message: '{}', file: '{}', value: '{}'",
        rule_name, message, filename, value
    )
}

pub fn rule_info(rule_name: &str, message: &str, file_path: &PathBuf, value: &str) {
    let filename = util::normalize_filename(file_path);
    info!(
        "rule: '{}', message: '{}', file: '{}', value: '{}'",
        rule_name, message, filename, value
    )
}
