use enum_as_inner::EnumAsInner;

use crate::common::{request::describe_topic_partitions::DescribeTopicPartitionsBody, DecodeFromBytes, EncodingError};

#[derive(Debug, EnumAsInner, Clone)]
pub enum KafRequestBody {
    Empty,
    DescribeTopicPartitions(DescribeTopicPartitionsBody),
}
