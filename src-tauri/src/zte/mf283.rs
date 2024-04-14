use crate::zte::{Config, ZteSession};
use async_trait::async_trait;

pub struct Session {
    config: Config,
}

impl Session {
    pub(crate) fn new(config: Config) -> Self {
        Self { config }
    }
}

#[async_trait]
impl ZteSession for Session {
    async fn toggle_connection(&mut self, _on: bool) -> anyhow::Result<()> {
        anyhow::bail!("unimplemented");
    }

    async fn toggle_mode(&mut self, _e5g: bool) -> anyhow::Result<()> {
        anyhow::bail!("unimplemented");
    }

    fn update_config(&mut self, _config: Config) {
        self.config = _config;
    }

    async fn gsm_info(&mut self) -> anyhow::Result<super::GsmInfo> {
        anyhow::bail!("unimplemented");
    }

    async fn modem_info(&mut self) -> anyhow::Result<super::ModemInfo> {
        anyhow::bail!("unimplemented");
    }
}
