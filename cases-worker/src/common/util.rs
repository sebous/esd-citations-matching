use std::path::PathBuf;

use super::Document;
use unidecode::unidecode;

const HYPHEN: char = '\u{2010}';
const NON_BREAK_HYPHEN: char = '\u{2011}';
const DASH: char = '-';
const SPACE: char = ' ';
const NON_BREAKABLE_SPACE: char = '\u{a0}';
const RETURN: char = '\n';
const RETURN_WIN: char = '\r';

lazy_static! {
    pub static ref KEYWORD_VARIANTS_DVUR: Vec<String> =
        vec!["dvůr", "dvora", "dvoře", "dvorem", "dvoru", "SDEU", "ESD", "unie", "eu"]
            .iter()
            .map(|str| unidecode(str).to_lowercase())
            .collect();
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

pub fn find_keyword_in_radius(
    document: &Document,
    start: usize,
    end: usize,
    radius: usize,
    keywords: Vec<String>,
) -> Option<(String, String)> {
    let text_l = document.full_text.len();
    let start = if start > radius { start - radius } else { 0 };
    let end = if end + radius < text_l {
        end + radius
    } else {
        text_l
    };

    let str_rad = &document
        .full_text_l
        .chars()
        .skip(start)
        .take(end - start)
        .collect::<String>();

    let found_keyword = keywords.iter().find(|&key| str_rad.contains(key));

    found_keyword.and_then(|k| Some((k.to_string(), str_rad.to_owned())))
}

pub fn check_if_t_code(document: &Document, start: usize) -> bool {
    if start < 2 {
        return false;
    }
    if let Some(ch) = document.full_text_l.chars().nth(start - 2) {
        return ch == 't';
    }
    false
}
