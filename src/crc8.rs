/// A standalone, single-file implementation of CRC-8.

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CRC8 {
    table: [u8; 256],
}

impl CRC8 {
    /// Apply the checksum to a stream of bytes, returning the result.
    pub fn of(&self, bytes: &[u8], starting_checksum: u8) -> u8 {
        let mut accum = starting_checksum;
        for byte in bytes {
            let combined = byte ^ accum;
            accum = self.table[combined as usize];
        }
        return accum;
    }

    /// Polynomial coefficients are interpreted per a convention where the
    /// coefficient for x⁸ is assumed to be 1. In this convention,
    /// x⁸ + x² + x + 1 is represented as `0x07` (`0b00000111`).
    pub fn new(coefficients: u8) -> Self {
        let mut table: [u8; 256] = [0; 256];

        // We build a table containing the CRC checksum of every byte value.
        // This table is later used to checksum a byte stream byte-by-byte,
        // instead of bit-by-bit.
        for i in 0..table.len() {
            let mut byte = i as u8;

            for _ in 0..8 {
                let leading_bit_is_set = byte & 0x80 != 0;
                // Always shift; we need to get to the next bit. This means we
                // skip the first bit. This works because of the implicit (per
                // the convention) x⁸ term -- if the first bit is set, it will
                // be cancelled (XOR'd to zero) by the leading bit of the
                // polynomial. If it is not set, we just keep going till we
                // find a bit that is.
                byte <<= 1;
                if leading_bit_is_set {
                    // The leading bit was cancelled by the x⁸ term, but we
                    // still need to apply the remaining coefficients.
                    byte ^= coefficients;
                }
            }

            table[i] = byte;
        }

        return CRC8 { table: table };
    }
}

const ATM8: u8 = 0x07;

impl Default for CRC8 {
    /// The default CRC-8 polynomial is x⁸ + x² + x + 1, sometimes called
    /// `ATM-8`.
    fn default() -> Self {
        CRC8::new(ATM8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Values checked with:
    /// http://www.sunshine2k.de/coding/javascript/crc/crc_js.html
    #[test]
    fn spot_check() {
        let atm8 = |num: u32| {
            CRC8::new(ATM8).of(&num.to_be_bytes(), 0x00)
        };

        assert_eq!(atm8(0x00000000), 0x00);
        assert_eq!(atm8(0x00000001), ATM8);
        assert_eq!(atm8(0xFFFFFFFF), 0xDE);

        // Drawn from:
        // "Correcting Single-bit Errors with CRC8 in ATM Cell Headers"
        // https://www.nxp.com.cn/docs/en/application-note/AN2918.pdf
        assert_eq!(atm8(0x30313233), 0x69);
        assert_eq!(atm8(0x31313233), 0x7F);
    }
}