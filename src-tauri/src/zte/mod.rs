use async_trait::async_trait;
use serde::{Deserialize, Serialize};

mod errors;
mod mc889;
mod mf283;

pub struct Config {
    host: String,
    auth: String,
}

impl Config {
    pub fn new(host: &str, auth: &str) -> Self {
        Self {
            host: String::from(host),
            auth: String::from(auth),
        }
    }
}

#[derive(Debug)]
pub enum DeviceModel {
    Mc889,
    Mf283,
}

impl From<std::string::String> for DeviceModel {
    fn from(s: std::string::String) -> Self {
        match s.as_str() {
            "mc889" => DeviceModel::Mc889,
            "mf283" => DeviceModel::Mf283,
            _ => panic!("unsupported device"),
        }
    }
}

// impl std::fmt::Display for DeviceModel {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

#[derive(Deserialize, Serialize, Default, Clone, Debug)]
pub struct ModemInfo {
    pub ppp_status: String,
    pub network_provider: String,
    pub signalbar: String,
    pub realtime_time: String,
    pub realtime_rx_bytes: String,
    pub realtime_tx_bytes: String,
    pub realtime_rx_thrpt: String,
    pub realtime_tx_thrpt: String,
    pub monthly_rx_bytes: String,
    pub monthly_tx_bytes: String,
}

#[derive(Deserialize, Serialize, Default, Clone, Debug)]
pub struct GsmInfo {
    pub network_type: String,
}

#[async_trait]
#[trait_variant::make(Send)]
pub trait ZteSession {
    async fn toggle_connection(&mut self, on: bool) -> anyhow::Result<()>;
    async fn toggle_mode(&mut self, e5g: bool) -> anyhow::Result<()>;
    fn update_config(&mut self, config: Config);
    async fn gsm_info(&mut self) -> anyhow::Result<GsmInfo>;
    async fn modem_info(&mut self) -> anyhow::Result<ModemInfo>;
}

pub fn get_session(device: DeviceModel, config: Config) -> Box<dyn ZteSession> {
    match device {
        DeviceModel::Mc889 => Box::new(mc889::Session::new(config)),
        DeviceModel::Mf283 => Box::new(mf283::Session::new(config)),
    }
}
