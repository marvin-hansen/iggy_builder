use crate::builder::event_processor::EventProcessorError;
use iggy::messages::send_messages::Message;

/// Event processor interface
pub trait EventProcessor {
    /// Send a single iggy message.
    ///
    /// The message is provided as an iggy `Message`.
    ///
    /// # Errors
    ///
    /// Returns an error if the message cannot be sent.
    ///
    async fn send_one_event(&self, message: Message) -> Result<(), EventProcessorError>;

    /// Send a batch of iggy messages.
    ///
    /// The messages are provided as a `Vec` of `Message`.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the messages cannot be sent.
    async fn send_event_batch(&self, messages: Vec<Message>) -> Result<(), EventProcessorError>;
}

// Default implementation for `&T`
// https://users.rust-lang.org/t/hashmap-get-dereferenced/33558
impl<T: EventProcessor + Send + Sync> EventProcessor for &T {
    async fn send_one_event(&self, message: Message) -> Result<(), EventProcessorError> {
        (**self).send_one_event(message).await
    }

    async fn send_event_batch(&self, messages: Vec<Message>) -> Result<(), EventProcessorError> {
        (**self).send_event_batch(messages).await
    }
}
