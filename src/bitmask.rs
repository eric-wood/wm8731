pub struct BitMask<'a> {
    data: &'a mut u16,
}

impl<'a> BitMask<'a> {
    pub fn new(data: &'a mut u16) -> Self {
        BitMask { data }
    }

    pub fn set(&mut self, index: u16) {
        *self.data = *self.data | (1 << index)
    }

    pub fn unset(&mut self, index: u16) {
        *self.data = *self.data & !(1 << index)
    }

    pub fn apply(&mut self, index: u16, length: u16, value: u16) {
        let mask = !(((1 << length) - 1) << index);
        *self.data = *self.data & mask;

        let shifted_value = (value << index) & !mask;
        *self.data = *self.data | shifted_value;
    }
}
