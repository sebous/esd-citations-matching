use std::path::PathBuf;

use ::regex::Regex;
use itertools::Itertools;

use crate::{
    lib::{
        db::{self, Code},
        logger, regex, util, Document, Error,
    },
    WorkerData,
};

use super::rules::{self, Rule};

pub struct NumCodeWithKey {}

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
                    util::find_keyword_in_radius(document, m.start(), m.end()).is_some()
                        && !util::check_if_t_code(document, m.start())
                }
                None => false,
            })
            .map(|c| format!("C-{}", util::normalize_code(&c[1])))
            .unique()
            .collect_vec();

        let mut matches = vec![];

        for code in &codes_found {
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
                    source_case: util::normalize_filename(path),
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

// impl Rule for NumCodeWithKey {
//     fn get_name(&self) -> &'static str {
//         "num_code_w_key"
//     }

//     fn check(
//         &self,
//         document: &Document,
//         path: &PathBuf,
//         data: &db::EsdCasesData,
//     ) -> Result<rules::RuleCheckResult, Error> {
//         let match_found = regex::CODE.is_match(&document.full_text);

//         if !match_found {
//             return Ok(rules::RuleCheckResult {
//                 is_match: false,
//                 message: None,
//                 matches: vec![],
//             });
//         }

//         let dvur_keyword_present = util::check_dvur_existence(document);

//         if dvur_keyword_present.is_none() {
//             return Ok(rules::RuleCheckResult {
//                 is_match: false,
//                 message: None,
//                 matches: vec![],
//             });
//         }

//         let cases = regex::CODE
//             .captures_iter(&document.full_text)
//             .filter(|c| !regex::T_CODE.is_match(&c[0]))
//             .map(|c| util::normalize_code(&c[1]))
//             .unique()
//             .map(|c| db::Match {
//                 source_case: util::normalize_filename(path),
//                 matched_case_table: None,
//                 matched_case_id: None,
//                 matched_value: Some(c.to_owned()),
//                 m_type: self.get_name().to_string(),
//             })
//             .collect_vec();

//         Ok(rules::RuleCheckResult {
//             is_match: true,
//             matches: cases,
//             message: None,
//         })
//     }
// }
