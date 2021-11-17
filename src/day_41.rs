#[allow(unused)]
pub mod fizzy {

  pub struct Matcher<T> {
    matcher: Box<dyn Fn(T) -> bool>,
    subs: String,
  }

  impl<T> Matcher<T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
      F: Fn(T) -> bool + 'static,
      S: AsRef<str>,
    {
      Matcher {
        matcher: Box::new(matcher),
        subs: subs.as_ref().to_string(),
      }
    }
  }

  pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
  }

  impl<T> Fizzy<T>
  where
    T: Default + ToString + Copy,
  {
    pub fn new() -> Self {
      Self { matchers: vec![] }
    }

    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
      self.matchers.push(matcher);
      self
    }

    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
      I: Iterator<Item = T>,
    {
      iter.map(move |item| {
        match self
          .matchers
          .iter()
          .filter(|matcher| (matcher.matcher)(item))
          .map(|matcher| matcher.subs.clone())
          .collect::<String>()
        {
          s if s.is_empty() => item.to_string(),
          s => s,
        }
      })
    }
  }

  /// convenience function: return a Fizzy which applies the standard fizz-buzz rules
  pub fn fizz_buzz<T>() -> Fizzy<T>
  where
    T: Copy
      + std::default::Default
      + std::fmt::Display
      + std::ops::Rem<Output = T>
      + PartialEq
      + ToString
      + From<u8>,
  {
    Fizzy::new()
      .add_matcher(Matcher::new(|n: T| n % 3.into() == 0.into(), "fizz"))
      .add_matcher(Matcher::new(|n: T| n % 5.into() == 0.into(), "buzz"))
  }
}
