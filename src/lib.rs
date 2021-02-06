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
    address: u8,
    value: u16,
}

/// A simple HAL for the Cirrus Logic/ Wolfson WM8731 audio codec
pub struct WM8731 {}

impl WM8731 {
    /// Left line input control register
    pub fn left_line_in(c: fn(&mut LineIn)) -> Register {
        let mut li = LineIn::new();
        c(&mut li);

        Register {
            address: 0,
            value: li.data,
        }
    }

    /// Right line in control register
    pub fn right_line_in(c: fn(&mut LineIn)) -> Register {
        let mut li = LineIn::new();
        c(&mut li);

        Register {
            address: 1,
            value: li.data,
        }
    }

    /// Left headphone out control register
    pub fn left_headphone_out(c: fn(&mut HeadphoneOut)) -> Register {
        let mut lho = HeadphoneOut::new();
        c(&mut lho);

        Register {
            address: 2,
            value: lho.data,
        }
    }

    /// Right headphone out control register
    pub fn right_headphone_out(c: fn(&mut HeadphoneOut)) -> Register {
        let mut rho = HeadphoneOut::new();
        c(&mut rho);

        Register {
            address: 3,
            value: rho.data,
        }
    }

    /// Analog audio path control register
    pub fn analog_audio_path(c: fn(&mut AnalogAudioPath)) -> Register {
        let mut aap = AnalogAudioPath::new();
        c(&mut aap);

        Register {
            address: 4,
            value: aap.data,
        }
    }

    /// Digital audio path control register
    pub fn digital_audio_path(c: fn(&mut DigitalAudioPath)) -> Register {
        let mut dap = DigitalAudioPath::new();
        c(&mut dap);

        Register {
            address: 5,
            value: dap.data,
        }
    }

    /// Power down control register
    pub fn power_down(c: fn(&mut PowerDown)) -> Register {
        let mut pd = PowerDown::new();
        c(&mut pd);

        Register {
            address: 6,
            value: pd.data,
        }
    }

    /// Digital audio interface format control register
    pub fn digital_audio_interface_format(c: fn(&mut DigitalAudioInterfaceFormat)) -> Register {
        let mut daif = DigitalAudioInterfaceFormat::new();
        c(&mut daif);

        Register {
            address: 7,
            value: daif.data,
        }
    }

    /// Sampling control register
    pub fn sampling(c: fn(&mut Sampling)) -> Register {
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
            address: 9,
            value: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use digital_audio_interface_format::{Format, Length};
    use digital_audio_path::Deemphasis;

    use super::*;

    #[test]
    fn power_down() {
        let result = WM8731::power_down(|c| {
            c.line_input().enable();
            c.adc().enable();
            c.dac().enable();
        });

        assert_eq!(result.address, 6);
        assert_eq!(result.value, 0b0_0000_1101);
    }

    #[test]
    fn sampling_rate() {
        let result = WM8731::sampling(|c| {
            c.usb_normal().normal();
            c.sample_rate().adc_96().dac_96();
        });

        assert_eq!(result.address, 8);
        assert_eq!(result.value, 0b0_0001_1100);
    }

    #[test]
    fn echo_pedal_registers() {
        let result = WM8731::reset();
        assert_eq!(result.address, 0xf /* reset */);
        assert_eq!(result.value, 0);

        let result = WM8731::power_down(|w| {
            w.power_off().disable();
            w.clock_output().enable();
            w.oscillator().enable();
            w.output().enable();
            w.dac().disable();
            w.adc().disable();
            w.mic().enable();
            w.line_input().disable();
        });
        assert_eq!(result.address, 0x6 /* power down */);
        assert_eq!(result.value, 0b0_0111_0010);

        // disable input mute, set to 0dB gain
        let result = WM8731::left_line_in(|w| {
            w.both().disable();
            w.mute().disable();
            w.volume(0);
        });
        assert_eq!(result.address, 0x0 /* left line in */);
        assert_eq!(result.value, 0b0_0001_0111);

        // sidetone off; DAC selected; bypass off; line input selected; mic muted; mic boost off
        let result = WM8731::analog_audio_path(|w| {
            w.sidetone().disable();
            w.dac_select().select();
            w.bypass().line_input(); // not "line_input" at all, but that's bit-clear
            w.input_select().line_input();
            w.mute_mic().enable();
            w.mic_boost().disable();
        });
        assert_eq!(result.address, 0x4 /* analogue audio path */);
        assert_eq!(result.value, 0b0_0001_0010);

        // disable DAC mute, deemphasis for 48k
        let result = WM8731::digital_audio_path(|w| {
            w.dac_mut();
            w.deemphasis(Deemphasis::SampleRate48);
        });
        assert_eq!(result.address, 0x5 /* digital audio path */);
        assert_eq!(result.value, 0b0_0000_0110);

        // nothing inverted, slave, 24-bits, MSB format
        let result = WM8731::digital_audio_interface_format(|w| {
            w.bit_clock_invert().disable();
            w.master().disable();
            w.left_right_dac_clock_swap().right();
            w.left_right_phase().disable();
            w.bit_length(Length::Bits24);
            w.format(Format::LeftJustified);
        });
        assert_eq!(result.address, 0x7 /* digital audio interface */);
        assert_eq!(result.value, 0b0_0000_1001);

        // no clock division, normal mode, 48k
        let result = WM8731::sampling(|w| {
            w.core_clock_divider_select().noraml();
            w.base_oversampling_rate().disable();
            w.sample_rate().adc_48();
            w.usb_normal().normal();
        });
        assert_eq!(result.address, 0x8 /* sampling control */);
        assert_eq!(result.value, 0b0_00_0000_00);

        // set active
        let result = WM8731::active().active();
        assert_eq!(result.address, 0x9 /* active */);
        assert_eq!(result.value, 0x1);

        // enable output
        let result = WM8731::power_down(|w| {
            w.power_off().disable();
            w.clock_output().enable();
            w.oscillator().enable();
            // it is non-obvious that output() is the only change from the earlier power_down()
            // call.
            w.output().disable();
            w.dac().disable();
            w.adc().disable();
            w.mic().enable();
            w.line_input().disable();
        });
        assert_eq!(result.address, 0x6 /* power down */);
        assert_eq!(result.value, 0b0_0110_0010);
    }
}
