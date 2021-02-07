use crate::bitmask::BitMask;
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

pub struct LeftRightPhase<'a> {
    index: u16,
    bitmask: BitMask<'a>,
}

impl<'a> LeftRightPhase<'a> {
    pub fn new(index: u16, data: &'a mut u16) -> Self {
        let bitmask = BitMask::new(data);

        LeftRightPhase { index, bitmask }
    }

    /// In I²S mode, right channel DAC data when DACLRC is high
    pub fn data_when_daclrc_high(&mut self) {
        self.bitmask.set(self.index);
    }

    /// In I²S mode, right channel DAC data when DACLRC is low
    pub fn data_when_daclrc_low(&mut self) {
        self.bitmask.unset(self.index);
    }

    /// In DSP mode, MSB is available on 2nd BCLK rising edge after DACLRC rising edge
    pub fn data_on_second_rising_edge(&mut self) {
        self.bitmask.set(self.index);
    }

    /// In DSP mode, MSB is available on 1st BCLK rising edge after DACLRC rising edge
    pub fn data_on_first_rising_edge(&mut self) {
        self.bitmask.unset(self.index);
    }
}

pub struct ClockSwap<'a> {
    index: u16,
    bitmask: BitMask<'a>,
}

impl<'a> ClockSwap<'a> {
    pub fn new(index: u16, data: &'a mut u16) -> Self {
        let bitmask = BitMask::new(data);

        ClockSwap { index, bitmask }
    }

    pub fn right_channel_dac_data_left(&mut self) {
        self.bitmask.set(self.index);
    }

    pub fn right_channel_dac_data_right(&mut self) {
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

    /// Audio data format select
    pub fn format(&mut self, format: Format) {
        let bits = match format {
            Format::DSP => 0b11,
            Format::I2S => 0b10,
            Format::LeftJustified => 0b01,
            Format::RightJustified => 0b00,
        };

        self.data |= bits
    }

    /// Input audio data bit length select
    pub fn bit_length(&mut self, length: Length) {
        let bits = match length {
            Length::Bits32 => 0b11,
            Length::Bits24 => 0b10,
            Length::Bits20 => 0b01,
            Length::Bits16 => 0b00,
        };

        self.data |= bits << 2
    }

    /// DACLRC phase control (in left, right, or I²S modes)
    pub fn left_right_phase(&mut self) -> LeftRightPhase {
        LeftRightPhase::new(4, &mut self.data)
    }

    /// DAC left/right clock swap
    pub fn left_right_dac_clock_swap(&mut self) -> ClockSwap {
        ClockSwap::new(5, &mut self.data)
    }

    /// Master slave mode control
    pub fn master(&mut self) -> EnableDisable {
        EnableDisable::new(6, &mut self.data)
    }

    /// Bit clock invert
    pub fn bit_clock_invert(&mut self) -> EnableDisable {
        EnableDisable::new(7, &mut self.data)
    }
}
