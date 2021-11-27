/**
 *
 * An integral:
 * ∫abf(x)dx
 *
 * Simpsone rule
 */

#[allow(unused)]
pub fn simpson(n: i32) -> f64 {
  let f = |x: f64| -> f64 { (3.0 / 2.0) * x.sin().powi(3) };
  let (a, b) = (0.0, std::f64::consts::PI);

  let h = (b - a) / n as f64;

  let sum4: f64 = (1..n / 2 + 1)
    .map(|i| f(a + (2.0 * i as f64 - 1.0) * h))
    .sum();

  let sum2: f64 = (1..n / 2).map(|i| f(a + 2.0 * i as f64 * h)).sum();

  ((b - a) / (3.0 * n as f64)) * (f(a) + f(b) + 4.0 * sum4 + 2.0 * sum2)
}

#[cfg(test)]
mod tests {
  use super::*;
  use rand::Rng;

  fn dotest(n: i32, exp: f64) -> () {
    let ans = format!("{:.5}", (simpson(n) - exp).abs());
    assert!(ans == String::from("0.00000"), "");
  }

  #[test]
  fn basic_tests_dist() {
    dotest(290, 1.9999999986);
    dotest(72, 1.9999996367);
    dotest(252, 1.9999999975);
    dotest(40, 1.9999961668);
    dotest(276, 1.9999999983);
    dotest(384, 1.9999999995);
    dotest(30, 1.9999878155);
    dotest(238, 1.9999999969);
    dotest(20, 1.9999372878);
  }

  #[allow(non_snake_case)]
  fn fEPQ(x: f64) -> f64 {
    return 1.5 * x.sin().powf(3.0);
  }

  #[allow(non_snake_case)]
  fn simpsonEPQ(n: i32) -> f64 {
    const M_PI: f64 = 3.14159265358979323846;
    let a: f64 = 0.0;
    let b: f64 = M_PI;
    let mut sum1: f64 = 0.0;
    let mut sum2: f64 = 0.0;
    let h: f64 = (b - a) / n as f64;
    let mut i: i32 = 0;
    while i <= (n / 2) as i32 {
      sum1 += fEPQ(a + (2.0 * i as f64 - 1.0) * h);
      sum2 += fEPQ(a + 2.0 * i as f64 * h);
      i += 1;
    }
    sum1 += fEPQ(a + (n - 1) as f64 * h);
    return (b - a) / 3.0 / n as f64 * (fEPQ(a) + fEPQ(b) + 4.0 * sum1 + 2.0 * sum2);
  }

  fn do_ex(k: i32) -> () {
    let mut rng = rand::thread_rng();
    for _ in 0..k {
      let n: i32 = rng.gen_range(5..100);
      let sol: f64 = simpsonEPQ(2 * n);
      dotest(2 * n, sol)
    }
  }

  #[test]
  fn random_tests() {
    do_ex(100)
  }
}
