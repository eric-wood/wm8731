//! Configuration for power to different parts of the device

use crate::bitmask::BitMask;

pub struct PowerOnOff<'a> {
    index: u16,
    bitmask: BitMask<'a>,
}

impl<'a> PowerOnOff<'a> {
    pub fn new(index: u16, data: &'a mut u16) -> Self {
        let bitmask = BitMask::new(data);

        PowerOnOff { index, bitmask }
    }

    pub fn power_off(&mut self) {
        self.bitmask.set(self.index);
    }

    pub fn power_on(&mut self) {
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

    /// Line input
    pub fn line_input(&mut self) -> PowerOnOff {
        PowerOnOff::new(0, &mut self.data)
    }

    /// Microphone input and bias
    pub fn mic(&mut self) -> PowerOnOff {
        PowerOnOff::new(1, &mut self.data)
    }

    /// ADC
    pub fn adc(&mut self) -> PowerOnOff {
        PowerOnOff::new(2, &mut self.data)
    }

    /// DAC
    pub fn dac(&mut self) -> PowerOnOff {
        PowerOnOff::new(3, &mut self.data)
    }

    /// Outputs
    pub fn output(&mut self) -> PowerOnOff {
        PowerOnOff::new(4, &mut self.data)
    }

    /// Oscillator
    pub fn oscillator(&mut self) -> PowerOnOff {
        PowerOnOff::new(5, &mut self.data)
    }

    /// CLKOUT
    pub fn clock_output(&mut self) -> PowerOnOff {
        PowerOnOff::new(6, &mut self.data)
    }

    /// POWEROFF mode
    pub fn power_off(&mut self) -> PowerOnOff {
        PowerOnOff::new(7, &mut self.data)
    }
}
