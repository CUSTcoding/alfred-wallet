use std::env;

use super::env::load_env;

#[derive(Clone)]
pub struct Settings {
    pub app_host: String,
    pub app_port: u16,

    pub bitcoin_rpc_url: String,
    pub bitcoin_rpc_user: String,
    pub bitcoin_rpc_pass: String,
    pub bitcoin_wallet: String,
    pub bitcoin_zmq_endpoint: String,

    pub tor_proxy: String,
    pub tor_control_host: String,
    pub tor_control_port: u16,
    pub tor_control_pass: String,
    #[allow(dead_code)]
    pub network: String,
}

impl Settings {
    pub fn new() -> Self {
        load_env();

        Self {
            app_host: env::var("APP_HOST").unwrap_or("127.0.0.1".to_string()),
            app_port: env::var("APP_PORT")
                .unwrap_or("8080".to_string())
                .parse()
                .unwrap(),

            bitcoin_rpc_url: env::var("BITCOIN_RPC_URL").unwrap(),
            bitcoin_rpc_user: env::var("BITCOIN_RPC_USER").unwrap(),
            bitcoin_rpc_pass: env::var("BITCOIN_RPC_PASS").unwrap(),
            bitcoin_wallet: env::var("BITCOIN_WALLET").unwrap_or_default(),
            bitcoin_zmq_endpoint: env::var("BITCOIN_ZMQ_ENDPOINT")
                .unwrap_or("tcp://127.0.0.1:28332".to_string()),

            tor_proxy: env::var("TOR_PROXY").unwrap_or_default(),
            tor_control_host: env::var("TOR_CONTROL_HOST").unwrap_or("127.0.0.1".to_string()),
            tor_control_port: env::var("TOR_CONTROL_PORT")
                .unwrap_or("9051".to_string())
                .parse()
                .unwrap_or(9051),
            tor_control_pass: env::var("TOR_CONTROL_PASS").unwrap_or_default(),
            network: env::var("NETWORK").unwrap_or("regtest".to_string()),
        }
    }
}
