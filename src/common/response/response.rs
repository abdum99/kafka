use crate::common::{request::{self, KafRequestHeader}, response::response_body::KafResponseBody, EncodeToBytes};

#[derive(Debug)]
pub enum KafResponseHeader {
    V0(V0Header),
    V1(V1Header),
}

#[derive(Debug, Default)]
pub struct V0Header {
    pub correlation_id: i32,
}

#[derive(Debug, Default)]
pub struct V1Header {
    pub correlation_id: i32,
    pub _tagged_fields: u8,
}

impl KafResponseHeader {
    pub fn v0(request_header: KafRequestHeader) -> KafResponseHeader {
        Self::V0(V0Header { correlation_id: request_header.correlation_id })
    }

    pub fn v1(request_header: KafRequestHeader) -> KafResponseHeader {
        Self::V1(V1Header {
            correlation_id: request_header.correlation_id,
            ..Default::default()
        })
    }

    pub fn from_request_header(request_header: KafRequestHeader) -> KafResponseHeader {
        match request_header.request_api_version {
            0 => KafResponseHeader::V0(V0Header {
                correlation_id: request_header.correlation_id
            }),
            _ => KafResponseHeader::V1(V1Header {
                correlation_id: request_header.correlation_id,
                ..Default::default()
            }),
        }
    }
}

impl Default for KafResponseHeader {
    fn default() -> Self {
        Self::V0(V0Header::default())
    }
}

impl EncodeToBytes for KafResponseHeader {
    fn encode_to_bytes(&self) -> Vec<u8> {
        let mut res = vec![];
        match self {
            Self::V0(inner) => { 
                res.extend(inner.correlation_id.encode_to_bytes());
            },
            Self::V1(inner) => { 
                res.extend(inner.correlation_id.encode_to_bytes());
                res.push(inner._tagged_fields);
            }
        }
        res
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
