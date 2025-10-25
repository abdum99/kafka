use crate::common::{api::{api_key::KafApiKey, api_version_entry}, config::SUPPORTED_API};

pub mod parse_primitive_types;

pub fn is_api_version_compatible(api_key: KafApiKey, api_version: i16) -> bool {
    SUPPORTED_API.get(&api_key).is_some_and(|version_entry| {
        api_version >= version_entry.min_version &&
        api_version <= version_entry.max_version
    })
}
