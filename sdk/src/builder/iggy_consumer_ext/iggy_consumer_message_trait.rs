use async_trait::async_trait;

use crate::builder::EventConsumer;
use iggy::error::IggyError;
use tokio_util::sync::CancellationToken;

#[async_trait]
pub trait IggyConsumerMessageExt {
    async fn consume_messages(
        mut self,
        event_processor: &'static (impl EventConsumer + Sync),
        cancellation_token: CancellationToken,
    ) -> Result<(), IggyError>;
}
