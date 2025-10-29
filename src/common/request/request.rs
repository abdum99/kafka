use crate::common::request::describe_topic_partitions::DescribeTopicPartitionsBody;

pub enum KafRequestBody {
    DescribeTopicPartitions(DescribeTopicPartitionsBody),
}
