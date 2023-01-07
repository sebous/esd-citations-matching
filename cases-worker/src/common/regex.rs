use regex::Regex;

lazy_static! {
    // u2010 -> u002D - various "dash" characters

    /// C-XXX/XX
    pub static ref C_CODE: Regex =
        Regex::new(r"([CTF]\s*[\u2010\u2011\u2012\u2013\u2014\u2212\u002D]\s*\d{1,4}[/\u2010\u2011\u2012\u2013\u2014\u2212\u002D]\d{2})([ \u202F\u00A0,.)]|$)").unwrap();
    /// T-XXX/XX
    pub static ref T_CODE: Regex =
        Regex::new(r"(T\s*[\u2010\u2011\u2012\u2013\u2014\u2212\u002D]\s*\d{1,4}[/\u2010\u2011\u2012\u2013\u2014\u2212\u002D]\d{2})([ \u202F\u00A0,.)]|$)").unwrap();
    /// num only XXX/XX
    pub static ref CODE: Regex =
        Regex::new(r"[\s\u202F\u00A0](\d{1,4}[/\u2010\u2011\u2012\u2013\u2014\u2212\u002D]\d{2})([\s\u202F\u00A0,.)]|$)").unwrap();
    /// ECLI:EU:C:1965:73
    pub static ref ECLI_CODE: Regex = Regex::new(r"ecli:eu:c:\d{4}:\d{1,3}").unwrap();

}
