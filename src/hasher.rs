use std::hash::Hasher;

use crate::sunshine_crc8::*;

#[derive(Debug)]
pub struct CRC8Hasher {
    pub crc8: CRC8,
    pub accum: u8,
}

impl CRC8Hasher {
    pub fn new(coefficients: u8, starting_checksum: u8) -> Self {
        CRC8Hasher {
            crc8: CRC8::new(coefficients),
            accum: starting_checksum,
        }
    }
}

impl Default for CRC8Hasher {
    fn default() -> Self {
        CRC8Hasher {
            crc8: CRC8::default(),
            accum: 0x00,
        }
    }
}

impl Hasher for CRC8Hasher {
    fn finish(&self) -> u64 {
        self.accum.into()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.accum = self.crc8.of(bytes, self.accum);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate rand;
    use rand::*;

    #[test]
    fn check_pass_through_word32() {
        let crc = CRC8::default();
        let mut tests = 0;

        while tests < 1024 {
            tests += 1;
            let bytes: [u8; 4] = random();

            let mut hasher = CRC8Hasher::default();
            hasher.write(&bytes);

            assert_eq!(hasher.finish(), crc.of(&bytes, 0).into());
        }
    }

    #[test]
    fn check_pass_through_word128() {
        let crc = CRC8::default();
        let mut tests = 0;

        while tests < 1024 {
            tests += 1;
            let bytes: [u8; 16] = random();

            let mut hasher = CRC8Hasher::default();
            hasher.write(&bytes);

            assert_eq!(hasher.finish(), crc.of(&bytes, 0).into());
        }
    }
}