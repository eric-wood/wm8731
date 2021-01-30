pub enum Format {
  DSP,
  I2S,
  LeftJustified,
  RightJustified,
}

pub enum Length {
  Bits32,
  Bits24,
  Bits20,
  Bits16,
}

#[derive(Debug, Copy, Clone)]
pub struct DigitalAudioInterfaceFormat {
  pub(crate) data: u16,
}

impl DigitalAudioInterfaceFormat {
  pub fn new() -> Self {
    DigitalAudioInterfaceFormat {
      data: 0b0_0000_0000,
    }
  }

  pub fn format(&mut self, format: Format) {
    let bits = match format {
      Format::DSP => 0b11,
      Format::I2S => 0b10,
      Format::LeftJustified => 0b01,
      Format::RightJustified => 0b00,
    };

    self.data = self.data | bits
  }

  pub fn bit_length(&mut self, length: Length) {
    let bits = match length {
      Length::Bits32 => 0b11,
      Length::Bits24 => 0b10,
      Length::Bits20 => 0b01,
      Length::Bits16 => 0b00,
    };

    self.data = self.data | (bits << 2)
  }

  pub fn left_right_phase(&mut self) {
    self.data = self.data | 0b0_0001_0000
  }

  pub fn left_right_dac_clock_swap(&mut self) {
    self.data = self.data | 0b0_0010_0000
  }

  pub fn master(&mut self) {
    self.data = self.data | 0b0_0100_0000
  }

  pub fn bit_clock_invert(&mut self) {
    self.data = self.data | 0b0_1000_0000
  }
}
