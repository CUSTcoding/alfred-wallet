use bip39::{Language, Mnemonic};
use bitcoin::bip32::{ChildNumber, DerivationPath, Xpriv, Xpub};
use bitcoin::network::Network;
use bitcoin::{Address, secp256k1};
use rand::{RngCore, rngs::OsRng};
use secp256k1::Secp256k1;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub mnemonic: String,
    pub seed: Vec<u8>,
    pub master_private_key: Xpriv,
    pub master_public_key: Xpub,
    pub addresses: Vec<String>,
}

impl Wallet {
    /// Cria uma nova wallet com seed de 12 ou 24 palavras
    pub fn create(word_count: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let entropy_len = if word_count == 24 { 32 } else { 16 }; // 12 words = 128 bits = 16 bytes, 24 = 256 bits = 32 bytes
        let mut entropy = vec![0u8; entropy_len];
        OsRng.fill_bytes(&mut entropy);
        let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy)?;
        let seed = mnemonic.to_seed("");

        let secp = Secp256k1::new();
        let master_private_key = Xpriv::new_master(Network::Bitcoin, &seed)?;
        let master_public_key = Xpub::from_priv(&secp, &master_private_key);

        let mut wallet = Wallet {
            mnemonic: mnemonic.to_string(),
            seed: seed.to_vec(),
            master_private_key,
            master_public_key,
            addresses: Vec::new(),
        };

        // Gera alguns endereços iniciais
        wallet.generate_addresses(10)?;

        Ok(wallet)
    }

    /// Restaura wallet a partir de mnemonic
    pub fn restore(mnemonic_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mnemonic = Mnemonic::parse_in(Language::English, mnemonic_str)?;
        let seed = mnemonic.to_seed("");

        let secp = Secp256k1::new();
        let master_private_key = Xpriv::new_master(Network::Bitcoin, &seed)?;
        let master_public_key = Xpub::from_priv(&secp, &master_private_key);

        let mut wallet = Wallet {
            mnemonic: mnemonic.to_string(),
            seed: seed.to_vec(),
            master_private_key,
            master_public_key,
            addresses: Vec::new(),
        };

        wallet.generate_addresses(10)?;

        Ok(wallet)
    }

    /// Gera endereços Taproot (P2TR)
    pub fn generate_addresses(&mut self, count: usize) -> Result<(), Box<dyn std::error::Error>> {
        let secp = Secp256k1::new();
        let derivation_path = "m/86'/0'/0'/0".parse::<DerivationPath>()?;

        for i in 0..count {
            let child_path = derivation_path.extend([ChildNumber::from_normal_idx(i as u32)?]);
            let child_priv = self.master_private_key.derive_priv(&secp, &child_path)?;
            let child_pub = Xpub::from_priv(&secp, &child_priv);

            // Para Taproot, usar P2TR
            let address = Address::p2tr(&secp, child_pub.public_key.into(), None, Network::Bitcoin);
            self.addresses.push(address.to_string());
        }

        Ok(())
    }

    /// Retorna o primeiro endereço
    pub fn get_first_address(&self) -> Option<&String> {
        self.addresses.first()
    }
}
