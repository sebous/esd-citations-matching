use std::path::PathBuf;

use itertools::Itertools;

use crate::{
    lib::{
        db::{self, Code},
        logger, regex,
        util::{self, KEYWORD_VARIANTS_DVUR},
        Document, Error,
    },
    WorkerData,
};

use super::rules::{self, Rule};

pub struct NumCodeWithKey {}

fn find_if_year_in_range(code: &str) -> bool {
    let code = code.to_owned();
    let year_str = code[code.len() - 2..].to_string();
    match year_str.parse::<u32>() {
        Ok(n) => {
            if n <= 88 && n >= 50 {
                return true;
            }
            return false;
        }
        Err(_) => return false,
    }
}

impl Rule for NumCodeWithKey {
    fn get_name(&self) -> &'static str {
        "num_code_w_key"
    }

    fn check(
        &self,
        document: &Document,
        path: &PathBuf,
        worker_data: &WorkerData,
    ) -> Result<super::RuleCheckResult, Error> {
        let match_found = regex::CODE.is_match(&document.full_text);

        if !match_found {
            return Ok(rules::RuleCheckResult {
                is_match: false,
                message: None,
                matches: vec![],
            });
        }

        // logger::rule_info(rule_name, message, file_path, value)
        let codes_found = regex::CODE
            .captures_iter(&document.full_text)
            .filter(|c| match &c.get(1) {
                Some(m) => {
                    util::find_keyword_in_radius(
                        document,
                        m.start(),
                        m.end(),
                        500,
                        KEYWORD_VARIANTS_DVUR.to_vec(),
                    )
                    .is_some()
                        && !util::check_if_t_code(document, m.start())
                }
                None => false,
            })
            .map(|c| format!("C-{}", util::normalize_code(&c[1])))
            .unique()
            .collect_vec();

        let mut matches = vec![];

        for code in &codes_found {
            if !find_if_year_in_range(&code) {
                continue;
            }
            let found_case = worker_data
                .data
                .iter()
                .find(|case| case.get_codes().contains(code));
            match found_case {
                None => logger::rule_warning(
                    self.get_name(),
                    "found matching code that doesn't exist in db",
                    path,
                    code,
                ),
                Some(case) => matches.push(db::Match {
                    source_case_id: worker_data
                        .source_data
                        .iter()
                        .find(|x| x.file_name == util::normalize_filename(path))
                        .unwrap()
                        .id,
                    matched_case_id: case.id,
                    matched_value: code.to_owned(),
                    m_type: self.get_name().to_string(),
                }),
            }
        }

        Ok(rules::RuleCheckResult {
            is_match: matches.len() > 0,
            matches,
            message: None,
        })
    }
}

#[test]
fn test() {
    for c in regex::CODE.captures_iter(" 123/22   aasdasd 222/23,") {
        let m = &c.get(1).unwrap();
        dbg!(m);
    }
}
