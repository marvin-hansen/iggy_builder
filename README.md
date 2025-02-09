# Iggy Builder 

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021_edition-orange.svg)](https://www.rust-lang.org)

Exploration of a new high-level SDK for [Iggy](https://iggy.rs), the persistent message streaming platform. 
This crate provides an ergonomic builder pattern for creating message clients, producers and consumers with minimal boilerplate.

## Features

- 🚀 Simple builder pattern for quick setup
- 🔧 Configurable message producers and consumers
- 🔄 Async/await support
- 🛡️ Type-safe message handling
- 🎯 Stream and topic abstraction

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sdk = { git = "https://github.com/marvin-hansen/iggy_builder.git", branch = "main" }
```

## Quick Start

Find a full example in the [examples](examples) directory.

```rust
use iggy::client::Client;
use iggy::models::messages::PolledMessage;
use sdk::builder::*;
use std::str::FromStr;
use tokio_util::sync::CancellationToken; 
const IGGY_URL: &str = "iggy://iggy:iggy@localhost:8090";
 
#[tokio::main]
async fn main() -> Result<(), IggyError> {
    println!("Build iggy client and connect it.");
    let iggy_client =  IggyClient::from_connection_string(IGGY_URL)?;
    iggy_client.connect().await?;

    println!("Build iggy stream & producer");
    let stream_config = stream_config();
    let iggy_stream =  IggyStream::new(&iggy_client, &stream_config).await?;
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
```

## Configuration 

A basic IggyStream only requires very little configuration. See example below.

```rust
use sdk::builder::IggyStreamConfig;

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
```  

For more advanced configuration, use the `with_all_fields` constructor.
If your requirements exceed these requirements, you can use the regular SDK
to construct fully customized producers and consumers.

## Message Processing

To process messages received from the consumer, you implement the `EventConsumer` trait,
pass it to the `consume_messages` method of the IggyStream, which then starts to consume messages from the stream.

```rust
use sdk::builder::{EventConsumer, EventConsumerError};

struct PrintEventConsumer;

impl EventConsumer for PrintEventConsumer {
    async fn consume(&self, message: PolledMessage) -> Result<(), EventConsumerError> {
        // Message payload is just a continuous slice of memory hence zero copy access.
        let raw_message = message.payload.as_ref();

        // convert raw bytes into string
        let message = String::from_utf8_lossy(raw_message);

        // Print message to stdout
        println!("Message received: {}", message);

        Ok(())
    }
}  
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## Dependencies

- async-trait: 0.1.85
- futures: 0.3.31
- iggy: 0.6
- tokio: 1.43.0
- tracing: 0.1.41


## Support

For questions and support, please open an issue in the GitHub repository.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Author
* [Marvin Hansen](https://github.com/marvin-hansen)
* Contact: https://deepcausality.com/contact/