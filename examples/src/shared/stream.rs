use iggy::models::messages::PolledMessage;
use sdk::builder::{EventConsumer, EventConsumerError};

#[derive(Debug)]
pub struct PrintEventConsumer {}

impl EventConsumer for PrintEventConsumer {
    async fn consume(&self, message: PolledMessage) -> Result<(), EventConsumerError> {
        // Extract message payload as raw bytes & convert into string
        let raw_message = message.payload.as_ref();
        let message = String::from_utf8_lossy(raw_message);
        println!("Message received: {}", message);
        Ok(())
    }
}
