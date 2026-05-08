use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

use crate::config::settings::Settings;

#[derive(Clone)]
pub struct TorService {
    pub control_host: String,
    pub control_port: u16,
    pub auth_password: Option<String>,
}

impl TorService {
    pub fn new(settings: &Settings) -> Self {
        Self {
            control_host: settings.tor_control_host.clone(),
            control_port: settings.tor_control_port,
            auth_password: if settings.tor_control_pass.is_empty() {
                None
            } else {
                Some(settings.tor_control_pass.clone())
            },
        }
    }

    /// Cria um novo circuito Tor
    pub async fn new_circuit(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut stream =
            TcpStream::connect(format!("{}:{}", self.control_host, self.control_port)).await?;
        let (reader, mut writer) = stream.split();
        let mut reader = BufReader::new(reader);

        // Authenticate if needed (simplified)
        if let Some(pass) = &self.auth_password {
            writer
                .write_all(format!("AUTHENTICATE \"{}\"\r\n", pass).as_bytes())
                .await?;
            let mut response = String::new();
            reader.read_line(&mut response).await?;
            if !response.starts_with("250") {
                return Err("Tor authentication failed".into());
            }
        } else {
            // Try cookie auth or no auth
            writer.write_all(b"AUTHENTICATE\r\n").await?;
            let mut response = String::new();
            reader.read_line(&mut response).await?;
            if !response.starts_with("250") {
                return Err("Tor authentication failed".into());
            }
        }

        // Create new circuit
        writer.write_all(b"EXTENDCIRCUIT 0\r\n").await?;
        let mut response = String::new();
        reader.read_line(&mut response).await?;

        if response.starts_with("250") {
            // Extract circuit ID
            let circuit_id = response.trim().split_whitespace().nth(1).unwrap_or("0");
            Ok(circuit_id.to_string())
        } else {
            Err("Failed to create new circuit".into())
        }
    }

    /// Fecha um circuito
    pub async fn close_circuit(&self, circuit_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut stream =
            TcpStream::connect(format!("{}:{}", self.control_host, self.control_port)).await?;
        let (reader, mut writer) = stream.split();
        let mut reader = BufReader::new(reader);

        // Authenticate
        writer.write_all(b"AUTHENTICATE\r\n").await?;
        let mut response = String::new();
        reader.read_line(&mut response).await?;

        // Close circuit
        writer
            .write_all(format!("CLOSECIRCUIT {}\r\n", circuit_id).as_bytes())
            .await?;
        let mut response = String::new();
        reader.read_line(&mut response).await?;

        if response.starts_with("250") {
            Ok(())
        } else {
            Err("Failed to close circuit".into())
        }
    }
}
