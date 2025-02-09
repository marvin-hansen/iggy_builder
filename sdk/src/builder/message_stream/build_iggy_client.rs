use iggy::client::Client;
use iggy::clients::client::IggyClient;
use iggy::error::IggyError;
use crate::builder::IggyStream;

impl IggyStream {

    pub async fn build_and_connect_iggy_client(connection_string: &str) -> Result<IggyClient, IggyError> {
        let iggy_client = match IggyClient::from_connection_string(connection_string) {
            Ok(iggy_client) => iggy_client,
            Err(err) => return Err(err),
        };

        match iggy_client.connect().await {
            Ok(_) => {}
            Err(err) => return Err(err),
        };

         Ok(iggy_client)
    }

}