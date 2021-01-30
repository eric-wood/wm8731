mod power_down;
use power_down::PowerDown;

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
}

pub struct Register {
    position: i16,
    value: i16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_down() {
        let result = WM8731::power_down(|c| c.line_input());

        assert_eq!(result.position, 9);
        assert_eq!(result.value, 0b0_0000_0001);
    }
}

pub fn main() {}
