use crate::{common::{types::{CompactArray, CompactString}, DecodeFromBytes}, utils::parse_primitive_types::read_u8_be};

/*
* DescribeTopicPartitions Request (Version: 0) => [topics] response_partition_limit cursor _tagged_fields 
* topics => name _tagged_fields 
*   name => COMPACT_STRING
* response_partition_limit => INT32
* cursor => topic_name partition_index _tagged_fields 
*   topic_name => COMPACT_STRING
*   partition_index => INT32
*/
pub(super) struct DescribeTopicPartitionsBody {
    pub topics: CompactArray<TopicsEntry>,
    pub response_partition_limit: i32,
    pub cursor: u8, // NOTE: NOT IMPLEMENTED
    pub _tagged_fields: u8, // NOTE: NOTE IMPLEMENTED
}

struct TopicsEntry {
    name: CompactString,
    _tagged_fields: u8,
}

impl DecodeFromBytes for TopicsEntry {
    fn read_from_u8(
        input: &[u8],
        offset: &mut usize,
    ) -> Result<Self, crate::common::EncodingError> {
        Ok(TopicsEntry {
            name: CompactString::read_from_u8(input, offset)?,
            _tagged_fields: read_u8_be(input, offset)?,
        })
    }
}
