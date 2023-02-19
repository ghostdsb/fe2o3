pub mod smallfuck {
  pub fn interpreter(code: &str, tape: &str) -> String {
    let mut tape = tape.chars().collect::<Vec<char>>();
    let mut needle_pos = 0_i32;
    let mut cmd_pos = 0_i32;

    while needle_pos < tape.len() as i32 && needle_pos >= 0 && cmd_pos < code.len() as i32 {
      match (
        code.chars().nth(cmd_pos as usize).unwrap(),
        tape.get(needle_pos as usize).unwrap(),
      ) {
        ('>', _) => {
          needle_pos += 1;
          cmd_pos += 1;
        }
        ('<', _) => {
          needle_pos -= 1;
          cmd_pos += 1;
        }
        ('*', '0') => {
          tape[needle_pos as usize] = '1';
          cmd_pos += 1;
        }
        ('*', '1') => {
          tape[needle_pos as usize] = '0';
          cmd_pos += 1;
        }
        ('[', '1') => cmd_pos += 1,
        (']', '0') => cmd_pos += 1,
        ('[', '0') => cmd_pos = move_command_cursor(code, '[', cmd_pos),
        (']', '1') => cmd_pos = move_command_cursor(code, ']', cmd_pos),
        _ => {}
      }
    }
    tape.iter().collect::<String>()
  }

  pub fn move_command_cursor(code: &str, bracket: char, current_pos: i32) -> i32 {
    match bracket {
      '[' => {
        let mut count = 1;
        let mut c_pos = current_pos + 1;
        while count != 0 {
          match code.chars().nth(c_pos as usize) {
            Some('[') => count += 1,
            Some(']') => count -= 1,
            _ => {}
          }
          c_pos += 1;
        }
        c_pos
      }
      ']' => {
        let mut count = 1;
        let mut c_pos = current_pos - 1;
        while count != 0 {
          match code.chars().nth(c_pos as usize) {
            Some('[') => count -= 1,
            Some(']') => count += 1,
            _ => {}
          }
          c_pos -= 1;
        }
        c_pos + 2
      }
      _ => unreachable!(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn example_test_cases1() {
    assert_eq!(smallfuck::interpreter("*", "00101100"), "10101100");
  }
  #[test]
  fn example_test_cases2() {
    // Flips the second and third cell of the tape
    assert_eq!(smallfuck::interpreter(">*>*", "00101100"), "01001100");
  }
  #[test]
  fn example_test_cases3() {
    // Flips all the bits in the tape
    assert_eq!(
      smallfuck::interpreter("*>*>*>*>*>*>*>*", "00101100"),
      "11010011"
    );
  }
  #[test]
  fn example_test_cases4() {
    // Flips all the bits that are initialized to 0
    assert_eq!(
      smallfuck::interpreter("*>*>>*>>>*>*", "00101100"),
      "11111111"
    );
  }
  #[test]
  fn example_test_cases5() {
    // Goes somewhere to the right of the tape and then flips all bits that are initialized to 1, progressing leftwards through the tape
    assert_eq!(
      smallfuck::interpreter(">>>>>*<*<<*", "00101100"),
      "00000000"
    );
  }
  #[test]
  fn example_test_cases6() {
    //needle goes out hence program terminates
    assert_eq!(smallfuck::interpreter("<>*", "00"), "00");
  }

  #[test]
  fn loops() {
    assert_eq!(
      smallfuck::interpreter("*[>*]", &"0".repeat(256)),
      format!("{}", "1".repeat(256))
    );
    //  "Your interpreter should evaluate a simple non-nested loop properly");
    assert_eq!(
      smallfuck::interpreter("[>*]", &"0".repeat(256)),
      format!("{}", "0".repeat(256))
    );
    //  "Your interpreter should jump to the matching \"]\" when it encounters a \"[\" and the bit under the pointer is 0");
    assert_eq!(
      smallfuck::interpreter("*>*>>>*>*>>>>>*>[>*]", &"0".repeat(256)),
      format!("11001100001{}", &"0".repeat(245))
    );
    //  "Your interpreter should jump to the matching \"]\" when it encounters a \"[\" and the bit under the pointer is 0");
    assert_eq!(
      smallfuck::interpreter("*>*>>>*>*>>>>>*[>*]", &"0".repeat(256)),
      format!("11001100001{}", &"1".repeat(245))
    );
    //  "Your interpreter should jump back to the matching \"[\" when it encounters a \"]\" and the bit under the pointer is nonzero");
    assert_eq!(
      smallfuck::interpreter("*[>[*]]", &"0".repeat(256)),
      format!("1{}", &"0".repeat(255))
    );
    //  "Your interpreter should also work properly with nested loops");
    assert_eq!(
      smallfuck::interpreter("*[>[*]]", &"1".repeat(256)),
      format!("0{}", &"1".repeat(255))
    );
    //  "Your interpreter should also work properly with nested loops");
    assert_eq!(smallfuck::interpreter("[[]*>*>*>]", "000"), "000");
    //  "Your interpreter should also work properly with nested loops");
    assert_eq!(smallfuck::interpreter("*>[[]*>]<*", "100"), "100");
    //  "Your interpreter should also work properly with nested loops");
    assert_eq!(smallfuck::interpreter("[*>[>*>]>]", "11001"), "01100");
    //  "Your interpreter should also work properly with nested loops");
    assert_eq!(smallfuck::interpreter("[>[*>*>*>]>]", "10110"), "10101");
    //  "Your interpreter should also work properly with nested loops");
  }
}
