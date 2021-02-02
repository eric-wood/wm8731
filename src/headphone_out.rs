use crate::EnableDisable;

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

  pub fn mute(&mut self) -> EnableDisable {
    EnableDisable::new(7, &mut self.data)
  }

  pub fn both(&mut self) -> EnableDisable {
    EnableDisable::new(8, &mut self.data)
  }
}
