//! Configuration for line inputs

use crate::EnableDisable;

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

  /// Line input volume
  ///
  /// Min: -34.5dB
  ///
  /// Max: +12dB
  ///
  /// Steps of 1.5dB
  ///
  /// *Note*: not implemented yet
  pub fn volume(&mut self, volume: u16) {
    self.data = self.data | 0b_0_0000_0000
  }

  /// Line input mute to ADC
  pub fn mute(&mut self) -> EnableDisable {
    EnableDisable::new(7, &mut self.data)
  }

  /// Left to right channel line input volume and mute data load
  /// When enabled, left and right channels will have same volume and mute values
  pub fn both(&mut self) -> EnableDisable {
    EnableDisable::new(8, &mut self.data)
  }
}
