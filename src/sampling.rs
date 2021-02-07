use crate::bitmask::BitMask;
use crate::EnableDisable;
use crate::SamplingRate;

pub struct UsbNormal<'a> {
    index: u16,
    bitmask: BitMask<'a>,
}

impl<'a> UsbNormal<'a> {
    pub fn new(index: u16, data: &'a mut u16) -> Self {
        let bitmask = BitMask::new(data);

        UsbNormal { index, bitmask }
    }

    pub fn usb(&mut self) {
        self.bitmask.set(self.index);
    }

    pub fn normal(&mut self) {
        self.bitmask.unset(self.index);
    }
}

pub struct ClockDivider<'a> {
    index: u16,
    bitmask: BitMask<'a>,
}

impl<'a> ClockDivider<'a> {
    pub fn new(index: u16, data: &'a mut u16) -> Self {
        let bitmask = BitMask::new(data);

        ClockDivider { index, bitmask }
    }

    pub fn divided_by_two(&mut self) {
        self.bitmask.set(self.index);
    }

    pub fn noraml(&mut self) {
        self.bitmask.unset(self.index);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Sampling {
    pub(crate) data: u16,
}

impl Sampling {
    pub fn new() -> Self {
        Sampling {
            data: 0b0_0000_0000,
        }
    }

    pub fn usb_normal(&mut self) -> UsbNormal {
        UsbNormal::new(0, &mut self.data)
    }

    pub fn base_oversampling_rate(&mut self) -> EnableDisable {
        EnableDisable::new(1, &mut self.data)
    }

    pub fn sample_rate(&mut self) -> SamplingRate {
        SamplingRate::new(2, &mut self.data)
    }

    pub fn core_clock_divider_select(&mut self) -> ClockDivider {
        ClockDivider::new(6, &mut self.data)
    }

    pub fn clock_out_divider_select(&mut self) -> ClockDivider {
        ClockDivider::new(7, &mut self.data)
    }
}
