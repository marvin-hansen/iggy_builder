use iggy::client::{Client, StreamClient, TopicClient, UserClient};
use iggy::clients::client::IggyClient;
use iggy::error::IggyError;
use iggy::identifier::Identifier;

/// Cleans up an Iggy client, deleting the topic and stream.
///
/// # Arguments
///
/// * `client` - The Iggy client to clean up.
/// * `iggy_config` - The configuration to use to clean up the client.
///
/// # Returns
///
/// A `Result` containing a `()` on success or an error on failure.
///
pub async fn cleanup(
    client: &IggyClient,
    stream_id: &Identifier,
    topic_id: &Identifier,
) -> Result<(), IggyError> {
    match client.delete_topic(stream_id, topic_id).await {
        Ok(_) => (),
        Err(err) => return Err(err),
    }

    match client.delete_stream(stream_id).await {
        Ok(_) => (),
        Err(err) => return Err(err),
    }

    Ok(())
}

/// Logs out an Iggy user client.
///
/// # Arguments
///
/// * `client` - The Iggy user client to log out.
///
/// # Returns
///
/// A `Result` containing a `()` on success or an error on failure.
///
pub async fn logout_user(client: &IggyClient) -> Result<(), IggyError> {
    match client.logout_user().await {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
/// Shuts down an Iggy client, disconnecting it from the server.
///
/// # Arguments
///
/// * `client` - The Iggy client to shut down.
///
/// # Returns
///
/// A `Result` containing a `()` on success or an error on failure.
///
pub async fn shutdown(client: &IggyClient) -> Result<(), IggyError> {
    match client.shutdown().await {
        Ok(_) => (),
        Err(err) => return Err(err),
    }

    Ok(())
}
