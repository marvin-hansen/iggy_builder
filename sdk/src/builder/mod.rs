use crate::builder::config::{Args, IggyConfig};
use crate::builder::message_consumer::MessageConsumer;
use crate::builder::message_producer::MessageProducer;
use iggy::clients::client::IggyClient;
use iggy::error::IggyError;

mod builder;
pub mod config;
mod event_consumer;
mod event_processor;
mod message_consumer;
mod message_producer;
mod utils;

pub struct IggyBuilder {
    iggy_producer: MessageProducer,
    iggy_consumer: MessageConsumer,
}

impl IggyBuilder {
    /// Creates a new `IggyBuilder` from the given `IggyConfig`.
    ///
    /// Assumptions:
    /// * The iggy client is connected and logged in after build.
    /// * The iggy producer and consumer using the same stream and topic.
    /// * The iggy consumer is set to consume last message by default i.e. no resend.
    ///
    /// # Args
    ///
    /// * `iggy_config` - The `IggyConfig` to use.
    ///
    /// Returns:
    /// A `Result` containing a tuple of:
    /// * A reference to the `IggyClient` created.
    /// * A reference to the `IggyBuilder` created.
    ///
    /// # Errors
    ///
    /// * `IggyError::InvalidConfiguration` - The `IggyConfig` is invalid.
    /// * `IggyError::ConnectionError` - Failed to create the underlying TCP client.
    ///
    pub async fn from_config(iggy_config: IggyConfig) -> Result<(IggyClient, Self), IggyError> {
        Self::build(Some(&iggy_config), None).await
    }

    /// Creates a new `IggyBuilder` from the given iggy `Args`.
    ///
    /// Assumptions:
    /// * The iggy client is connected and logged in after build.
    /// * The iggy producer and consumer using the same stream and topic.
    /// * The iggy consumer is set to consume last message by default i.e. no resend.
    ///
    /// # Args
    ///
    /// * `iggy_config` - The `IggyConfig` to use.
    ///
    /// Returns:
    /// A `Result` containing a tuple of:
    /// * A reference to the `IggyClient` created.
    /// * A reference to the `IggyBuilder` created.
    ///
    /// # Errors
    ///
    /// * `IggyError::InvalidConfiguration` - The `IggyConfig` is invalid.
    /// * `IggyError::ConnectionError` - Failed to create the underlying TCP client.
    ///
    pub async fn from_args(
        args: Args,
        consumer_name: String,
    ) -> Result<(IggyClient, Self), IggyError> {
        Self::build(None, Some((args, consumer_name))).await
    }
}
