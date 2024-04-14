use log::error;
use tauri::Manager;
use tokio::sync::oneshot;
use tokio::time as tt;

use crate::endpoints::Stats;
use crate::settings::Settings;
use crate::zte;

pub(crate) enum Command {
    Query {
        tx: oneshot::Sender<anyhow::Result<Stats, String>>,
    },
    ToggleMode {
        e5g: bool,
        tx: oneshot::Sender<bool>,
    },
    ToggleConnection {
        on: bool,
        tx: oneshot::Sender<bool>,
    },
    UpdateSettings {
        cfg: Settings,
        tx: oneshot::Sender<()>,
    },
}

pub(crate) type LockedChannel = tokio::sync::mpsc::Sender<Command>;

pub(crate) fn worker(settings: &Settings, app: &mut tauri::App) -> anyhow::Result<()> {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Command>(1);
    app.manage(tx);
    let (ip, passwd, model) = (
        settings.device_ip.clone(),
        settings.password.clone(),
        settings.device_model.clone(),
    );

    tauri::async_runtime::spawn(async move {
        let mut session = zte::get_session(model.into(), zte::Config::new(&ip, &passwd));

        let mut interval = tt::interval(tt::Duration::from_millis(1500));
        let mut gsm_info = zte::GsmInfo::default();
        let mut modem_info = zte::ModemInfo::default();
        let mut succesful_fetch = false;
        let mut last_err: Option<String> = None;

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    succesful_fetch = true;
                    last_err = None;
                    match session.modem_info().await {
                        Ok(m) => {
                            modem_info = m;
                        }
                        Err(e) => {
                            error!("modem info error: {:?}", e);
                            modem_info = zte::ModemInfo::default();
                            succesful_fetch = false;
                            last_err = format!("{:?}", e).into();
                            interval.reset();
                            continue;
                        }
                    }

                    match session.gsm_info().await {
                        Ok(g) => {
                            gsm_info = g;
                        }
                        Err(e) => {
                            error!("gsm info error");
                            gsm_info = zte::GsmInfo::default();
                            last_err = format!("{:?}", e).into();
                            succesful_fetch = false;
                        }
                    }
                }
                Some(cmd) = rx.recv() => {
                match cmd {
                    Command::Query { tx } => {
                        if let Some(e) = last_err.as_deref() {
                            let _ = tx.send(Err(e.to_string()));
                            continue;
                        }
                        let _ = tx.send(Ok(Stats {
                            online: succesful_fetch,
                            status: modem_info.ppp_status.clone(),
                            mode: match gsm_info.network_type.as_str() {
                                "ENDC" => "5G".to_string(),
                                _ => "LTE".to_string(),
                            },
                            provider: modem_info.network_provider.clone(),
                            signal: modem_info.signalbar.parse().unwrap_or(0),
                            time: modem_info.realtime_time.parse().unwrap_or(0),
                            total_rx: modem_info.realtime_rx_bytes.parse().unwrap_or(0),
                            total_tx: modem_info.realtime_tx_bytes.parse().unwrap_or(0),
                            current_rx: modem_info.realtime_rx_thrpt.parse().unwrap_or(0),
                            current_tx: modem_info.realtime_tx_thrpt.parse().unwrap_or(0),
                            month_rx: modem_info.monthly_rx_bytes.parse().unwrap_or(0),
                            month_tx: modem_info.monthly_tx_bytes.parse().unwrap_or(0)
                        }));
                    }
                    Command::ToggleMode { e5g, tx } => {
                        println!("Toggle mode {}", e5g);
                        let _ = tx.send(session.toggle_mode(e5g).await.is_ok());
                    }
                    Command::ToggleConnection { on, tx } => {
                        println!("Toggle connection {}", on);
                        let _ = tx.send(session.toggle_connection(on).await.is_ok());
                    }
                    Command::UpdateSettings { tx, cfg } => {
                        println!("Update settings");
                        if !cfg.password.is_empty() {
                            session.update_config(zte::Config::new(&cfg.device_ip, &cfg.password));
                        } else {
                            session.update_config(zte::Config::new(&cfg.device_ip, &passwd));
                        }
                        let _ = tx.send(());
                    }
                }
            }
            }
        }
    });
    Ok(())
}
