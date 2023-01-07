use crate::{
    common::{self, logger, util, Document, Error},
    WorkerData,
};
use itertools::Itertools;
use regex::Regex;
use unidecode::unidecode;

use super::rules::{self, Rule};

pub struct NumCodeWithKey {}

impl Rule for NumCodeWithKey {
    fn get_name(&self) -> &'static str {
        "num_code_w_keyword"
    }

    fn check(
        &self,
        document: &Document,
        worker_data: &WorkerData,
    ) -> Result<super::RuleCheckResult, Error> {
        let match_found = common::regex::CODE.is_match(&document.full_text);

        if !match_found {
            return Ok(rules::RuleCheckResult {
                is_match: false,
                message: None,
                matches: vec![],
            });
        }

        let codes_found = common::regex::CODE
            .captures_iter(&document.full_text)
            .filter_map(|c| {
                c.get(1).and_then(|m| {
                    if !check_forbidden_subrules_ok(&m, document) {
                        return None;
                    }
                    check_keywords(&m, document).map(|str_ctx| {
                        (
                            format!("C-{}", util::normalize_code(&m.as_str().to_owned())),
                            str_ctx,
                        )
                    })
                })
            })
            .unique_by(|(str, _)| str.to_owned())
            .collect_vec();

        let mut matches = vec![];

        for (code, radius) in &codes_found {
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
                Some(case) => matches.push(rules::Match {
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

lazy_static! {
    static ref FORBIDDEN_KWS: Vec<&'static str> = vec!["smernic", "stiznost"];
    static ref FORBIDDEN_DISTANT_KWS: Vec<&'static str> =
        vec!["spolkovy", "spolkoveho", "narizeni"];
    static ref KEYWORDS: Vec<Regex> =
        vec!["dvůr", "dvora", "dvoře", "dvorem", "dvoru", "SDEU", "ESD"]
            .iter()
            .map(|str| Regex::new(&format!(
                r"[^a-zA-Z\d]{}[^a-zA-Z\d]",
                unidecode(str).to_lowercase()
            ))
            .unwrap())
            .collect();
}
fn check_forbidden_subrules_ok(m: &regex::Match, document: &Document) -> bool {
    let text_l = document.full_text_l.len();
    let sbnu_found = document.full_text_l[m.end()..if (m.end() + 10) > text_l {
        text_l
    } else {
        m.end() + 10
    }]
        .trim()
        .starts_with("sbnu");

    let us_found = document.full_text_l[if m.start() < 10 { 0 } else { m.start() - 10 }..m.start()]
        .trim()
        .ends_with("us");

    let view = &document.full_text_l[if m.start() < 15 { 0 } else { m.start() - 15 }..m.start()];
    let forbidden_kw_found = FORBIDDEN_KWS.iter().any(|key| view.contains(key));

    let view = &document.full_text_l[if m.start() < 100 { 0 } else { m.start() - 100 }..m.start()];
    let forbidden_dist_kws_found = FORBIDDEN_DISTANT_KWS.iter().any(|key| view.contains(key));

    !sbnu_found && !us_found && !forbidden_kw_found && !forbidden_dist_kws_found
}

fn check_keywords(m: &regex::Match, document: &Document) -> Option<String> {
    let view = util::extract_match_context(m, document);
    KEYWORDS
        .iter()
        .any(|re| re.is_match(view))
        .then_some(view.to_owned())
}
