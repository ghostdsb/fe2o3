#[allow(unused)]
pub mod rotational_cipher {
  pub fn rotate(input: &str, key: i8) -> String {
    let rot = |c, key| match c {
      c if c >= 'A' as i8 && c <= 'Z' as i8 => (c - 'A' as i8 + key % 26) % 26 + 'A' as i8,
      c if c >= 'a' as i8 && c <= 'z' as i8 => (c - 'a' as i8 + key % 26) % 26 + 'a' as i8,
      _ => c,
    };

    input
      .chars()
      .map(|c| rot(c as i8, key) as u8 as char)
      .collect()
  }
}
