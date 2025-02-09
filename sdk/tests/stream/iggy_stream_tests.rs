use std::str::FromStr;
use iggy::client::Client;
use iggy::models::messages::PolledMessage;
use tokio_util::sync::CancellationToken;
use sdk::builder::{EventConsumer, EventConsumerError, IggyStream, IggyStreamConfig};

const IGGY_URL: &str = "iggy://iggy:iggy@localhost:8090";

#[tokio::test]
async fn test_iggy_stream() {

    let res = IggyStream::build_and_connect_iggy_client(IGGY_URL).await;
    assert!(res.is_ok());
    let iggy_client = res.unwrap();
    println!("✅ iggy client build");

    let stream_config = stream_config();
    let res = IggyStream::new(&iggy_client, &stream_config).await;
    assert!(res.is_ok());
    let iggy_stream = res.unwrap();
    println!("✅ iggy stream build");

    let message_producer = iggy_stream.producer().to_owned();
    println!("✅ iggy producer build");

    println!("Start iggy stream");
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
    println!("✅ iggy stream started");

    // println!("Send a test message via producer");
    // let payload = "Hello Iggy";
    // let message = Message::from_str(payload).expect("Failed to create test message");
    //
    // let res = message_producer.send_one(message).await;
    // assert!(res.is_ok());
    // println!("✅ test message send");

    println!("Stop iggy consumer");
    token_consumer.cancel();
    println!("✅ iggy consumer stopped");

    println!("Stop iggy client");
    let res = iggy_client.shutdown().await;
    assert!(res.is_ok());
    println!("✅ iggy client stopped");
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
