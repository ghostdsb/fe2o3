pub mod custom_set {
  #[derive(Debug)]
  pub struct CustomSet<T> {
    set: Vec<T>,
  }

  impl<T> PartialEq for CustomSet<T>
  where
    T: Copy + PartialEq,
  {
    fn eq(&self, other: &Self) -> bool {
      other.set.iter().all(|item| self.contains(item)) && self.set.len() == other.set.len()
    }
  }

  impl<T> CustomSet<T>
  where
    T: Copy + PartialEq,
  {
    pub fn new(input: &[T]) -> Self {
      Self {
        set: input.iter().fold(Vec::new(), |mut acc, val| {
          acc.push(val.clone());
          acc
        }),
      }
    }

    pub fn contains(&self, element: &T) -> bool {
      self.set.contains(element)
    }

    pub fn add(&mut self, element: T) {
      match self.contains(&element) {
        true => {}
        false => self.set.push(element),
      }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
      self.set.iter().all(|item| other.contains(item))
    }

    pub fn is_empty(&self) -> bool {
      self.set.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
      !other.set.iter().any(|item| self.contains(item))
    }

    pub fn intersection(&self, other: &Self) -> Self {
      self.set.iter().fold(Self::new(&[]), |mut acc, val| {
        if other.contains(val) {
          acc.add(val.clone());
        }
        acc
      })
    }

    pub fn difference(&self, other: &Self) -> Self {
      self.set.iter().fold(Self::new(&[]), |mut acc, val| {
        if !other.contains(val) {
          acc.add(val.clone());
        }
        acc
      })
    }

    pub fn union(&self, other: &Self) -> Self {
      let set = self.set.iter().fold(Self::new(&[]), |mut acc, item| {
        acc.add(item.clone());
        acc
      });
      other.set.iter().fold(set, |mut acc, item| {
        acc.add(item.clone());
        acc
      })
    }
  }
}
