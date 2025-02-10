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
        match consumer.consume_messages(&PrintEventConsumer {}, token).await {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Failed to consume messages: {err}");
            }
        }
    });

    println!("Send a test message");
    let message = Message::from_str("Hello Iggy")?;
    producer.send_one(message).await?;

    println!("Stop the message stream and shutdown iggy client");
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

For more advanced configuration, use the `with_all_fields` constructor from the `IggyStreamConfig`.
If your requirements exceed these configuration parameters, you can use the regular SDK
to construct fully customized producers and consumers.

## IggyConsumerMessageExt

Notice, the consume_messages method has been implemented as a type extension meaning,
even if you build your own consumer, you can still use the `consume_messages` method
simply by importing the type extension trait i.e.

```rust
use sdk::builder::IggyConsumerMessageExt;
````  

## Message Processing

To process messages received from the consumer, you implement the `EventConsumer` trait,
pass it to the `consume_messages` method of the IggyStream, which then starts to consume messages from the stream.

```rust
use sdk::builder::{EventConsumer, EventConsumerError};

struct PrintEventConsumer;

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
```

## Support

For questions and support, please open an issue in the GitHub repository.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Author
* [Marvin Hansen](https://github.com/marvin-hansen)
* Contact: https://deepcausality.com/contact/