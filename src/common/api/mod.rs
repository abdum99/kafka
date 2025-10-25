use crate::common::{api::api_key::KafApiKey, EncodeToBytes};

pub mod api_key;

impl EncodeToBytes for KafApiKey {
    fn write_to_bytes(&self) -> Vec<u8> {
        (self.clone() as i16).write_to_bytes()
    }
}
