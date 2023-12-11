/// Types which have a canonical network byte-order representation.
pub trait NetEndian {
    type Bytes: AsRef<[u8]>;
    fn bytes(&self) -> Self::Bytes;
}

impl NetEndian for usize {
    type Bytes = [u8; (usize::BITS / 8) as usize];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl NetEndian for u8 {
    type Bytes = [u8; 1];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl NetEndian for u16 {
    type Bytes = [u8; 2];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl NetEndian for u32 {
    type Bytes = [u8; 4];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl NetEndian for u64 {
    type Bytes = [u8; 8];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl NetEndian for u128 {
    type Bytes = [u8; 16];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl NetEndian for isize {
    type Bytes = [u8; (isize::BITS / 8) as usize];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl NetEndian for i8 {
    type Bytes = [u8; 1];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl NetEndian for i16 {
    type Bytes = [u8; 2];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl NetEndian for i32 {
    type Bytes = [u8; 4];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl NetEndian for i64 {
    type Bytes = [u8; 8];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl NetEndian for i128 {
    type Bytes = [u8; 16];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}
