use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager, WindowEvent,
};
use tauri_plugin_sql::{Migration, MigrationKind};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::Command};

const RELEASES_API: &str = "https://api.github.com/repos/zhaoxinyi02/LanMind/releases";

#[derive(Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Deserialize)]
struct GithubRelease {
    tag_name: String,
    html_url: String,
    body: Option<String>,
    draft: bool,
    assets: Vec<GithubAsset>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateInfo {
    current_version: String,
    latest_version: String,
    release_url: String,
    notes: String,
    download_url: String,
    available: bool,
}

#[tauri::command]
fn open_pet_window(app: tauri::AppHandle) -> Result<(), String> {
    app.get_webview_window("pet")
        .ok_or_else(|| "pet window is unavailable".to_string())?
        .show()
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn hide_pet_window(app: tauri::AppHandle) -> Result<(), String> {
    app.get_webview_window("pet")
        .ok_or_else(|| "pet window is unavailable".to_string())?
        .hide()
        .map_err(|error| error.to_string())
}

#[tauri::command]
async fn check_for_update(app: tauri::AppHandle) -> Result<UpdateInfo, String> {
    let current_version = app.package_info().version.to_string();
    let client = reqwest::Client::builder()
        .user_agent("LanMind Desktop")
        .build()
        .map_err(|error| error.to_string())?;
    let releases = client
        .get(RELEASES_API)
        .send()
        .await
        .map_err(|error| error.to_string())?
        .error_for_status()
        .map_err(|error| error.to_string())?
        .json::<Vec<GithubRelease>>()
        .await
        .map_err(|error| error.to_string())?;
    let release = releases
        .into_iter()
        .find(|release| !release.draft)
        .ok_or_else(|| "没有找到可用版本".to_string())?;
    let latest_version_text = release.tag_name.trim_start_matches('v').to_string();
    let current = semver::Version::parse(&current_version).map_err(|error| error.to_string())?;
    let latest = semver::Version::parse(&latest_version_text).map_err(|error| error.to_string())?;
    let asset = release
        .assets
        .into_iter()
        .find(|asset| asset.name.ends_with("_x64-setup.exe"))
        .ok_or_else(|| "最新版本没有 Windows 安装包".to_string())?;

    Ok(UpdateInfo {
        current_version,
        latest_version: latest_version_text,
        release_url: release.html_url,
        notes: release.body.unwrap_or_default(),
        download_url: asset.browser_download_url,
        available: latest > current,
    })
}

#[tauri::command]
async fn download_update(download_url: String) -> Result<String, String> {
    if !download_url.starts_with("https://github.com/zhaoxinyi02/LanMind/releases/download/") {
        return Err("拒绝非官方 GitHub Release 下载地址".to_string());
    }
    let response = reqwest::Client::builder()
        .user_agent("LanMind Desktop")
        .build()
        .map_err(|error| error.to_string())?
        .get(download_url)
        .send()
        .await
        .map_err(|error| error.to_string())?
        .error_for_status()
        .map_err(|error| error.to_string())?;
    let installer_path: PathBuf = std::env::temp_dir().join("LanMind-latest-setup.exe");
    let mut file = tokio::fs::File::create(&installer_path)
        .await
        .map_err(|error| error.to_string())?;
    let mut stream = response.bytes_stream();
    use tokio::io::AsyncWriteExt;
    while let Some(chunk) = stream.next().await {
        file.write_all(&chunk.map_err(|error| error.to_string())?)
            .await
            .map_err(|error| error.to_string())?;
    }
    file.flush().await.map_err(|error| error.to_string())?;
    Ok(installer_path.to_string_lossy().into_owned())
}

#[tauri::command]
fn install_update(app: tauri::AppHandle, installer_path: String) -> Result<(), String> {
    let expected_path = std::env::temp_dir().join("LanMind-latest-setup.exe");
    if PathBuf::from(&installer_path) != expected_path || !expected_path.is_file() {
        return Err("更新安装包路径无效".to_string());
    }
    Command::new(expected_path)
        .arg("/S")
        .spawn()
        .map_err(|error| error.to_string())?;
    app.exit(0);
    Ok(())
}

pub fn run() {
    let migrations = vec![
        Migration {
            version: 1,
            description: "initialize_lanmind_local_database",
            sql: include_str!("../migrations/0001_init.sql"),
            kind: MigrationKind::Up,
        },
    ];

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:lanmind.db", migrations)
                .build(),
        )
        .setup(|app| {
            let show = MenuItem::with_id(app, "show", "显示澜灵", true, None::<&str>)?;
            let pet = MenuItem::with_id(app, "pet", "显示桌宠", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &pet, &quit])?;

            TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "pet" => {
                        if let Some(window) = app.get_webview_window("pet") {
                            let _ = window.show();
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            open_pet_window,
            hide_pet_window,
            check_for_update,
            download_update,
            install_update
        ])
        .run(tauri::generate_context!())
        .expect("failed to run LanMind");
}
