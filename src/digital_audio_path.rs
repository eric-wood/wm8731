//! Configuration for digital audio path

use crate::bitmask::BitMask;
use crate::EnableDisable;

pub struct Deemphasis<'a> {
    index: u16,
    bitmask: BitMask<'a>,
}

impl<'a> Deemphasis<'a> {
    pub fn new(index: u16, data: &'a mut u16) -> Self {
        let bitmask = BitMask::new(data);

        Deemphasis { index, bitmask }
    }

    pub fn frequency_48(&mut self) {
        self.bitmask.apply(self.index, 2, 0b11)
    }

    pub fn frequency_441(&mut self) {
        self.bitmask.apply(self.index, 2, 0b10)
    }

    pub fn frequency_32(&mut self) {
        self.bitmask.apply(self.index, 2, 0b01)
    }

    pub fn disable(&mut self) {
        self.bitmask.apply(self.index, 2, 0b00)
    }
}

pub struct HpfDc<'a> {
    index: u16,
    bitmask: BitMask<'a>,
}

impl<'a> HpfDc<'a> {
    pub fn new(index: u16, data: &'a mut u16) -> Self {
        let bitmask = BitMask::new(data);

        HpfDc { index, bitmask }
    }

    pub fn store(&mut self) {
        self.bitmask.set(self.index);
    }

    pub fn clear(&mut self) {
        self.bitmask.unset(self.index);
    }
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

    /// Disable ADC high pass filter. Enabling this disable the high pass filter
    pub fn adc_hpf_disable(&mut self) -> EnableDisable {
        EnableDisable::new(0, &mut self.data)
    }

    /// De-emphasis control
    pub fn deemphasis(&mut self) -> Deemphasis {
        Deemphasis::new(1, &mut self.data)
    }

    /// DAC soft mute control
    pub fn dac_mute(&mut self) -> EnableDisable {
        EnableDisable::new(3, &mut self.data)
    }

    /// Store DC offset when high pass filter disabled
    pub fn hpor(&mut self) -> HpfDc {
        HpfDc::new(4, &mut self.data)
    }
}
