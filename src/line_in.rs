//! Configuration for line inputs

use crate::bitmask::BitMask;
use crate::EnableDisable;

#[derive(Debug, Copy, Clone)]
pub struct LineIn {
    pub(crate) data: u16,
}

pub struct Volume<'a> {
    index: u16,
    bitmask: BitMask<'a>,
}

impl<'a> Volume<'a> {
    /// Line input volume in half-dB steps.
    ///
    /// The `half_dBs` parameter value must be twice the desired dB gain; for example:
    ///    * for a 1.5dB gain, call `half_dB_steps(3)`
    ///    * for 9dB attenuation, call `half_dB_steps(-18)`
    ///
    /// # Panics
    ///
    /// Panics if the `half_dBs` parameter is out of range (below 34.5dB or above 12dB), or if the
    /// `half_dBs` parameter does not correspond to an exact multiple of 1.5dB.
    #[allow(non_snake_case)]
    fn half_dB_steps(&mut self, half_dBs: i16) {
        // The WM8731 supports -34.5dB up to 12dB.  Make sure the input is in range:
        assert!((-69..=24).contains(&half_dBs));

        let offset = half_dBs + 69;
        // and make sure the input lines up on the 1.5dB steps:
        assert_eq!(offset % 3, 0);

        self.bitmask.apply(self.index, 5, (offset / 3) as u16);
    }

    /// Set line input volume to nearest representable value
    ///
    /// Set the line input volume to the nearest gain available.  The WM8731 only supports 1.5dB
    /// steps, so the actual results may be rounded by ±0.5dB.  For example:
    ///    * 0dB, 3dB, etc. are exact multiples of 1.5dB, and will not be rounded.
    ///    * 1dB will get rounded up to 1.5dB
    ///    * 2dB will get rounded down to 1.5dB
    ///
    /// # Panics
    ///
    /// Panics if `dB_gain` is out of range (below -35 or above 12, covering the hardware's
    /// capability of -34.5dB to 12dB).
    #[allow(non_snake_case)]
    pub fn nearest_dB(&mut self, dB_gain: i16) {
        let half_dBs = dB_gain * 2;
        let rounded_to_multiple_of_three = match half_dBs.rem_euclid(3) {
            0 => half_dBs,
            1 => half_dBs - 1,
            2 => half_dBs + 1,
            x => panic!("{} cannot possibly be the result of .rem_euclid(3)", x),
        };
        self.half_dB_steps(rounded_to_multiple_of_three);
    }
}

impl LineIn {
    pub fn new() -> Self {
        LineIn {
            data: 0b0_0000_0000,
        }
    }

    /// Line input volume
    pub fn volume(&mut self) -> Volume {
        Volume {
            index: 0,
            bitmask: BitMask::new(&mut self.data),
        }
    }

    /// Line input mute to ADC
    pub fn mute(&mut self) -> EnableDisable {
        EnableDisable::new(7, &mut self.data)
    }

    /// Left to right channel line input volume and mute data load
    /// When enabled, left and right channels will have same volume and mute values
    pub fn both(&mut self) -> EnableDisable {
        EnableDisable::new(8, &mut self.data)
    }
}
