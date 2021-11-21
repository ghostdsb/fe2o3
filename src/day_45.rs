/**
 * Persistent Bugger.
 * Write a function, persistence, that takes in a positive parameter num and returns its multiplicative persistence, which is the number of times you must multiply the digits in num until you reach a single digit.
 * For example:
 * persistence(39) // returns 3, because 3*9=27, 2*7=14, 1*4=4
 * // and 4 has only one digit
 *
 * persistence(999) // returns 4, because 9*9*9=729, 7*2*9=126,
 * // 1*2*6=12, and finally 1*2=2
 *
 * persistence(4) // returns 0, because 4 is already a one-digit number
 */

#[allow(unused)]
pub fn persistence(num: u64) -> u64 {
  let mut mp = 0;
  let mut num = num;
  loop {
    let (prod, count) = prod_n_digs(num);
    if count == 1 {
      break;
    }
    num = prod;
    mp += 1;
  }
  mp
}

fn prod_n_digs(num: u64) -> (u64, u64) {
  num.to_string().chars().fold((1, 0), |(p, count), c| {
    (p * c.to_digit(10).unwrap() as u64, count + 1)
  })
}

#[cfg(test)]
mod test {

  use super::*;
  #[test]
  fn persistence_tests() {
    assert_eq!(persistence(39), 3);
    assert_eq!(persistence(4), 0);
    assert_eq!(persistence(25), 2);
    assert_eq!(persistence(999), 4);
  }

  #[test]
  fn prod_digs_test() {
    assert_eq!(prod_n_digs(39), (27, 2));
    assert_eq!(prod_n_digs(4), (4, 1));
    assert_eq!(prod_n_digs(100), (0, 3));
    assert_eq!(prod_n_digs(111), (1, 3));
    assert_eq!(prod_n_digs(999), (729, 3));
  }
}
