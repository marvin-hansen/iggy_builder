use iggy::identifier::Identifier;
use sdk::builder::config::{IggyConfig, IggyUser};
use sdk::builder::IggyTcpTLSConfig;

fn create_test_user() -> IggyUser {
    IggyUser::new("test_user", "test_pass")
}
fn create_test_iggy_config() -> IggyConfig {
    IggyConfig::new(
        create_test_user(),
        1,
        "stream_1".to_string(),
        1,
        2,
        "topic_2".to_string(),
        Some("localhost:8090".to_string()),
        None,
        3,
        "consumer_1".to_string(),
        100,
        true,
    )
}

#[test]
fn test_builder() {
    let user = IggyUser::builder()
        .username("test_user".to_string())
        .password("test_pass".to_string())
        .build();

    assert_eq!(user.username(), "test_user");
    assert_eq!(user.password(), "test_pass");

    let tcp_tls_config = IggyTcpTLSConfig::builder()
        .tcp_server_address("127.0.0.1:8090".to_string())
        .tcp_tls_enabled(true)
        .tcp_tls_domain("example.com".to_string())
        .tcp_tls_ca_file("ca.pem".to_string())
        .build();

    assert_eq!(tcp_tls_config.tcp_server_address(), "127.0.0.1:8090");
    assert!(tcp_tls_config.tcp_tls_enabled());
    assert_eq!(tcp_tls_config.tcp_tls_domain(), "example.com");
    assert_eq!(
        tcp_tls_config.tcp_tls_ca_file(),
        &Some("ca.pem".to_string())
    );

    let iggy_config = IggyConfig::builder()
        .user(user.clone())
        .stream_id(Identifier::numeric(1).unwrap())
        .stream_name("stream_1".to_string())
        .stream_partition_count(1)
        .topic_id(Identifier::numeric(2).unwrap())
        .topic_name("topic_2".to_string())
        .tcp_server_addr("localhost:8090".to_string())
        .tcp_tls_config(tcp_tls_config.clone())
        .partition_id(3)
        .messages_per_batch(100)
        .auto_commit(true)
        .message_consumer_name("consumer_1".to_string())
        .build();

    assert_eq!(iggy_config.stream_id(), Identifier::numeric(1).unwrap());
    assert_eq!(iggy_config.stream_name(), "stream_1");
    assert_eq!(iggy_config.topic_id(), Identifier::numeric(2).unwrap());
    assert_eq!(iggy_config.topic_name(), "topic_2");
    assert!(iggy_config.tcp_server_addr().is_some());
    assert!(iggy_config.tcp_tls_config().is_some());
    assert_eq!(iggy_config.partition_id(), 3);
    assert_eq!(iggy_config.messages_per_batch(), 100);
    assert!(iggy_config.auto_commit());
    //
    assert_eq!(iggy_config.user(), &user);
    assert_eq!(iggy_config.tcp_tls_config(), Some(tcp_tls_config));
}

#[test]
fn test_new_iggy_config() {
    let user = create_test_user();
    let config = create_test_iggy_config();

    assert_eq!(config.stream_id(), Identifier::numeric(1).unwrap());
    assert_eq!(config.stream_name(), "stream_1");
    assert_eq!(config.topic_id(), Identifier::numeric(2).unwrap());
    assert_eq!(config.topic_name(), "topic_2");
    assert!(config.tcp_server_addr().is_some());
    assert!(config.tcp_tls_config().is_none());
    assert_eq!(config.partition_id(), 3);
    assert_eq!(config.messages_per_batch(), 100);
    assert!(config.auto_commit());
    assert_eq!(config.user(), &user);
}

#[test]
fn test_clone_iggy_config() {
    let config = create_test_iggy_config();
    let cloned_config = config.clone();
    assert_eq!(config, cloned_config);
}

#[test]
fn test_display_iggy_config() {
    let config = create_test_iggy_config();
    let display_str = format!("{}", config);
    assert!(display_str.contains("test_user"));
    assert!(display_str.contains("localhost:8090"));
    assert!(display_str.contains("stream_1"));
    assert!(display_str.contains("topic_2"));
}
