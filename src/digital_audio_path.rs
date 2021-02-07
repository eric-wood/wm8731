use crate::bitmask::BitMask;
use crate::EnableDisable;

pub enum Deemphasis {
    SampleRate48,
    SampleRate441,
    SampleRate32,
    Disable,
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

    pub fn adc_hpf(&mut self) -> EnableDisable {
        EnableDisable::new(0, &mut self.data)
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

    pub fn dac_mut(&mut self) -> EnableDisable {
        EnableDisable::new(3, &mut self.data)
    }

    pub fn hpor(&mut self) -> HpfDc {
        HpfDc::new(4, &mut self.data)
    }
}
