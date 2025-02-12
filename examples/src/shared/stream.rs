use iggy::models::messages::PolledMessage;
use sdk::builder::{EventConsumer, EventConsumerError};

#[derive(Debug)]
pub struct PrintEventConsumer {}

impl EventConsumer for PrintEventConsumer {
    async fn consume(&self, message: PolledMessage) -> Result<(), EventConsumerError> {
        // Extract message payload as raw bytes
        let raw_message = message.payload.as_ref();
        // convert raw bytes into string
        let message = String::from_utf8_lossy(raw_message);
        // Print message to stdout
        println!("###################");
        println!("Message received: {}", message);
        println!("###################");
        Ok(())
    }
}
