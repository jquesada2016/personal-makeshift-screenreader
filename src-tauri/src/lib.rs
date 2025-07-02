#[macro_use]
extern crate serde;

use mouse_position::mouse_position;
use mouse_position::Mouse;
use tauri::Manager;
use tauri_plugin_store::StoreExt;

#[derive(Default, Serialize)]
struct Position {
    x: i32,
    y: i32,
}

#[tauri::command]
fn get_mouse_position() -> Position {
    match Mouse::get_mouse_position() {
        Mouse::Position { x, y } => Position { x, y },
        Mouse::Error => Position::default(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![get_mouse_position])
        .setup(|app| {
            let crosshair_window = app.get_webview_window("crosshair").expect("`main` window");

            crosshair_window.set_ignore_cursor_events(true).unwrap();

            app.store_builder("settings.json")
                .default("settings", Settings)
                .build()
                .expect("error building settings store");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct Settings;

impl From<Settings> for serde_json::Value {
    fn from(_: Settings) -> Self {
        serde_json::json!({
            "lineThickness": 1.5,
            "pointerGap": 8.0,
            "shortcuts": {
                "showSettings": {
                    "meta": true,
                    "ctrl": true,
                    "key": "KeyC",
                },
                "increaseLineThickness": {
                    "meta": true,
                    "ctrl": true,
                    "key": "KeyT",
                },
                "decreaseLineThickness": {
                    "meta": true,
                    "ctrl": true,
                    "shift": true,
                    "key": "KeyT",
                },
                "increasePointerGap": {
                    "meta": true,
                    "ctrl": true,
                    "key": "KeyP",
                },
                "decreasePointerGap": {
                    "meta": true,
                    "ctrl": true,
                    "shift": true,
                    "key": "KeyP",
                },
            }
        })
    }
}
