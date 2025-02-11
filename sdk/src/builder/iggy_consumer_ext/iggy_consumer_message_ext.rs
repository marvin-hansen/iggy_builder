use crate::builder::{EventConsumer, IggyConsumerMessageExt};
use async_trait::async_trait;
use futures_util::StreamExt;
use iggy::clients::consumer::IggyConsumer;
use iggy::error::IggyError;
use tokio::select;
use tokio::sync::oneshot;
use tracing::error;

#[async_trait]
impl IggyConsumerMessageExt for IggyConsumer {
    async fn consume_messages(
        mut self, // self is of type mut IggyConsumer
        event_processor: &'static (impl EventConsumer + Sync),
        _shutdown_rx: oneshot::Receiver<()>,
    ) -> Result<(), IggyError> {
        while let Some(message) = self.next().await {
            if let Ok(received_message) = message {
                match event_processor.consume(received_message.message).await {
                    Ok(()) => {}
                    Err(err) => {
                        error!("Error while handling message: {err}");
                    }
                };
            } else if let Err(err) = message {
                // Handle the most egregious error
                match err {
                    IggyError::Disconnected => {
                        error!("Disconnected: shutdown client: {err}");
                        return Err(err);
                    }
                    IggyError::CannotEstablishConnection => {
                        error!("CannotEstablishConnection: shutdown client : {err}");
                        return Err(err);
                    }
                    IggyError::StaleClient => {
                        error!("StaleClient:  shutdown client: {err}");
                        return Err(err);
                    }
                    IggyError::InvalidServerAddress => {
                        error!("InvalidServerAddress:  shutdown client: {err}");
                        return Err(err);
                    }
                    IggyError::InvalidClientAddress => {
                        error!("InvalidClientAddress:  shutdown client: {err}");
                        return Err(err);
                    }
                    IggyError::NotConnected => {
                        error!("NotConnected:  shutdown client: {err}");
                        return Err(err);
                    }
                    IggyError::ClientShutdown => {
                        error!("ClientShutdown:  shutdown client: {err}");
                        return Err(err);
                    }
                    _ => {
                        error!("Error while handling message: {err}",);
                        continue;
                    }
                } // end match error
            } // end else if let Err(err)
        } // end  while let Some(message)
        Ok(())
    } // end consume_messages(..)
}
