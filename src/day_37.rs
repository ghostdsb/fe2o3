pub mod accumulate {
  pub fn map<I, F, R>(input: Vec<I>, mut function: F) -> Vec<R>
  where
    F: FnMut(I) -> R,
  {
    let mut ans = vec![];

    for item in input {
      ans.push(function(item));
    }

    ans
  }
}
