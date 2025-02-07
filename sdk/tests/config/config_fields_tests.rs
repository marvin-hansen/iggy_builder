use sdk::builder::{Args, ConfigFields, IggyConfig, IggyUser};

#[test]
fn test_from_iggy_config() {
    let iggy_config = IggyConfig::new(
        IggyUser::new("test_user", "test_pass"),
        1,
        "test_stream".to_string(),
        1,
        1,
        "test_topic".to_string(),
        Some("localhost:8090".to_string()),
        None,
        1,
        "test_consumer".to_string(),
        1,
        true,
    );
    let config_fields = ConfigFields::from_iggy_config(&iggy_config);

    assert_eq!(config_fields.consumer_name(), "test_consumer");
    assert_eq!(config_fields.stream_id(), "test_stream");
    assert_eq!(config_fields.topic_id(), "test_topic");
    assert_eq!(config_fields.username(), "test_user");
    assert_eq!(config_fields.password(), "test_pass");
}

#[test]
fn test_from_args() {
    let mut args = Args::new("test_stream".to_string(), "test_topic".to_string());
    args.username = "test_user".to_string();
    args.password = "test_pass".to_string();

    let consumer_name = "test_consumer".to_string();
    let config_fields = ConfigFields::from_args(args, consumer_name.clone());

    assert_eq!(config_fields.consumer_name(), consumer_name);
    assert_eq!(config_fields.stream_id(), "test_stream");
    assert_eq!(config_fields.topic_id(), "test_topic");
    assert_eq!(config_fields.username(), "test_user");
    assert_eq!(config_fields.password(), "test_pass");
}

#[test]
fn test_getters() {
    let config_fields = ConfigFields {
        consumer_name: "test_consumer".to_string(),
        stream_id: "test_stream".to_string(),
        topic_id: "test_topic".to_string(),
        username: "test_user".to_string(),
        password: "test_pass".to_string(),
    };

    assert_eq!(config_fields.consumer_name(), "test_consumer");
    assert_eq!(config_fields.stream_id(), "test_stream");
    assert_eq!(config_fields.topic_id(), "test_topic");
    assert_eq!(config_fields.username(), "test_user");
    assert_eq!(config_fields.password(), "test_pass");
}

#[test]
fn test_display() {
    let config_fields = ConfigFields {
        consumer_name: "test_consumer".to_string(),
        stream_id: "test_stream".to_string(),
        topic_id: "test_topic".to_string(),
        username: "test_user".to_string(),
        password: "test_pass".to_string(),
    };

    let expected = "consumer_name: test_consumer\nstream_id: test_stream\ntopic_id: test_topic\nusername: test_user\npassword: test_pass";
    assert_eq!(config_fields.to_string(), expected);
}
