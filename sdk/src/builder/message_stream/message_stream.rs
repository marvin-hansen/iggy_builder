use crate::builder::IggyStreamConfig;
use iggy::clients::client::IggyClient;
use iggy::clients::consumer::IggyConsumer;
use iggy::clients::producer::IggyProducer;
use iggy::error::IggyError;
use iggy::identifier::Identifier;
use std::sync::Arc;

pub struct IggyStream {
    iggy_producer: Arc<IggyProducer>,
    iggy_consumer: IggyConsumer,
    stream_id: Identifier,
    topic_id: Identifier,
}

impl IggyStream {
    pub async fn new(client: &IggyClient, config: &IggyStreamConfig) -> Result<Self, IggyError> {
        let iggy_producer = match Self::build_iggy_producer(client, config).await {
            Ok(iggy_producer) => iggy_producer,
            Err(err) => return Err(err),
        };

        let iggy_consumer = match Self::build_iggy_consumer(client, config).await {
            Ok(iggy_consumer) => iggy_consumer,
            Err(err) => return Err(err),
        };

        Ok(Self {
            iggy_producer: Arc::new(iggy_producer),
            iggy_consumer,
            stream_id: config.stream_id().to_owned(),
            topic_id: config.topic_id().to_owned(),
        })
    }
}

impl IggyStream {
    /// Returns a reference to the stream identifier.
    #[inline]
    pub const fn stream_id(&self) -> &Identifier {
        &self.stream_id
    }

    /// Returns a reference to the topic identifier.
    #[inline]
    pub const fn topic_id(&self) -> &Identifier {
        &self.topic_id
    }

    /// Returns a mutable reference to the 'IggyConsumer'.
    pub fn consumer_mut(&mut self) -> &mut IggyConsumer {
        &mut self.iggy_consumer
    }

    /// Returns a reference to the `IggyProducer`.
    pub fn producer(&self) -> &Arc<IggyProducer> {
        &self.iggy_producer
    }
}
