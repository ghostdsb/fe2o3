/*
You will have a list of rationals in the form

{ {numer_1, denom_1} , ... {numer_n, denom_n} }
or
[ [numer_1, denom_1] , ... [numer_n, denom_n] ]
or
[ (numer_1, denom_1) , ... (numer_n, denom_n) ]
where all numbers are positive ints. You have to produce a result in the form:

(N_1, D) ... (N_n, D)
or
[ [N_1, D] ... [N_n, D] ]
or
[ (N_1', D) , ... (N_n, D) ]
or
{{N_1, D} ... {N_n, D}}
or
"(N_1, D) ... (N_n, D)"
*/
pub fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
  let den = l
    .iter()
    .fold(1, |d_lcm, (n, d)| lcm(d_lcm, d / gcd(*n, *d)));
  l.iter()
    .map(|(n, d)| (*n * den / d, den))
    .collect::<Vec<(i64, i64)>>()
}

pub fn lcm(num_a: i64, num_b: i64) -> i64 {
  num_a / gcd(num_a, num_b) * num_b
}

pub fn gcd(num_a: i64, num_b: i64) -> i64 {
  if num_b == 0 {
    num_a
  } else {
    gcd(num_b, num_a % num_b)
  }
}

#[test]
fn basics_convert_fracts() {
  fn testing(l: Vec<(i64, i64)>, exp: Vec<(i64, i64)>) -> () {
    assert_eq!(convert_fracts(l), exp)
  }

  testing(
    vec![(69, 130), (87, 1310), (3, 4)],
    vec![(18078, 34060), (2262, 34060), (25545, 34060)],
  );
  testing(
    vec![(690, 1300), (87, 1310), (30, 40)],
    vec![(18078, 34060), (2262, 34060), (25545, 34060)],
  );
}
