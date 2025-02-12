use iggy::client::Client;
use iggy::error::IggyError;
use iggy_examples::shared;
use iggy_examples::shared::stream::PrintEventConsumer;
use sdk::builder::{IggyConsumerConfig, IggyConsumerMessageExt, IggyStreamConsumer};
use std::str::FromStr;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() -> Result<(), IggyError> {
    println!("Build iggy client and connect it.");
    let client = shared::client::build_client("test_stream", "test_topic", true).await?;

    println!("Build iggy consumer");
    let config = stream_consumer_config();
    let consumer = IggyStreamConsumer::new(&client, &config).await?;

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

    // wait some time
    tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
    println!("Stop the message stream and shutdown iggy client");
    sender.send(()).expect("Failed to send shutdown signal");
    client.shutdown().await?;

    Ok(())
}

fn stream_consumer_config() -> IggyConsumerConfig {
    IggyConsumerConfig::from_stream_topic(
        "test_stream",
        "test_topic",
        100,
        iggy::utils::duration::IggyDuration::from_str("1ms").unwrap(),
    )
}
