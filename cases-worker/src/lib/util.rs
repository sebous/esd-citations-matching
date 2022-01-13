use std::path::PathBuf;

use super::Document;

const HYPHEN: char = '\u{2010}';
const NON_BREAK_HYPHEN: char = '\u{2011}';
const DASH: char = '-';
const SPACE: char = ' ';
const NON_BREAKABLE_SPACE: char = '\u{a0}';
const RETURN: char = '\n';
const RETURN_WIN: char = '\r';

lazy_static! {
    static ref DVUR_VARIANTS: Vec<&'static str> = vec!["dvůr", "dvora", "dvoře", "dvorem"];
}

pub fn normalize_code(code: &str) -> String {
    code.chars()
        .map(|ch| match ch {
            HYPHEN => DASH,
            NON_BREAK_HYPHEN => DASH,
            NON_BREAKABLE_SPACE => SPACE,
            RETURN => SPACE,
            RETURN_WIN => SPACE,
            _ => ch,
        })
        .collect::<String>()
        .replace(SPACE, "")
}

pub fn normalize_filename(path: &PathBuf) -> String {
    format!("{:?}", path.file_name().unwrap()).replace("\"", "")
}

pub fn check_dvur_existence(document: &Document) -> Option<String> {
    for word in &document.words {
        let found_variant = DVUR_VARIANTS
            .iter()
            .find(|&v| v == &word.to_lowercase().as_str());

        if found_variant.is_some() {
            return found_variant.map(|v| v.to_string());
        }
    }
    None
}
