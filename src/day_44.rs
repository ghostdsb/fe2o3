#[allow(unused)]
pub mod rotational_cipher {
  pub fn rotate(input: &str, key: i8) -> String {
    let rot = |c: char, key| match c {
      c if !c.is_alphabetic() => c as i8,
      c if c.is_uppercase() => (c as i8 - 'A' as i8 + key % 26) % 26 + 'A' as i8,
      _ => (c as i8 - 'a' as i8 + key % 26) % 26 + 'a' as i8,
    };

    input.chars().map(|c| rot(c, key) as u8 as char).collect()
  }
}
