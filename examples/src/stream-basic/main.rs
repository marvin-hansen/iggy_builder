use iggy::client::{Client, StreamClient, TopicClient};
use iggy::models::messages::PolledMessage;
use sdk::builder::*;
use std::str::FromStr;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<(), IggyError> {
    println!("Build iggy client and connect it.");
    let iggy_client = IggyClient::from_connection_string("iggy://iggy:iggy@localhost:8090")?;
    iggy_client.connect().await?;

    println!("Build iggy producer & consumer");
    let stream_config = IggyStreamConfig::from_stream_topic("test_stream", "test_topic", 100);
    let (producer, consumer) = IggyStream::new(&iggy_client, &stream_config).await?;

    println!("Start message stream");
    let token = CancellationToken::new();
    let token_consumer = token.clone();
    tokio::spawn(async move {
        match consumer
            .consume_messages(&PrintEventConsumer {}, token)
            .await
        {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Failed to consume messages: {err}");
            }
        }
    });

    println!("Send a test message");
    let message = Message::from_str("Hello Iggy")?;
    producer.send_one(message).await?;

    println!("Stop the message stream, cleanup, and shutdown iggy client");
    token_consumer.cancel();

    // Any of these or both triggers the server side errors.
    iggy_client
        .delete_topic(stream_config.stream_id(), stream_config.topic_id())
        .await?;
    iggy_client.delete_stream(stream_config.stream_id()).await?;

    iggy_client.shutdown().await?;

    Ok(())
}

#[derive(Debug)]
struct PrintEventConsumer {}

impl EventConsumer for PrintEventConsumer {
    async fn consume(&self, message: PolledMessage) -> Result<(), EventConsumerError> {
        // Extract message payload as raw bytes
        let raw_message = message.payload.as_ref();
        // convert raw bytes into string
        let message = String::from_utf8_lossy(raw_message);

        println!("###################");
        println!("Message received: {}", message);
        println!("###################");

        Ok(())
    }
}
