use crate::common::types::{CompactArray, CompactString};

#[derive(Debug)]
pub struct DescribeTopicPartitionsResponse {
    pub throttle_time: i32,
    pub topics: CompactArray<TopicsEntry>,
    pub next_cursor: u8,
    pub _tagged_fields: u8,
}

#[derive(Debug)]
struct TopicsEntry {
    pub error_code: i16,
    pub name: CompactString,
    pub topic_id: [u8; 16],
    pub is_internal: u8,
    pub partitions: CompactArray<PartitionsEntry>,
    pub topic_authorized_operations: i32, // NOT IMPLEMENTED
    pub _tagged_fields: u8, // NOT IMPLEMENTED
}

#[derive(Debug)]
struct PartitionsEntry {
    pub error_code: i16,
    pub partition_index: i32,
    pub leader_id: i32,
    pub leader_epoch: i32,
    pub replica_nodes: i32,
    pub isr_nodes: i32,
    pub eligible_leader_replicas: i32,
    pub last_known_elr: i32,
    pub offline_replicas: i32,
    pub _tagged_fields: u8, // NOT IMPLEMENTED
}
