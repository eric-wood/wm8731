use crate::BitMask;

pub struct Adc48<'a> {
  index: u16,
  bitmask: BitMask<'a>,
}

impl<'a> Adc48<'a> {
  pub fn new(index: u16, data: &'a mut u16) -> Self {
    let bitmask = BitMask::new(data);
    Adc48 { index, bitmask }
  }

  pub fn dac_48(&mut self) {
    self.bitmask.unset(self.index + 0);
    self.bitmask.unset(self.index + 1);
    self.bitmask.unset(self.index + 2);
    self.bitmask.unset(self.index + 3);
  }

  pub fn dac_8(&mut self) {
    self.bitmask.set(self.index + 0);
    self.bitmask.unset(self.index + 1);
    self.bitmask.unset(self.index + 2);
    self.bitmask.unset(self.index + 3);
  }
}

pub struct Adc8<'a> {
  index: u16,
  bitmask: BitMask<'a>,
}

impl<'a> Adc8<'a> {
  pub fn new(index: u16, data: &'a mut u16) -> Self {
    let bitmask = BitMask::new(data);
    Adc8 { index, bitmask }
  }

  pub fn dac_48(&mut self) {
    self.bitmask.unset(self.index + 0);
    self.bitmask.set(self.index + 1);
    self.bitmask.unset(self.index + 2);
    self.bitmask.unset(self.index + 3);
  }

  pub fn dac_8(&mut self) {
    self.bitmask.set(self.index + 0);
    self.bitmask.set(self.index + 1);
    self.bitmask.unset(self.index + 2);
    self.bitmask.unset(self.index + 3);
  }
}

pub struct Adc32<'a> {
  index: u16,
  bitmask: BitMask<'a>,
}

impl<'a> Adc32<'a> {
  pub fn new(index: u16, data: &'a mut u16) -> Self {
    let bitmask = BitMask::new(data);
    Adc32 { index, bitmask }
  }

  pub fn dac_32(&mut self) {
    self.bitmask.unset(self.index + 0);
    self.bitmask.set(self.index + 1);
    self.bitmask.set(self.index + 2);
    self.bitmask.unset(self.index + 3);
  }
}

pub struct Adc96<'a> {
  index: u16,
  bitmask: BitMask<'a>,
}

impl<'a> Adc96<'a> {
  pub fn new(index: u16, data: &'a mut u16) -> Self {
    let bitmask = BitMask::new(data);
    Adc96 { index, bitmask }
  }

  pub fn dac_96(&mut self) {
    self.bitmask.set(self.index + 0);
    self.bitmask.set(self.index + 1);
    self.bitmask.set(self.index + 2);
    self.bitmask.unset(self.index + 3);
  }
}

pub struct Adc441<'a> {
  index: u16,
  bitmask: BitMask<'a>,
}

impl<'a> Adc441<'a> {
  pub fn new(index: u16, data: &'a mut u16) -> Self {
    let bitmask = BitMask::new(data);
    Adc441 { index, bitmask }
  }

  pub fn dac_441(&mut self) {
    self.bitmask.unset(self.index + 0);
    self.bitmask.unset(self.index + 1);
    self.bitmask.unset(self.index + 2);
    self.bitmask.set(self.index + 3);
  }

  pub fn dac_8018(&mut self) {
    self.bitmask.set(self.index + 0);
    self.bitmask.unset(self.index + 1);
    self.bitmask.unset(self.index + 2);
    self.bitmask.set(self.index + 3);
  }
}

pub struct Adc8018<'a> {
  index: u16,
  bitmask: BitMask<'a>,
}

impl<'a> Adc8018<'a> {
  pub fn new(index: u16, data: &'a mut u16) -> Self {
    let bitmask = BitMask::new(data);
    Adc8018 { index, bitmask }
  }

  pub fn dac_441(&mut self) {
    self.bitmask.unset(self.index + 0);
    self.bitmask.set(self.index + 1);
    self.bitmask.unset(self.index + 2);
    self.bitmask.set(self.index + 3);
  }

  pub fn dac_8018(&mut self) {
    self.bitmask.set(self.index + 0);
    self.bitmask.set(self.index + 1);
    self.bitmask.unset(self.index + 2);
    self.bitmask.set(self.index + 3);
  }
}

pub struct Adc882<'a> {
  index: u16,
  bitmask: BitMask<'a>,
}

impl<'a> Adc882<'a> {
  pub fn new(index: u16, data: &'a mut u16) -> Self {
    let bitmask = BitMask::new(data);
    Adc882 { index, bitmask }
  }

  pub fn dac_882(&mut self) {
    self.bitmask.set(self.index + 0);
    self.bitmask.set(self.index + 1);
    self.bitmask.set(self.index + 2);
    self.bitmask.set(self.index + 3);
  }
}

pub struct SamplingRate<'a> {
  index: u16,
  data: &'a mut u16,
}

impl<'a> SamplingRate<'a> {
  pub fn new(index: u16, data: &'a mut u16) -> Self {
    SamplingRate { index, data }
  }

  pub fn adc_48(&mut self) -> Adc48 {
    Adc48::new(self.index, self.data)
  }

  pub fn adc_8(&mut self) -> Adc8 {
    Adc8::new(self.index, self.data)
  }

  pub fn adc_32(&mut self) -> Adc32 {
    Adc32::new(self.index, self.data)
  }

  pub fn adc_96(&mut self) -> Adc96 {
    Adc96::new(self.index, self.data)
  }

  pub fn adc_441(&mut self) -> Adc441 {
    Adc441::new(self.index, self.data)
  }

  pub fn adc_8018(&mut self) -> Adc8018 {
    Adc8018::new(self.index, self.data)
  }

  pub fn adc_882(&mut self) -> Adc882 {
    Adc882::new(self.index, self.data)
  }
}
