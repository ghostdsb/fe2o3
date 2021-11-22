pub mod simple_cipher {

  use rand::{thread_rng, Rng};
  pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.chars().any(|c| !c.is_ascii_lowercase()) || key.len() == 0 {
      return None;
    }
    Some(
      s.chars()
        .zip(key.chars().cycle())
        .map(|(c, k)| rot(c, k as i8 - 'a' as i8))
        .collect(),
    )
  }

  pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.chars().any(|c| !c.is_ascii_lowercase()) || key.len() == 0 {
      return None;
    }
    Some(
      s.chars()
        .zip(key.chars().cycle())
        .map(|(c, k)| rot(c, (k as i8 - 'a' as i8) * -1))
        .collect(),
    )
  }

  pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = thread_rng();
    let key: String = (0..100)
      .map(|_| (rng.gen_range('a' as u8..'z' as u8 + 1)) as char)
      .collect();
    println!("key-> {:?}", key);
    let encoded_string = encode(&key, s);
    (key, encoded_string.unwrap())
  }

  fn rot(c: char, key: i8) -> char {
    match c {
      c if !c.is_alphabetic() => c,
      c if c.is_uppercase() => (((c as i8 - 'A' as i8 + key) % 26) as i8 + 'A' as i8) as u8 as char,
      _ => (((c as i8 - 'a' as i8 + key) % 26) as i8 + 'a' as i8) as u8 as char,
    }
  }
}
