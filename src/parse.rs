use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use solana_sdk::signer::keypair::Keypair;
use std::{env, fs, path::Path};

#[derive(Debug, Deserialize, Serialize)]
pub struct SolanaConfig {
    pub json_rpc_url: String,
    pub keypair_path: String,
    pub commitment: String,
}

pub fn parse_keypair(path: &String) -> Result<Keypair> {
    let secret: Vec<u8> = fs::read_to_string(path)
        .context("Can't find key file")?
        .trim_start_matches("[")
        .trim_end_matches("]")
        .split(",")
        .map(|c| c.parse::<u8>().unwrap())
        .collect();
    let keypair = Keypair::from_bytes(&secret)?;
    Ok(keypair)
}

pub fn parse_solana_config() -> Option<SolanaConfig> {
    let key = "HOME";
    let home = match env::var_os(key) {
        Some(val) => val,
        None => return None,
    };

    let config_path = Path::new(&home)
        .join(".config")
        .join("solana")
        .join("cli")
        .join("config.yml");

    let conf_file = match fs::File::open(config_path) {
        Ok(f) => f,
        Err(_) => return None,
    };
    serde_yaml::from_reader(&conf_file).ok()
}
