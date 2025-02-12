use crate::shared::stream::PrintEventConsumer;
use iggy::client::{Client, StreamClient};
use iggy_examples::shared;
use sdk::builder::*;
use std::str::FromStr;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() -> Result<(), IggyError> {
    println!("Build iggy client and connect it.");
    let client = shared::client::build_client("test_stream", "test_topic", true).await?;

    println!("Build iggy producer & consumer");
    let stream_config = stream_config();
    let (producer, consumer) = IggyStream::new(&client, &stream_config).await?;

    println!("Start message stream");
    let (sender, receiver) = oneshot::channel();
    tokio::spawn(async move {
        match consumer
            // PrintEventConsumer is imported from examples/src/shared/stream.rs
            .consume_messages(&PrintEventConsumer {}, receiver)
            .await
        {
            Ok(_) => {}
            Err(err) => eprintln!("Failed to consume messages: {err}"),
        }
    });

    println!("Send 3 test messages...");
    producer.send_one(Message::from_str("Hello World")?).await?;
    producer.send_one(Message::from_str("Hola Iggy")?).await?;
    producer.send_one(Message::from_str("Hi Apache")?).await?;

    // wait a bit for all messages to arrive.
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    println!("Stop the message stream and shutdown iggy client");
    sender.send(()).expect("Failed to send shutdown signal");
    client.delete_stream(stream_config.stream_id()).await?;
    client.shutdown().await?;

    Ok(())
}

fn stream_config() -> IggyStreamConfig {
    IggyStreamConfig::from_stream_topic(
        "test_stream",
        "test_topic",
        100,
        iggy::utils::duration::IggyDuration::from_str("1ms").unwrap(),
        iggy::utils::duration::IggyDuration::from_str("1ms").unwrap(),
    )
}
