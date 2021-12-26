/*
You will be given a string of English digits "stuck" together, like this:

"zeronineoneoneeighttwoseventhreesixfourtwofive"

Your task is to split the string into separate digits:

"zero nine one one eight two seven three six four two five"
*/
pub fn uncollapse(digits: &str) -> String {
  let str_digits = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero",
  ];
  let mut ans = Vec::new();
  let mut buf = String::new();
  for s in digits.chars() {
    buf.push(s);
    if str_digits.contains(&&buf[..]) {
      ans.push(buf);
      buf = String::from("");
    }
  }
  println!("{:?}", ans);
  ans.join(" ")
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic() {
    assert_eq!(uncollapse("three"), "three".to_string());
    assert_eq!(uncollapse("eightsix"), "eight six".to_string());
    assert_eq!(uncollapse("fivefourseven"), "five four seven".to_string());
    assert_eq!(
      uncollapse("ninethreesixthree"),
      "nine three six three".to_string()
    );
    assert_eq!(
      uncollapse("foursixeighttwofive"),
      "four six eight two five".to_string()
    );
  }
}
