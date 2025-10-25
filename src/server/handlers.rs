use crate::{common::{
    api::api_key::KafApiKey::*, error::error_code, request::KafRequest, response::{response_body::{ApiVersionsResponse, KafResponseBody}, KafResponse, KafResponseHeader}
}, StrError};

fn handle_api_versions(request: KafRequest) -> Result<KafResponse, StrError> {
    let response_body = KafResponseBody::ApiVersions(
        ApiVersionsResponse::with_error_code(error_code::UNSUPPORTED_VERSION),
    );
    Ok(KafResponse {
        header: KafResponseHeader::from_request_header(request.header),
        body: response_body,
    })
}

fn handle_unsupported_request(_: KafRequest) -> Result<KafResponse, StrError> {
    Ok(KafResponse::default())
}

// going to be main logic
pub fn handle_request(request: KafRequest) -> Result<KafResponse, StrError> {
    match &request.header.request_api_key {
        ApiVersions => handle_api_versions(request),
        _ => handle_unsupported_request(request),
    }
}

    // Ok(KafResponse::new(4, request.header.correlation_id))
