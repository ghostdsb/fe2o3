/**Instructions
Determine if a word or phrase is an isogram.
An isogram (also known as a "nonpattern word") is a word or phrase without a repeating letter, however spaces and hyphens are allowed to appear multiple times.

Examples of isograms:
-lumberjacks
-background
-downstream
-six-year-old

The word isograms, however, is not an isogram, because the s repeats.
*/
use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    candidate
        .chars()
        .filter(|x| x.is_alphanumeric())
        .map(|x| x.to_lowercase().to_string())
        .fold((HashMap::new(), true), |(mut map, iso), x| {
            match map.insert(x, 1) {
                Some(_val) => (map, iso & false),
                None => (map, iso & true),
            }
        })
        .1
}

#[cfg(test)]
mod test {

    use super::check;
    #[test]
    fn empty_string() {
        assert!(check(""), "An empty string should be an isogram.")
    }
    #[test]
    fn only_lower_case_characters() {
        assert!(check("isogram"), "\"isogram\" should be an isogram.")
    }
    #[test]
    fn one_duplicated_character() {
        assert!(
            !check("eleven"),
            "\"eleven\" has more than one \'e\', therefore it is no isogram."
        )
    }
    #[test]
    fn longest_reported_english_isogram() {
        assert!(
            check("subdermatoglyphic"),
            "\"subdermatoglyphic\" should be an isogram."
        )
    }
    #[test]
    fn one_duplicated_character_mixed_case() {
        assert!(
            !check("Alphabet"),
            "\"Alphabet\" has more than one \'a\', therefore it is no isogram."
        )
    }
    #[test]
    fn hypothetical_isogramic_word_with_hyphen() {
        assert!(
            check("thumbscrew-japingly"),
            "\"thumbscrew-japingly\" should be an isogram."
        )
    }
    #[test]
    fn isogram_with_duplicated_hyphen() {
        assert!(
            check("six-year-old"),
            "\"six-year-old\" should be an isogram."
        )
    }
    #[test]
    fn made_up_name_that_is_an_isogram() {
        assert!(
            check("Emily Jung Schwartzkopf"),
            "\"Emily Jung Schwartzkopf\" should be an isogram."
        )
    }
    #[test]
    fn duplicated_character_in_the_middle() {
        assert!(
            !check("accentor"),
            "\"accentor\" has more than one \'c\', therefore it is no isogram."
        )
    }
}
