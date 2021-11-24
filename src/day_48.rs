pub mod affine_cipher {
  #[derive(Debug, Eq, PartialEq)]
  pub enum AffineCipherError {
    NotCoprime(i32),
  }

  pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    match are_coprime(a, 26) {
      false => Err(AffineCipherError::NotCoprime(a)),
      true => Ok(
        plaintext
          .to_lowercase()
          .chars()
          .filter(|c| !c.is_whitespace() && c.is_alphanumeric())
          .map(|c| {
            if c.is_ascii_lowercase() {
              encd(c, a, b)
            } else {
              c
            }
          })
          .enumerate()
          .fold(String::new(), |mut acc, (i, c)| {
            if i != 0 && i % 5 == 0 {
              acc.push(' ');
            }
            acc.push(c);
            acc
          }),
      ),
    }
  }

  pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    match are_coprime(a, 26) {
      false => Err(AffineCipherError::NotCoprime(a)),
      true => Ok(
        ciphertext
          .chars()
          .filter(|c| !c.is_whitespace())
          .map(|c| {
            if c.is_ascii_lowercase() {
              decd(c, a, b)
            } else {
              c
            }
          })
          .collect::<String>(),
      ),
    }
  }

  fn encd(chr: char, a: i32, b: i32) -> char {
    let x = chr as i32 - b'a' as i32;
    (((x * a + b) % 26) + b'a' as i32) as u8 as char
  }

  fn decd(chr: char, a: i32, b: i32) -> char {
    let y = chr as i32 - b'a' as i32;
    let decd_n = (mmi(a, 26) * (y - b)) % 26;
    if decd_n >= 0 {
      (decd_n + b'a' as i32) as u8 as char
    } else {
      (1 + decd_n + b'z' as i32) as u8 as char
    }
  }

  fn mmi(a: i32, b: i32) -> i32 {
    let mut n = 1;
    loop {
      if a * n % b != 1 {
        n += 1;
      } else {
        break;
      }
    }
    n
  }

  fn are_coprime(a: i32, b: i32) -> bool {
    gcd(a as u32, b as u32) == 1
  }

  fn gcd(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;
    let mut rem = a;
    while a != 0 {
      rem = a;
      a = b % a;
      b = rem;
    }
    rem
  }
}
