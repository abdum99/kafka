use crate::common::EncodeToBytes;

// TODO: probably best as a builder but for later
#[derive(Debug)]
pub enum KafResponseBody {
    Unsupported(UnsupportedResponse),
    ApiVersions(ApiVersionsResponse),
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
    error_code: i16,
    throttle_time: i32,
    tagged_buffer: u8, // I honestly don't know
}

impl ApiVersionsResponse {
    pub fn with_error_code(error_code: i16) -> ApiVersionsResponse {
        ApiVersionsResponse { error_code, ..Default::default() }
    }
}

impl EncodeToBytes for ApiVersionsResponse {
    fn encode_to_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![];

        res.extend(self.error_code.encode_to_bytes());
        res.extend(self.throttle_time.encode_to_bytes());
        res.push(self.tagged_buffer);

        res
    }
}
