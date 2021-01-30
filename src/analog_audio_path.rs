#[derive(Debug, Copy, Clone)]
pub struct AnalogAudioPath {
  pub(crate) data: u16,
}

impl AnalogAudioPath {
  pub fn new() -> Self {
    AnalogAudioPath {
      data: 0b0_0000_0000,
    }
  }

  pub fn mic_boost(&mut self) {
    self.data = self.data | 0b0_0000_0001
  }

  pub fn mute_mic(&mut self) {
    self.data = self.data | 0b0_0000_0010
  }

  pub fn mic_select(&mut self) {
    self.data = self.data | 0b0_0000_0100
  }

  pub fn bypass(&mut self) {
    self.data = self.data | 0b0_0000_1000
  }

  pub fn dac_select(&mut self) {
    self.data = self.data | 0b0_0001_0000
  }

  pub fn sidetone(&mut self) {
    self.data = self.data | 0b0_0010_0000
  }

  pub fn sidetone_attenuation(&mut self) {
    // TODO: figure this out
    self.data = self.data | 0b0_0000_0000
  }
}
