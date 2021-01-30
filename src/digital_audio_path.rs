pub enum Deemphasis {
  SampleRate48,
  SampleRate441,
  SampleRate32,
  Disable,
}

#[derive(Debug, Copy, Clone)]
pub struct DigitalAudioPath {
  pub(crate) data: u16,
}

impl DigitalAudioPath {
  pub fn new() -> Self {
    DigitalAudioPath {
      data: 0b0_0000_0000,
    }
  }

  pub fn adc_hpf(&mut self) {
    self.data = self.data | 0b0_0000_0001
  }

  pub fn deemphasis(&mut self, value: Deemphasis) {
    self.data = self.data
      | match value {
        Deemphasis::SampleRate48 => 0b110,
        Deemphasis::SampleRate441 => 0b100,
        Deemphasis::SampleRate32 => 0b010,
        Deemphasis::Disable => 0b000,
      }
  }

  pub fn dac_mut(&mut self) {
    self.data = self.data | 0b0_0000_1000
  }

  pub fn hpor(&mut self) {
    self.data = self.data | 0b0_0001_0000
  }
}
