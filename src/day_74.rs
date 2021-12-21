/*
Your task is to divide the numbers 1,2,…,n into two sets of equal sum.

Input

The only input line contains an integer n.

Output

Print "YES", if the division is possible, and "NO" otherwise.

After this, if the division is possible, print an example of how to create the sets. First, print the number of elements in the first set followed by the elements themselves in a separate line, and then, print the second set in a similar way.

Constraints
1≤n≤106
Example 1

Input:
7

Output:
YES
4
1 2 4 7
3
3 5 6

Example 2

Input:
6

Output:
NO

*/

pub mod two_set {
  use std::fmt::Write;
  use std::io;
  pub fn solve(number: u32) -> Option<(Vec<u32>, Vec<u32>)> {
    let mut n_sum: i32 = if number % 2 == 0 {
      (number / 2 * (number + 1)) as i32
    } else {
      (number * (number + 1) / 2) as i32
    };
    if n_sum % 2 == 1 {
      return None;
    } else {
      let mut vec1 = vec![];
      let mut vec2 = vec![];
      n_sum /= 2;
      let mut start = number as i32;
      while start > 0 {
        if n_sum as i32 - start as i32 >= 0 {
          vec1.push(start as u32);
          n_sum -= start;
        } else {
          vec2.push(start as u32);
        }
        start -= 1;
      }
      return Some((vec1, vec2));
    }
  }

  pub fn print(solution: Option<(Vec<u64>, Vec<u64>)>) -> String {
    match solution {
      None => "NO".to_string(),
      Some((v1, v2)) => {
        let mut buf = String::new();
        buf.push_str("YES\n");
        writeln!(buf, "{}", v1.len()).unwrap();
        writeln!(buf, "{}", space_sep(&v1)).unwrap();
        writeln!(buf, "{}", v2.len()).unwrap();
        writeln!(buf, "{}", space_sep(&v2)).unwrap();
        buf
      }
    }
  }

  fn space_sep<T: std::fmt::Display>(xs: &[T]) -> String {
    let mut res = String::new();
    for x in xs {
      if !res.is_empty() {
        res.push(' ');
      }
      write!(res, "{}", x).unwrap();
    }
    res
  }
}
