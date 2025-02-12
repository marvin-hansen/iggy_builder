use iggy::client::Client;
use iggy::error::IggyError;
use iggy_examples::shared::stream::PrintEventConsumer;
use sdk::builder::{IggyConsumerConfig, IggyConsumerMessageExt, IggyStreamConsumer};
use tokio::sync::oneshot;

const IGGY_URL: &str = "iggy://iggy:iggy@localhost:8090";

#[tokio::main]
async fn main() -> Result<(), IggyError> {
    println!("Build iggy client & consumer");
    let (client, consumer) =
        IggyStreamConsumer::with_client_from_url(IGGY_URL, &stream_consumer_config()).await?;

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
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    println!("Stop the message stream and shutdown iggy client");
    sender.send(()).expect("Failed to send shutdown signal");
    client.shutdown().await?;

    Ok(())
}

fn stream_consumer_config() -> IggyConsumerConfig {
    // For full configuration, use the `new` constructor
    IggyConsumerConfig::default()
}
