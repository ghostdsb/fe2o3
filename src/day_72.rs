#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
  pub x: usize,
  pub y: usize,
}

impl Position {
  pub fn new(x: usize, y: usize) -> Self {
    Self { x, y }
  }
}

fn super_street_fighter_selection(
  fighters: &[&[&str]],
  position: Position,
  moves: &[Direction],
) -> Vec<String> {
  let is_empty = |x: usize, y: usize, v: &[&[&str]]| -> bool {
    v.get(y)
      .map_or(true, |r| r.get(x).map_or(true, |&s| s.is_empty()))
  };

  let rc = fighters.len();
  let cc = fighters[0].len();
  moves
    .iter()
    .scan(position, |p, m| {
      match m {
        Direction::Up => {
          p.y = if p.y == 0 || is_empty(p.x, p.y - 1, fighters) {
            p.y
          } else {
            p.y - 1
          };
        }
        Direction::Down => {
          p.y = if p.y == rc - 1 || is_empty(p.x, p.y + 1, fighters) {
            p.y
          } else {
            p.y + 1
          };
        }
        Direction::Left => {
          let mut nx = (p.x + cc - 1) % cc;
          p.x = loop {
            if is_empty(nx, p.y, fighters) {
              nx = (nx + cc - 1) % cc;
            } else {
              break nx;
            }
          };
        }
        Direction::Right => {
          let mut nx = (p.x + 1) % cc;
          p.x = loop {
            if is_empty(nx, p.y, fighters) {
              nx = (nx + 1) % cc;
            } else {
              break nx;
            }
          };
        }
      };
      Some(fighters[p.y][p.x].into())
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::{super_street_fighter_selection, Direction, Position};

  #[rustfmt::skip]
    const FIGHTERS_A: [&[&'static str]; 3] = [
        &[       "",    "Ryu",  "E.Honda",  "Blanka",   "Guile", ""       ],
        &[ "Balrog",    "Ken",  "Chun Li", "Zangief", "Dhalsim", "Sagat"  ],
        &[   "Vega", "T.Hawk", "Fei Long",  "Deejay",   "Cammy", "M.Bison"],
    ];

  #[rustfmt::skip]
    const FIGHTERS_B: [&[&'static str]; 3] = [
        &[       "",    "Ryu",  "E.Honda",  "Cammy",  "Blanka",   "Guile",        "", "Chun Li" ],
        &[ "Balrog",    "Ken",  "Chun Li",       "", "M.Bison", "Zangief", "Dhalsim", "Sagat"   ],
        &[   "Vega",       "", "Fei Long", "Balrog",  "Deejay",   "Cammy",        "", "T.Hawk"  ],
    ];

  #[rustfmt::skip]
    const FIGHTERS_C: [&[&'static str]; 6] = [
        &[        "",     "Ryu",  "E.Honda",  "Cammy" ],
        &[  "Balrog",     "Ken",  "Chun Li",       "" ],
        &[    "Vega",        "", "Fei Long", "Balrog",],
        &[  "Blanka",   "Guile",         "", "Chun Li"],
        &[ "M.Bison", "Zangief",  "Dhalsim", "Sagat"  ],
        &[  "Deejay",   "Cammy",         "", "T.Hawk" ],
    ];

  #[test]
  fn no_selection() {
    assert_eq!(
      super_street_fighter_selection(&FIGHTERS_A[..], Position::new(0, 0), &[] as &[Direction]),
      vec![] as Vec<String>,
      "it should work with no selection cursor moves",
    );
  }

  #[test]
  fn empty_vertical_spaces_single_move() {
    use Direction::*;

    assert_eq!(
      super_street_fighter_selection(&FIGHTERS_A[..], Position::new(0, 1), &[Up]),
      vec!["Balrog"],
      "it should stop on empty spaces vertically",
    );
  }

  #[test]
  fn empty_vertical_spaces_multiple_moves() {
    use Direction::*;

    assert_eq!(
      super_street_fighter_selection(&FIGHTERS_A[..], Position::new(0, 1), &[Up, Up, Up, Up]),
      vec!["Balrog", "Balrog", "Balrog", "Balrog"],
      "it should stop on empty spaces vertically (up)",
    );

    assert_eq!(
      super_street_fighter_selection(
        &FIGHTERS_A[..],
        Position::new(0, 1),
        &[Down, Down, Down, Down]
      ),
      vec!["Vega", "Vega", "Vega", "Vega"],
      "it should stop on empty spaces vertically (down)",
    );

    assert_eq!(
      super_street_fighter_selection(&FIGHTERS_A[..], Position::new(5, 1), &[Up, Up, Up, Up]),
      vec!["Sagat", "Sagat", "Sagat", "Sagat"],
      "it should stop on empty spaces vertically (up)",
    );

    assert_eq!(
      super_street_fighter_selection(
        &FIGHTERS_A[..],
        Position::new(5, 1),
        &[Down, Down, Down, Down]
      ),
      vec!["M.Bison", "M.Bison", "M.Bison", "M.Bison"],
      "it should stop on empty spaces vertically (down)",
    );
  }

  #[test]
  fn rotate_horizontally() {
    use Direction::*;

    assert_eq!(
      super_street_fighter_selection(
        &FIGHTERS_A[..],
        Position::new(2, 0),
        &[Left, Left, Left, Left, Left, Left, Left, Left]
      ),
      vec!["Ryu", "Guile", "Blanka", "E.Honda", "Ryu", "Guile", "Blanka", "E.Honda"],
      "it should rotate horizontally (left)",
    );

    assert_eq!(
      super_street_fighter_selection(
        &FIGHTERS_A[..],
        Position::new(3, 1),
        &[Left, Left, Left, Left, Left, Left, Left, Left]
      ),
      vec!["Chun Li", "Ken", "Balrog", "Sagat", "Dhalsim", "Zangief", "Chun Li", "Ken"],
      "it should rotate horizontally (left)",
    );
  }

  #[test]
  fn rotate_horizontally_with_empty_spaces() {
    use Direction::*;

    assert_eq!(
      super_street_fighter_selection(
        &FIGHTERS_A[..],
        Position::new(2, 0),
        &[Right, Right, Right, Right, Right, Right, Right, Right]
      ),
      vec!["Blanka", "Guile", "Ryu", "E.Honda", "Blanka", "Guile", "Ryu", "E.Honda"],
      "it should rotate horizontally with empty spaces",
    );
  }

  #[test]
  fn rotate_on_all_rows() {
    use Direction::*;

    assert_eq!(
      super_street_fighter_selection(
        &FIGHTERS_A[..],
        Position::new(2, 0),
        &[
          Right, Right, Right, Right, Right, Right, Down, Left, Left, Left, Left, Left, Left, Left,
          Left, Left, Left, Left, Left, Down, Right, Right, Right, Right, Right, Right, Right,
          Right, Right, Right, Right, Right
        ]
      ),
      vec![
        "Blanka", "Guile", "Ryu", "E.Honda", "Blanka", "Guile", "Dhalsim", "Zangief", "Chun Li",
        "Ken", "Balrog", "Sagat", "Dhalsim", "Zangief", "Chun Li", "Ken", "Balrog", "Sagat",
        "Dhalsim", "Cammy", "M.Bison", "Vega", "T.Hawk", "Fei Long", "Deejay", "Cammy", "M.Bison",
        "Vega", "T.Hawk", "Fei Long", "Deejay", "Cammy"
      ],
      "it should rotate on all rows",
    );

    assert_eq!(
      super_street_fighter_selection(
        &FIGHTERS_B[..],
        Position::new(2, 0),
        &[
          Right, Right, Right, Right, Right, Right, Down, Left, Left, Left, Left, Left, Left, Left,
          Left, Left, Left, Left, Left, Down, Right, Right, Right, Right, Right, Right, Right,
          Right, Right, Right, Right, Right
        ]
      ),
      vec![
        "Cammy", "Blanka", "Guile", "Chun Li", "Ryu", "E.Honda", "Chun Li", "Ken", "Balrog",
        "Sagat", "Dhalsim", "Zangief", "M.Bison", "Chun Li", "Ken", "Balrog", "Sagat", "Dhalsim",
        "Zangief", "Cammy", "T.Hawk", "Vega", "Fei Long", "Balrog", "Deejay", "Cammy", "T.Hawk",
        "Vega", "Fei Long", "Balrog", "Deejay", "Cammy"
      ],
      "it should rotate on all rows",
    );
  }

  #[test]
  fn should_rotate_on_all_rows() {
    use Direction::*;

    assert_eq!(
      super_street_fighter_selection(
        &FIGHTERS_B[..],
        Position::new(3, 0),
        &[Down, Right, Right, Right, Down, Left, Left, Down, Right, Right, Right, Up]
      ),
      vec![
        "Cammy", "Blanka", "Guile", "Chun Li", "Sagat", "Dhalsim", "Zangief", "Cammy", "T.Hawk",
        "Vega", "Fei Long", "Chun Li"
      ],
      "it should rotate on all rows",
    );
  }

  #[test]
  fn should_work_with_longer_grid() {
    use Direction::*;

    assert_eq!(
      super_street_fighter_selection(
        &FIGHTERS_C[..],
        Position::new(3, 0),
        &[
          Left, Left, Down, Right, Right, Right, Right, Down, Left, Left, Left, Left, Down, Right,
          Right, Down, Right, Right, Right, Down, Left, Left, Left, Down, Left, Left, Left
        ]
      ),
      vec![
        "E.Honda", "Ryu", "Ken", "Chun Li", "Balrog", "Ken", "Chun Li", "Fei Long", "Vega",
        "Balrog", "Fei Long", "Vega", "Blanka", "Guile", "Chun Li", "Sagat", "M.Bison", "Zangief",
        "Dhalsim", "Dhalsim", "Zangief", "M.Bison", "Sagat", "T.Hawk", "Cammy", "Deejay", "T.Hawk"
      ],
      "it should work with longer grid",
    );
  }

  #[test]
  fn should_work_with_odd_initial_position() {
    use Direction::*;

    assert_eq!(
      super_street_fighter_selection(
        &FIGHTERS_C[..],
        Position::new(3, 3),
        &[
          Left, Left, Down, Right, Right, Right, Right, Down, Left, Left, Left, Left, Up, Right,
          Right, Up, Right, Right, Right
        ]
      ),
      vec![
        "Guile", "Blanka", "M.Bison", "Zangief", "Dhalsim", "Sagat", "M.Bison", "Deejay", "T.Hawk",
        "Cammy", "Deejay", "T.Hawk", "Sagat", "M.Bison", "Zangief", "Guile", "Chun Li", "Blanka",
        "Guile"
      ],
      "it should work with odd initial position",
    );
  }
}
