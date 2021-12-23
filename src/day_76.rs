/*
Your task is to write the word to number converter. Digits in the number should match letters in the word. Plus generated number should be the smallest possible number you can get.

Words will contain of maximum 10 distinct letters, but word can be any length, even longer than 10 characters long.
Number can NOT start with 0
Same letters share the same digit regardless of case
For empty string return 0

Examples:
"A" -> 1 - OK

"ABA" -> 353 - WRONG ( number is OK, but it's not the smallest number )

"ABA" -> 333 - WRONG ( different letters map to same digits )

"ABA" -> 357 - WRONG ( same letters map to different digits )
*/
use std::collections::HashMap;
pub fn convert(word: &str) -> u64 {
  let mut store: HashMap<char, i64> = HashMap::new();
  let mut number = 0_i64;
  let mut last_max = -1;
  for chr in word.to_lowercase().chars() {
    match store.get_mut(&chr) {
      Some(idx) => {
        number = number * 10 + *idx;
      }
      None => {
        last_max = match last_max {
          -1 => 1,
          1 => 0,
          0 => 2,
          val => val + 1,
        };
        store.insert(chr, last_max);
        number = number * 10 + last_max;
      }
    }
  }
  println!("{}", number);
  number as u64
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test0() {
    let expected = 10234567;
    let s = "CodeWars";

    assert_eq!(expected, convert(s));
  }

  #[test]
  fn test1() {
    let expected = 1020;
    let s = "KATA";

    assert_eq!(expected, convert(s));
  }
}
