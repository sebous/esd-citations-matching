use std::fs::read_to_string;

use regex::Regex;

extern crate test;

lazy_static! {
    pub static ref INPUT: String = read_to_string("test_data.txt").unwrap();
    pub static ref REG: Regex = Regex::new(r"Wightman a další").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn contains(b: &mut Bencher) {
        b.iter(|| INPUT.contains("Wightman a další"))
    }
    #[bench]
    fn regex(b: &mut Bencher) {
        b.iter(|| Regex::new(r"Wightman a další").unwrap().is_match(&INPUT))
    }
}
