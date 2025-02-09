use iggy::clients::client::IggyClient;
use iggy::clients::consumer::{AutoCommit, AutoCommitWhen, IggyConsumer};
use iggy::clients::producer::IggyProducer;
use iggy::consumer::ConsumerKind;
use iggy::error::IggyError;
use iggy::messages::poll_messages::PollingStrategy;
use tracing::error;
use crate::builder::IggyStreamConfig;
use crate::builder::message_stream::IggyStream;

impl IggyStream {
    pub(crate) async fn build_iggy_consumer(
        client: &IggyClient,
        stream_config: &IggyStreamConfig,
    ) -> Result<IggyConsumer, IggyError> {

        let consumer_group_name = stream_config.consumer_group_name();
        let stream = stream_config.stream_name();
        let topic = stream_config.topic_name();
        let batch_size = stream_config.batch_size();
        let polling_interval = stream_config.polling_interval();
        let polling_strategy = stream_config.polling_strategy();

        let mut consumer = client
            .consumer_group(consumer_group_name, stream, topic)?
            .auto_commit(AutoCommit::IntervalOrWhen(
                polling_interval,
                AutoCommitWhen::ConsumingAllMessages,
            ))
            .create_consumer_group_if_not_exists()
            .auto_join_consumer_group()
            .polling_strategy(polling_strategy)
            .poll_interval(polling_interval)
            .batch_size(batch_size)
            .build();

        match consumer.init().await {
            Ok(_) => {}
            Err(err) => {
                error!("Failed to initialize consumer: {}", err);
                return Err(err);
            }
        }

        Ok(consumer)

    }

}