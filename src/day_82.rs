#[allow(unused)]
fn solequa(n: u64) -> Vec<(u64, u64)> {
  let mut vec: Vec<(u64, u64)> = vec![];
  let limit = (n as f64).sqrt().floor() as u64 + 1;
  for i in 1..limit {
    if n % i == 0 {
      let a = i;
      let b = n / a;
      if (b - a) % 4 == 0 {
        vec.push(((a + b) / 2, (b - a) / 4));
      }
    }
  }
  vec.sort_by(|a, b| b.cmp(a));
  vec
}

#[cfg(test)]
mod tests {
  use super::*;

  fn testing(n: u64, exp: Vec<(u64, u64)>) -> () {
    assert_eq!(solequa(n), exp)
  }

  #[test]
  fn basics_solequa() {
    testing(5, vec![(3, 1)]);
    testing(20, vec![(6, 2)]);
    testing(9001, vec![(4501, 2250)]);
    testing(9004, vec![(2252, 1125)]);
  }
}
