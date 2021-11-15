use crate::day_28::sieve;

pub fn num_primorial(n: usize) -> u64 {
  let primes_100 = sieve::primes_up_to(100);
  primes_100[0..n].iter().fold(1_u64, |pm, prime| pm * prime)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic() {
    assert_eq!(num_primorial(3), 30);
    assert_eq!(num_primorial(4), 210);
    assert_eq!(num_primorial(5), 2310);
    assert_eq!(num_primorial(8), 9699690);
    assert_eq!(num_primorial(9), 223092870);
  }
}
