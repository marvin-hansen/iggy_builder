mod builder;
pub mod config;
mod event_consumer;
mod event_producer;
pub mod message_consumer;
pub mod message_producer;
mod utils;

// Re-exports
pub use iggy::clients::client::IggyClient;
pub use iggy::error::IggyError;
pub use iggy::messages::send_messages::Message;
pub use crate::builder::config::*;
pub use crate::builder::event_consumer::*;
pub use crate::builder::event_producer::*;
pub use crate::builder::message_consumer::MessageConsumer;
pub use crate::builder::message_producer::MessageProducer;

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
    /// * The iggy consumer requires an implementation of the `EventConsumer` trait to run.
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
    pub async fn from_config(iggy_config: &IggyConfig) -> Result<(IggyClient, Self), IggyError> {
        Self::build(Some(iggy_config), None).await
    }

    /// Creates a new `IggyBuilder` from the given iggy `Args`.
    ///
    /// Assumptions:
    /// * The iggy client is connected and logged in after build.
    /// * The iggy producer and consumer using the same stream and topic.
    /// * The iggy consumer is set to consume last message by default i.e. no resend.
    /// * The iggy consumer requires an implementation of the `EventConsumer` trait to run.
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
