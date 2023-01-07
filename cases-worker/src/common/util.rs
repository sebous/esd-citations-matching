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

pub fn check_if_t_code(document: &Document, start: usize) -> bool {
    if start < 2 {
        return false;
    }
    if let Some(ch) = document.full_text_l.chars().nth(start - 2) {
        return ch == 't';
    }
    false
}

const RADIUS_KW: usize = 150;

pub fn extract_match_context<'a>(m: &regex::Match, document: &'a Document) -> &'a str {
    let text_l = document.full_text_l.len();
    let start = if m.start() > RADIUS_KW {
        m.start() - RADIUS_KW
    } else {
        0
    };
    let end = if m.end() + RADIUS_KW < text_l {
        m.end() + RADIUS_KW
    } else {
        text_l
    };
    &document.full_text_l[start..end]
}
