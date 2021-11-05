/**
 *
Implement a function, multiples(m, n), which returns an array of the first m multiples of the real number n. Assume that m is a positive integer.
Ex.
-multiples(3, 5.0); should return
[5.0, 10.0, 15.0]
 */

#[allow(dead_code)]
fn multiples(m: i32, n: f64) -> Vec<f64> {
  (1..=m).map(|x| x as f64 * n).collect()
}

#[cfg(test)]
mod test {
  use super::multiples;

  #[test]
  fn three_multiples_of_positive_float() {
    assert_eq!(multiples(3, 4.0), vec![4.0, 8.0, 12.0]);
  }

  #[test]
  fn five_multiples_of_negative_float() {
    assert_eq!(multiples(5, -1.0), vec![-1.0, -2.0, -3.0, -4.0, -5.0]);
  }

  #[test]
  fn multiples_of_real_number() {
    assert_eq!(multiples(1, 3.14), vec![3.14]);
  }
}
