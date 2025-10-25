use crate::common::EncodeToBytes;

#[derive(Debug)]
pub struct KafHeader {
    pub correlation_id: i32,
}

impl EncodeToBytes for KafHeader {
    fn write_to_bytes(&self) -> Vec<u8> {
        self.correlation_id.write_to_bytes()
    }
}

#[derive(Debug)]
pub struct KafResponse {
    pub message_size: i32,
    pub header: KafHeader,
}

impl KafResponse {
    pub fn new(message_size: i32, correlation_id: i32) -> KafResponse {
        KafResponse {
            message_size,
            header: KafHeader {
                correlation_id,
            },
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut res = vec![];

        res.extend(self.message_size.write_to_bytes());

        res.extend(self.header.write_to_bytes());

        res
    }
}

