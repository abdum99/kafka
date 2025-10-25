use crate::common::{request::KafRequestHeader, response::response_body::KafResponseBody, EncodeToBytes};

#[derive(Debug, Default)]
pub struct KafResponseHeader {
    pub correlation_id: i32,
}

impl KafResponseHeader {
    pub fn from_request_header(request_header: KafRequestHeader) -> KafResponseHeader {
        KafResponseHeader { 
            correlation_id: request_header.correlation_id,
        }
    }
}

impl EncodeToBytes for KafResponseHeader {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.correlation_id.encode_to_bytes()
    }
}

#[derive(Debug, Default)]
pub struct KafResponse {
    pub header: KafResponseHeader,
    pub body: KafResponseBody,
}

impl KafResponse {
    pub fn new(header: KafResponseHeader, body: KafResponseBody) -> KafResponse {
        KafResponse {
            header,
            body,
        }
    }
}

impl EncodeToBytes for KafResponse {
    fn encode_to_bytes(&self) -> Vec<u8> {
        // start with zero size, will be filled after serializing header and body
        let mut res: Vec<u8> = vec![]; 

        let header_bytes = self.header.encode_to_bytes();
        let body_bytes = self.body.encode_to_bytes();

        // + 4 bytes for total_length
        let total_length: i32 = (header_bytes.len() + body_bytes.len()).try_into().unwrap_or(-1i32);

        res.extend(total_length.to_be_bytes());
        res.extend(header_bytes);
        res.extend(body_bytes);

        res
    }
}
