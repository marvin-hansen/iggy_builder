use crate::builder::{IggyConsumerConfig, IggyProducerConfig};
use iggy::identifier::Identifier;
use iggy::utils::duration::IggyDuration;

#[derive(Debug, Clone)]
pub struct IggyStreamConfig {
    consumer_config: IggyConsumerConfig,
    producer_config: IggyProducerConfig,
}

impl Default for IggyStreamConfig {
    /// Creates a default `IggyStreamConfig`.
    fn default() -> Self {
        Self {
            consumer_config: IggyConsumerConfig::default(),
            producer_config: IggyProducerConfig::default(),
        }
    }
}

impl IggyStreamConfig {
    pub fn new(consumer_config: IggyConsumerConfig, producer_config: IggyProducerConfig) -> Self {
        Self {
            consumer_config,
            producer_config,
        }
    }

    pub fn from_stream_topic(
        stream: &str,
        topic: &str,
        batch_size: u32,
        send_interval: IggyDuration,
        polling_interval: IggyDuration,
    ) -> Self {
        let consumer_config =
            IggyConsumerConfig::from_stream_topic(stream, topic, batch_size, polling_interval);

        let producer_config =
            IggyProducerConfig::from_stream_topic(stream, topic, batch_size, send_interval);

        Self {
            consumer_config,
            producer_config,
        }
    }
}

// Getters.
impl IggyStreamConfig {
    pub fn consumer_config(&self) -> &IggyConsumerConfig {
        &self.consumer_config
    }

    pub fn producer_config(&self) -> &IggyProducerConfig {
        &self.producer_config
    }

    pub fn stream_id(&self) -> &Identifier {
        &self.producer_config.stream_id()
    }

    pub fn stream_name(&self) -> &str {
        &self.producer_config.stream_name()
    }

    pub fn topic_id(&self) -> &Identifier {
        &self.producer_config.topic_id()
    }

    pub fn topic_name(&self) -> &str {
        &self.producer_config.topic_name()
    }
}
