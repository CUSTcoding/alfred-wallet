use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};

use crate::state::app_state::AppState;

#[derive(Serialize)]
pub struct BalanceResponse {
    pub balance: f64,
}

#[derive(Deserialize)]
pub struct SendRequest {
    pub hex: String,
}

#[derive(Serialize)]
pub struct SendResponse {
    pub txid: String,
}

pub async fn get_balance(state: web::Data<AppState>) -> impl Responder {
    println!("🔍 Get balance called");
    match state.rpc.get_balance().await {
        Ok(balance) => {
            println!(" Balance retrieved: {:?}", balance);
            // Assume balance is a number
            let balance_val = balance.as_f64().unwrap_or(0.0);
            HttpResponse::Ok().json(BalanceResponse {
                balance: balance_val,
            })
        }
        Err(e) => {
            println!(" Failed to get balance: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to get balance: {}", e)
            }))
        }
    }
}

pub async fn send_transaction(
    state: web::Data<AppState>,
    req: web::Json<SendRequest>,
) -> impl Responder {
    println!(
        " Send transaction called with hex length: {}",
        req.hex.len()
    );
    // 🔥 Cria novo circuito Tor para anonimato
    let circuit_id = match state.tor.new_circuit().await {
        Ok(id) => {
            println!(" New Tor circuit created: {}", id);
            Some(id)
        }
        Err(e) => {
            println!(" Failed to create Tor circuit: {}", e);
            None
        }
    };

    let result = state.rpc.send_raw_transaction(&req.hex).await;

    // Fecha o circuito após uso
    if let Some(id) = circuit_id {
        if let Err(e) = state.tor.close_circuit(&id).await {
            println!(" Failed to close Tor circuit: {}", e);
        }
    }

    match result {
        Ok(txid) => {
            let txid_str = txid.as_str().unwrap_or("").to_string();
            println!(" Transaction sent: {}", txid_str);
            HttpResponse::Ok().json(SendResponse { txid: txid_str })
        }
        Err(e) => {
            println!(" Failed to send transaction: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to send transaction: {}", e)
            }))
        }
    }
}
