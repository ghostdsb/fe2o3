/*
1, 246, 2, 123, 3, 82, 6, 41 are the divisors of number 246.
Squaring these divisors we get: 1, 60516, 4, 15129, 9, 6724, 36, 1681. The sum of these squares is 84100 which is 290 * 290.

Task

Find all integers between m and n (m and n integers with 1 <= m <= n) such that the sum of their squared divisors is itself a square.
We will return an array of subarrays or of tuples (in C an array of Pair) or a string.
The subarrays (or tuples or Pairs) will have two elements:
- first the number the squared divisors of which is a square and then
- the sum of the squared divisors.

Example:
list_squared(1, 250) --> [[1, 1], [42, 2500], [246, 84100]]
list_squared(42, 250) --> [[42, 2500], [246, 84100]]
*/

pub fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
  let mut ls = vec![];
  for i in m..=n {
    let fct_sqr_sum = factors_square_sum(i);
    if is_square(fct_sqr_sum) {
      ls.push((i, fct_sqr_sum));
    }
  }
  ls
}

fn factors_square_sum(n: u64) -> u64 {
  let lim = (n as f64).sqrt().floor() as u64;
  (1..=lim)
    .filter(|num| n % num == 0)
    .fold(vec![], |mut fs, val| {
      fs.push(val * val);
      if val * val != n {
        fs.push((n * n) / (val * val));
      }
      fs
    })
    .iter()
    .sum()
}

fn is_square(n: u64) -> bool {
  let sqrt = (n as f64).sqrt().floor() as u64;
  sqrt * sqrt == n
}

#[cfg(test)]
mod test {
  use super::*;

  fn testing(m: u64, n: u64, exp: Vec<(u64, u64)>) -> () {
    assert_eq!(list_squared(m, n), exp)
  }

  #[test]
  fn basics_list_squared() {
    testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    testing(42, 250, vec![(42, 2500), (246, 84100)]);
    testing(300, 600, vec![]);
  }
}
