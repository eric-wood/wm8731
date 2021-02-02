pub struct BitMask<'a> {
  data: &'a mut u16,
}

impl<'a> BitMask<'a> {
  pub fn new(data: &'a mut u16) -> Self {
    BitMask { data }
  }

  pub fn set(&mut self, index: u16) {
    *self.data = *self.data | (1 << index)
  }

  pub fn unset(&mut self, index: u16) {
    *self.data = *self.data & !(1 << index)
  }
}
