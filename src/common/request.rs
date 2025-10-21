use crate::utils::parse_primitive_types::*;

#[derive(Debug)]
pub struct KafRequestHeader {
    pub request_api_key: i16,           // INT16 (big-endian)
    pub request_api_version: i16,       // INT16 (big-endian)
    pub correlation_id: i32,            // INT32 (big-endian)
    pub client_id: Option<String>,      // NULLABLE_STRING
    pub tags: Option<Vec<TaggedField>>, // COMPACT_ARRAY (nullable)
}

impl ReadFromU8 for KafRequestHeader {
    // Parse a request header from `input`, returning the header and the number of bytes consumed.
    fn read_from_u8(input: &[u8], mut offset: &mut usize) -> Result<KafRequestHeader, ParseError> {
        let request_api_key = read_i16_be(input, &mut offset)?;
        let request_api_version = read_i16_be(input, &mut offset)?;
        let correlation_id = read_i32_be(input, &mut offset)?;
        let client_id = read_nullable_string(input, &mut offset)?;

        // TAG_BUFFER: COMPACT_ARRAY of TaggedField
        let tags = read_compact_tag_buffer(input, &mut offset)?;

        Ok(KafRequestHeader {
                request_api_key,
                request_api_version,
                correlation_id,
                client_id,
                tags,
            },
        )
    }
}

#[derive(Debug)]
pub struct KafRequest {
    pub header: KafRequestHeader,
}

impl ReadFromU8 for KafRequest {
    fn read_from_u8(input: &[u8], mut offset: &mut usize) -> Result<KafRequest, ParseError> {
        let header = KafRequestHeader::read_from_u8(input, &mut offset).unwrap();

        Ok(KafRequest {
            header,
        })
    }
}

/* ---------------------- demo ---------------------- */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_header_example() {
        // Build a buffer: api_key=1, api_version=2, correlation_id=3, client_id="abc",
        // tags = [ (tag=1, data=[0xAA,0xBB]), (tag=7, data=[]) ]
        let mut buf = Vec::new();

        // INT16 1,2
        buf.extend_from_slice(&1i16.to_be_bytes());
        buf.extend_from_slice(&2i16.to_be_bytes());

        // INT32 3
        buf.extend_from_slice(&3i32.to_be_bytes());

        // NULLABLE_STRING "abc"
        let s = "abc".as_bytes();
        buf.extend_from_slice(&(s.len() as i16).to_be_bytes());
        buf.extend_from_slice(s);

        // COMPACT_ARRAY length = (N+1) as uvarint where N=2 -> 3
        write_uvar(&mut buf, 3);

        // tagged field 0: tag=1, size=2, data=[0xAA,0xBB]
        write_uvar(&mut buf, 1);
        write_uvar(&mut buf, 2);
        buf.extend_from_slice(&[0xAA, 0xBB]);

        // tagged field 1: tag=7, size=0, data=[]
        write_uvar(&mut buf, 7);
        write_uvar(&mut buf, 0);

        let mut offset = 0;
        let hdr = KafRequestHeader::read_from_u8(&buf, &mut offset).unwrap();
        assert_eq!(offset, buf.len());
        assert_eq!(hdr.request_api_key, 1);
        assert_eq!(hdr.request_api_version, 2);
        assert_eq!(hdr.correlation_id, 3);
        assert_eq!(hdr.client_id.as_deref(), Some("abc"));
        let tags = hdr.tags.unwrap();
        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0].tag, 1);
        assert_eq!(tags[0].data, vec![0xAA, 0xBB]);
        assert_eq!(tags[1].tag, 7);
        assert!(tags[1].data.is_empty());
    }

    fn write_uvar(buf: &mut Vec<u8>, mut x: u32) {
        // Unsigned varint (Kafka)
        loop {
            let mut byte = (x & 0x7F) as u8;
            x >>= 7;
            if x != 0 {
                byte |= 0x80;
                buf.push(byte);
            } else {
                buf.push(byte);
                break;
            }
        }
    }
}
