pub trait AsNetworkByteOrder {
    type Bytes: AsRef<[u8]>;
    fn bytes(&self) -> Self::Bytes;
}

impl AsNetworkByteOrder for usize {
    type Bytes = [u8; 8];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl AsNetworkByteOrder for u8 {
    type Bytes = [u8; 1];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
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

impl AsNetworkByteOrder for u128 {
    type Bytes = [u8; 16];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl AsNetworkByteOrder for isize {
    type Bytes = [u8; 8];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl AsNetworkByteOrder for i8 {
    type Bytes = [u8; 1];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl AsNetworkByteOrder for i16 {
    type Bytes = [u8; 2];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl AsNetworkByteOrder for i32 {
    type Bytes = [u8; 4];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl AsNetworkByteOrder for i64 {
    type Bytes = [u8; 8];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}

impl AsNetworkByteOrder for i128 {
    type Bytes = [u8; 16];
    fn bytes(&self) -> Self::Bytes {
        return self.to_be_bytes();
    }
}