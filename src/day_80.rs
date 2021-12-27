/*
You get a list of non-zero integers A, its length is always even and always greater than one. Your task is to find such non-zero integers W that the weighted sum

A0⋅W0+A1⋅W1+..+An⋅Wn is equal to 0.

Examples
// One of the possible solutions: W = [-10, -1, -1, 1, 1, 1]
// 1*(-10) + 2*(-1) + 3*(-1) + 4*1 + 5*1 + 6*1
weigh_the_list([1, 2, 3, 4, 5, 6]);

// One of the possible solutions: W = [4, 1]
// -13*4 + 52*1 = 0
weigh_the_list([-13, 52]);

// One of the possible solutions: W = [1, 1]
// -1*1 + 1*1 = 0
weigh_the_list([-1, 1]);
*/

pub fn weigh_the_list(a: Vec<i64>) -> Vec<i64> {
  let mut weights = vec![];
  let mut iter_a = a.iter();
  loop {
    match (iter_a.next(), iter_a.next()) {
      (Some(n), Some(m)) => {
        weights.push(*m);
        weights.push(-1 * *n);
      }
      (None, None) => {
        break;
      }
      _ => unreachable!(),
    };
  }
  weights
}

#[cfg(test)]
mod tests {
  use super::*;

  fn one_test(a: Vec<i64>) {
    let w = weigh_the_list(a.clone());

    let mut problems: Vec<String> = Vec::new();
    if a.len() != w.len() {
      problems.push("Dimensions don't match".to_string())
    }
    if w.contains(&0i64) {
      problems.push("All coefficients must be non-zero".to_string())
    }

    if a.len() == w.len() {
      let ws: i64 = (0..a.len()).map(|i| a[i] * w[i]).sum();
      if ws != 0 {
        problems.push(format!("The weighted sum is equal to {}, must be 0", ws))
      }
    }

    assert_eq!(problems.len(), 0, "{}", problems.join("; ") + &".");
  }

  #[test]
  fn fixed_tests() {
    one_test(vec![1, 2, 3, 4, 5, 6]);
    one_test(vec![-13, 52]);
    one_test(vec![-1, 1]);
    one_test(vec![2, 2, 2, 2]);
    one_test(vec![2, 7, 3, 11, 5, 23, 47, 3]);
    one_test(vec![-1, 100, -100, 1]);
    one_test(vec![1, 1, 1, -2]);
    one_test(vec![-2, 1, 1, 1]);
  }
}
