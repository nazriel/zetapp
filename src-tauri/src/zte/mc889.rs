use crate::zte::errors::Error;
use crate::zte::{Config, GsmInfo, ModemInfo, ZteSession};

use async_trait::async_trait;
use log::debug;
use reqwest::header::HeaderValue;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::time::{Duration, SystemTime};

pub struct Session {
    config: Config,
    token: String,
    last_token_refresh: SystemTime,
    last_login_error: SystemTime,
    login_errors_cnt: u8,
}

#[async_trait]
impl ZteSession for Session {
    fn update_config(&mut self, config: Config) {
        self.config = config;
        self.token.clear();
        self.login_errors_cnt = 0;
    }

    async fn toggle_connection(&mut self, on: bool) -> anyhow::Result<()> {
        self.check_token().await?;

        let ad = self.fetch_ad().await?;

        let params = [
            (
                "goformId",
                if on {
                    "CONNECT_NETWORK"
                } else {
                    "DISCONNECT_NETWORK"
                },
            ),
            ("AD", &ad),
            ("isTest", "false"),
            ("notCallback", "true"),
        ];

        let client = self
            .client()?
            .build()?
            .post(self.url("goform_set_cmd_process"))
            .form(&params);

        let res = client.send().await?;
        if !&res.status().is_success() {
            anyhow::bail!(":(");
        }

        Ok(())
    }

    async fn toggle_mode(&mut self, e5g: bool) -> anyhow::Result<()> {
        self.check_token().await?;

        let ad = self.fetch_ad().await?;

        let mask = if e5g {
            "1,3,7,8,28,38,41,78"
        } else {
            "1,3,7,8,28,38,78"
        };

        let params = [
            ("goformId", "WAN_PERFORM_NR5G_SANSA_BAND_LOCK"),
            ("AD", &ad),
            ("isTest", "false"),
            ("nr5g_band_mask", mask),
            ("type", "1"),
        ];

        let client = self
            .client()?
            .build()?
            .post(self.url("goform_set_cmd_process"))
            .form(&params);

        let res = client.send().await?;
        if !&res.status().is_success() {
            anyhow::bail!(":(");
        }
        debug!("set mode: {:?}", res.text().await?);

        Ok(())
    }

    async fn modem_info(&mut self) -> anyhow::Result<ModemInfo> {
        self.check_token().await?;

        let fields = [
            "signalbar",
            "network_type",
            "network_provider",
            "ppp_status",
            "realtime_tx_bytes",
            "realtime_rx_bytes",
            "realtime_time",
            "realtime_tx_thrpt",
            "realtime_rx_thrpt",
            "monthly_rx_bytes",
            "monthly_tx_bytes",
            "monthly_time",
            "date_month",
        ];
        let modes = format!(
            "multi_data=1&isTest=false&sms_received_flag_flag=0&sts_received_flag_flag=0&cmd={}",
            fields.join("%2C")
        );
        let client = self
            .client()?
            .build()?
            .get(self.url(&format!("goform_get_cmd_process?{modes}")));

        let res = client.send().await?.text().await?;
        let data: ModemInfo = serde_json::from_str(&res)?;
        Ok(data)
    }

    async fn gsm_info(&mut self) -> anyhow::Result<GsmInfo> {
        self.check_token().await?;

        let fields = ["network_type"];
        let modes = format!(
            "multi_data=1&isTest=false&sms_received_flag_flag=0&sts_received_flag_flag=0&cmd={}",
            fields.join("%2C")
        );
        let client = self
            .client()?
            .build()?
            .get(self.url(&format!("goform_get_cmd_process?{modes}")));
        let res = client.send().await?.text().await?;
        let data: GsmInfo = serde_json::from_str(&res)?;
        Ok(data)
    }
}

impl Session {
    pub(crate) fn new(config: Config) -> Self {
        Self {
            config,
            token: String::default(),
            last_token_refresh: SystemTime::now(),
            last_login_error: SystemTime::UNIX_EPOCH,
            login_errors_cnt: 0,
        }
    }

