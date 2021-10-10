/**
An anagram is a rearrangement of letters to form a new word. Given a word and a list of candidates, select the sublist of anagrams of the given word.
Given "listen" and a list of candidates like "enlists" "google" "inlets" "banana" the program should return a list containing "inlets".
The solution is case insensitive, which means "WOrd" is the same as "word" or "woRd".
It may help to take a peek at the std library for functions that can convert between them.
The solution cannot contain the input word. A word is always an anagram of itself, which means it is not an interesting result. Given "hello" and the list ["hello", "olleh"] the answer is ["olleh"].
*/
use std::collections::HashSet;

#[allow(dead_code)]
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagram_hashset = HashSet::new();
    let mut word_vect: Vec<char> = word.to_lowercase().chars().collect();
    word_vect.sort_unstable();

    for anagram in possible_anagrams {
        if word.len() == anagram.len() && word.to_lowercase() != *anagram.to_lowercase() {
            let mut anagram_vect: Vec<char> = anagram.to_lowercase().chars().collect();
            anagram_vect.sort_unstable();
            if anagram_vect == word_vect {
                anagram_hashset.insert(*anagram);
            }
        }
    }
    anagram_hashset
}

#[cfg(test)]
mod tests {
    use super::anagrams_for;
    use std::collections::HashSet;

    fn process_anagram_case(word: &str, inputs: &[&str], expected: &[&str]) {
        let result = anagrams_for(word, inputs);
        let expected: HashSet<&str> = expected.iter().cloned().collect();
        assert_eq!(result, expected);
    }
    #[test]
    fn test_no_matches() {
        let word = "diaper";
        let inputs = ["hello", "world", "zombies", "pants"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_detect_simple_anagram() {
        let word = "ant";
        let inputs = ["tan", "stand", "at"];
        let outputs = vec!["tan"];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_does_not_confuse_different_duplicates() {
        let word = "galea";
        let inputs = ["eagle"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_eliminate_anagram_subsets() {
        let word = "good";
        let inputs = ["dog", "goody"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_detect_anagram() {
        let word = "listen";
        let inputs = ["enlists", "google", "inlets", "banana"];
        let outputs = vec!["inlets"];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_multiple_anagrams() {
        let word = "allergy";
        let inputs = [
            "gallery",
            "ballerina",
            "regally",
            "clergy",
            "largely",
            "leading",
        ];
        let outputs = vec!["gallery", "regally", "largely"];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_case_insensitive_anagrams() {
        let word = "Orchestra";
        let inputs = ["cashregister", "Carthorse", "radishes"];
        let outputs = vec!["Carthorse"];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_unicode_anagrams() {
        let word = "ΑΒΓ";
        // These words don't make sense, they're just greek letters cobbled together.
        let inputs = ["ΒΓΑ", "ΒΓΔ", "γβα"];
        let outputs = vec!["ΒΓΑ", "γβα"];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_misleading_unicode_anagrams() {
        // Despite what a human might think these words different letters, the input uses Greek A and B
        // while the list of potential anagrams uses Latin A and B.
        let word = "ΑΒΓ";
        let inputs = ["ABΓ"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_does_not_detect_a_word_as_its_own_anagram() {
        let word = "banana";
        let inputs = ["banana"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_does_not_detect_a_differently_cased_word_as_its_own_anagram() {
        let word = "banana";
        let inputs = ["bAnana"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_does_not_detect_a_differently_cased_unicode_word_as_its_own_anagram() {
        let word = "ΑΒΓ";
        let inputs = ["ΑΒγ"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_same_bytes_different_chars() {
        let word = "a⬂"; // 61 E2 AC 82
        let inputs = ["€a"]; // E2 82 AC 61
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }
    #[test]
    fn test_different_words_but_same_ascii_sum() {
        let word = "bc";
        let inputs = ["ad"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }
}
