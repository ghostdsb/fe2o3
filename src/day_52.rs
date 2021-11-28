pub mod circular_buffer {

  pub struct CircularBuffer<T> {
    capacity: usize,
    items: Vec<Option<T>>,
    write_pos: usize,
    read_pos: usize,
    oldest_write_pos: usize,
  }

  #[derive(Debug, PartialEq)]
  pub enum Error {
    EmptyBuffer,
    FullBuffer,
  }

  impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
      let mut items: Vec<Option<T>> = Vec::with_capacity(capacity);
      for _ in 0..capacity {
        items.push(None);
      }

      Self {
        capacity,
        items,
        write_pos: 0,
        read_pos: 0,
        oldest_write_pos: 0,
      }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
      match self.items[self.write_pos] {
        Some(_) => Err(Error::FullBuffer),
        None => {
          self.items[self.write_pos] = Some(element);
          self.write_pos = (self.write_pos + 1) % self.capacity;
          Ok(())
        }
      }
    }

    pub fn read(&mut self) -> Result<T, Error> {
      if let Some(val) = self.items.get_mut(self.read_pos) {
        if let Some(ret) = val.take() {
          self.read_pos = (self.read_pos + 1) % self.capacity;
          return Ok(ret);
        }
      }
      Err(Error::EmptyBuffer)
    }

    pub fn clear(&mut self) {
      for i in 0..self.capacity {
        self.items[i] = None;
      }
      self.read_pos = 0;
      self.write_pos = 0;
      self.oldest_write_pos = 0;
    }

    pub fn overwrite(&mut self, element: T) {
      match self.items[self.write_pos] {
        Some(_) => self.items[self.write_pos] = Some(element),
        None => self.items[self.oldest_write_pos] = Some(element),
      }
      self.oldest_write_pos = (self.oldest_write_pos + 1) % self.capacity;
    }
  }
}
