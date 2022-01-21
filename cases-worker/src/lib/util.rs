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
    static ref KEYWORD_VARIANTS: Vec<&'static str> = vec!["dvůr", "dvora", "dvoře", "dvorem"];
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

const KEYWORD_SEARCH_RADIUS: usize = 500;

pub fn find_keyword_in_radius(
    document: &Document,
    start: usize,
    end: usize,
) -> Option<(String, String)> {
    let text_l = document.full_text.len();
    let start = if start > KEYWORD_SEARCH_RADIUS {
        start - KEYWORD_SEARCH_RADIUS
    } else {
        0
    };
    let end = if end + KEYWORD_SEARCH_RADIUS < text_l {
        end + KEYWORD_SEARCH_RADIUS
    } else {
        text_l
    };

    let str_rad = &document
        .full_text
        .chars()
        .skip(start)
        .take(end - start)
        .collect::<String>();

    let found_keyword = KEYWORD_VARIANTS.iter().find(|&key| str_rad.contains(key));
    // if found_keyword.is_some() {
    //     println!("{}\n{}\n------", str_rad, found_keyword.unwrap());
    // }

    found_keyword.and_then(|&k| Some((k.to_string(), str_rad.to_owned())))
}

pub fn check_if_t_code(document: &Document, start: usize) -> bool {
    if start < 2 {
        return false;
    }
    if let Some(ch) = document.full_text.chars().nth(start - 2) {
        return ch == 't';
    }
    false
}
