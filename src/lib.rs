#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CRC8 {
    table: [u8; 256],
}

impl CRC8 {
    /// Apply the checksum to a stream of bytes, returning the result.
    pub fn of<Bytes: IntoIterator<Item = u8>>(&self, bytes: Bytes) -> u8 {
        let mut accum: u8 = 0x00;
        for byte in bytes {
            let combined = byte ^ accum;
            accum = self.table[combined as usize];
        }
        return accum;
    }

    /// Polynomial coefficients are interpreted per a convention where the
    /// coefficient for x⁸ is assumed to be 1. In this convention,
    /// x⁸ + x² + x + 1 is represented as `0x07` (`0b00000111`).
    pub fn per(coefficients: u8) -> Self {
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

    pub fn check<T: AsNetworkByteOrder>(&self, t: T) -> u8 {
        return self.of(t.bytes());
    }
}

pub trait AsNetworkByteOrder {
    type Bytes: IntoIterator<Item = u8>;
    fn bytes(&self) -> Self::Bytes;
}

impl AsNetworkByteOrder for u16 {
    type Bytes = [u8; 2];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl AsNetworkByteOrder for u32 {
    type Bytes = [u8; 4];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl AsNetworkByteOrder for u64 {
    type Bytes = [u8; 8];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_two() {
        let atm8 = CRC8::per(0x07);

        assert_eq!(atm8.check(0x30313233 as u32), 0x69);
        assert_eq!(atm8.check(0x31313233 as u32), 0x7F);
    }
}