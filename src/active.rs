//! Control to activate/deactivate interface

use crate::Register;

pub struct Active {
    address: u8,
}

impl Active {
    pub fn new(address: u8) -> Self {
        Active { address }
    }

    /// Activate interface
    pub fn inactive(&self) -> Register {
        Register {
            address: self.address,
            value: 0,
        }
    }

    /// Deactivate interface
    pub fn active(&self) -> Register {
        Register {
            address: self.address,
            value: 1,
        }
    }
}