    async fn check_token(&mut self) -> anyhow::Result<()> {
        if self.token.is_empty()
            || self.last_token_refresh < (SystemTime::now() - Duration::from_secs(60))
        {
            if self.login_errors_cnt >= 3
                && self.last_login_error.elapsed()? < Duration::from_secs(60)
            {
                return Err(Error::LoginError("too many login errors".into()).into());
            }

            self.token.clear();
            match self.login().await {
                Ok(tok) => {
                    self.login_errors_cnt = 0;
                    self.token = tok;
                }
                Err(e) => {
                    self.login_errors_cnt += 1;
                    self.last_login_error = SystemTime::now();
                    return Err(e);
                }
            }
            self.last_token_refresh = SystemTime::now();
        }
        Ok(())
    }

    async fn login(&self) -> anyhow::Result<String> {
        let client = self
            .client()?
            .build()?
            .get(self.url("goform_get_cmd_process?isTest=false&cmd=LD"));

        let tok = client.send().await?.json::<LD>().await?.ld;

        let pass = sha(&self.config.auth)?;
        let pass = format!("{pass}{tok}");
        let pass = sha(&pass)?;

        let params = [
            ("goformId", "LOGIN"),
            ("password", &pass),
            ("isTest", "false"),
        ];

        let req = self
            .client()?
            .build()?
            .post(self.url("goform_set_cmd_process"))
            .form(&params);

        let res = req.send().await?;
        if !&res.status().is_success() {
            anyhow::bail!(Error::LoginError("ZTE API error".into()));
        }

        let stok_cookie = &res
            .cookies()
            .find(|cookie| cookie.name() == "stok")
            .map(|cookie| String::from(cookie.value()));

        let status = &res.json::<LoginResult>().await?;

        anyhow::ensure!(
            status.result == "0",
            Error::LoginError(if status.result == "3" {
                "password is wrong".into()
            } else {
                "login failed".into()
            })
        );

        stok_cookie
            .clone()
            .ok_or_else(|| anyhow::anyhow!(Error::LoginError("cookie with token not found".into())))
    }

    fn url(&self, path: &str) -> String {
        let unixts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let url = format!("http://{}/goform/{path}", self.config.host);
        if url.contains('?') {
            format!("{url}&_={unixts}")
        } else {
            format!("{url}?_={unixts}")
        }
    }

    fn client(&self) -> anyhow::Result<reqwest::ClientBuilder> {
        let device_host = &self.config.host;
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            "Referer",
            HeaderValue::from_str(format!("http://{device_host}/index.html").as_str())?,
        );
        headers.append(
            "Accept",
            HeaderValue::from_str("application/json, text/javascript, */*; q=0.01")?,
        );
        if !self.token.is_empty() {
            headers.append(
                "Cookie",
                HeaderValue::from_str(format!("stok={}", self.token).as_str())?,
            );
        }
        let builder = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_millis(250));
        Ok(builder)
    }

    async fn fetch_ad(&self) -> anyhow::Result<String> {
        // self.check_token() ?
        let client = self.client()?.build()?
            .get(self.url("goform_get_cmd_process?isTest=false&cmd=Language%2Ccr_version%2Cwa_inner_version%2Cloginfo&multi_data=1"));

        let data = client.send().await?.json::<Lang>().await?;
        let rd0 = data.wa_inner_version;
        let rd1 = data.cr_version;
        let rds_sha = sha(&format!("{rd0}{rd1}"))?;

        let client = self
            .client()?
            .build()?
            .get(self.url("goform_get_cmd_process?isTest=false&cmd=RD"));
        let rd = client.send().await?.json::<RD>().await?.rd;

        sha(&format!("{rds_sha}{rd}"))
    }
}

#[derive(Deserialize, Debug)]
struct LD {
    #[serde(rename(deserialize = "LD"))]
    ld: String,
}

#[derive(Deserialize, Debug)]
struct RD {
    #[serde(rename(deserialize = "RD"))]
    rd: String,
}

#[derive(Debug, Deserialize)]
struct Lang {
    cr_version: String,
    wa_inner_version: String,
}

#[derive(Deserialize, Debug)]
struct LoginResult {
    result: String,
}

fn sha(input: &str) -> anyhow::Result<String> {
    let mut sha256 = Sha256::new();
    sha256.update(input);
    Ok(format!("{:X}", sha256.finalize()))
}
