use bincode::Options;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Header {
    pub correlation_id: i32,
}

#[derive(Serialize, Debug)]
pub struct Response {
    pub message_size: i32,
    pub header: Header,
}

impl Response {
    pub fn new(message_size: i32, correlation_id: i32) -> Response {
        Response {
            message_size,
            header: Header {
                correlation_id,
            },
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        // Use big-endian instead of default little-endian
        let big_endian = bincode::DefaultOptions::new()
            .with_fixint_encoding() // fixed width (no varint)
            .with_big_endian();     // 👈 big endian
        //
        big_endian.serialize(&[self.message_size, self.header.correlation_id])
            .expect(&format!("failed to serialize response: {:#?}", self))
    }
}

