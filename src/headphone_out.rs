#[derive(Debug, Copy, Clone)]
pub struct HeadphoneOut {
  pub(crate) data: u16,
}

impl HeadphoneOut {
  pub fn new() -> Self {
    HeadphoneOut {
      data: 0b0_0000_0000,
    }
  }

  pub fn mute(&mut self) {
    self.data = self.data | 0b_0_1000_0000
  }

  pub fn both(&mut self) {
    self.data = self.data | 0b_1_0000_0000
  }
}
