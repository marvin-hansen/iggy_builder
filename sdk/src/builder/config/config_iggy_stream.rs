use bon::Builder;
use iggy::identifier::Identifier;
use iggy::utils::duration::IggyDuration;
use std::str::FromStr;
use std::sync::Arc;
use iggy::error::IggyError;
use iggy::messages::poll_messages::PollingStrategy;
use iggy::messages::send_messages::Partitioning;
use iggy::utils::crypto::EncryptorKind;
use tracing::error;

#[derive(Builder, Debug,Clone)]
pub struct IggyStreamConfig {
    stream_id: Identifier,
    stream_name: String,
    topic_id: Identifier,
    topic_name: String,
    batch_size: u32,
    consumer_group_name: String,
    send_interval: IggyDuration,
    polling_interval: IggyDuration,
    polling_strategy: PollingStrategy,
    // Advanced options
    encryptor: Option<Arc<EncryptorKind>>,
    partitioning: Partitioning,
    partitions_count: u32,
    replication_factor: Option<u8>,
}

impl IggyStreamConfig {
    /// Creates a new `IggyStreamConfig` from the given arguments.
    ///
    /// # Args
    ///
    /// * `stream` - The stream name.
    /// * `topic` - The topic name.
    /// * `batch_size` - The max number of messages to send in a batch.
    /// * `send_interval` - The interval between messages sent.
    /// * `polling_interval` - The interval between polling for new messages.
    /// * `polling_strategy` - The polling strategy to use.
    ///
    /// # Errors
    ///
    /// * `IggyError::InvalidIdentifier` - If the provided stream or topic identifier is invalid.
    ///
    /// Returns:
    /// A new `IggyStreamConfig`.
    ///
    pub fn new(
        stream: &str,
        topic: &str,
        batch_size: u32,
        send_interval: IggyDuration,
        polling_interval: IggyDuration,
        polling_strategy: PollingStrategy,
    ) -> Result<Self, IggyError> {

        let stream_id = match Identifier::from_str_value(stream) {
            Ok(id) => id,
            Err(err) => {
                error!(
                    "Failed to parse stream id due to error: {}",
                    err
                );
                panic!("{}", err.as_string());            }
        };

        let topic_id = match Identifier::from_str_value(topic) {
            Ok(id) => id,
            Err(err) => {
                error!(
                    "Failed to parse topic id due to error: {}",
                    err
                );
                panic!("{}", err.as_string());            }
        };

        let consumer_group_name = format!("consumer-group-{}-{}", stream, topic);

        Ok(Self {
            stream_id,
            stream_name: stream.to_string(),
            topic_id,
            topic_name: topic.to_string(),
            batch_size,
            consumer_group_name,
            send_interval,
            polling_interval,
            polling_strategy,
            // Advanced options set to defaults
            encryptor: None,
            partitioning: Partitioning::balanced(),
            partitions_count: 1,
            replication_factor: None,
        })
    }

    /// Creates a fully customized `IggyStreamConfig` with all fields defined.
    ///
    /// # Args
    ///
    /// * `stream_id` - The stream identifier.
    /// * `stream_name` - The stream name.
    /// * `topic_id` - The topic identifier.
    /// * `topic_name` - The topic name.
    /// * `batch_size` - The max number of messages to send in a batch.
    /// * `consumer_group_name` - The consumer group name.
    /// * `send_interval` - The interval between messages sent.
    /// * `polling_interval` - The interval between polling for new messages.
    /// * `polling_strategy` - The polling strategy to use.
    /// * `encryptor` - The encryptor to use.
    /// * `partitioning` - The partitioning strategy to use.
    /// * `partitions_count` - The number of partitions to use.
    /// * `replication_factor` - The replication factor to use.
    ///
    /// # Errors
    ///
    /// * `IggyError::InvalidIdentifier` - If the provided stream or topic identifier is invalid.
    ///
    /// Returns:
    /// A new `IggyStreamConfig`.
    ///
    pub fn with_all_fields(
        stream_id: Identifier,
        stream_name: String,
        topic_id: Identifier,
        topic_name: String,
        batch_size: u32,
        consumer_group_name: String,
        send_interval: IggyDuration,
        polling_interval: IggyDuration,
        polling_strategy: PollingStrategy,
        encryptor: Option<Arc<EncryptorKind>>,
        partitioning: Partitioning,
        partitions_count: u32,
        replication_factor: Option<u8>,
    ) -> Result<Self, IggyError> {
        Ok(Self {
            stream_id,
            stream_name,
            topic_id,
            topic_name,
            batch_size,
            consumer_group_name,
            send_interval,
            polling_interval,
            polling_strategy,
            encryptor,
            partitioning,
            partitions_count,
            replication_factor,
        })
    }
}

impl IggyStreamConfig {
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

    pub fn encryptor(&self) -> &Option<Arc<EncryptorKind>> {
        &self.encryptor
    }

    pub fn send_interval(&self) -> IggyDuration {
        self.send_interval
    }

    pub fn polling_interval(&self) -> IggyDuration {
        self.polling_interval
    }

    pub fn polling_strategy(&self) -> PollingStrategy {
        self.polling_strategy
    }

    pub fn partitions_count(&self) -> u32 {
        self.partitions_count
    }

    pub fn replication_factor(&self) -> Option<u8> {
        self.replication_factor
    }

    pub fn partitioning(&self) -> Partitioning {
        self.partitioning.to_owned()
    }

    pub fn consumer_group_name(&self) -> &str {
        &self.consumer_group_name
    }
}

impl Default for IggyStreamConfig {
     fn default() -> Self {

         let stream = "test_stream";
         let topic = "test_topic";
         let consumer_group_name = format!("consumer-group-{}-{}", stream, topic);

         Self {
            stream_id: Identifier::from_str_value(stream).unwrap(),
            stream_name: stream.to_string(),
            topic_id: Identifier::from_str_value(topic).unwrap(),
            topic_name: topic.to_string(),
            batch_size: 100,
             consumer_group_name,
            encryptor: None,
            send_interval: IggyDuration::from_str("1ms").unwrap(),
            polling_interval: IggyDuration::from_str("1ms").unwrap(),
            polling_strategy: PollingStrategy::last(),
            partitions_count: 0,
            partitioning: Partitioning::balanced(),
            replication_factor: None,
        }
    }
}
