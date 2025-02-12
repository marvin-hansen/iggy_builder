use crate::builder::IggyProducerConfig;
use iggy::clients::client::IggyClient;
use iggy::clients::producer::IggyProducer;
use iggy::error::IggyError;
use iggy::utils::expiry::IggyExpiry;
use iggy::utils::topic_size::MaxTopicSize;
use tracing::error;

/// Build a producer from the stream configuration.
///
/// # Arguments
///
/// * `client` - The Iggy client.
/// * `config` - The configuration.
///
/// # Errors
///
/// * `IggyError` - If the iggy producer cannot be build.
///
/// # Details
///
/// This function will create a new `IggyProducer` with the given `IggyClient` and `IggyProducerConfig`.
/// The `IggyProducerConfig` fields are used to configure the `IggyProducer`.
///
pub(crate) async fn build_iggy_producer(
    client: &IggyClient,
    config: &IggyProducerConfig,
) -> Result<IggyProducer, IggyError> {
    // Extract config fields.
    let stream = config.stream_name();
    let topic = config.topic_name();
    let batch_size = config.batch_size();
    let send_interval = config.send_interval();
    let partitions_count = config.partition();
    let partitioning = config.partitioning().to_owned();
    let replication_factor = config.replication_factor();
    // let encryptor = config.encryptor().to_owned().unwrap();

    // Build producer.
    let mut producer = client
        .producer(stream, topic)?
        .batch_size(batch_size)
        .send_interval(send_interval)
        .partitioning(partitioning)
        .create_topic_if_not_exists(
            partitions_count,
            replication_factor,
            IggyExpiry::ServerDefault,
            MaxTopicSize::ServerDefault,
        )
        .build();

    match producer.init().await {
        Ok(_) => {}
        Err(err) => {
            error!("Failed to initialize producer: {}", err);
            return Err(err);
        }
    };

    Ok(producer)
}
