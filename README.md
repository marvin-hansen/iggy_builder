# Iggy Builder 

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021_edition-orange.svg)](https://www.rust-lang.org)

A high-level SDK for building applications rapidly with [Iggy](https://iggy.rs), a persistent message streaming platform. 
This crate provides an ergonomic builder pattern for creating message producers and consumers with minimal boilerplate.

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

Find a full example in the [tests](sdk/tests/builder/iggy_builder_tests.rs) directory.

```rust
use sdk::builder::{IggyBuilder, IggyConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new default configuration
    let config = IggyConfig::default();

    // Build Iggy client and builder
    let (iggy_client, iggy_client_builder) = IggyBuilder::from_config(&config).await?
    let message_producer = iggy_client_builder.iggy_producer().to_owned();
    let message_consumer = iggy_client_builder.iggy_consumer(); 
  
    // Start iggy consumer
    let token = CancellationToken::new();
    let token_consumer = token.clone();
    tokio::spawn(async move {
         message_consumer
             // Your processing logic goes into PrintEventConsumer that implements EventConsumer 
            .consume_messages(&PrintEventConsumer {}, token)
            .await.expect("Failed to start iggy consumer");
    });
    
    // Send a message
    let payload = "Hello Iggy";
    let message = Message::from_str(payload).expect("Failed to create test message"); 
    message_producer.producer().send_one(message).await?;
     
    // Stop iggy consumer  
    token_consumer.cancel();

    // Shutdown iggy client 
    iggy_client.shutdown().await?;
       
    Ok(())
}
```

## Configuration 

The builder pattern extents to the configuration which means defining a custom IggyConfig is as simple as:

```rust
use sdk::builder::IggyConfig;

let user = IggyUser::builder()
        .username("iggy".to_string())
        .password("iggy".to_string())
        .build();  

let iggy_config = IggyConfig::builder()
        .user(user)
        .stream_id(Identifier::numeric(42).unwrap())
        .stream_name("stream_42".to_string())
        .stream_partition_count(1)
        .topic_id(Identifier::numeric(23).unwrap())
        .topic_name("topic_23".to_string())
        .partition_id(1)
        .messages_per_batch(10)
        .auto_commit(true)
        .tcp_server_addr("localhost:8090".to_string())
        .message_consumer_name("consumer_data".to_string())
        .build()
```  

## Message Processing

To process messages received from the consumer, you implement the `EventConsumer` trait,
pass it to the `consume_messages` method of the MessageConsumer, which then starts to consume messages
from the configured stream.

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
 
  
  

## Architecture

The SDK is built around several core components:

- `IggyBuilder`: Main entry point for creating producers and consumers
- `MessageProducer`: Handles message publishing
- `MessageConsumer`: Manages message consumption
- `EventProducer`: Trait to implement a producer for sending out messages
- `EventConsumer`: Trait to implement a consumer for incoming messages

The MessageProducer already implements EventProducer, which allows you to use it as a default producer.
However, you can replace it with a custom producer by implementing the `EventProducer` trait.

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