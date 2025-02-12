use crate::builder::iggy_stream::build::{build_iggy_client, build_iggy_consumer};
use crate::builder::IggyConsumerConfig;
use iggy::clients::client::IggyClient;
use iggy::clients::consumer::IggyConsumer;
use iggy::error::IggyError;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct IggyStreamConsumer {}

impl IggyStreamConsumer {
    pub async fn new(
        client: &IggyClient,
        config: &IggyConsumerConfig,
    ) -> Result<IggyConsumer, IggyError> {
        // Build iggy consumer
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
        // Build and connect iggy client
        let client = build_iggy_client::build_iggy_client(connection_string).await?;

        let iggy_consumer = match build_iggy_consumer::build_iggy_consumer(&client, config).await {
            Ok(iggy_consumer) => iggy_consumer,
            Err(err) => return Err(err),
        };

        Ok((client, iggy_consumer))
    }
}
