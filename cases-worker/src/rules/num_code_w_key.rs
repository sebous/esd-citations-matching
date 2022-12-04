use itertools::Itertools;
use rules::Match;

use crate::{
    common::{
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
        "num_code_w_keyword"
    }

    fn check(
        &self,
        document: &Document,
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

        let codes_found = regex::CODE
            .captures_iter(&document.full_text)
            .filter_map(|c| match &c.get(1) {
                Some(m) => util::find_keyword_in_radius(
                    document,
                    m.start(),
                    m.end(),
                    300,
                    KEYWORD_VARIANTS_DVUR.to_vec(),
                )
                .map(|(_, ctx)| {
                    (
                        format!("C-{}", util::normalize_code(&m.as_str().to_owned())),
                        ctx,
                    )
                }),
                None => None,
            })
            .unique_by(|(str, _)| str.to_owned())
            .collect_vec();

        let mut matches = vec![];

        for (code, radius) in &codes_found {
            if !find_if_year_in_range(&code) {
                continue;
            }
            let found_case = worker_data
                .source_data
                .iter()
                .find(|case| case.codes_list.contains(code));
            match found_case {
                None => logger::rule_warning(
                    self.get_name(),
                    "found matching code that doesn't exist in db",
                    &document.id,
                    code,
                ),
                Some(case) => matches.push(Match {
                    source_case_id: document.id.clone(),
                    matched_case_code: case.code.clone(),
                    m_type: self.get_name().to_string(),
                    match_context: Some(radius.to_owned()),
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
