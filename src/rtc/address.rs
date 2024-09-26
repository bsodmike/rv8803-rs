//! Device Address

const DEFAULT_ADDRESS: u8 = 0x32;

/// I2C device address
#[derive(Debug, Clone, Copy)]
#[allow(clippy::module_name_repetitions)]
pub enum SlaveAddress {
    /// Default slave address
    Default,
    /// Alternative slave address
    Alternative(u8),
}

impl SlaveAddress {
    /// Creates a [`SlaveAddress`] at the provided [`SlaveAddress::Alternative`] address.
    #[must_use]
    pub fn at_address(addr: u8) -> Self {
        Self::Alternative(addr)
    }
}

impl From<SlaveAddress> for u8 {
    fn from(slave_address: SlaveAddress) -> Self {
        match slave_address {
            SlaveAddress::Default => DEFAULT_ADDRESS,
            SlaveAddress::Alternative(address) => address,
        }
    }
}

impl PartialEq for SlaveAddress {
    fn eq(&self, other: &Self) -> bool {
        let (lhs, rhs): (u8, u8) = ((*self).into(), (*other).into());
        lhs == rhs
    }
}

#[cfg(test)]
mod tests {
    use super::SlaveAddress;

    #[test]
    fn satisfies_partialeq() {
        let default_address = SlaveAddress::Default;
        let alt_address = SlaveAddress::Alternative(0x32);

        assert_eq!(default_address, alt_address);

        let alt_address = SlaveAddress::at_address(0x032);
        assert_eq!(default_address, alt_address);
    }
}
