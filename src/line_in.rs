#[derive(Debug, Copy, Clone)]
pub struct LineIn {
  pub(crate) data: u16,
}

impl LineIn {
  pub fn new() -> Self {
    LineIn {
      data: 0b0_0000_0000,
    }
  }

  pub fn volume(&mut self, volume: u16) {
    // TODO: figure out how to handle this one
    self.data = self.data | 0b_0_0000_0000
  }

  pub fn mute(&mut self) {
    self.data = self.data | 0b_0_0000_0010
  }

  pub fn both(&mut self) {
    // what the heck does this do?!
    self.data = self.data | 0b_0_0000_0001
  }
}
