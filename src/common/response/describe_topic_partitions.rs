use crate::common::{error::error_code::{INVALID_REQUEST, UNKNOWN_TOPIC_OR_PARTITION}, types::{CompactArray, CompactString}, EncodeToBytes};

#[derive(Debug, Default, Clone)]
pub struct DescribeTopicPartitionsResponse {
    pub throttle_time: i32,
    pub topics: CompactArray<TopicsEntry>,
    pub next_cursor: u8,
    pub _tagged_fields: u8,
}

impl DescribeTopicPartitionsResponse {
    pub fn bad_request() -> Self {
        DescribeTopicPartitionsResponse {
            throttle_time: 0,
            topics: CompactArray(Some(vec![TopicsEntry {
                error_code: INVALID_REQUEST,
                topic_id: [0; 16],
                is_internal: false,
                partitions: CompactArray(Some(vec![])),
                topic_authorized_operations: 0x00000df8i32,
                ..Default::default()
            }])),
            next_cursor: 0xFF,
            ..Default::default()
        }
    }

    pub fn from_topics(topics: Vec<TopicsEntry>) -> Self {
        DescribeTopicPartitionsResponse {
            throttle_time: 0,
            topics: CompactArray(Some(topics)),
            next_cursor: 0xFF,
            ..Default::default()
        }
    }
}

impl EncodeToBytes for DescribeTopicPartitionsResponse {
    fn encode_to_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![];

        res.extend(self.throttle_time.encode_to_bytes());
        res.extend(self.topics.encode_to_bytes());
        res.push(self.next_cursor);
        res.push(self._tagged_fields);

        res
    }
}

#[derive(Debug, Default, Clone)]
pub struct TopicsEntry {
    pub error_code: i16,
    pub name: CompactString,
    pub topic_id: [u8; 16],
    pub is_internal: bool,
    pub partitions: CompactArray<PartitionsEntry>,
    pub topic_authorized_operations: i32, // NOT IMPLEMENTED
    pub _tagged_fields: u8, // NOT IMPLEMENTED
}

impl TopicsEntry {
    pub fn unknown_topic(topic_name: String) -> Self {
        TopicsEntry {
            error_code: UNKNOWN_TOPIC_OR_PARTITION,
            name: CompactString(topic_name),
            topic_id: [0; 16],
            is_internal: false,
            partitions: CompactArray(Some(vec![])),
            topic_authorized_operations: 0x00000df8i32,
            ..Default::default()
        }
    }
}

impl EncodeToBytes for TopicsEntry {
    fn encode_to_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![];

        res.extend(self.error_code.encode_to_bytes());
        res.extend(self.name.encode_to_bytes());
        res.extend(self.topic_id);
        res.extend(self.is_internal.encode_to_bytes());
        res.extend(self.partitions.encode_to_bytes());
        res.extend(self.topic_authorized_operations.encode_to_bytes());
        res.push(self._tagged_fields);

        res
    }
}

#[derive(Debug, Default, Clone)]
pub(super) struct PartitionsEntry {
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

impl EncodeToBytes for PartitionsEntry {
    fn encode_to_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![];

        res.extend(self.error_code.encode_to_bytes());

        res.extend(self.partition_index.encode_to_bytes());
        res.extend(self.leader_id.encode_to_bytes());
        res.extend(self.leader_epoch.encode_to_bytes());
        res.extend(self.replica_nodes.encode_to_bytes());
        res.extend(self.isr_nodes.encode_to_bytes());
        res.extend(self.eligible_leader_replicas.encode_to_bytes());
        res.extend(self.last_known_elr.encode_to_bytes());
        res.extend(self.offline_replicas.encode_to_bytes());
        res.push(self._tagged_fields);

        res
    }
}
