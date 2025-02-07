use crate::builder::config::{Args, IggyConfig};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConfigFields {
    pub consumer_name: String,
    pub stream_id: String,
    pub topic_id: String,
    pub username: String,
    pub password: String,
}

impl ConfigFields {
    pub fn from_iggy_config(iggy_config: &IggyConfig) -> Self {
        Self {
            consumer_name: iggy_config.message_consumer_name().to_string(),
            stream_id: iggy_config.stream_name().to_string(),
            topic_id: iggy_config.topic_name().to_string(),
            username: iggy_config.user().username().to_string(),
            password: iggy_config.user().password().to_string(),
        }
    }

    pub fn from_args(args: Args, consumer_name: String) -> Self {
        Self {
            consumer_name,
            stream_id: args.stream_id,
            topic_id: args.topic_id,
            username: args.username.to_string(),
            password: args.password.to_string(),
        }
    }
}

impl ConfigFields {
    pub fn consumer_name(&self) -> &str {
        &self.consumer_name
    }

    pub fn stream_id(&self) -> &str {
        &self.stream_id
    }

    pub fn topic_id(&self) -> &str {
        &self.topic_id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

impl Display for ConfigFields {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "consumer_name: {}\nstream_id: {}\ntopic_id: {}\nusername: {}\npassword: {}",
            self.consumer_name, self.stream_id, self.topic_id, self.username, self.password
        )
    }
}
