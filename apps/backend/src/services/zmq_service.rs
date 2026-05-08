use std::sync::Arc;
use tokio::sync::Mutex;
use zmq::Context;

use crate::config::settings::Settings;

#[derive(Clone)]
pub struct ZmqService {
    #[allow(dead_code)]
    pub context: Arc<Mutex<Context>>,
    #[allow(dead_code)]
    pub endpoint: String,
}

impl ZmqService {
    pub fn new(settings: &Settings) -> Self {
        let context = Context::new();
        let endpoint = settings.bitcoin_zmq_endpoint.clone();

        Self {
            context: Arc::new(Mutex::new(context)),
            endpoint,
        }
    }

    /// Monitora transações raw (rawtx)
    #[allow(dead_code)]
    pub async fn monitor_rawtx(&self) -> Result<(), Box<dyn std::error::Error>> {
        let ctx = self.context.lock().await;
        let subscriber = ctx.socket(zmq::SUB)?;
        subscriber.connect(&self.endpoint)?;
        subscriber.set_subscribe(b"rawtx")?;

        loop {
            let msg = subscriber.recv_msg(0)?;
            let topic = msg.as_str().unwrap_or("");
            if topic == "rawtx" {
                // Aqui podemos processar a transação raw
                // Para MVP, apenas log
                println!("Nova transação recebida via ZMQ: {:?}", msg.len());
            }
        }
    }
}
