use crate::common::api::api_key::KafApiKey;

pub struct ApiVersionEntry {
    api_key: KafApiKey,
    min_version: i16,
    max_version: i16,
    _tagged_fields: u8, // honestly not sure
}
