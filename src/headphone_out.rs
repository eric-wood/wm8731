#[derive(Debug, Copy, Clone)]
pub struct HeadphoneOut {
  pub data: u16,
}

impl HeadphoneOut {
  pub fn new() -> Self {
    HeadphoneOut {
      data: 0b0_0000_0000,
    }
  }

  pub fn mute(&mut self) {
    self.data = self.data | 0b_0_0000_0010
  }

  pub fn both(&mut self) {
    // what the heck does this do?!
    self.data = self.data | 0b_0_0000_0001
  }
}
