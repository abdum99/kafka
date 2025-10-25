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
    fn write_to_bytes(&self) -> Vec<u8> {
        use self::KafResponseBody::*;
        match self {
            ApiVersions(res) => res.write_to_bytes(),
            _ => UnsupportedResponse::with_error_code(-1).write_to_bytes(),
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
    fn write_to_bytes(&self) -> Vec<u8> {
        self.error_code.to_be_bytes().to_vec()
    }
}

#[derive(Debug, Default)]
pub struct ApiVersionsResponse {
    error_code: i16,
}

impl ApiVersionsResponse {
    pub fn with_error_code(error_code: i16) -> ApiVersionsResponse {
        ApiVersionsResponse { error_code }
    }
}

impl EncodeToBytes for ApiVersionsResponse {
    fn write_to_bytes(&self) -> Vec<u8> {
        self.error_code.to_be_bytes().to_vec()
    }
}
