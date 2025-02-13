use crate::builder::iggy_stream::build::{build_iggy_client, build_iggy_producer};
use crate::builder::IggyProducerConfig;
use iggy::client::SystemClient;
use iggy::clients::client::IggyClient;
use iggy::clients::producer::IggyProducer;
use iggy::error::IggyError;
use tracing::info;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct IggyStreamProducer {}

impl IggyStreamProducer {
    /// Creates a new `IggyProducer` instance and its associated producer using the `client` and
    /// `config` parameters.
    ///
    /// # Arguments
    ///
    /// * `client`: The Iggy client to use to connect to the Iggy server.
    /// * `config`: The configuration for the producer.
    ///
    /// # Errors
    ///
    /// If the client is not connected or the producer cannot be built, an `IggyError` is returned.
    ///
    pub async fn new(
        client: &IggyClient,
        config: &IggyProducerConfig,
    ) -> Result<IggyProducer, IggyError> {
        info!("Check if client is connected");
        if client.ping().await.is_err() {
            return Err(IggyError::NotConnected);
        }

        // The producer creates stream and topic if it doesn't exist
        info!("Build iggy producer");
        let iggy_producer = match build_iggy_producer::build_iggy_producer(client, config).await {
            Ok(iggy_producer) => iggy_producer,
            Err(err) => return Err(err),
        };

        Ok(iggy_producer)
    }

    /// Creates a new `IggyStreamProducer` instance and its associated client using the `connection_string`
    /// and `config` parameters.
    ///
    /// # Arguments
    ///
    /// * `connection_string`: The connection string to use to connect to the Iggy server.
    /// * `config`: The configuration for the producer.
    ///
    /// # Errors
    ///
    /// If the client cannot be connected or the producer cannot be built, an `IggyError` is returned.
    ///
    pub async fn with_client_from_url(
        connection_string: &str,
        config: &IggyProducerConfig,
    ) -> Result<(IggyClient, IggyProducer), IggyError> {
        info!("Build and connect iggy client");
        let client = build_iggy_client::build_iggy_client(connection_string).await?;

        info!("Build iggy producer");
        let iggy_producer = match build_iggy_producer::build_iggy_producer(&client, config).await {
            Ok(iggy_producer) => iggy_producer,
            Err(err) => return Err(err),
        };

        Ok((client, iggy_producer))
    }
}
