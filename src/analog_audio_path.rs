use crate::bitmask::BitMask;
use crate::EnableDisable;

pub struct InputSelect<'a> {
    index: u16,
    bitmask: BitMask<'a>,
}

impl<'a> InputSelect<'a> {
    pub fn new(index: u16, data: &'a mut u16) -> Self {
        let bitmask = BitMask::new(data);

        InputSelect { index, bitmask }
    }

    pub fn mic(&mut self) {
        self.bitmask.set(self.index);
    }

    pub fn line_input(&mut self) {
        self.bitmask.unset(self.index);
    }
}

pub struct DacSelect<'a> {
    index: u16,
    bitmask: BitMask<'a>,
}

impl<'a> DacSelect<'a> {
    pub fn new(index: u16, data: &'a mut u16) -> Self {
        let bitmask = BitMask::new(data);

        DacSelect { index, bitmask }
    }

    pub fn select(&mut self) {
        self.bitmask.set(self.index);
    }

    pub fn deselect(&mut self) {
        self.bitmask.unset(self.index);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AnalogAudioPath {
    pub(crate) data: u16,
}

impl AnalogAudioPath {
    pub fn new() -> Self {
        AnalogAudioPath {
            data: 0b0_0000_0000,
        }
    }

    pub fn mic_boost(&mut self) -> EnableDisable {
        EnableDisable::new(0, &mut self.data)
    }

    pub fn mute_mic(&mut self) -> EnableDisable {
        EnableDisable::new(1, &mut self.data)
    }

    pub fn input_select(&mut self) -> InputSelect {
        InputSelect::new(2, &mut self.data)
    }

    pub fn bypass(&mut self) -> InputSelect {
        InputSelect::new(3, &mut self.data)
    }

    pub fn dac_select(&mut self) -> DacSelect {
        DacSelect::new(4, &mut self.data)
    }

    pub fn sidetone(&mut self) -> EnableDisable {
        EnableDisable::new(5, &mut self.data)
    }

    pub fn sidetone_attenuation(&mut self) {
        // TODO: figure this out
        self.data = self.data | 0b0_0000_0000
    }
}
