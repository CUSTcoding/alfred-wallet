use std::sync::Arc;

use crate::{
    config::settings::Settings,
    services::{bitcoin_rpc::BitcoinRpc, tor_service::TorService, zmq_service::ZmqService},
    wallet::Wallet,
};

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub rpc: Arc<BitcoinRpc>,
    #[allow(dead_code)]
    pub zmq: Arc<ZmqService>,
    #[allow(dead_code)]
    pub tor: Arc<TorService>,
    #[allow(dead_code)]
    pub wallet: Option<Arc<Wallet>>,
}
