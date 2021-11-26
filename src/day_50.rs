#[allow(unused)]
pub mod word_count {
  use std::collections::HashMap;

  pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
      .trim()
      .split(|c: char| !(c.is_alphanumeric() || c == '\''))
      .filter(|e| !e.is_empty())
      .fold(HashMap::new(), |mut wc, word| {
        wc.entry(word.trim_matches('\'').to_lowercase())
          .and_modify(|frq| *frq += 1)
          .or_insert(1);
        wc
      })
  }
}
