use crate::builder::iggy_stream::build::build_iggy_client::build_iggy_client;
use crate::builder::iggy_stream::build::build_iggy_consumer;
use crate::builder::iggy_stream::build::build_iggy_consumer::build_iggy_consumer;
use crate::builder::iggy_stream::build::build_stream_topic::build_iggy_stream_topic_if_not_exists;
use crate::builder::IggyConsumerConfig;
use iggy::client::SystemClient;
use iggy::clients::client::IggyClient;
use iggy::clients::consumer::IggyConsumer;
use iggy::error::IggyError;
use tracing::info;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct IggyStreamConsumer {}

impl IggyStreamConsumer {
    pub async fn new(
        client: &IggyClient,
        config: &IggyConsumerConfig,
    ) -> Result<IggyConsumer, IggyError> {
        info!("Check if client is connected");
        if client.ping().await.is_err() {
            return Err(IggyError::ClientShutdown);
        }

        info!("Check if stream and topic exist");
        match build_iggy_stream_topic_if_not_exists(client, config).await {
            Ok(_) => (),
            Err(err) => return Err(err),
        }

        info!("Build iggy consumer");
        let iggy_consumer = match build_iggy_consumer::build_iggy_consumer(client, config).await {
            Ok(iggy_consumer) => iggy_consumer,
            Err(err) => return Err(err),
        };

        Ok(iggy_consumer)
    }

    pub async fn with_client_from_url(
        connection_string: &str,
        config: &IggyConsumerConfig,
    ) -> Result<(IggyClient, IggyConsumer), IggyError> {
        info!("Build and connect iggy client");
        let client = build_iggy_client(connection_string).await?;

        info!("Check if stream and topic exist");
        match build_iggy_stream_topic_if_not_exists(&client, config).await {
            Ok(_) => (),
            Err(err) => return Err(err),
        }

        info!("Build iggy consumer");
        let iggy_consumer = match build_iggy_consumer(&client, config).await {
            Ok(iggy_consumer) => iggy_consumer,
            Err(err) => return Err(err),
        };

        Ok((client, iggy_consumer))
    }
}
