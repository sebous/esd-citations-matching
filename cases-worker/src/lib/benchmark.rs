use std::fs::read_to_string;

use regex::Regex;

extern crate test;

lazy_static! {
    pub static ref INPUT: String = read_to_string("test_data.txt").unwrap();
    pub static ref REG: Regex = Regex::new(r" Wightman a další ").unwrap();
    pub static ref REG_LOWER: Regex =
        Regex::new(format!(r" Wightman a další ").to_lowercase().as_str()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn contains(b: &mut Bencher) {
        let search = " Wightman a další ";
        b.iter(|| INPUT.contains(search))
    }
    #[bench]
    fn regex(b: &mut Bencher) {
        b.iter(|| REG.is_match(&INPUT))
    }

    #[bench]
    fn regex_lower(b: &mut Bencher) {
        let input = INPUT.to_lowercase();
        b.iter(|| REG_LOWER.is_match(input.as_str()))
    }

    #[bench]
    fn contains_bytes(b: &mut Bencher) {
        let input = INPUT.to_lowercase();
        let search = " Wightman a další ".to_lowercase();
        b.iter(|| {
            input
                .as_bytes()
                .windows(search.len())
                .position(|window| window == search.as_bytes())
        })
    }
}
