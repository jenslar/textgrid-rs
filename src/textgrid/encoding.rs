//! TextGrid may use UTF-16.

pub enum Utf16Endian {
    Big = 0xFEFF,
    Little = 0xFFEF,
    Invalid
}

impl From<[u8; 2]> for Utf16Endian {
    fn from(bytes: [u8; 2]) -> Self {
        match u16::from_be_bytes(bytes) {
            0xFEFF => Utf16Endian::Big,
            0xFFEF => Utf16Endian::Little,
            _ => Utf16Endian::Invalid
        }
    }
}