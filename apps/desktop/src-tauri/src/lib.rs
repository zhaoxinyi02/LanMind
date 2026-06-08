use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
};
use tauri_plugin_sql::{Migration, MigrationKind};

#[tauri::command]
fn open_pet_window(app: tauri::AppHandle) -> Result<(), String> {
    app.get_webview_window("pet")
        .ok_or_else(|| "pet window is unavailable".to_string())?
        .show()
        .map_err(|error| error.to_string())
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
        .invoke_handler(tauri::generate_handler![open_pet_window])
        .run(tauri::generate_context!())
        .expect("failed to run LanMind");
}
