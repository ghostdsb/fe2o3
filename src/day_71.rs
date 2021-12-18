pub fn prime_factors(n: i64) -> String {
  let mut n = n;
  let mut prime_factorisation = String::from("");
  for i in 2.. {
    let mut factor_count = 0;
    while n % i == 0 {
      factor_count += 1;
      n /= i;
    }
    if factor_count > 0 {
      prime_factorisation.push_str(&make_factor_string(i, factor_count)[..]);
    }
    if n == 1 {
      break;
    }
  }
  prime_factorisation
}

fn make_factor_string(base: i64, power: i64) -> String {
  if power == 1 {
    format!("({})", base)
  } else {
    format!("({}**{})", base, power)
  }
}

#[cfg(test)]
mod test {

  use super::*;

  fn testing(n: i64, exp: &str) -> () {
    assert_eq!(&prime_factors(n), exp)
  }

  #[test]
  fn basics_prime_factors() {
    testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
    testing(17 * 17 * 93 * 677, "(3)(17**2)(31)(677)");
  }
}
