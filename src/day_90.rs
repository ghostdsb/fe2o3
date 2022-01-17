pub mod paintfuck {
  use crate::day_89::smallfuck;

  pub fn interpreter(code: &str, iterations: usize, width: usize, height: usize) -> String {
    // Implement your interpreter here
    let mut tape: Vec<Vec<u8>> = vec![];
    for _ in 0..height {
      let mut line: Vec<u8> = vec![];
      for _ in 0..width {
        line.push(0);
      }
      tape.push(line);
    }

    let mut iterations_done = 0;
    let mut cmd_pos = 0;

    let mut needle_x = 0;
    let mut needle_y = 0;

    loop {
      if iterations_done == iterations || cmd_pos == code.len() {
        break;
      }

      match (
        code.chars().nth(cmd_pos as usize).unwrap(),
        tape[needle_y][needle_x],
      ) {
        ('*', 1) => {
          tape[needle_y][needle_x] = 0;
          cmd_pos += 1;
          iterations_done += 1
        }
        ('*', 0) => {
          tape[needle_y][needle_x] = 1;
          cmd_pos += 1;
          iterations_done += 1
        }
        ('e', _) => {
          needle_x = wrap_add(needle_x as i32, 1i32, width as i32);
          cmd_pos += 1;
          iterations_done += 1
        }
        ('w', _) => {
          needle_x = wrap_add(needle_x as i32, -1i32, width as i32);
          cmd_pos += 1;
          iterations_done += 1
        }
        ('n', _) => {
          needle_y = wrap_add(needle_y as i32, -1i32, height as i32);
          cmd_pos += 1;
          iterations_done += 1
        }
        ('s', _) => {
          needle_y = wrap_add(needle_y as i32, 1i32, height as i32);
          cmd_pos += 1;
          iterations_done += 1
        }
        ('[', 1) => cmd_pos += 1,
        (']', 0) => cmd_pos += 1,
        ('[', 0) => cmd_pos = smallfuck::move_command_cursor(code, '[', cmd_pos as i32) as usize,
        (']', 1) => cmd_pos = smallfuck::move_command_cursor(code, ']', cmd_pos as i32) as usize,
        _ => {}
      }
    }
    printer(tape)
  }

  fn printer(tape: Vec<Vec<u8>>) -> String {
    tape
      .iter()
      .map(|line| line.iter().map(ToString::to_string).collect::<String>())
      .collect::<Vec<String>>()
      .join("\r\n")
  }

  fn wrap_add(base: i32, add_val: i32, max: i32) -> usize {
    let base = base + add_val;
    if base >= 0 {
      (base % max) as usize
    } else {
      (base + max) as usize
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple_cases() {
    assert_eq!(
      display_actual(&paintfuck::interpreter(
        "*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*",
        0,
        6,
        9
      )),
      display_expected(
        "000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"
      ),
      "Your interpreter should initialize all cells in the datagrid to 0"
    );
    assert_eq!(
      display_actual(&paintfuck::interpreter(
        "*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*",
        7,
        6,
        9
      )),
      display_expected(
        "111100\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"
      ),
      "Your interpreter should adhere to the number of iterations specified"
    );
    assert_eq!(
      display_actual(&paintfuck::interpreter(
        "*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*",
        19,
        6,
        9
      )),
      display_expected(
        "111100\r\n000010\r\n000001\r\n000010\r\n000100\r\n000000\r\n000000\r\n000000\r\n000000"
      ),
      "Your interpreter should traverse the 2D datagrid correctly"
    );
    assert_eq!(display_actual(&paintfuck::interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 42, 6, 9)), display_expected("111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000"), "Your interpreter should traverse the 2D datagrid correctly for all of the \"n\", \"e\", \"s\" and \"w\" commands");
    assert_eq!(display_actual(&paintfuck::interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 100, 6, 9)), display_expected("111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000"), "Your interpreter should terminate normally and return a representation of the final state of the 2D datagrid when all commands have been considered from left to right even if the number of iterations specified have not been fully performed");
  }

  /// Prints representation of datagrid - 0's are black and 1's are white.
  /// Note: it only works properly if your interpreter returns a representation
  /// of the datagrid in the correct format.
  fn pretty_print(datagrid: &str) -> &str {
    let rows = datagrid.split("\r\n");
    let mut output = String::new();
    output += "<pre>";
    for row in rows {
      for cell in row.chars() {
        output += "<span style=\"color:";
        output += if cell == '0' { "black" } else { "white" };
        output += ";background-color:";
        output += if cell == '0' { "black" } else { "white" };
        output += "\">xx</span>";
      }
      output += "<br />";
    }
    output += "</pre>";
    println!("{}", output);
    datagrid
  }

  /// Displays the grid the interpreter returns
  fn display_actual(actual: &str) -> &str {
    println!("You returned:");
    pretty_print(actual)
  }

  /// Displays the expected final state of datagrid
  fn display_expected(expected: &str) -> &str {
    println!("Expected final state of data grid:");
    pretty_print(expected)
  }
}
