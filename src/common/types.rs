use crate::{common::{EncodeToBytes, EncodingError}, utils::parse_primitive_types::{encode_unsigned_varint, read_unsigned_varint}, StrError};

impl EncodeToBytes for i16 {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl EncodeToBytes for i32 {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

impl EncodeToBytes for Option<String> {
    fn encode_to_bytes(&self) -> Vec<u8> {
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

pub struct UnsignedVarInt {
    pub val: u32
}

impl UnsignedVarInt {
    fn read_from_bytes(input: &[u8], off: &mut usize) -> Result<UnsignedVarInt, EncodingError> {
        Ok(UnsignedVarInt { val: read_unsigned_varint(input, off)? })
    }
}

impl EncodeToBytes for UnsignedVarInt {
    fn encode_to_bytes(&self) -> Vec<u8> {
        encode_unsigned_varint(self.val)
    }
}

pub struct CompactArray<T> {
    items: Some(Vec<T>), // option cause it can be null with -1 not 0
}


// Encode an Option<&[T]> as a COMPACT_ARRAY:
// - None  -> length = 0
// - Some  -> length = (N + 1) followed by N encoded elements
// i.e. empty array = [0x1]
impl<T> EncodeToBytes for CompactArray<T> 
    where T: EncodeToBytes
{
    fn encode_to_bytes(&self) -> Vec<u8> {

    let mut res: Vec<u8> = vec![];
    match &self.items {
        None => {
            res.extend(UnsignedVarInt { val: 0 }.encode_to_bytes());
        }
        Some(items) => {
            // write N + 1 as varint
            let len_field = (items.len() as u32) + 1;
            res.extend(UnsignedVarInt { val: len_field }.encode_to_bytes());

            for item in items {
                res.extend(item.encode_to_bytes());
            }
        }
    }
}
