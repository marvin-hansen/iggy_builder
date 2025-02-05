use crate::builder::config::config_iggy::IggyConfig;
use crate::builder::config::IggyTcpTLSConfig;
use iggy::users::defaults::{DEFAULT_ROOT_PASSWORD, DEFAULT_ROOT_USERNAME};
use iggy::utils::duration::IggyDuration;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Args {
    pub message_batches_limit: u64,
    pub username: String,
    pub password: String,
    pub interval: String,
    pub stream_id: String,
    pub topic_id: String,
    pub partition_id: u32,
    pub partitions_count: u32,
    pub compression_algorithm: u8,
    pub consumer_kind: u8,
    pub consumer_id: u32,
    pub messages_per_batch: u32,
    pub offset: u64,
    pub auto_commit: bool,
    pub transport: String,
    pub encryption_key: String,
    pub http_api_url: String,
    pub http_retries: u32,
    pub tcp_reconnection_enabled: bool,
    pub tcp_reconnection_max_retries: Option<u32>,
    pub tcp_reconnection_interval: String,
    pub tcp_reconnection_reestablish_after: String,
    pub tcp_heartbeat_interval: String,
    pub tcp_server_address: String,
    pub tcp_tls_enabled: bool,
    pub tcp_tls_domain: String,
    pub tcp_tls_ca_file: Option<String>,
    pub quic_client_address: String,
    pub quic_server_address: String,
    pub quic_server_name: String,
    pub quic_reconnection_enabled: bool,
    pub quic_reconnection_max_retries: Option<u32>,
    pub quic_reconnection_interval: String,
    pub quic_reconnection_reestablish_after: String,
    pub quic_max_concurrent_bidi_streams: u64,
    pub quic_datagram_send_buffer_size: u64,
    pub quic_initial_mtu: u16,
    pub quic_send_window: u64,
    pub quic_receive_window: u64,
    pub quic_response_buffer_size: u64,
    pub quic_keep_alive_interval: u64,
    pub quic_max_idle_timeout: u64,
    pub quic_validate_certificate: bool,
    pub quic_heartbeat_interval: String,
}

impl Args {
    #[inline]
    pub fn new(stream_id: String, topic_id: String) -> Self {
        Self {
            stream_id,
            topic_id,
            ..Default::default()
        }
    }

    #[inline]
    pub fn from_iggy_config(iggy_config: &IggyConfig) -> Self {
        // Extract required arguments
        let stream_id = iggy_config.stream_name().to_string();
        let topic_id = iggy_config.topic_name().to_string();

        // Extract optional arguments
        let server_address = iggy_config.tcp_server_addr();
        let tcp_tls_config = iggy_config.tcp_tls_config();

        // Build config

        match (server_address, tcp_tls_config) {
            (Some(server_address), Some(tcp_tls_config)) => Self::with_server_and_tls_config(
                stream_id,
                topic_id,
                server_address,
                &tcp_tls_config,
            ),
            (Some(server_address), None) => Self::with_server(stream_id, topic_id, server_address),
            (None, Some(tcp_tls_config)) => {
                Self::with_tls_config(stream_id, topic_id, &tcp_tls_config)
            }
            (None, None) => Self::new(stream_id, topic_id),
        }
    }

    #[inline]
    pub fn with_server(stream_id: String, topic_id: String, tcp_server_address: String) -> Self {
        Self {
            stream_id,
            topic_id,
            tcp_server_address,
            ..Default::default()
        }
    }

    #[inline]
    pub fn with_tls_config(
        stream_id: String,
        topic_id: String,
        tcp_tls_config: &IggyTcpTLSConfig,
    ) -> Self {
        Self {
            stream_id,
            topic_id,
            // Tls config
            tcp_tls_enabled: tcp_tls_config.tcp_tls_enabled(),
            tcp_tls_domain: tcp_tls_config.tcp_tls_domain().to_string(),
            tcp_tls_ca_file: tcp_tls_config.tcp_tls_ca_file().to_owned(),
            ..Default::default()
        }
    }

    #[inline]
    pub fn with_server_and_tls_config(
        stream_id: String,
        topic_id: String,
        tcp_server_address: String,
        tcp_tls_config: &IggyTcpTLSConfig,
    ) -> Self {
        Self {
            stream_id,
            topic_id,
            tcp_server_address,
            // Tls config
            tcp_tls_enabled: tcp_tls_config.tcp_tls_enabled(),
            tcp_tls_domain: tcp_tls_config.tcp_tls_domain().to_string(),
            tcp_tls_ca_file: tcp_tls_config.tcp_tls_ca_file().to_owned(),
            ..Default::default()
        }
    }
}

