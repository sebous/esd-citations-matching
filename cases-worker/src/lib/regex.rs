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

    pub static ref VS: Regex = Regex::new(r"\s(v.)\s").unwrap();
}

pub fn gen_shname_regx(data: &Vec<db::EsdCase>) -> Vec<(usize, Regex)> {
    data.iter()
        .filter(|case| case.short_name.len() > 5)
        .flat_map(|case| {
            let str = unidecode::unidecode(&case.short_name.to_lowercase());
            let mut res = vec![];
            res.push((case.id, Regex::new(format!(r" {} ", str).as_str()).unwrap()));

            if str.contains(" v. ") {
                res.push((
                    case.id,
                    Regex::new(format!(r" {} ", str.replace(" v. ", " proti ")).as_str()).unwrap(),
                ));
            }
            res
        })
        .collect_vec()
}

pub fn gen_fname_regx(data: &Vec<db::EsdCase>) -> Vec<(usize, Regex)> {
    data.iter()
        .filter(|case| case.full_name.is_some() && case.full_name.as_ref().unwrap().len() > 5)
        .flat_map(|case| {
            let full_name = case.full_name.clone().unwrap();
            let str = unidecode::unidecode(&full_name.to_lowercase());
            let mut res = vec![];
            res.push((
                case.id,
                Regex::new(regex::escape(format!(r" {} ", str).as_str()).as_str()).unwrap(),
            ));

            if str.contains(" v. ") {
                res.push((
                    case.id,
                    Regex::new(
                        regex::escape(format!(r" {} ", str.replace(" v. ", " proti ")).as_str())
                            .as_str(),
                    )
                    .unwrap(),
                ));
            }
            res
        })
        .collect_vec()
}
