//! An implementation of 8-bit CRC, following conventions established by the
//! `sunshine2k.de` technical [note].
//!
//! [note]: http://www.sunshine2k.de/articles/coding/crc/understanding_crc.html

mod crc8;
mod netendian;
mod net_crc8;
mod hasher;

pub use crc8::CRC8;
pub use hasher::CRC8Hasher;
pub use netendian::NetEndian;
pub use net_crc8::NetCRC8;
