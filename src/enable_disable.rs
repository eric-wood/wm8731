use crate::bitmask::BitMask;

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
