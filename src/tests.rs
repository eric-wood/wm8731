mod tests {
    use crate::digital_audio_interface_format::{Format, Length};
    use crate::digital_audio_path::Deemphasis;
    use crate::WM8731;

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
    fn possible_real_world() {
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
            // TODO: uncomment when implemented
            // w.volume(0);
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
