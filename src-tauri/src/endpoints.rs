use log::error;
use serde::{Deserialize, Serialize};
use tauri::{Manager, State};
use tokio::sync::oneshot;

use crate::settings::{self, Settings};
use crate::worker::{Command, LockedChannel};

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct Stats {
    pub(crate) online: bool,
    pub(crate) status: String,
    pub(crate) mode: String,
    pub(crate) provider: String,
    pub(crate) signal: u8,
    pub(crate) time: u64,
    pub(crate) total_rx: u64,
    pub(crate) total_tx: u64,
    pub(crate) current_rx: u64,
    pub(crate) current_tx: u64,
    pub(crate) month_rx: u64,
    pub(crate) month_tx: u64,
}

impl std::fmt::Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[tauri::command]
pub(crate) async fn stats(cc: State<'_, LockedChannel>) -> Result<Stats, String> {
    let (tx, rx) = oneshot::channel();
    let _ = cc.send(Command::Query { tx }).await;
    rx.await.map_err(|_| "error".to_string())?
}

#[tauri::command]
pub(crate) async fn toggle_connection(on: bool, cc: State<'_, LockedChannel>) -> Result<(), ()> {
    let (tx, rx) = oneshot::channel();
    let _ = cc.send(Command::ToggleConnection { on, tx }).await;
    rx.await.map_err(|_| ()).map(|_| ())
}

#[tauri::command]
pub(crate) async fn toggle_mode(enable_5g: bool, cc: State<'_, LockedChannel>) -> Result<(), ()> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let _ = cc.send(Command::ToggleMode { e5g: enable_5g, tx }).await;
    rx.await.map_err(|_| ()).map(|_| ())
}

#[tauri::command]
pub(crate) async fn force_refresh(
    cc: State<'_, LockedChannel>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let settings =
        settings::load(&app.path().app_data_dir().unwrap()).map_err(|e| e.to_string())?;
    let (tx, rx) = tokio::sync::oneshot::channel();
    let _ = cc.send(Command::UpdateSettings { tx, cfg: settings }).await;
    rx.await.unwrap();
    Ok(())
}

#[tauri::command]
pub(crate) async fn get_settings(app: tauri::AppHandle) -> Result<Settings, String> {
    settings::load(&app.path().app_data_dir().unwrap()).map_err(|e| e.to_string())
}

#[tauri::command]
pub(crate) async fn set_settings(
    settings: Settings,
    cc: State<'_, LockedChannel>,
    app: tauri::AppHandle,
) -> Result<(), ()> {
    let data_dir = app.path().app_data_dir().unwrap();
    if let Err(e) = crate::settings::store(&data_dir, &settings).await {
        error!("Failed to store settings: {}", e);
        return Err(());
    }

    let (tx, rx) = tokio::sync::oneshot::channel();
    let _ = cc
        .send(Command::UpdateSettings {
            tx,
            cfg: settings::load(&data_dir).unwrap(),
        })
        .await;
    rx.await.unwrap();

    Ok(())
}
