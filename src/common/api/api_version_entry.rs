use crate::common::{api::api_key::KafApiKey, EncodeToBytes};

#[derive(Debug)]
pub struct ApiVersionEntry {
    pub api_key: KafApiKey,
    pub min_version: i16,
    pub max_version: i16,
    pub _tagged_fields: u8, // honestly not sure
}

impl ApiVersionEntry {
    pub const fn new(api_key: KafApiKey, min_version: i16, max_version: i16) -> ApiVersionEntry {
        ApiVersionEntry {
            api_key,
            min_version,
            max_version,
            _tagged_fields: 0u8,
        }
    }
}

impl EncodeToBytes for &ApiVersionEntry {
    fn encode_to_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![];

        res.extend(self.api_key.encode_to_bytes());
        res.extend(self.min_version.encode_to_bytes());
        res.extend(self.max_version.encode_to_bytes());
        res.push(self._tagged_fields);

        res
    }
}
