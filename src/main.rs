mod power_down;
use power_down::PowerDown;

mod line_in;
use line_in::LineIn;

mod headphone_out;
use headphone_out::HeadphoneOut;

pub struct WM8731 {}

impl WM8731 {
    pub fn power_down(c: fn(&mut PowerDown)) -> Register {
        let mut pd = PowerDown::new();
        c(&mut pd);

        Register {
            position: 9,
            value: pd.data,
        }
    }

    pub fn left_line_in(c: fn(&mut LineIn)) -> Register {
        let mut li = LineIn::new();
        c(&mut li);

        Register {
            position: 0,
            value: li.data,
        }
    }

    pub fn right_line_in(c: fn(&mut LineIn)) -> Register {
        let mut li = LineIn::new();
        c(&mut li);

        Register {
            position: 1,
            value: li.data,
        }
    }

    pub fn right_headphone_out(c: fn(&mut HeadphoneOut)) -> Register {
        let mut ho = HeadphoneOut::new();
        c(&mut ho);

        Register {
            position: 2,
            value: ho.data,
        }
    }

    pub fn left_headphone_out(c: fn(&mut HeadphoneOut)) -> Register {
        let mut ho = HeadphoneOut::new();
        c(&mut ho);

        Register {
            position: 3,
            value: ho.data,
        }
    }
}

pub struct Register {
    position: u8,
    value: u16,
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

        assert_eq!(result.position, 9);
        assert_eq!(result.value, 0b0_0000_1101);
    }
}

pub fn main() {}
