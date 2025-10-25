use crate::common::{api::api_key::KafApiKey, EncodeToBytes};

pub mod api_key;
pub mod api_version_entry;

impl EncodeToBytes for KafApiKey {
    fn encode_to_bytes(&self) -> Vec<u8> {
        (self.clone() as i16).encode_to_bytes()
    }
}
