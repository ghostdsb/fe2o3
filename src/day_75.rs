pub mod factorial_trailing_zero {
  pub fn solve(n: u64) -> u64 {
    let mut count = 0;
    let mut n = n;
    while n > 0 {
      n /= 5;
      count += n;
    }
    count
  }
}

#[cfg(test)]
mod tests {
  use super::factorial_trailing_zero::*;

  #[test]
  fn basic() {
    assert_eq!(solve(20), 4);
    assert_eq!(solve(100), 24);
    assert_eq!(solve(200), 49);
  }
}
