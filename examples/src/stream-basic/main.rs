use iggy::client::Client;
use iggy::messages::poll_messages::PollingStrategy;
use iggy::models::messages::PolledMessage;
use iggy::utils::duration::IggyDuration;
use iggy_examples::shared;
use sdk::builder::*;
use std::str::FromStr;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<(), IggyError> {
    println!("Build iggy client and connect it.");
    let stream = "test_stream";
    let topic = "test_topic";
    let iggy_client = shared::client::build_client(stream, topic, true).await?;

    println!("Build iggy producer & consumer");
    let stream_config = IggyStreamConfig::new(
        stream,
        topic,
        100,
        IggyDuration::from_str("5ms").unwrap(),
        IggyDuration::from_str("5ms").unwrap(),
        PollingStrategy::last(),
    );
    let (producer, consumer) = IggyStream::new(&iggy_client, &stream_config).await?;
    // wait 1 second
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

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

    println!("Send first test message");
    let message = Message::from_str("Hello World")?;
    producer.send_one(message).await?;

    println!("Send second test message");
    let message = Message::from_str("Hello Iggy")?;
    producer.send_one(message).await?;

    println!("Send third test message");
    let message = Message::from_str("Hello Apache")?;
    producer.send_one(message).await?;

    // wait a bit for all messages to arrive at the consumer
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    println!("Stop the message stream and shutdown iggy client");
    token_consumer.cancel();
    // iggy_client.delete_stream(stream_config.stream_id()).await?;
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
        // Print message to stdout
        println!("###################");
        println!("Message received: {}", message);
        println!("###################");
        Ok(())
    }
}
