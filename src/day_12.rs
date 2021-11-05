pub fn strong(n: u64) -> String {
  match n
    .to_string()
    .chars()
    .map(|c| factorial(c.to_digit(10).unwrap_or(0) as u64, 1))
    .sum::<u64>()
    == n
  {
    true => String::from("STRONG"),
    _ => String::from("NOT STRONG"),
  }
}

fn factorial(n: u64, fact: u64) -> u64 {
  if n == 0 {
    fact
  } else {
    factorial(n - 1, fact * n)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn strong145() {
    assert_eq!(strong(145), String::from("STRONG"))
  }
  #[test]
  fn strong1() {
    assert_eq!(strong(1), String::from("STRONG"))
  }

  #[test]
  fn weak185() {
    assert_eq!(strong(185), String::from("NOT STRONG"))
  }
}
