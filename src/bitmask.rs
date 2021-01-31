pub struct BitMask {
  data: u16,
}

impl BitMask {
  pub fn new(data: u16) -> Self {
    BitMask { data }
  }

  pub fn set(&mut self, index: u16) {
    self.data = self.data | (0b1 << index)
  }

  pub fn unset(&mut self, index: u16) {
    self.data = self.data & !(0b1 << index)
  }
}
