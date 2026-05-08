use reqwest::Client;
use serde_json::{Value, json};

use crate::config::settings::Settings;

#[derive(Clone)]
pub struct BitcoinRpc {
    pub client: Client,
    pub url: String,
    pub user: String,
    pub pass: String,
}

impl BitcoinRpc {
    pub fn new(settings: &Settings) -> Self {
        let mut client_builder = Client::builder();
        let url = if settings.bitcoin_wallet.trim().is_empty()
            || settings.bitcoin_rpc_url.contains("/wallet/")
        {
            settings.bitcoin_rpc_url.clone()
        } else {
            format!(
                "{}/wallet/{}",
                settings.bitcoin_rpc_url.trim_end_matches('/'),
                settings.bitcoin_wallet.trim_matches('/')
            )
        };

        if !settings.tor_proxy.is_empty() && !is_loopback_rpc_url(&url) {
            client_builder =
                client_builder.proxy(reqwest::Proxy::all(&settings.tor_proxy).unwrap());
        }

        let client = client_builder.build().unwrap();

        Self {
            client,
            url,
            user: settings.bitcoin_rpc_user.clone(),
            pass: settings.bitcoin_rpc_pass.clone(),
        }
    }

    /// Função base que chama o Bitcoin RPC e retorna apenas o campo "result"
    async fn call(&self, method: &str, params: Value) -> Result<Value, reqwest::Error> {
        let body = json!({
            "jsonrpc": "1.0",
            "id": "alfred",
            "method": method,
            "params": params
        });

        let response = self
            .client
            .post(&self.url)
            .basic_auth(&self.user, Some(&self.pass))
            .json(&body)
            .send()
            .await?
            .json::<Value>()
            .await?;

        // ⚠️ Aqui é o ponto mais importante: extrai apenas o "result"
        Ok(response["result"].clone())
    }

    /// Informações da blockchain
    #[allow(dead_code)]
    pub async fn get_blockchain_info(&self) -> Result<Value, reqwest::Error> {
        self.call("getblockchaininfo", json!([])).await
    }

    /// Gera novo endereço Taproot
    pub async fn get_new_taproot_address(&self) -> Result<Value, reqwest::Error> {
        self.call("getnewaddress", json!(["", "bech32m"])).await
    }

    /// Obtém saldo da wallet
    pub async fn get_balance(&self) -> Result<Value, reqwest::Error> {
        self.call("getbalance", json!([])).await
    }

    /// Lista UTXOs não gastos
    #[allow(dead_code)]
    pub async fn list_unspent(&self) -> Result<Value, reqwest::Error> {
        self.call("listunspent", json!([])).await
    }

    /// Envia transação raw
    pub async fn send_raw_transaction(&self, hex: &str) -> Result<Value, reqwest::Error> {
        self.call("sendrawtransaction", json!([hex])).await
    }

    /// Obtém informações de uma transação
    #[allow(dead_code)]
    pub async fn get_transaction(&self, txid: &str) -> Result<Value, reqwest::Error> {
        self.call("gettransaction", json!([txid])).await
    }
}

fn is_loopback_rpc_url(url: &str) -> bool {
    let Ok(parsed_url) = reqwest::Url::parse(url) else {
        return false;
    };

    matches!(
        parsed_url.host_str(),
        Some("127.0.0.1") | Some("localhost") | Some("::1") | Some("[::1]")
    )
}
