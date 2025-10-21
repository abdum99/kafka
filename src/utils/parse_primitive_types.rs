use std::convert::TryFrom;
use std::str;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("unexpected end of input")]
    Eof,
    #[error("utf-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("varint too long")]
    VarIntTooLong,
    #[error("varint overflow")]
    VarIntOverflow,
    #[error("invalid length: {0}")]
    InvalidLength(i64),
}

pub trait ReadFromU8 {
    fn read_from_u8(input: &[u8], offset: &mut usize) -> Result<Self, ParseError>
        where Self: Sized;
}

pub fn read_exact<'a>(input: &'a [u8], off: &mut usize, n: usize) -> Result<&'a [u8], ParseError> {
    if input.len().saturating_sub(*off) < n {
        return Err(ParseError::Eof);
    }
    let s = &input[*off..*off + n];
    *off += n;
    Ok(s)
}

pub fn read_i16_be(input: &[u8], off: &mut usize) -> Result<i16, ParseError> {
    let b = read_exact(input, off, 2)?;
    Ok(i16::from_be_bytes([b[0], b[1]]))
}

pub fn read_i32_be(input: &[u8], off: &mut usize) -> Result<i32, ParseError> {
    let b = read_exact(input, off, 4)?;
    Ok(i32::from_be_bytes([b[0], b[1], b[2], b[3]]))
}

/// Kafka UNSIGNED_VARINT (LEB128-like, 7 bits per byte, MSB=continue)
pub fn read_unsigned_varint(input: &[u8], off: &mut usize) -> Result<u32, ParseError> {
    let mut x: u64 = 0;
    let mut shift = 0u32;

    for i in 0..5 {
        // u32 fits in at most 5 bytes
        let b = *read_exact(input, off, 1)?.first().unwrap();
        let val = (b & 0x7F) as u64;

        x |= val << shift;
        if (b & 0x80) == 0 {
            if x > u32::MAX as u64 {
                return Err(ParseError::VarIntOverflow);
            }
            return Ok(x as u32);
        }
        shift += 7;

        if i == 4 {
            // 5th byte had MSB=1 -> too long for u32
            return Err(ParseError::VarIntTooLong);
        }
    }
    Err(ParseError::VarIntTooLong)
}

/// NULLABLE_STRING: INT16 length; -1 => null; else length bytes UTF-8
pub fn read_nullable_string(input: &[u8], off: &mut usize) -> Result<Option<String>, ParseError> {
    let len = read_i16_be(input, off)? as i32;
    if len == -1 {
        return Ok(None);
    }
    if len < 0 {
        return Err(ParseError::InvalidLength(len as i64));
    }
    let len = usize::try_from(len).map_err(|_| ParseError::InvalidLength(len as i64))?;
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
) -> Result<Option<Vec<TaggedField>>, ParseError> {
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
            usize::try_from(size).map_err(|_| ParseError::InvalidLength(size as i64))?;
        let data = read_exact(input, off, size_usize)?.to_vec();
        out.push(TaggedField { tag, data });
    }
    Ok(Some(out))
}
