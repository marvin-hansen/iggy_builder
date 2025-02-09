use crate::builder::message_stream::IggyStream;
use crate::builder::IggyStreamConfig;
use iggy::clients::client::IggyClient;
use iggy::clients::producer::IggyProducer;
use iggy::error::IggyError;
use iggy::messages::send_messages::Partitioning;
use iggy::utils::expiry::IggyExpiry;
use iggy::utils::topic_size::MaxTopicSize;
use tracing::error;

impl IggyStream {
    pub(crate) async fn build_iggy_producer(
        client: &IggyClient,
        stream_config: &IggyStreamConfig,
    ) -> Result<IggyProducer, IggyError> {
        let stream = stream_config.stream_name();
        let topic = stream_config.topic_name();
        let batch_size = stream_config.batch_size();
        let send_interval = stream_config.send_interval();
        let partitions_count = stream_config.partitions_count();
        let partitioning = stream_config.partitioning();
        let replication_factor = stream_config.replication_factor();

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
            .create_stream_if_not_exists()
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
}
