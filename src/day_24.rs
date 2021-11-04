/// Determine whether a sentence is a pangram.
#[allow(unused)]
pub mod pangram {
    use std::collections::HashSet;
    pub fn is_pangram(sentence: &str) -> bool {
        sentence
            .to_lowercase()
            .chars()
            .filter(|c| c.is_ascii() && c.is_alphabetic())
            .collect::<HashSet<char>>()
            .len()
            == 26
    }
}
