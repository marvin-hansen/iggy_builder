use iggy::clients::consumer::{AutoCommit, AutoCommitWhen};
use iggy::consumer::ConsumerKind;
use iggy::error::IggyError;
use iggy::identifier::Identifier;
use iggy::messages::poll_messages::PollingStrategy;
use iggy::messages::send_messages::Partitioning;
use iggy::utils::crypto::EncryptorKind;
use iggy::utils::duration::IggyDuration;
use std::str::FromStr;
use std::sync::Arc;
use tracing::error;

#[derive(Debug, Clone)]
pub struct IggyStreamConfig {
    stream_id: Identifier,
    stream_name: String,
    topic_id: Identifier,
    topic_name: String,
    batch_size: u32,
    consumer_name: String,
    send_interval: IggyDuration,
    polling_interval: IggyDuration,
    polling_strategy: PollingStrategy,
    // Advanced options
    auto_commit: AutoCommit,
    consumer_kind: ConsumerKind,
    encryptor: Option<Arc<EncryptorKind>>,
    partitioning: Partitioning,
    partition: u32,
    replication_factor: Option<u8>,
}

impl Default for IggyStreamConfig {
    /// Creates a default `IggyStreamConfig`.
    fn default() -> Self {
        let stream = "test_stream";
        let topic = "test_topic";
        let consumer_name = format!("consumer-{}-{}", stream, topic);

        Self {
            stream_id: Identifier::from_str_value(stream).unwrap(),
            stream_name: stream.to_string(),
            topic_id: Identifier::from_str_value(topic).unwrap(),
            topic_name: topic.to_string(),
            batch_size: 100,
            consumer_name,
            encryptor: None,
            send_interval: IggyDuration::from_str("5ms").unwrap(),
            polling_interval: IggyDuration::from_str("5ms").unwrap(),
            polling_strategy: PollingStrategy::last(),
            auto_commit: AutoCommit::When(AutoCommitWhen::PollingMessages),
            consumer_kind: ConsumerKind::Consumer,
            partition: 1,
            partitioning: Partitioning::balanced(),
            replication_factor: None,
        }
    }
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

        let consumer_name = format!("consumer-{}-{}", stream, topic);

        Self {
            stream_id,
            stream_name: stream.to_string(),
            topic_id,
            topic_name: topic.to_string(),
            batch_size,
            consumer_name,
            send_interval,
            polling_interval,
            polling_strategy,
            // Advanced options set to defaults
            auto_commit: AutoCommit::When(AutoCommitWhen::PollingMessages),
            consumer_kind: ConsumerKind::Consumer,
            encryptor: None,
            partitioning: Partitioning::balanced(),
            partition: 1,
            replication_factor: None,
        }
    }

    pub fn from_stream_topic(stream: &str, topic: &str, batch_size: u32) -> Self {
        Self::new(
            stream,
            topic,
            batch_size,
            IggyDuration::from_str("1ms").unwrap(),
            IggyDuration::from_str("1ms").unwrap(),
            PollingStrategy::last(),
        )
    }

    /// Creates a fully customized `IggyStreamConfig` with all fields defined.
    ///
    /// # Args
    ///
    /// * `auto_commit` - The auto-commit configuration to use.
    /// * `batch_size` - The max number of messages to send in a batch.
    /// * `consumer_name` - The name of the consumer group.
    /// * `consumer_kind` - The consumer kind to use.
    /// * `encryptor` - The encryptor to use for encrypting the messages' payloads.
    /// * `partitioning` - The partitioning strategy to use.
    /// * `partition` - The number of partitions to create.
    /// * `polling_interval` - The interval between polling for new messages.
    /// * `polling_strategy` - The polling strategy to use.
    /// * `replication_factor` - The replication factor to use.
    /// * `send_interval` - The interval between messages sent.
    /// * `stream_id` - The stream identifier.
    /// * `stream_name` - The stream name.
    /// * `topic_id` - The topic identifier.
    /// * `topic_name` - The topic name.
    ///
    /// Returns:
    /// A new `IggyStreamConfig`.
    ///
    /// Errors:
    /// * `IggyError` if the given arguments are invalid.
    ///
    pub fn with_all_fields(
        auto_commit: AutoCommit,
        batch_size: u32,
        consumer_name: String,
        consumer_kind: ConsumerKind,
        encryptor: Option<Arc<EncryptorKind>>,
        partitioning: Partitioning,
        partition: u32,
        polling_interval: IggyDuration,
        polling_strategy: PollingStrategy,
        replication_factor: Option<u8>,
        send_interval: IggyDuration,
        stream_id: Identifier,
        stream_name: String,
        topic_id: Identifier,
        topic_name: String,
    ) -> Result<Self, IggyError> {
        Ok(Self {
            stream_id,
            stream_name,
            topic_id,
            topic_name,
            batch_size,
            consumer_name,
            send_interval,
            polling_interval,
            polling_strategy,
            auto_commit,
            consumer_kind,
            encryptor,
            partitioning,
            partition,
            replication_factor,
        })
    }
}

// Getters.
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

    pub fn consumer_group_name(&self) -> &str {
        &self.consumer_name
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

    pub fn auto_commit(&self) -> AutoCommit {
        self.auto_commit
    }

    pub fn consumer_kind(&self) -> ConsumerKind {
        self.consumer_kind
    }

    pub fn encryptor(&self) -> &Option<Arc<EncryptorKind>> {
        &self.encryptor
    }

    pub fn partitioning(&self) -> &Partitioning {
        &self.partitioning
    }

    pub fn partitions_count(&self) -> u32 {
        self.partition
    }

    pub fn replication_factor(&self) -> Option<u8> {
        self.replication_factor
    }
}
