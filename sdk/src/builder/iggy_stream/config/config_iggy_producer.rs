use iggy::identifier::Identifier;
use iggy::messages::send_messages::Partitioning;
use iggy::utils::duration::IggyDuration;
use std::str::FromStr;
use tracing::error;

#[derive(Debug, Clone)]
pub struct IggyProducerConfig {
    stream_id: Identifier,
    stream_name: String,
    topic_id: Identifier,
    topic_name: String,
    batch_size: u32,
    send_interval: IggyDuration,
    partitioning: Partitioning,
    partitions_count: u32,
    replication_factor: Option<u8>,
}

impl Default for IggyProducerConfig {
    fn default() -> Self {
        Self {
            stream_id: Identifier::from_str_value("test_stream").unwrap(),
            stream_name: "test_stream".to_string(),
            topic_id: Identifier::from_str_value("test_topic").unwrap(),
            topic_name: "test_topic".to_string(),
            batch_size: 100,
            send_interval: IggyDuration::from_str("5ms").unwrap(),
            partitioning: Partitioning::balanced(),
            partitions_count: 1,
            replication_factor: None,
        }
    }
}

impl IggyProducerConfig {
    /// Creates a new `IggyProducerConfig` with all fields defined.
    ///
    /// # Args
    ///
    /// * `stream_id` - The stream identifier.
    /// * `stream_name` - The stream name.
    /// * `topic_id` - The topic identifier.
    /// * `topic_name` - The topic name.
    /// * `batch_size` - The max number of messages to send in a batch.
    /// * `send_interval` - The interval between messages sent.
    /// * `partitioning` - The partitioning strategy to use.
    /// * `partition` - The number of partitions to create.
    /// * `replication_factor` - The replication factor to use.
    ///
    /// Returns:
    /// A new `IggyProducerConfig`.
    ///
    pub fn new(
        stream_id: Identifier,
        stream_name: String,
        topic_id: Identifier,
        topic_name: String,
        batch_size: u32,
        send_interval: IggyDuration,
        partitioning: Partitioning,
        partitions_count: u32,
        replication_factor: Option<u8>,
    ) -> Self {
        Self {
            stream_id,
            stream_name,
            topic_id,
            topic_name,
            batch_size,
            send_interval,
            partitioning,
            partitions_count,
            replication_factor,
        }
    }

    /// Creates a new `IggyProducerConfig` from the given stream and topic names, along with the
    /// max batch size and the send interval.
    ///
    /// # Args
    ///
    /// * `stream` - The stream name.
    /// * `topic` - The topic name.
    /// * `batch_size` - The max number of messages to send in a batch.
    /// * `send_interval` - The interval between messages sent.
    ///
    /// Returns:
    /// A new `IggyProducerConfig`.
    ///
    pub fn from_stream_topic(
        stream: &str,
        topic: &str,
        batch_size: u32,
        send_interval: IggyDuration,
    ) -> Self {
        let stream_id = match Identifier::from_str_value(stream) {
            Ok(id) => id,
            Err(err) => {
                error!("Failed to parse stream id due to error: {}", err);
                panic!("{}", err.as_string());
            }
        };

        let topic_id = match Identifier::from_str_value(topic) {
            Ok(id) => id,
            Err(err) => {
                error!("Failed to parse topic id due to error: {}", err);
                panic!("{}", err.as_string());
            }
        };

        Self {
            stream_id,
            stream_name: stream.to_string(),
            topic_id,
            topic_name: topic.to_string(),
            batch_size,
            send_interval,
            partitioning: Partitioning::balanced(),
            partitions_count: 1,
            replication_factor: None,
        }
    }
}

impl IggyProducerConfig {
    pub fn stream_id(&self) -> &Identifier {
        &self.stream_id
    }

    pub fn stream_name(&self) -> &str {
        &self.stream_name
    }

    pub fn topic_id(&self) -> &Identifier {
        &self.topic_id
    }

    pub fn topic_name(&self) -> &str {
        &self.topic_name
    }

    pub fn batch_size(&self) -> u32 {
        self.batch_size
    }

    pub fn send_interval(&self) -> IggyDuration {
        self.send_interval
    }

    pub fn partitioning(&self) -> &Partitioning {
        &self.partitioning
    }

    pub fn partitions_count(&self) -> u32 {
        self.partitions_count
    }

    pub fn replication_factor(&self) -> Option<u8> {
        self.replication_factor
    }
}
