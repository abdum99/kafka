use crate::common::{api::{api_key, api_version_entry::ApiVersionEntry}, error::error_code, response::describe_topic_partitions::DescribeTopicPartitionsResponse, types::CompactArray, EncodeToBytes};

// TODO: probably best as a builder but for later
#[derive(Debug)]
pub enum KafResponseBody {
    Unsupported(UnsupportedResponse),
    ApiVersions(ApiVersionsResponse),
    DescribeTopicPartitions(DescribeTopicPartitionsResponse)
}

impl Default for KafResponseBody {
    fn default() -> Self {
        Self::Unsupported(UnsupportedResponse::default())
    }
}

// TODO: I should make a macro for this impl once it grows
//
impl EncodeToBytes for KafResponseBody {
    fn encode_to_bytes(&self) -> Vec<u8> {
        use self::KafResponseBody::*;
        match self {
            ApiVersions(res) => res.encode_to_bytes(),
            DescribeTopicPartitions(res) => res.encode_to_bytes(),
            _ => UnsupportedResponse::with_error_code(-1).encode_to_bytes(),
        }
    }
}

#[derive(Debug, Default)]
pub struct UnsupportedResponse {
    error_code: i16,
}

impl UnsupportedResponse {
    fn with_error_code(ec: i16) -> UnsupportedResponse {
        UnsupportedResponse { error_code: ec }
    }
}

impl EncodeToBytes for UnsupportedResponse {
    fn encode_to_bytes(&self) -> Vec<u8> {
        self.error_code.to_be_bytes().to_vec()
    }
}

#[derive(Debug, Default)]
pub struct ApiVersionsResponse {
    pub error_code: i16,
    // ApiVersionEntry is maintained statically
    // TODO: is it worth making KafResponse generic over lifetimes
    pub api_keys: CompactArray<&'static ApiVersionEntry>,
    pub throttle_time: i32,
    pub tagged_buffer: u8, // I honestly don't know
}

impl ApiVersionsResponse {
    pub fn new(
        api_keys: CompactArray<&'static ApiVersionEntry>,
    ) -> ApiVersionsResponse {
        ApiVersionsResponse {
            error_code: error_code::NONE,
            api_keys,
            ..Default::default()
        }
    }

    pub fn with_error_code(error_code: i16) -> ApiVersionsResponse {
        ApiVersionsResponse {
            error_code,
            ..Default::default()
        }
    }
}

impl EncodeToBytes for ApiVersionsResponse {
    fn encode_to_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![];

        res.extend(self.error_code.encode_to_bytes());
        res.extend(self.api_keys.encode_to_bytes());
        res.extend(self.throttle_time.encode_to_bytes());
        res.push(self.tagged_buffer);

        res
    }
}
