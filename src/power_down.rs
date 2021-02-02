use crate::BitMask;

pub struct EnableDisable<'a> {
  index: u16,
  bitmask: BitMask<'a>,
}

impl<'a> EnableDisable<'a> {
  pub fn new(index: u16, data: &'a mut u16) -> Self {
    let bitmask = BitMask::new(data);

    EnableDisable { index, bitmask }
  }

  pub fn enable(&mut self) {
    self.bitmask.set(self.index);
  }

  pub fn disable(&mut self) {
    self.bitmask.unset(self.index);
  }
}

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

  pub fn line_input(&mut self) -> EnableDisable {
    EnableDisable::new(0, &mut self.data)
  }

  pub fn mic(&mut self) -> EnableDisable {
    EnableDisable::new(1, &mut self.data)
  }

  pub fn adc(&mut self) -> EnableDisable {
    EnableDisable::new(2, &mut self.data)
  }

  pub fn dac(&mut self) -> EnableDisable {
    EnableDisable::new(3, &mut self.data)
  }

  pub fn output(&mut self) -> EnableDisable {
    EnableDisable::new(4, &mut self.data)
  }

  pub fn oscillator(&mut self) -> EnableDisable {
    EnableDisable::new(5, &mut self.data)
  }

  pub fn clock_output(&mut self) -> EnableDisable {
    EnableDisable::new(6, &mut self.data)
  }

  pub fn power_off(&mut self) -> EnableDisable {
    EnableDisable::new(7, &mut self.data)
  }
}
