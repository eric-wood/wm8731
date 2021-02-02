use crate::BitMask;
use crate::EnableDisable;

pub struct LeftRight<'a> {
  index: u16,
  bitmask: BitMask<'a>,
}

impl<'a> LeftRight<'a> {
  pub fn new(index: u16, data: &'a mut u16) -> Self {
    let bitmask = BitMask::new(data);

    LeftRight { index, bitmask }
  }

  pub fn left(&mut self) {
    self.bitmask.set(self.index);
  }

  pub fn right(&mut self) {
    self.bitmask.unset(self.index);
  }
}

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

  pub fn left_right_phase(&mut self) -> EnableDisable {
    EnableDisable::new(4, &mut self.data)
  }

  pub fn left_right_dac_clock_swap(&mut self) -> LeftRight {
    LeftRight::new(5, &mut self.data)
  }

  pub fn master(&mut self) -> EnableDisable {
    EnableDisable::new(6, &mut self.data)
  }

  pub fn bit_clock_invert(&mut self) -> EnableDisable {
    EnableDisable::new(7, &mut self.data)
  }
}
