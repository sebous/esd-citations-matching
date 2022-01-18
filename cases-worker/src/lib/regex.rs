use itertools::Itertools;
use regex::Regex;

use super::db;

lazy_static! {
    /// C-XXX/XX
    pub static ref C_CODE: Regex =
        Regex::new(r"(C\s*[--]\s*\d{1,4}[/\--]\d{1,2})([ \u202F\u00A0,.)]|$)").unwrap();
    /// T-XXX/XX
    pub static ref T_CODE: Regex =
        Regex::new(r"(T\s*[--]\s*\d{1,4}[/\--]\d{1,2})([ \u202F\u00A0,.)]|$)").unwrap();
    /// num only XXX/XX
    pub static ref CODE: Regex =
        Regex::new(r"[\s\u202F\u00A0](\d{1,4}[/\--]\d{2})([\s\u202F\u00A0,.)]|$)").unwrap();
}

pub fn generate_short_name_regexes(data: &Vec<db::EsdCase>) -> Vec<(usize, Regex)> {
    data.iter()
        .filter(|case| case.short_name.len() > 5)
        .map(|case| {
            (
                case.id,
                Regex::new(format!(r"{}", &case.short_name).as_str()).unwrap(),
            )
        })
        .collect_vec()
}
