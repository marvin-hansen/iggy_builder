use iggy::client::Client;
use iggy::models::messages::PolledMessage;
use sdk::builder::{
    EventConsumer, EventConsumerError, IggyClient, IggyError, IggyStream, IggyStreamConfig, Message,
};
use std::str::FromStr;
use tokio_util::sync::CancellationToken;

const IGGY_URL: &str = "iggy://iggy:iggy@localhost:8090";

#[tokio::main]
async fn main() -> Result<(), IggyError> {
    println!("Build iggy client and connect it.");
    let iggy_client = build_and_connect_iggy_client(IGGY_URL).await?;
    iggy_client.connect().await?;

    println!("Build iggy stream & producer");
    let stream_config = stream_config();
    let iggy_stream = IggyStream::new(&iggy_client, &stream_config).await?;
    let message_producer = iggy_stream.producer().to_owned();

    println!("Start message stream");
    let token = CancellationToken::new();
    let token_consumer = token.clone();
    tokio::spawn(async move {
        match iggy_stream
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
    message_producer.send_one(message).await?;

    println!("Stop the message stream and iggy client");
    token_consumer.cancel();
    iggy_client.shutdown().await?;

    Ok(())
}

fn stream_config() -> IggyStreamConfig {
    IggyStreamConfig::new(
        "test_stream",
        "test_topic",
        100,
        iggy::utils::duration::IggyDuration::from_str("1ms").unwrap(),
        iggy::utils::duration::IggyDuration::from_str("1ms").unwrap(),
        iggy::messages::poll_messages::PollingStrategy::last(),
    )
}

#[derive(Debug)]
struct PrintEventConsumer {}

impl EventConsumer for PrintEventConsumer {
    async fn consume(&self, message: PolledMessage) -> Result<(), EventConsumerError> {
        // Message payload is just a continuous slice of memory hence zero copy access.
        let raw_message = message.payload.as_ref();

        // convert raw bytes into string
        let message = String::from_utf8_lossy(raw_message);

        // Print message to stdout
        println!("###################");
        println!("[PrintEventConsumer]");
        println!("Message received: {}", message);
        println!("###################");

        Ok(())
    }
}
async fn build_and_connect_iggy_client(connection_string: &str) -> Result<IggyClient, IggyError> {
    let iggy_client = match IggyClient::from_connection_string(connection_string) {
        Ok(iggy_client) => iggy_client,
        Err(err) => return Err(err),
    };

    match iggy_client.connect().await {
        Ok(_) => {}
        Err(err) => return Err(err),
    };

    Ok(iggy_client)
}
