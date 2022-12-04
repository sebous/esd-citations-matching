use serde::Serialize;

use crate::{
    common::{Document, Error},
    rules::{EcliCodeRule, FullCodeRule, NumCodeWithKey},
    WorkerData,
};

#[derive(Debug, Serialize)]
pub struct Match {
    pub source_case_id: String,
    pub matched_case_code: String,
    pub m_type: String,
    pub match_context: Option<String>,
}

pub struct RuleCheckResult {
    pub message: Option<String>,
    pub is_match: bool,
    pub matches: Vec<Match>,
}
pub trait Rule {
    fn check(
        &self,
        document: &Document,
        worker_data: &WorkerData,
    ) -> Result<RuleCheckResult, Error>;
    fn get_name(&self) -> &'static str;
}

pub type BoxedRule = Box<dyn Rule + Send + Sync>;

pub fn get_rules() -> Vec<BoxedRule> {
    // rules have to be ordered by speed
    vec![
        Box::new(EcliCodeRule {}),
        Box::new(FullCodeRule {}),
        // Box::new(ShortNameRule {}),
        Box::new(NumCodeWithKey {}),
    ]
}
