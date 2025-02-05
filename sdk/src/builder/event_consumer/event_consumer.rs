use crate::builder::event_consumer::EventConsumerError;

/// Trait for event consumer
pub trait EventConsumer {
    /// Consume a event from the message bus.
    ///
    /// # Arguments
    ///
    /// * `data` - The event data
    ///
    /// # Errors
    ///
    /// * `EventConsumerError` - If the event consumer fails to consume the event
    async fn consume(&self, data: Vec<u8>) -> Result<(), EventConsumerError>;
}

// This is a default implementation for referenced types that implement EventConsumer
impl<T: EventConsumer + Send + Sync> EventConsumer for &T {
    /// Consume a event from the message bus.
    ///
    /// # Arguments
    ///
    /// * `data` - The event data
    ///
    /// # Errors
    ///
    /// * `EventConsumerError` - If the event consumer fails to consume the event
    async fn consume(&self, data: Vec<u8>) -> Result<(), EventConsumerError> {
        (**self).consume(data).await
    }
}
