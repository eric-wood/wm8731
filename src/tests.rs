extern crate std;

use std::panic::catch_unwind;

use crate::WM8731;

use super::*;

#[test]
fn power_down() {
    let result = WM8731::power_down(|c| {
        c.line_input().power_off();
        c.adc().power_off();
        c.dac().power_off();
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
fn possible_real_world() {
    fn final_power_settings(w: &mut power_down::PowerDown) {
        w.power_off().power_on();
        w.clock_output().power_off();
        w.oscillator().power_off();
        w.output().power_on();
        w.dac().power_on();
        w.adc().power_on();
        w.mic().power_off();
        w.line_input().power_on();
    }

    let result = WM8731::reset();
    assert_eq!(result.address, 0xf /* reset */);
    assert_eq!(result.value, 0);

    let result = WM8731::power_down(|w| {
        final_power_settings(w);
        w.output().power_off();
    });
    assert_eq!(result.address, 0x6 /* power down */);
    assert_eq!(result.value, 0b0_0111_0010);

    // disable input mute, set to 0dB gain
    let result = WM8731::left_line_in(|w| {
        w.both().disable();
        w.mute().disable();
        w.volume().nearest_dB(0);
    });
    assert_eq!(result.address, 0x0 /* left line in */);
    assert_eq!(result.value, 0b0_0001_0111);

    // sidetone off; DAC selected; bypass off; line input selected; mic muted; mic boost off
    let result = WM8731::analog_audio_path(|w| {
        w.sidetone().disable();
        w.dac_select().select();
        w.bypass().disable();
        w.input_select().line_input();
        w.mute_mic().enable();
        w.mic_boost().disable();
    });
    assert_eq!(result.address, 0x4 /* analogue audio path */);
    assert_eq!(result.value, 0b0_0001_0010);

    // disable DAC mute, deemphasis for 48k
    let result = WM8731::digital_audio_path(|w| {
        w.dac_mut();
        w.deemphasis().frequency_48();
    });
    assert_eq!(result.address, 0x5 /* digital audio path */);
    assert_eq!(result.value, 0b0_0000_0110);

    // nothing inverted, slave, 24-bits, MSB format
    let result = WM8731::digital_audio_interface_format(|w| {
        w.bit_clock_invert().no_invert();
        w.master_slave().slave();
        w.left_right_dac_clock_swap().right_channel_dac_data_right();
        w.left_right_phase().data_when_daclrc_low();
        w.bit_length().bits_24();
        w.format().left_justified();
    });
    assert_eq!(result.address, 0x7 /* digital audio interface */);
    assert_eq!(result.value, 0b0_0000_1001);

    // no clock division, normal mode, 48k
    let result = WM8731::sampling(|w| {
        w.core_clock_divider_select().normal();
        w.base_oversampling_rate().normal_256();
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
    let result = WM8731::power_down(final_power_settings);
    assert_eq!(result.address, 0x6 /* power down */);
    assert_eq!(result.value, 0b0_0110_0010);
}

#[test]
fn line_input_volume() {
    // Make sure valid values result in the expected bitfields
    let result = WM8731::left_line_in(|w| w.volume().nearest_dB(0));
    assert_eq!(result.value, 0b1_0111);

    let result = WM8731::left_line_in(|w| w.volume().nearest_dB(12));
    assert_eq!(result.value, 0b1_1111);

    let result = WM8731::left_line_in(|w| w.volume().nearest_dB(-34));
    assert_eq!(result.value, 0b0_0000);

    // Make sure that in-between values get rounded
    // 1dB gets rounded up to 1.5dB
    let result = WM8731::left_line_in(|w| w.volume().nearest_dB(1));
    assert_eq!(result.value, 0b1_1000);

    // 2dB gets rounded down to 1.5dB
    let result = WM8731::left_line_in(|w| w.volume().nearest_dB(2));
    assert_eq!(result.value, 0b1_1000);

    // 3dB does not round at all
    let result = WM8731::left_line_in(|w| w.volume().nearest_dB(3));
    assert_eq!(result.value, 0b1_1001);

    // -1dB gets rounded down to -1.5dB
    let result = WM8731::left_line_in(|w| w.volume().nearest_dB(-1));
    assert_eq!(result.value, 0b1_0110);

    // -2dB gets rounded up to -1.5dB
    let result = WM8731::left_line_in(|w| w.volume().nearest_dB(-2));
    assert_eq!(result.value, 0b1_0110);

    // -3dB does not round at all
    let result = WM8731::left_line_in(|w| w.volume().nearest_dB(-3));
    assert_eq!(result.value, 0b1_0101);

    // Make sure that out-of-range values panic
    let result = catch_unwind(|| WM8731::left_line_in(|w| w.volume().nearest_dB(13)));
    assert!(result.is_err());

    let result = catch_unwind(|| WM8731::left_line_in(|w| w.volume().nearest_dB(-36)));
    assert!(result.is_err());

    // Make sure that all in-range values do not panic
    for gain in -35..=12 {
        let _ = WM8731::left_line_in(|w| w.volume().nearest_dB(gain));
    }
}
