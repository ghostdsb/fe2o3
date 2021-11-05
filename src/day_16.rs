/*
Once upon a time, on a way through the old wild mountainous west,…
… a man was given directions to go from one point to another. The directions were "NORTH", "SOUTH", "WEST", "EAST". Clearly "NORTH" and "SOUTH" are opposite, "WEST" and "EAST" too.

Going to one direction and coming back the opposite direction right away is a needless effort. Since this is the wild west, with dreadfull weather and not much water, it's important to save yourself some energy, otherwise you might die of thirst!

How I crossed a mountainous desert the smart way.
The directions given to the man are, for example, the following (depending on the language):

["NORTH", "SOUTH", "SOUTH", "EAST", "WEST", "NORTH", "WEST"].
or
{ "NORTH", "SOUTH", "SOUTH", "EAST", "WEST", "NORTH", "WEST" };
or
[North, South, South, East, West, North, West]
You can immediatly see that going "NORTH" and immediately "SOUTH" is not reasonable, better stay to the same place! So the task is to give to the man a simplified version of the plan. A better plan in this case is simply:

["WEST"]
or
{ "WEST" }
or
[West]
Other examples:
In ["NORTH", "SOUTH", "EAST", "WEST"], the direction "NORTH" + "SOUTH" is going north and coming back right away.

The path becomes ["EAST", "WEST"], now "EAST" and "WEST" annihilate each other, therefore, the final result is [] (nil in Clojure).

In ["NORTH", "EAST", "WEST", "SOUTH", "WEST", "WEST"], "NORTH" and "SOUTH" are not directly opposite but they become directly opposite after the reduction of "EAST" and "WEST" so the whole path is reducible to ["WEST", "WEST"].

*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
  North,
  East,
  West,
  South,
}

pub fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
  let mut vec_dir = vec![];
  for dir in arr.iter() {
    match (dir, vec_dir.last()) {
      (Direction::North, Some(Direction::South)) => {
        vec_dir.pop();
      }
      (Direction::South, Some(Direction::North)) => {
        vec_dir.pop();
      }
      (Direction::East, Some(Direction::West)) => {
        vec_dir.pop();
      }
      (Direction::West, Some(Direction::East)) => {
        vec_dir.pop();
      }
      (dir1, _) => {
        vec_dir.push(*dir1);
      }
    }
  }
  vec_dir
}

#[cfg(test)]
mod tests {
  use super::{dir_reduc, Direction::*};

  #[test]
  fn with_duplicates() {
    let a = [North, South, South, East, West, North, West];
    assert_eq!(dir_reduc(&a), [West]);
  }

  #[test]
  fn as_it_is() {
    let a = [North, West, South, East];
    assert_eq!(dir_reduc(&a), [North, West, South, East]);
  }
}
