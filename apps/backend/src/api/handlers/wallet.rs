use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::state::app_state::AppState;
use crate::wallet::Wallet;

#[derive(Deserialize)]
pub struct CreateWalletRequest {
    pub word_count: Option<usize>, // 12 or 24
}

#[derive(Serialize)]
pub struct CreateWalletResponse {
    pub mnemonic: String,
    pub first_address: String,
}

#[derive(Deserialize)]
pub struct RestoreWalletRequest {
    pub mnemonic: String,
}

#[derive(Serialize)]
pub struct RestoreWalletResponse {
    pub first_address: String,
}

#[allow(unused_variables)]
pub async fn create_wallet(
    state: web::Data<AppState>,
    req: web::Json<CreateWalletRequest>,
) -> impl Responder {
    println!("Create wallet called with word_count: {:?}", req.word_count);
    let word_count = req.word_count.unwrap_or(12);

    match Wallet::create(word_count) {
        Ok(wallet) => {
            println!(" Wallet created successfully");
            // Store wallet in state (for MVP, in-memory)
            #[allow(unused_variables)]
            let wallet_arc = Arc::new(wallet.clone());
            let first_address = wallet.get_first_address().unwrap().clone();

            HttpResponse::Ok().json(CreateWalletResponse {
                mnemonic: wallet.mnemonic,
                first_address,
            })
        }
        Err(e) => {
            println!(" Failed to create wallet: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to create wallet: {}", e)
            }))
        }
    }
}

#[allow(unused_variables)]
pub async fn restore_wallet(
    state: web::Data<AppState>,
    req: web::Json<RestoreWalletRequest>,
) -> impl Responder {
    println!(
        "Restore wallet called with mnemonic length: {}",
        req.mnemonic.len()
    );
    match Wallet::restore(&req.mnemonic) {
        Ok(wallet) => {
            println!(" Wallet restored successfully");
            let first_address = wallet.get_first_address().unwrap().clone();

            HttpResponse::Ok().json(RestoreWalletResponse { first_address })
        }
        Err(e) => {
            println!(" Failed to restore wallet: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to restore wallet: {}", e)
            }))
        }
    }
}
