// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
#[allow(unused)]
pub mod robot_simulator {
  #[derive(PartialEq, Debug)]
  pub enum Direction {
    North,
    East,
    South,
    West,
  }
  pub struct Robot {
    position: (i32, i32),
    direction: Direction,
  }
  impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
      Self {
        position: (x, y),
        direction: d,
      }
    }
    pub fn turn_right(mut self) -> Self {
      self.direction = match self.direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
      };
      self
    }
    pub fn turn_left(mut self) -> Self {
      self.direction = match self.direction {
        Direction::North => Direction::West,
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
      };
      self
    }
    pub fn advance(mut self) -> Self {
      self.position = match self.direction {
        Direction::North => (self.position.0, self.position.1 + 1),
        Direction::East => (self.position.0 + 1, self.position.1),
        Direction::South => (self.position.0, self.position.1 - 1),
        Direction::West => (self.position.0 - 1, self.position.1),
      };
      self
    }
    pub fn instructions(mut self, instructions: &str) -> Self {
      instructions.chars().fold(self, |r, ins| match ins {
        'R' => r.turn_right(),
        'L' => r.turn_left(),
        'A' => r.advance(),
        _ => r,
      })
    }
    pub fn position(&self) -> (i32, i32) {
      self.position
    }
    pub fn direction(&self) -> &Direction {
      &self.direction
    }
  }
}
