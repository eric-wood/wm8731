//! Configuration for the digital audio interface

use crate::bitmask::BitMask;

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

pub struct MasterSlave<'a> {
    index: u16,
    bitmask: BitMask<'a>,
}

impl<'a> MasterSlave<'a> {
    pub fn new(index: u16, data: &'a mut u16) -> Self {
        let bitmask = BitMask::new(data);

        MasterSlave { index, bitmask }
    }

    pub fn master(&mut self) {
        self.bitmask.set(self.index);
    }

    pub fn slave(&mut self) {
        self.bitmask.unset(self.index);
    }
}

pub struct Invert<'a> {
    index: u16,
    bitmask: BitMask<'a>,
}

impl<'a> Invert<'a> {
    pub fn new(index: u16, data: &'a mut u16) -> Self {
        let bitmask = BitMask::new(data);

        Invert { index, bitmask }
    }

    pub fn invert(&mut self) {
        self.bitmask.set(self.index);
    }

    pub fn no_invert(&mut self) {
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

pub struct Format<'a> {
    index: u16,
    bitmask: BitMask<'a>,
}

impl<'a> Format<'a> {
    pub fn new(index: u16, data: &'a mut u16) -> Self {
        let bitmask = BitMask::new(data);

        Format { index, bitmask }
    }

    pub fn dsp(&mut self) {
        self.bitmask.apply(self.index, 2, 0b11)
    }

    pub fn i2s(&mut self) {
        self.bitmask.apply(self.index, 2, 0b10)
    }

    pub fn left_justified(&mut self) {
        self.bitmask.apply(self.index, 2, 0b01)
    }

    pub fn right_justified(&mut self) {
        self.bitmask.apply(self.index, 2, 0b00)
    }
}

pub struct BitLength<'a> {
    index: u16,
    bitmask: BitMask<'a>,
}

impl<'a> BitLength<'a> {
    pub fn new(index: u16, data: &'a mut u16) -> Self {
        let bitmask = BitMask::new(data);

        BitLength { index, bitmask }
    }

    pub fn bits_32(&mut self) {
        self.bitmask.apply(self.index, 2, 0b11)
    }

    pub fn bits_24(&mut self) {
        self.bitmask.apply(self.index, 2, 0b10)
    }

    pub fn bits_20(&mut self) {
        self.bitmask.apply(self.index, 2, 0b01)
    }

    pub fn bits_16(&mut self) {
        self.bitmask.apply(self.index, 2, 0b00)
    }
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
    pub fn format(&mut self) -> Format {
        Format::new(0, &mut self.data)
    }

    /// Input audio data bit length select
    pub fn bit_length(&mut self) -> BitLength {
        BitLength::new(2, &mut self.data)
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
    pub fn master_slave(&mut self) -> MasterSlave {
        MasterSlave::new(6, &mut self.data)
    }

    /// Bit clock invert
    pub fn bit_clock_invert(&mut self) -> Invert {
        Invert::new(7, &mut self.data)
    }
}
