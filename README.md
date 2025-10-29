Writing my own implementation of some Kafka stuff cause I'm bored and want to become a kafka expert
<br/><br/>
Plan to implement a few fun things:
- Logs, partitions, and segments
- Basic Replication. Probably no fun rebalancing or anything, just basic replicated partitions with consensus handled by [raft-rs](https://github.com/tikv/raft-rs) maybe
- Parts of the [kafka wire protocol](https://kafka.apache.org/protocol.html)
- Some basic caching mechanism. Mainly for topic and partition metadata and so on
- Transactions maybe? This would be a serious undertaking so we'll see if I want to do that when I'm done with the above.
