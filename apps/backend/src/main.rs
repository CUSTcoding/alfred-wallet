use actix_web::{App, HttpServer, web};
use std::sync::Arc;

mod api;
mod config;
mod services;
mod state;
mod wallet;

use config::settings::Settings;
use services::bitcoin_rpc::BitcoinRpc;
use services::tor_service::TorService;
use services::zmq_service::ZmqService;
use state::app_state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let settings = Arc::new(Settings::new());

    let rpc = Arc::new(BitcoinRpc::new(&settings));
    let zmq = Arc::new(ZmqService::new(&settings));
    let tor = Arc::new(TorService::new(&settings));

    let app_state = AppState {
        settings: settings.clone(),
        rpc,
        zmq,
        tor,
        wallet: None,
    };

    let host = settings.app_host.clone();
    let port = settings.app_port;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route(
                "/health",
                web::get().to(api::handlers::health::health_check),
            )
            .route(
                "/tor-status",
                web::get().to(api::handlers::health::tor_status),
            )
            .route(
                "/new-address",
                web::get().to(api::handlers::health::new_address),
            )
            .route(
                "/create-wallet",
                web::post().to(api::handlers::wallet::create_wallet),
            )
            .route(
                "/restore-wallet",
                web::post().to(api::handlers::wallet::restore_wallet),
            )
            .route(
                "/balance",
                web::get().to(api::handlers::transaction::get_balance),
            )
            .route(
                "/send",
                web::post().to(api::handlers::transaction::send_transaction),
            )
    })
    .bind((host, port))?
    .run()
    .await
}
