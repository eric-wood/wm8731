mod power_down;
use power_down::PowerDown;

mod line_in;
use line_in::LineIn;

mod headphone_out;
use headphone_out::HeadphoneOut;

mod analog_audio_path;
use analog_audio_path::AnalogAudioPath;

mod digital_audio_path;
use digital_audio_path::DigitalAudioPath;

mod digital_audio_interface_format;
use digital_audio_interface_format::DigitalAudioInterfaceFormat;

pub struct Register {
    address: u8,
    value: u16,
}

pub struct WM8731 {}

impl WM8731 {
    pub fn left_line_in(c: fn(&mut LineIn)) -> Register {
        let mut li = LineIn::new();
        c(&mut li);

        Register {
            address: 0,
            value: li.data,
        }
    }

    pub fn right_line_in(c: fn(&mut LineIn)) -> Register {
        let mut li = LineIn::new();
        c(&mut li);

        Register {
            address: 1,
            value: li.data,
        }
    }

    pub fn left_headphone_out(c: fn(&mut HeadphoneOut)) -> Register {
        let mut lho = HeadphoneOut::new();
        c(&mut lho);

        Register {
            address: 2,
            value: lho.data,
        }
    }

    pub fn right_headphone_out(c: fn(&mut HeadphoneOut)) -> Register {
        let mut rho = HeadphoneOut::new();
        c(&mut rho);

        Register {
            address: 3,
            value: rho.data,
        }
    }

    pub fn analog_audio_path(c: fn(&mut AnalogAudioPath)) -> Register {
        let mut aap = AnalogAudioPath::new();
        c(&mut aap);

        Register {
            address: 4,
            value: aap.data,
        }
    }

    pub fn digital_audio_path(c: fn(&mut DigitalAudioPath)) -> Register {
        let mut dap = DigitalAudioPath::new();
        c(&mut dap);

        Register {
            address: 5,
            value: dap.data,
        }
    }

    pub fn power_down(c: fn(&mut PowerDown)) -> Register {
        let mut pd = PowerDown::new();
        c(&mut pd);

        Register {
            address: 6,
            value: pd.data,
        }
    }

    pub fn digital_audio_interface_format(c: fn(&mut DigitalAudioInterfaceFormat)) -> Register {
        let mut daif = DigitalAudioInterfaceFormat::new();
        c(&mut daif);

        Register {
            address: 7,
            value: daif.data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_down() {
        let result = WM8731::power_down(|c| {
            c.line_input();
            c.adc();
            c.dac();
        });

        assert_eq!(result.address, 6);
        assert_eq!(result.value, 0b0_0000_1101);
    }
}

pub fn main() {}
