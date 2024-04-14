use log::{info, LevelFilter};
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};

mod endpoints;
mod settings;
mod worker;
mod zte;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    // Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .level(LevelFilter::Trace)
                .filter(|metadata| metadata.target().starts_with("zetapp"))
                .build(),
        )
        .setup(|app| {
            info!("Starting zetapp v");
            let settings = settings::load(&app.path().app_data_dir().unwrap())?;
            worker::worker(&settings, app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            endpoints::stats,
            endpoints::toggle_connection,
            endpoints::toggle_mode,
            endpoints::get_settings,
            endpoints::set_settings,
            endpoints::force_refresh
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
