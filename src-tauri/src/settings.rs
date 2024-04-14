use log::{error, info, trace, warn};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Settings {
    #[serde(alias = "deviceModel")]
    pub(crate) device_model: String,

    #[serde(alias = "deviceIp")]
    pub(crate) device_ip: String,

    #[serde(default, skip_serializing)]
    pub(crate) password: String,

    #[serde(default)]
    pub(crate) defaults: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            device_model: "mc889".into(), // TODO: stringify ZTE::device::DeviceModel
            device_ip: "192.168.0.1".into(),
            password: "admin".into(),
            defaults: false,
        }
    }
}

pub fn load(data_dir: &Path) -> anyhow::Result<Settings> {
    trace!("loading settings");

    let mut settings = Settings::default();
    let mut cfg_file = data_dir.to_path_buf();
    cfg_file.push("store.bin");

    if let Ok(contents) = std::fs::read(&cfg_file) {
        let contents = String::from_utf8(contents)?;
        let data = serde_json::from_str::<Settings>(&contents);
        if data.is_ok() {
            settings = data.unwrap();
        } else {
            error!("Failed to parse settings file: {:#?}", data);
        }
    } else {
        warn!(
            "Failed to read settings file '{:#?}' - runing with defaults",
            &cfg_file
        );
        return Ok(settings);
    }

    // password
    let client = keyring::Entry::new("zetapp", &settings.device_ip)?;
    settings.password = client.get_password().unwrap_or_else(|_| {
        warn!("no password found in keychain - using default");
        Settings::default().password
    });

    Ok(settings)
}

pub async fn store(data_dir: &Path, settings: &Settings) -> anyhow::Result<()> {
    let mut cfg_file = data_dir.to_path_buf();

    tokio::fs::create_dir_all(&cfg_file).await?;

    cfg_file.push("store.bin");

    info!("Storing settings to '{:#?}'", &cfg_file);

    let contents = serde_json::to_string(settings)?;
    tokio::fs::write(&cfg_file, contents).await?;

    if !settings.password.is_empty() {
        let client = keyring::Entry::new("zetapp", &settings.device_ip)?;
        client.set_password(&settings.password)?;
    }
    Ok(())
}
