use crate::state::app_state::AppState;
use actix_web::{HttpResponse, Responder, web};

pub async fn new_address(state: web::Data<AppState>) -> impl Responder {
    println!(" New address called");
    match state.rpc.get_new_taproot_address().await {
        Ok(address) => {
            println!("New address generated: {:?}", address);
            HttpResponse::Ok().json(serde_json::json!({
                "address": address
            }))
        }

        Err(e) => {
            println!(" Failed to get new address: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string()
            }))
        }
    }
}

pub async fn health_check() -> impl Responder {
    println!("Health check called");
    HttpResponse::Ok().body("OK")
}

pub async fn tor_status(state: web::Data<AppState>) -> impl Responder {
    println!("Tor status called");
    let tor_enabled = !state.settings.tor_proxy.is_empty();

    // Testa se consegue criar circuito
    let circuit_test = state.tor.new_circuit().await;
    let circuit_working = circuit_test.is_ok();

    if circuit_working {
        // Fecha o circuito de teste
        if let Ok(id) = circuit_test {
            let _ = state.tor.close_circuit(&id).await;
        }
    }

    println!(
        "Tor enabled: {}, Circuits working: {}",
        tor_enabled, circuit_working
    );
    HttpResponse::Ok().json(serde_json::json!({
        "tor_enabled": tor_enabled,
        "tor_proxy": if tor_enabled { &state.settings.tor_proxy } else { "" },
        "circuits_working": circuit_working
    }))
}
