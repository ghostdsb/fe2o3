#[allow(unused)]
pub mod acronym {

  pub fn abbreviate(phrase: &str) -> String {
    phrase
      .split(&[' ', '-'][..])
      .map(|word| {
        word
          .chars()
          .filter(|c| c.is_alphabetic())
          .collect::<String>()
      })
      .map(|word| {
        word
          .chars()
          .enumerate()
          .filter(|(index, _c)| *index == 0_usize)
          .map(|(_index, c)| c)
          .chain(
            word
              .chars()
              .skip_while(|c| c.is_uppercase())
              .filter(|c| c.is_uppercase()),
          )
          .collect::<String>()
      })
      .collect::<String>()
      .to_uppercase()
  }
}
