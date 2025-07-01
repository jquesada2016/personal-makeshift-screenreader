#[macro_use]
extern crate serde;
#[macro_use]
extern crate typed_builder;

use tauri_plugin_global_shortcut::Code;
use tauri_plugin_store::StoreExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            app.store_builder("settings.json")
                .default("settings", Settings)
                .build()
                .expect("error while building settings store");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Default)]
struct Settings;

impl From<Settings> for serde_json::Value {
    fn from(_: Settings) -> Self {
        serde_json::json!({
            "lineThickness": 1.5,
            "pointerGap": 8.0,
            "shortcuts": {
                "showSettings": {
                    "metaKey": true,
                    "altKey": true,
                    "key": "KeyC",
                },
                "increaseLineThickness": {
                    "metaKey": true,
                    "ctrlKey": true,
                    "key": "KeyT",
                },
                "decreaseLineThickness": {
                    "metaKey": true,
                    "ctrlKey": true,
                    "shiftKey": true,
                    "key": "KeyT",
                },
                "increasePointerGap": {
                    "metaKey": true,
                    "ctrlKey": true,
                    "key": "KeyP",
                },
                "decreasePointerGap": {
                    "metaKey": true,
                    "ctrlKey": true,
                    "shiftKey": true,
                    "key": "KeyP",
                },
            }
        })
    }
}
