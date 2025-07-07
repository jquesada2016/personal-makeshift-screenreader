use std::str::FromStr;

use tauri::Manager;
use tauri_plugin_global_shortcut::{GlobalShortcut, GlobalShortcutExt};
use tauri_plugin_store::{Store, StoreExt};
use wasm::settings::{Settings, Shortcuts};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let crosshair_window = app
                .get_webview_window("crosshair")
                .expect("getting crosshair window");

            crosshair_window.set_ignore_cursor_events(true).unwrap();

            let store = app
                .store_builder("settings.json")
                .create_new()
                .default("settings", default_json_settings())
                .build()
                .expect("failed to building settings store");

            let settings = get_settings(&store);

            let shortcuts = app.global_shortcut();

            register_shortcuts(shortcuts, &settings.shortcuts);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn default_json_settings() -> serde_json::Value {
    let settings = Settings::default();

    serde_json::to_value(&settings).unwrap()
}

fn get_settings<R: tauri::Runtime>(store: &Store<R>) -> Settings {
    let settings = store.get("settings").unwrap();
    serde_json::from_value::<Settings>(settings).expect("reading settings")
}

fn register_shortcuts<R: tauri::Runtime>(
    shortcuts_manager: &GlobalShortcut<R>,
    shortcuts: &Shortcuts,
) {
    let Shortcuts {
        show_settings,
        show_crosshair,
        increase_line_thickness: _,
        decrease_line_thickness: _,
        increase_pointer_gap: _,
        decrease_pointer_gap: _,
    } = shortcuts;

    shortcuts_manager
        .on_shortcut(show_settings.as_str(), |app, _, _| {
            let settings_window = app.get_webview_window("settings").unwrap_or_else(|| {
                tauri::WebviewWindow::builder(
                    app,
                    "settings",
                    tauri::WebviewUrl::App("/settings.html".into()),
                )
                .title("Settings")
                .center()
                .inner_size(800.0, 600.0)
                .build()
                .unwrap()
            });

            settings_window.show().unwrap();
            settings_window.set_focus().unwrap();
        })
        .unwrap();

    shortcuts_manager
        .on_shortcut(show_crosshair.as_str(), |app, _, _| {
            let crosshair_window = app.get_webview_window("crosshair").unwrap_or_else(|| {
                tauri::WebviewWindow::builder(
                    app,
                    "crosshair",
                    tauri::WebviewUrl::App("/crosshair.html".into()),
                )
                .title("Crosshair")
                .maximized(true)
                .transparent(true)
                .decorations(false)
                .always_on_top(true)
                .visible_on_all_workspaces(true)
                .visible(false)
                .build()
                .unwrap()
            });

            crosshair_window.set_ignore_cursor_events(true).unwrap();

            if crosshair_window.is_visible().unwrap() {
                crosshair_window.hide().unwrap();
            } else {
                crosshair_window.show().unwrap()
            }
        })
        .unwrap();
}
