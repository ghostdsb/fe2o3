pub fn sf<'a>(code: &str, tape: &str) -> String {
  let mut tape = tape.chars().collect::<Vec<char>>();
  let mut needle_pos = 0_usize;
  for c in code.chars() {
    match c {
      '>' => {
        if needle_pos >= tape.len() {
          break;
        }
        needle_pos += 1
      }
      '<' => {
        if needle_pos == 0 {
          break;
        }
        needle_pos -= 1
      }
      '*' => {
        if tape.iter().nth(needle_pos).unwrap() == &'1' {
          tape[needle_pos] = '0';
        } else {
          tape[needle_pos] = '1';
        }
      }
      _ => {}
    }
  }
  tape.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn example_test_cases() {
    assert_eq!(sf("*", "00101100"), "10101100");
    // Flips the second and third cell of the tape
    assert_eq!(sf(">*>*", "00101100"), "01001100");
    // Flips all the bits in the tape
    assert_eq!(sf("*>*>*>*>*>*>*>*", "00101100"), "11010011");
    // Flips all the bits that are initialized to 0
    assert_eq!(sf("*>*>>*>>>*>*", "00101100"), "11111111");
    // Goes somewhere to the right of the tape and then flips all bits that are initialized to 1, progressing leftwards through the tape
    assert_eq!(sf(">>>>>*<*<<*", "00101100"), "00000000");
    //needle goes out hence program terminates
    assert_eq!(sf("<>*", "00"), "00");
  }
}
