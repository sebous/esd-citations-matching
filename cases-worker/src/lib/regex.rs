use regex::Regex;

lazy_static! {
    /// C-XXX/XX
    pub static ref C_CODE: Regex =
        Regex::new(r"(C\s*[--]\s*\d{1,4}[/\--]\d{1,2})([ \u202F\u00A0,.)]|$)").unwrap();
    /// T-XXX/XX
    pub static ref T_CODE: Regex =
        Regex::new(r"(T\s*[--]\s*\d{1,4}[/\--]\d{1,2})([ \u202F\u00A0,.)]|$)").unwrap();
    /// num only XXX/XX
    pub static ref CODE: Regex =
        Regex::new(r"[\s\u202F\u00A0](\d{1,4}[/\--]\d{1,2})([\s\u202F\u00A0,.)]|$)").unwrap();
}
