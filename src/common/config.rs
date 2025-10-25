use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::common::{api::{api_key::KafApiKey, api_version_entry::ApiVersionEntry}, types::CompactArray};


lazy_static! {
    pub static ref SUPPORTED_API: HashMap<KafApiKey, ApiVersionEntry> = HashMap::from([
        // (KafApiKey::Fetch, ApiVersionEntry::new(KafApiKey::Fetch, 0, 17)),
        (KafApiKey::ApiVersions, ApiVersionEntry::new(KafApiKey::ApiVersions, 0, 4)),
        // (KafApiKey::DescribeTopicPartitions, ApiVersionEntry::new(KafApiKey::DescribeTopicPartitions, 0, 0)),
    ]);
}
