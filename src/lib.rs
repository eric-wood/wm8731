//! A simple HAL for the Cirrus Logic/Wolfson WM8731 audio codec
//!
//! This crate provides simple abstractions for the various control registers on the WM8731.
//!
//! Each function returns a [`Register`] struct, representing the address and value for the register.
//! With the exception of `active` and `reset`, registers accept a function for configuration.
//!
//! For example, to power on/off certain features, we can call `power_down`:
//!
//! ```
//! # use wm8731::WM8731;
//! WM8731::power_down(|c| {
//!     c.line_input().enable();
//!     c.adc().enable();
//!     c.dac().enable();
//! });
//! ```

mod bitmask;

pub mod enable_disable;
pub use enable_disable::EnableDisable;

pub mod line_in;
use line_in::LineIn;

pub mod headphone_out;
use headphone_out::HeadphoneOut;

pub mod analog_audio_path;
use analog_audio_path::AnalogAudioPath;

pub mod digital_audio_path;
use digital_audio_path::DigitalAudioPath;

pub mod power_down;
use power_down::PowerDown;

pub mod digital_audio_interface_format;
use digital_audio_interface_format::DigitalAudioInterfaceFormat;

pub mod sampling;
use sampling::Sampling;

pub mod active;
use active::Active;

pub mod sampling_rate;
use sampling_rate::SamplingRate;

pub struct Register {
    pub address: u8,
    pub value: u16,
}

#[cfg(test)]
mod tests;

/// A simple HAL for the Cirrus Logic/ Wolfson WM8731 audio codec
pub struct WM8731 {}

impl WM8731 {
    /// Left line input control register
    pub fn left_line_in<F: FnOnce(&mut LineIn)>(c: F) -> Register {
        let mut li = LineIn::new();
        c(&mut li);

        Register {
            address: 0,
            value: li.data,
        }
    }

    /// Right line in control register
    pub fn right_line_in<F: FnOnce(&mut LineIn)>(c: F) -> Register {
        let mut li = LineIn::new();
        c(&mut li);

        Register {
            address: 1,
            value: li.data,
        }
    }

    /// Left headphone out control register
    pub fn left_headphone_out<F: FnOnce(&mut HeadphoneOut)>(c: F) -> Register {
        let mut lho = HeadphoneOut::new();
        c(&mut lho);

        Register {
            address: 2,
            value: lho.data,
        }
    }

    /// Right headphone out control register
    pub fn right_headphone_out<F: FnOnce(&mut HeadphoneOut)>(c: F) -> Register {
        let mut rho = HeadphoneOut::new();
        c(&mut rho);

        Register {
            address: 3,
            value: rho.data,
        }
    }

    /// Analog audio path control register
    pub fn analog_audio_path<F: FnOnce(&mut AnalogAudioPath)>(c: F) -> Register {
        let mut aap = AnalogAudioPath::new();
        c(&mut aap);

        Register {
            address: 4,
            value: aap.data,
        }
    }

    /// Digital audio path control register
    pub fn digital_audio_path<F: FnOnce(&mut DigitalAudioPath)>(c: F) -> Register {
        let mut dap = DigitalAudioPath::new();
        c(&mut dap);

        Register {
            address: 5,
            value: dap.data,
        }
    }

    /// Power down control register
    pub fn power_down<F: FnOnce(&mut PowerDown)>(c: F) -> Register {
        let mut pd = PowerDown::new();
        c(&mut pd);

        Register {
            address: 6,
            value: pd.data,
        }
    }

    /// Digital audio interface format control register
    pub fn digital_audio_interface_format<F: FnOnce(&mut DigitalAudioInterfaceFormat)>(
        c: F,
    ) -> Register {
        let mut daif = DigitalAudioInterfaceFormat::new();
        c(&mut daif);

        Register {
            address: 7,
            value: daif.data,
        }
    }

    /// Sampling control register
    pub fn sampling<F: FnOnce(&mut Sampling)>(c: F) -> Register {
        let mut s = Sampling::new();
        c(&mut s);

        Register {
            address: 8,
            value: s.data,
        }
    }

    /// Active control register
    pub fn active() -> Active {
        Active::new(9)
    }

    /// Reset register
    pub fn reset() -> Register {
        Register {
            address: 15,
            value: 0,
        }
    }
}
