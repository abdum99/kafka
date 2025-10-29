use crate::{
    common::{
        api::{
            api_key::KafApiKey,
            api_version_entry::ApiVersionEntry,
        }, 
        config::SUPPORTED_API, 
        error::error_code,
        request::{describe_topic_partitions::DescribeTopicPartitionsBody, KafRequest, KafRequestHeader},
        response::{
            self,
            describe_topic_partitions::{DescribeTopicPartitionsResponse, TopicsEntry},
            response_body::{self, ApiVersionsResponse, KafResponseBody::{self, *}},
            KafResponse,
            KafResponseHeader,
        },
        types::CompactArray
    },
    utils::is_api_version_compatible,
    StrError
};

fn handle_api_versions(request: KafRequest) -> Result<KafResponse, StrError> {
    if !is_api_version_compatible(
        request.header.request_api_key.clone(),
        request.header.request_api_version.clone(),
    ) {
        Ok(KafResponse {
            header: KafResponseHeader::v0(request.header),
            body: ApiVersions(ApiVersionsResponse::with_error_code(error_code::UNSUPPORTED_VERSION))
        })
    } else {
        let api_keys_vec: Vec<&ApiVersionEntry> = SUPPORTED_API.values().collect();
        Ok(KafResponse {
            header: KafResponseHeader::v0(request.header),
            body: ApiVersions(ApiVersionsResponse::new(CompactArray(Some(api_keys_vec))))
        })
    }
}

fn handle_describe_topic_partitions_request(
    request: KafRequest,
) -> Result<KafResponse, StrError> {
    let body = request.body.into_describe_topic_partitions().map_err(|_| "Bad Request".to_string())?;

    let Some(topics) = body.topics.0 else {
        return Ok(KafResponse {
            header: KafResponseHeader::v1(request.header),
            body: DescribeTopicPartitions(DescribeTopicPartitionsResponse::bad_request())
        });
    };

    let response_topics = topics.iter().map(|t| TopicsEntry::unknown_topic(t.name.0.clone())).collect();

    Ok(KafResponse {
        header: KafResponseHeader::v1(request.header),
        body: DescribeTopicPartitions(
            DescribeTopicPartitionsResponse::from_topics(response_topics)
        ),
    })
}

fn handle_unsupported_request(request: KafRequest) -> Result<KafResponse, StrError> {
    Ok(KafResponse {
        header: KafResponseHeader::v0(request.header),
        body: KafResponseBody::default()
    })
}

// going to be main logic
pub fn handle_request(request: KafRequest) -> Result<KafResponse, StrError> {
    match &request.header.request_api_key {
        KafApiKey::ApiVersions => handle_api_versions(request),
        KafApiKey::DescribeTopicPartitions => handle_describe_topic_partitions_request(request),
        _ => handle_unsupported_request(request),
    }
}

    // Ok(KafResponse::new(4, request.header.correlation_id))
