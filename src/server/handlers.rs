use crate::{
    common::{
        api::{
            api_key::KafApiKey::*,
            api_version_entry::ApiVersionEntry,
        }, 
        config::SUPPORTED_API, 
        error::error_code,
        request::KafRequest,
        response::{
            response_body::{ApiVersionsResponse, KafResponseBody},
            KafResponse,
            KafResponseHeader,
        }, types::CompactArray
    },
    utils::is_api_version_compatible,
    StrError
};

fn handle_api_versions(request: KafRequest) -> Result<KafResponse, StrError> {
    let response_body = if is_api_version_compatible(
        request.header.request_api_key.clone(),
        request.header.request_api_version.clone(),
    ) {
        let api_keys_vec: Vec<&ApiVersionEntry> = SUPPORTED_API.values().collect();
        ApiVersionsResponse::new( CompactArray { items: Some(api_keys_vec) })
    } else {
        ApiVersionsResponse::with_error_code(error_code::UNSUPPORTED_VERSION)
    };

    Ok(KafResponse {
        header: KafResponseHeader::from_request_header(request.header),
        body: KafResponseBody::ApiVersions(response_body),
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
