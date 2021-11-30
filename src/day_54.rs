/**
 * Buddy pairs
 * You know what divisors of a number are.
 * The divisors of a positive integer n are said to be proper when you consider only the divisors other than n itself.
 *
 * In the following description, divisors will mean proper divisors.
 *
 * For example for 100 they are 1, 2, 4, 5, 10, 20, 25, and 50.
 *
 * Let s(n) be the sum of these proper divisors of n.
 * Call buddy two positive integers such that the sum of the proper divisors of each number is one more than the other number:
 * (n, m) are a pair of buddy if s(m) = n + 1 and s(n) = m + 1
 * For example 48 & 75 is such a pair:
 * Divisors of 48 are: 1, 2, 3, 4, 6, 8, 12, 16, 24 --> sum: 76 = 75 + 1
 * Divisors of 75 are: 1, 3, 5, 15, 25 --> sum: 49 = 48 + 1
 *
 * Task
 * Given two positive integers start and limit, the function buddy(start, limit) should return the first pair (n m) of buddy pairs such that n (positive integer) is between start (inclusive) and limit (inclusive); m can be greater than limit and has to be greater than n
*/
pub fn buddy(start: i64, limit: i64) -> Option<(i64, i64)> {
  for num in start..=limit {
    match get_buddy(num) {
      Some(bud) if bud > num => return Some((num, bud)),
      _ => {}
    }
  }
  None
}

fn proper_divisors_sum(n: u64) -> u64 {
  let lim = (n as f64).sqrt().floor() as u64;
  (1..=lim)
    .filter(|num| n % num == 0)
    .fold(vec![], |mut fs, val| {
      fs.push(val);
      if val * val != n && val != 1 {
        fs.push(n / val);
      }
      fs
    })
    .iter()
    .sum()
}

fn get_buddy(n: i64) -> Option<i64> {
  let prpr_divisor_sum = proper_divisors_sum(n as u64);
  let prosp_divisor_sum = proper_divisors_sum(prpr_divisor_sum - 1);
  match prosp_divisor_sum as i64 - 1 == n {
    true => Some(prpr_divisor_sum as i64 - 1),
    false => None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn dotest(start: i64, limit: i64, exp: Option<(i64, i64)>) -> () {
    println!("start:{}", start);
    println!("limit:{}", limit);
    let ans = buddy(start, limit);
    println!("actual:\n{:?}", ans);
    println!("expect:\n{:?}", exp);
    println!("{}", ans == exp);
    assert_eq!(ans, exp);
    println!("{}", "-");
  }

  #[test]
  fn basic_tests() {
    dotest(10, 50, Some((48, 75)));
    dotest(1081180, 1103735, Some((1081184, 1331967)));
  }
}
