use crate::builder::{EventConsumer, IggyConsumerMessageExt};
use async_trait::async_trait;
use futures_util::StreamExt;
use iggy::clients::consumer::IggyConsumer;
use iggy::error::IggyError;
use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::error;

#[async_trait]
impl IggyConsumerMessageExt for IggyConsumer {
    async fn consume_messages(
        mut self, // self is of type mut IggyConsumer
        event_processor: &'static (impl EventConsumer + Sync),
        cancellation_token: CancellationToken,
    ) -> Result<(), IggyError> {
        select! {
             _ = cancellation_token.cancelled() => {
                    return Ok(())
                }

            received_message = self.next() => {
                match received_message {

                    // Message received, process it
                    Some(Ok(received_message)) => {

                        match event_processor
                            .consume(received_message.message)
                            .await{
                                Ok(()) => {}
                                Err(err) => {
                                    error!("Error while handling message: {err}");
                                }
                        };
                    }

                    // E{or received, handle it
                    Some(Err(err)) => {
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
                                error!("Error while handling message: {err}", );
                            }
                        } // end match error

                    } // end Some(error)

                    // Nothing received, continue
                    None => {}

                } // end received_message

            }  // end polled messages

        } // end tokio select

        Ok(())
    }
}
