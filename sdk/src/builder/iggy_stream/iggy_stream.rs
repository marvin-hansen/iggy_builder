use crate::builder::iggy_stream::build::{
    build_iggy_client, build_iggy_consumer, build_iggy_producer,
};
use crate::builder::IggyStreamConfig;
use iggy::client::SystemClient;
use iggy::clients::client::IggyClient;
use iggy::clients::consumer::IggyConsumer;
use iggy::clients::producer::IggyProducer;
use iggy::error::IggyError;
use tracing::info;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct IggyStream {}

impl IggyStream {
    /// Build and connect iggy client, producer and consumer
    ///
    /// # Arguments
    ///
    /// * `client` - reference to the iggy client
    /// * `config` - configuration for the iggy stream
    ///
    /// # Errors
    ///
    /// If the builds fails, an `IggyError` is returned.
    ///
    pub async fn new(
        client: &IggyClient,
        config: &IggyStreamConfig,
    ) -> Result<(IggyProducer, IggyConsumer), IggyError> {
        info!("Check if client is connected");
        if client.ping().await.is_err() {
            return Err(IggyError::NotConnected);
        }

        info!("Build iggy producer");
        // The producer creates stream and topic if it doesn't exist
        let iggy_producer = match build_iggy_producer::build_iggy_producer(
            client,
            config.producer_config(),
        )
        .await
        {
            Ok(iggy_producer) => iggy_producer,
            Err(err) => return Err(err),
        };

        info!("Build iggy consumer");
        let iggy_consumer = match build_iggy_consumer::build_iggy_consumer(
            client,
            config.consumer_config(),
        )
        .await
        {
            Ok(iggy_consumer) => iggy_consumer,
            Err(err) => return Err(err),
        };

        Ok((iggy_producer, iggy_consumer))
    }

    /// Build and connect iggy client, producer and consumer from connection string
    ///
    /// # Arguments
    ///
    /// * `connection_string` - connection string for the iggy server
    /// * `config` - configuration for the iggy stream
    ///
    /// # Errors
    ///
    /// If the builds fails, an `IggyError` is returned.
    ///
    pub async fn with_client_from_connection_string(
        connection_string: &str,
        config: &IggyStreamConfig,
    ) -> Result<(IggyClient, IggyProducer, IggyConsumer), IggyError> {
        info!("Build and connect iggy client");
        let client = build_iggy_client::build_iggy_client(connection_string).await?;

        info!("Build iggy producer and consumer");
        let (iggy_producer, iggy_consumer) = Self::new(&client, config).await?;
        Ok((client, iggy_producer, iggy_consumer))
    }
}
