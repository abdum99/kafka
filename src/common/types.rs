use crate::{common::EncodeToBytes, StrError};

pub enum KafType {
    I16(i16),
    I32(i32),
    NullableString(NullableString),
    CompactArray(CompactArray),
}

#[derive(Debug)]
pub struct NullableString {
    pub length: i16,
    pub value: Option<String>, // ideally this would be Vec<char> and override ==, +, etc, but eh
}

impl NullableString {
    pub fn new_null_string() -> NullableString {
        NullableString {
            length: -1,
            value: None,
        }
    }
}

pub struct CompactArray {

}

impl EncodeToBytes for i16 {
    fn write_to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl EncodeToBytes for i32 {
    fn write_to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl EncodeToBytes for Option<String> {
    fn write_to_bytes(&self) -> Vec<u8> {
        let mut res = vec![];
        let length: i16 = match self {
            Some(s) => s.len().try_into().expect("string too long"),
            None => -1i16,
        };

        // write length, 2 bytes
        res.extend(length.to_be_bytes());

        if let Some(s) = self {
            let utf8_vec: Vec<u8> = s.as_bytes().to_vec();
            res.extend(utf8_vec);
        }

        res
    }
}
