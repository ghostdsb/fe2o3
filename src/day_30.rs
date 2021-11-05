#[allow(unused)]
pub mod pythagorean_triplet {
  use std::collections::HashSet;

  pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut pyt: HashSet<[u32; 3]> = HashSet::new();
    let mut a: u32 = sum / 3;
    while a >= 3 {
      let b: i32 = (-2 * sum as i32 * a as i32 + (sum * sum) as i32) / (2 * (sum - a) as i32);
      let c: i32 = sum as i32 - a as i32 - b;
      if c.pow(2) == (a as i32).pow(2) + b.pow(2) && c > a as i32 && c > b && b > a as i32 {
        pyt.insert([a, b as u32, c as u32]);
      }
      a -= 1;
    }
    pyt
  }
}
