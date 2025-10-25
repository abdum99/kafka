use std::convert::TryFrom;
use std::str;

use crate::common::{types::UnsignedVarInt, EncodeToBytes, EncodingError};

pub fn read_exact<'a>(input: &'a [u8], off: &mut usize, n: usize) -> Result<&'a [u8], EncodingError> {
    if input.len().saturating_sub(*off) < n {
        return Err(EncodingError::Eof);
    }
    let s = &input[*off..*off + n];
    *off += n;
    Ok(s)
}

pub fn read_i16_be(input: &[u8], off: &mut usize) -> Result<i16, EncodingError> {
    let b = read_exact(input, off, 2)?;
    Ok(i16::from_be_bytes([b[0], b[1]]))
}

pub fn read_i32_be(input: &[u8], off: &mut usize) -> Result<i32, EncodingError> {
    let b = read_exact(input, off, 4)?;
    Ok(i32::from_be_bytes([b[0], b[1], b[2], b[3]]))
}

/// Kafka UNSIGNED_VARINT (LEB128-like, 7 bits per byte, MSB=continue)
pub fn read_unsigned_varint(input: &[u8], off: &mut usize) -> Result<u32, EncodingError> {
    let mut x: u64 = 0;
    let mut shift = 0u32;

    for i in 0..5 {
        // u32 fits in at most 5 bytes
        let b = *read_exact(input, off, 1)?.first().unwrap();
        let val = (b & 0x7F) as u64;

        x |= val << shift;
        if (b & 0x80) == 0 {
            if x > u32::MAX as u64 {
                return Err(EncodingError::VarIntOverflow);
            }
            return Ok(x as u32);
        }
        shift += 7;

        if i == 4 {
            // 5th byte had MSB=1 -> too long for u32
            return Err(EncodingError::VarIntTooLong);
        }
    }
    Err(EncodingError::VarIntTooLong)
}

/// NULLABLE_STRING: INT16 length; -1 => null; else length bytes UTF-8
pub fn read_nullable_string(input: &[u8], off: &mut usize) -> Result<Option<String>, EncodingError> {
    let len = read_i16_be(input, off)? as i32;
    if len == -1 {
        return Ok(None);
    }
    if len < 0 {
        return Err(EncodingError::InvalidLength(len as i64));
    }
    let len = usize::try_from(len).map_err(|_| EncodingError::InvalidLength(len as i64))?;
    let bytes = read_exact(input, off, len)?;
    let s = str::from_utf8(bytes)?.to_string();
    Ok(Some(s))
}

/* -------- COMPACT_ARRAY[TAGGED_FIELD] helpers ---------- */
#[derive(Debug)]
pub struct TaggedField {
    pub tag: u32,
    pub data: Vec<u8>,
}

/// COMPACT_ARRAY: first length is UNSIGNED_VARINT of (N + 1).
/// 0 => null array; else N elements follow.
///
/// For TaggedField element encoding (Kafka flexible):
///   tag_id: UNSIGNED_VARINT
///   size:   UNSIGNED_VARINT
///   data:   [u8; size]
pub fn read_compact_tag_buffer(
    input: &[u8],
    off: &mut usize,
) -> Result<Option<Vec<TaggedField>>, EncodingError> {
    let len_plus_one = read_unsigned_varint(input, &mut *off)?;
    if len_plus_one == 0 {
        return Ok(None); // null array
    }
    let n = len_plus_one - 1;
    let mut out = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let tag = read_unsigned_varint(input, off)?;
        let size = read_unsigned_varint(input, off)?;
        let size_usize =
            usize::try_from(size).map_err(|_| EncodingError::InvalidLength(size as i64))?;
        let data = read_exact(input, off, size_usize)?.to_vec();
        out.push(TaggedField { tag, data });
    }
    Ok(Some(out))
}

pub fn encode_unsigned_varint(mut input: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity(5); // u32 fits in ≤5 bytes
    while input >= 0x80 {
        out.push(((input as u8) & 0x7F) | 0x80); // set continuation bit
        input >>= 7;
    }
    out.push(input as u8); // last byte without continuation
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_examples() {
        // 0 -> [0x00]
        assert_eq!(encode_unsigned_varint(0), vec![0x00]);

        // 1 -> [0x01]
        assert_eq!(encode_unsigned_varint(1), vec![0x01]);

        // 127 -> [0x7F]
        assert_eq!(encode_unsigned_varint(127), vec![0x7F]);

        // 128 -> [0x80, 0x01]
        assert_eq!(encode_unsigned_varint(128), vec![0x80, 0x01]);

        // 300 -> [0xAC, 0x02]
        assert_eq!(encode_unsigned_varint(300), vec![0xAC, 0x02]);
    }
}
