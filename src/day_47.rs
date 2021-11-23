pub mod atbash_cipher {
  /// "Encipher" with the Atbash cipher.
  pub fn encode(plain: &str) -> String {
    plain
      .to_lowercase()
      .chars()
      .filter(|c| c.is_ascii_alphanumeric())
      .map(|c| get_complement(c))
      .enumerate()
      .fold(String::new(), |mut acc, (i, c)| {
        if i != 0 && i % 5 == 0 {
          acc.push(' ');
        }
        acc.push(c);
        acc
      })
  }

  /// "Decipher" with the Atbash cipher.
  pub fn decode(cipher: &str) -> String {
    cipher
      .chars()
      .filter(|c| !c.is_whitespace())
      .map(|c| get_complement(c))
      .collect()
  }

  fn get_complement(chr: char) -> char {
    match chr.is_ascii_lowercase() {
      true => ('z' as u8 - chr as u8 + 'a' as u8) as char,
      false => chr,
    }
  }
}
