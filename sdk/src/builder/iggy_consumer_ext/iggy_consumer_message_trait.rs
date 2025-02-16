use async_trait::async_trait;

use crate::builder::EventConsumer;
use iggy::error::IggyError;
use tokio::sync::oneshot;

#[async_trait]
pub trait IggyConsumerMessageExt {
    async fn consume_messages<P>(
        mut self,
        event_processor: &'static P,
        shutdown_rx: oneshot::Receiver<()>,
    ) -> Result<(), IggyError>
    where
        P: EventConsumer + Sync;
}
