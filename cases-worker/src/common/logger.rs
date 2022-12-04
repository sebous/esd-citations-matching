use log::{info, warn};

pub fn rule_warning(rule_name: &str, message: &str, id: &str, value: &str) {
    warn!(
        "rule: \"{}\", message: \"{}\", case: \"{}\", value: \"{}\"",
        rule_name, message, id, value
    )
}

pub fn rule_info(rule_name: &str, message: &str, id: &str, value: &str) {
    info!(
        "rule: \"{}\", message: \"{}\", case: \"{}\", value: \"{}\"",
        rule_name, message, id, value
    )
}
