pub mod largest_series_product {

  #[derive(Debug, PartialEq)]
  pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
  }

  pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    let digits = to_integer_digits(string_digits)?;

    match span {
      0 => Ok(1),
      _ if span > string_digits.len() => Err(Error::SpanTooLong),
      _ => Ok(
        digits
          .windows(span)
          .map(|win| win.iter().product())
          .max()
          .unwrap(),
      ),
    }
  }

  fn to_integer_digits(string_digits: &str) -> Result<Vec<u64>, Error> {
    let mut dig_vector = vec![];
    for ch in string_digits.chars() {
      match ch.to_digit(10) {
        Some(dig) => dig_vector.push(dig as u64),
        None => return Err(Error::InvalidDigit(ch)),
      }
    }
    Ok(dig_vector)
  }
}
