#[allow(unused)]
pub fn len_curve(n: i32) -> f64 {
  let h: f64 = 1.0 / n as f64;

  (0..n)
    .map(|dx| ((dx + 1) as f64 * h).powi(2) - (dx as f64 * h).powi(2))
    .map(|dy| (dy * dy + h * h).sqrt())
    .sum()
}

use float_eq::float_eq;

fn assert_float_equals(actual: f64, expected: f64) {
  let merr = 1.0e-6;
  let res = float_eq!(actual, expected, abs <= merr) || float_eq!(actual, expected, rmax <= merr);
  assert!(
    res,
    "Expected value must be near: {:e} but was:{:e}",
    expected, actual
  );
}

fn dotest(n: i32, exp: f64) -> () {
  assert_float_equals(len_curve(n), exp);
}

#[test]
fn basic_tests_len_curve() {
  dotest(1, 1.414213562);
  dotest(10, 1.478197397);
  dotest(40, 1.478896272);
  dotest(200, 1.478940994);
  dotest(1200, 1.478942805);
}
