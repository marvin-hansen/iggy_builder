use crate::builder::config::{Args, ConfigFields, IggyConfig};
use crate::builder::message_consumer::MessageConsumer;
use crate::builder::message_producer::MessageProducer;
use crate::builder::{utils, IggyBuilder};
use iggy::client::{Client, UserClient};
use iggy::clients::client::IggyClient;
use iggy::error::IggyError;
use tracing::error;

impl IggyBuilder {
    pub(crate) async fn build(
        iggy_config: Option<&IggyConfig>,
        args: Option<(Args, String)>,
    ) -> Result<(IggyClient, Self), IggyError> {
        if iggy_config.is_none() && args.is_none() {
            error!("Configuration missing. Please provide a config to build iggy client.");
            return Err(IggyError::InvalidConfiguration);
        }

        let config_fields = if let Some(iggy_config) = iggy_config {
            ConfigFields::from_iggy_config(iggy_config)
        } else {
            let (args, consumer_name) = args.clone().unwrap();
            ConfigFields::from_args(args, consumer_name)
        };

        let consumer_name = config_fields.consumer_name();
        let stream_id = config_fields.stream_id().to_string();
        let topic_id = config_fields.topic_id().to_string();
        let username = config_fields.username();
        let password = config_fields.password();

        let iggy_client = if let Some(iggy_config) = iggy_config {
            match utils::build_tcp_client_from_config(iggy_config).await {
                Ok(client) => client,
                Err(err) => {
                    error!("Failed to create iggy client: {}", err);
                    return Err(err);
                }
            }
        } else if let Some((args, _)) = args {
            match utils::build_tcp_client_from_args(args).await {
                Ok(client) => client,
                Err(err) => {
                    error!("Failed to create iggy client: {}", err);
                    return Err(err);
                }
            }
        } else {
            error!("Configuration missing. Please provide a config to build iggy client.");
            return Err(IggyError::InvalidConfiguration);
        };

        match iggy_client.connect().await {
            Ok(_) => {}
            Err(err) => return Err(err),
        };

        match iggy_client.login_user(username, password).await {
            Ok(_) => {}
            Err(err) => return Err(err),
        };

        let iggy_producer =
            match MessageProducer::from_client(&iggy_client, stream_id.clone(), topic_id.clone())
                .await
            {
                Ok(producer) => producer,
                Err(err) => return Err(err),
            };

        let iggy_consumer = match MessageConsumer::from_client(
            &iggy_client,
            consumer_name,
            stream_id.clone(),
            topic_id.clone(),
        )
        .await
        {
            Ok(consumer) => consumer,
            Err(err) => return Err(err),
        };
        Ok((
            iggy_client,
            Self {
                iggy_producer,
                iggy_consumer,
            },
        ))
    }
}

impl IggyBuilder {
    /// Returns a reference to the `MessageProducer` created for this client.
    pub fn iggy_producer(&self) -> &MessageProducer {
        &self.iggy_producer
    }
    /// Returns the `MessageConsumer` created for this client.
    ///
    /// Note that this method consumes `self`.
    pub fn iggy_consumer(self) -> MessageConsumer {
        self.iggy_consumer
    }
}
