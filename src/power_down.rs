#[derive(Debug, Copy, Clone)]
pub struct PowerDown {
  pub(crate) data: u16,
}

impl PowerDown {
  pub fn new() -> Self {
    PowerDown {
      data: 0b0_0000_0000,
    }
  }

  pub fn line_input(&mut self) {
    self.data = self.data | 0b0_0000_0001
  }

  pub fn mic(&mut self) {
    self.data = self.data | 0b0_0000_0010
  }

  pub fn adc(&mut self) {
    self.data = self.data | 0b0_0000_0100
  }

  pub fn dac(&mut self) {
    self.data = self.data | 0b0_0000_1000
  }

  pub fn output(&mut self) {
    self.data = self.data | 0b0_0001_0000
  }

  pub fn oscillator(&mut self) {
    self.data = self.data | 0b0_0010_0000
  }

  pub fn clock_output(&mut self) {
    self.data = self.data | 0b0_0100_0000
  }

  pub fn power_off(&mut self) {
    self.data = self.data | 0b0_1000_0000
  }
}