impl Default for Args {
    fn default() -> Self {
        Self {
            // Default values from official sample config
            // https://github.com/iggy-rs/iggy/blob/master/examples/src/shared/args.rs
            message_batches_limit: 10,
            username: DEFAULT_ROOT_PASSWORD.to_string(),
            password: DEFAULT_ROOT_USERNAME.to_string(),
            interval: "1ms".to_string(),
            stream_id: "example-stream".to_string(),
            topic_id: "example-topic".to_string(),
            partition_id: 1,
            partitions_count: 1,
            compression_algorithm: 1,
            consumer_kind: 1,
            consumer_id: 1,
            messages_per_batch: 1,
            offset: 0,
            auto_commit: false,
            transport: "tcp".to_string(),
            encryption_key: String::new(),
            http_api_url: "http://localhost:3000".to_string(),
            http_retries: 3,
            tcp_reconnection_enabled: true,
            tcp_reconnection_max_retries: Some(3),
            tcp_reconnection_interval: "1s".to_string(),
            tcp_reconnection_reestablish_after: "5s".to_string(),
            tcp_heartbeat_interval: "5s".to_string(),
            tcp_server_address: "127.0.0.1:8090".to_string(),
            tcp_tls_enabled: false,
            tcp_tls_domain: "localhost".to_string(),
            tcp_tls_ca_file: None,
            quic_client_address: "127.0.0.1:0".to_string(),
            quic_server_address: "127.0.0.1:8080".to_string(),
            quic_server_name: "localhost".to_string(),
            quic_reconnection_enabled: true,
            quic_reconnection_max_retries: None,
            quic_reconnection_interval: "1s".to_string(),
            quic_reconnection_reestablish_after: "5s".to_string(),
            quic_max_concurrent_bidi_streams: 10000,
            quic_datagram_send_buffer_size: 100000,
            quic_initial_mtu: 1200,
            quic_send_window: 100000,
            quic_receive_window: 100000,
            quic_response_buffer_size: 1048576,
            quic_keep_alive_interval: 5000,
            quic_max_idle_timeout: 10000,
            quic_validate_certificate: false,
            quic_heartbeat_interval: "5s".to_string(),
        }
    }
}

impl Args {
    #[inline]
    pub fn to_sdk_args(&self) -> iggy::args::Args {
        iggy::args::Args {
            transport: self.transport.clone(),
            encryption_key: self.encryption_key.clone(),
            http_api_url: self.http_api_url.clone(),
            http_retries: self.http_retries,
            username: self.username.clone(),
            password: self.password.clone(),
            tcp_server_address: self.tcp_server_address.clone(),
            tcp_reconnection_enabled: self.tcp_reconnection_enabled,
            tcp_reconnection_max_retries: self.tcp_reconnection_max_retries,
            tcp_reconnection_interval: self.tcp_reconnection_interval.clone(),
            tcp_reconnection_reestablish_after: self.tcp_reconnection_reestablish_after.clone(),
            tcp_heartbeat_interval: self.tcp_heartbeat_interval.clone(),
            tcp_tls_enabled: self.tcp_tls_enabled,
            tcp_tls_domain: self.tcp_tls_domain.clone(),
            tcp_tls_ca_file: self.tcp_tls_ca_file.clone(),
            quic_client_address: self.quic_client_address.clone(),
            quic_server_address: self.quic_server_address.clone(),
            quic_server_name: self.quic_server_name.clone(),
            quic_reconnection_enabled: self.quic_reconnection_enabled,
            quic_reconnection_max_retries: self.quic_reconnection_max_retries,
            quic_reconnection_reestablish_after: self.quic_reconnection_reestablish_after.clone(),
            quic_reconnection_interval: self.quic_reconnection_interval.clone(),
            quic_max_concurrent_bidi_streams: self.quic_max_concurrent_bidi_streams,
            quic_datagram_send_buffer_size: self.quic_datagram_send_buffer_size,
            quic_initial_mtu: self.quic_initial_mtu,
            quic_send_window: self.quic_send_window,
            quic_receive_window: self.quic_receive_window,
            quic_response_buffer_size: self.quic_response_buffer_size,
            quic_keep_alive_interval: self.quic_keep_alive_interval,
            quic_max_idle_timeout: self.quic_max_idle_timeout,
            quic_validate_certificate: self.quic_validate_certificate,
            quic_heartbeat_interval: self.quic_heartbeat_interval.clone(),
        }
    }

    #[inline]
    pub fn get_interval(&self) -> Option<IggyDuration> {
        match self.interval.to_lowercase().as_str() {
            "" | "0" | "none" => None,
            x => Some(IggyDuration::from_str(x).expect("Invalid interval format")),
        }
    }
}
