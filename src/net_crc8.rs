use crate::crc8::*;
use crate::netendian::*;

/// Note that most polynomials don't produce a checksum of all ones even when
/// the value is all ones.
const CHECKSUM_SALT: u8 = 0xFF;

/// A convenience wrapper for handling checksums of fixed-length machine words
/// in network byte order.
pub struct NetCRC8(CRC8);

impl NetCRC8 {
    /// Apply the checksum to a word, represented in network byte order. The
    /// starting value for the checksum is non-zero, to catch cases where bits
    /// flipped to all ones or all zeroes.
    pub fn of<T: NetEndian>(&self, item: T) -> u8 {
        self.0.of(item.bytes().as_ref(), CHECKSUM_SALT)
    }

    /// Polynomial coefficients are interpreted per the CRC8 implementation.
    pub fn new(coefficients: u8) -> Self {
        NetCRC8(CRC8::new(coefficients))
    }
}

impl Default for NetCRC8 {
    /// The default CRC-8 polynomial is x⁸ + x² + x + 1, sometimes called
    /// `ATM-8`.
    fn default() -> Self {
        NetCRC8(CRC8::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_flipped_fails_sum() {
        let net8 = |num: u32| {
            NetCRC8::default().of(num)
        };

        assert_ne!(net8(0x00000000), 0x00);
        assert_ne!(net8(0xFFFFFFFF), 0xFF);
    }
}