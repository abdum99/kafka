use crate::{common::{DecodeFromBytes, EncodeToBytes, EncodingError}, utils::parse_primitive_types::{encode_unsigned_varint, read_string_exact, read_unsigned_varint}, StrError};

// TODO: @abdu
// For this I should write ser::Serializer, call KafWireSerializer
// Then create a few KafPrimitiveTypes: [UnsignedVarInt, CompactArray, ...]
// then implement Serialize for each KafPrimitiveTypes
// And produce the output as bytes, then all serializer.serialize_bytes()
// (even better you wouldn't hardcode serializing to bytes,
//  instead you serialize to some intermediary that encapsulates the meaning
//  e.g. for a CompactArray of value null it produces -1,
//  but for a single element array it produces (N + 1, element)
//  but there are some quirks to figure out there
//  and enums might be helpful
//  )

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

impl EncodeToBytes for bool {
    fn encode_to_bytes(&self) -> Vec<u8> {
        match self {
            false => vec![0x0],
            true => vec![0x1],
        }
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

#[derive(Debug)]
// TODO: This shouldn't be u32, it should u64 or [u8] if actually unbounded
pub struct UnsignedVarInt(pub u32);

impl DecodeFromBytes for UnsignedVarInt {
    fn read_from_u8(
        input: &[u8],
        off: &mut usize,
    ) -> Result<UnsignedVarInt, EncodingError> {
        Ok(UnsignedVarInt(read_unsigned_varint(input, off)?))
    }
}

impl EncodeToBytes for UnsignedVarInt {
    fn encode_to_bytes(&self) -> Vec<u8> {
        encode_unsigned_varint(self.0)
    }
}


#[derive(Debug, Default, Clone)]
pub struct CompactString(pub String);

impl DecodeFromBytes for CompactString {
    fn read_from_u8(input: &[u8], offset: &mut usize) -> Result<Self, EncodingError> {
        let length: u32 = read_unsigned_varint(input, offset)?;
        Ok(CompactString(
            read_string_exact(input, offset, length - 1)?, // encoded as N + 1
        ))
    }
}

impl EncodeToBytes for CompactString {
    fn encode_to_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![];
        let bytes = self.0.as_bytes().to_vec();
        res.extend(UnsignedVarInt(bytes.len() as u32 + 1).encode_to_bytes());
        res.extend(bytes);

        res
    }
}


// option cause it can be null with -1 not 0
#[derive(Debug, Clone)]
pub struct CompactArray<T>(pub Option<Vec<T>>); 

impl<T> Default for CompactArray<T> {
    fn default() -> Self {
        CompactArray(None)
    }
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
        match &self.0 {
            None => {
                res.extend(UnsignedVarInt(0).encode_to_bytes());
            },
            Some(items) => {
                // write N + 1 as varint
                let len_field = (items.len() as u32) + 1;
                res.extend(UnsignedVarInt(len_field).encode_to_bytes());

                for item in items {
                    res.extend(item.encode_to_bytes());
                }
            }
        }

        println!("printing compact array: {:#?}", res);
        res
    }
}

impl<T> DecodeFromBytes for CompactArray<T> 
    where T: DecodeFromBytes
{
    fn read_from_u8(input: &[u8], offset: &mut usize) -> Result<Self, EncodingError> {
        let length = UnsignedVarInt::read_from_u8(input, offset)?.0;

        if length == 0 {
            return Ok(CompactArray(None));
        }

        let mut items_arr: Vec<T> = vec![];

        for _ in 0..length - 1 {
            items_arr.push(T::read_from_u8(input, offset)?);
        }

        Ok(CompactArray(Some(items_arr)))
    }
}

