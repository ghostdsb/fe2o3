#[allow(unused)]
#[derive(Debug)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[allow(unused)]
pub fn street_fighter_selection(
  fighters: &[[&str; 6]; 2],
  position: &[i64; 2],
  moves: &[Direction],
) -> Vec<String> {
  let mut initial_position = position;
  let cursors = moves.iter().fold(
    (vec![], *initial_position),
    |(mut players, mut ps), movement| {
      match movement {
        Direction::Left => {
          if ps[0] > 0 {
            players.push([ps[0] - 1, ps[1]]);
            (players, [ps[0] - 1, ps[1]])
          } else {
            players.push([5, ps[1]]);
            (players, [5, ps[1]])
            // players
          }
        }
        Direction::Right => {
          if ps[0] < 5 {
            players.push([ps[0] + 1, ps[1]]);
            (players, [ps[0] + 1, ps[1]])
            // players
          } else {
            players.push([0, ps[1]]);
            (players, [0, ps[1]])
            // players
          }
        }
        Direction::Up => {
          if ps[1] == 0 {
            players.push([ps[0], ps[1]]);
            (players, ps)
          } else {
            players.push([ps[0], 0]);
            (players, [ps[0], 0])
          }
        }
        Direction::Down => {
          if ps[1] == 0 {
            players.push([ps[0], 1]);
            (players, [ps[0], 1])
          } else {
            players.push([ps[0], ps[1]]);
            (players, ps)
          }
        }
      }
    },
  );
  let fighter_names = cursors
    .0
    .iter()
    .map(|ps| String::from(fighters[ps[1] as usize][ps[0] as usize]))
    .collect::<Vec<String>>();
  fighter_names
}

#[cfg(test)]
mod tests {

  use super::{street_fighter_selection, Direction};

  const FIGHTERS: [[&str; 6]; 2] = [
    ["Ryu", "E.Honda", "Blanka", "Guile", "Balrog", "Vega"],
    ["Ken", "Chun Li", "Zangief", "Dhalsim", "Sagat", "M.Bison"],
  ];

  #[test]
  fn few_moves() {
    use Direction::*;
    let moves = [Up, Left, Right, Left, Left];

    assert_eq!(
      street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
      ["Ryu", "Vega", "Ryu", "Vega", "Balrog"],
    );
  }

  #[test]
  fn no_moves() {
    let moves: [Direction; 0] = [];

    assert_eq!(
      street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
      [] as [String; 0]
    );
  }

  #[test]
  fn moving_left() {
    use Direction::*;
    let moves = [Left, Left, Left, Left, Left, Left, Left, Left];

    assert_eq!(
      street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
      ["Vega", "Balrog", "Guile", "Blanka", "E.Honda", "Ryu", "Vega", "Balrog"],
    );
  }

  #[test]
  fn moving_right() {
    use Direction::*;
    let moves = [Right, Right, Right, Right, Right, Right, Right, Right];

    assert_eq!(
      street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
      ["E.Honda", "Blanka", "Guile", "Balrog", "Vega", "Ryu", "E.Honda", "Blanka"],
    );
  }

  #[test]
  fn uses_all_4_directions_clockwise_twice() {
    use Direction::*;
    let moves = [Up, Left, Down, Right, Up, Left, Down, Right];

    assert_eq!(
      street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
      ["Ryu", "Vega", "M.Bison", "Ken", "Ryu", "Vega", "M.Bison", "Ken"],
    );
  }

  #[test]
  fn always_moving_down() {
    use Direction::*;
    let moves = [Down, Down, Down, Down];

    assert_eq!(
      street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
      ["Ken", "Ken", "Ken", "Ken"],
    );
  }

  #[test]
  fn always_moving_up() {
    use Direction::*;
    let moves = [Up, Up, Up, Up];

    assert_eq!(
      street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
      ["Ryu", "Ryu", "Ryu", "Ryu"],
    );
  }
}
