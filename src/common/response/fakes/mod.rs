use lazy_static::lazy_static;

use crate::common::{
    error::error_code::UNKNOWN_TOPIC_OR_PARTITION, response::describe_topic_partitions::{DescribeTopicPartitionsResponse, TopicsEntry}, types::{CompactArray, CompactString}
};

lazy_static! {
    pub static ref DescribeTopicPartitionsFake: DescribeTopicPartitionsResponse =
        DescribeTopicPartitionsResponse {
            throttle_time: 0,
            topics: CompactArray(Some(vec![TopicsEntry {
                error_code: UNKNOWN_TOPIC_OR_PARTITION,
                name: CompactString("UNKNOWN_TOPIC_13".to_string()),
                topic_id: [0; 16],
                is_internal: false,
                partitions: CompactArray(Some(vec![])),
                topic_authorized_operations: 0x00000df8i32,
                ..Default::default()
            }])),
            next_cursor: 0xFF,
            ..Default::default()
        };
}

