use regex::Regex;

lazy_static! {
    pub static ref C_CODE: Regex =
        Regex::new(r"(C[--]\d{1,4}[/\--]\d{1,2})([ \u202F\u00A0,.)]|$)").unwrap();
    pub static ref T_CODE: Regex =
        Regex::new(r"(T[--]\d{1,4}[/\--]\d{1,2})([ \u202F\u00A0,.)]|$)").unwrap();
}
