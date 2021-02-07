use crate::Register;

pub struct Active {
    address: u8,
}

impl Active {
    pub fn new(address: u8) -> Self {
        Active { address }
    }

    pub fn inactive(&self) -> Register {
        Register {
            address: self.address,
            value: 0,
        }
    }

    pub fn active(&self) -> Register {
        Register {
            address: self.address,
            value: 1,
        }
    }
}
