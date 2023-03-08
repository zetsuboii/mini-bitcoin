pub trait ToBytesBigEndian {
    fn to_bytes_be(&self) -> Vec<u8>;
}

impl ToBytesBigEndian for String {
    fn to_bytes_be(&self) -> Vec<u8> {
        self.as_bytes()
            .chunks(2) // Split into pairs of bytes
            .map(|chunk| {
                let hex_str = std::str::from_utf8(chunk).unwrap();
                u8::from_str_radix(hex_str, 16).unwrap()
            })
            .collect()
    }
}

impl ToBytesBigEndian for &str {
    fn to_bytes_be(&self) -> Vec<u8> {
        self.as_bytes()
            .chunks(2) // Split into pairs of bytes
            .map(|chunk| {
                let hex_str = std::str::from_utf8(chunk).unwrap();
                u8::from_str_radix(hex_str, 16).unwrap()
            })
            .collect()
    }
}
