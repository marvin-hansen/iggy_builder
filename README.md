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
@TODO
```

## Configuration 

A basic IggyStream only requires very little configuration. See example below.

```rust
@TODO
```  


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
use iggy::models::messages::PolledMessage;

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