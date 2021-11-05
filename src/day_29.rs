#[allow(unused)]
pub mod triangle {
  use std::collections::HashSet;

  pub struct Triangle {
    sides: [u64; 3],
  }

  impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
      for i in 0..3 {
        if sides[i % 3] + sides[(i + 1) % 3] > sides[(i + 2) % 3] {
          continue;
        } else {
          return None;
        }
      }

      Some(Triangle { sides })
    }

    pub fn is_equilateral(&self) -> bool {
      self.equal_side_count() == 3
    }

    pub fn is_isosceles(&self) -> bool {
      self.equal_side_count() == 2
    }

    pub fn is_scalene(&self) -> bool {
      self.equal_side_count() == 1
    }

    fn equal_side_count(&self) -> usize {
      4 - self
        .sides
        .iter()
        .fold(HashSet::new(), |mut set, side| {
          set.insert(side);
          set
        })
        .len()
    }
  }
}
