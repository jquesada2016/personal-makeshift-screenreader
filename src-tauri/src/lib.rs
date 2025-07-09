use tauri::Manager;
use tauri_plugin_global_shortcut::{GlobalShortcut, GlobalShortcutExt, ShortcutState};
use tauri_plugin_store::{Store, StoreExt};
use wasm::settings::{self, Settings, Shortcuts};

const CROSSHAIR_WINDOW_LABEL: &str = "crosshair";
const SETTINGS_WINDOW_LABEL: &str = "settings";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(setup_app)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_app<R: tauri::Runtime>(app: &mut tauri::App<R>) -> Result<(), Box<dyn std::error::Error>> {
    get_crosshair_window(app.handle())
        .set_ignore_cursor_events(true)
        .unwrap();

    let store = app
        .store_builder(settings::STORAGE_PATH)
        .default(settings::STORAGE_KEY, default_json_settings())
        .build()
        .expect("failed to building settings store");

    let settings = get_settings(&store);

    let shortcuts_manager = app.global_shortcut();

    register_shortcuts(shortcuts_manager, &settings.shortcuts);

    Ok(())
}

fn default_json_settings() -> serde_json::Value {
    let settings = Settings::default();

    serde_json::to_value(&settings).unwrap()
}

fn get_settings<R: tauri::Runtime>(store: &Store<R>) -> Settings {
    let settings = store.get(settings::STORAGE_KEY).unwrap();
    serde_json::from_value::<Settings>(settings).expect("reading settings")
}

fn register_shortcuts<R: tauri::Runtime>(
    shortcuts_manager: &GlobalShortcut<R>,
    shortcuts: &Shortcuts,
) {
    let Shortcuts {
        show_settings,
        show_crosshair,
    } = shortcuts;

    shortcuts_manager
        .on_shortcut(show_settings.as_str(), |app, _, e| {
            if e.state == ShortcutState::Released {
                return;
            }

            let settings_window = get_settings_window(app);

            settings_window.show().unwrap();
            settings_window.set_focus().unwrap();
        })
        .unwrap();

    shortcuts_manager
        .on_shortcut(show_crosshair.as_str(), |app, _, e| {
            if e.state == ShortcutState::Released {
                return;
            }

            let crosshair_window = get_crosshair_window(app);

            if crosshair_window.is_visible().unwrap() {
                crosshair_window.hide().unwrap();
            } else {
                crosshair_window.show().unwrap();
                crosshair_window
                    .set_position(tauri::PhysicalPosition::<i32>::from((0, 0)))
                    .unwrap();
                crosshair_window.maximize().unwrap();
            }
        })
        .unwrap();

    #[cfg(debug_assertions)]
    shortcuts_manager
        .on_shortcut("Cmd + Ctrl + Alt + C", |app, _, e| {
            if e.state == ShortcutState::Released {
                return;
            }

            let crosshair_window = get_crosshair_window(app);

            if crosshair_window.is_devtools_open() {
                crosshair_window.close_devtools();
                crosshair_window.set_ignore_cursor_events(true).unwrap();
            } else {
                crosshair_window.open_devtools();
                crosshair_window.set_ignore_cursor_events(false).unwrap();
            }
        })
        .unwrap();
}

fn get_settings_window<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> tauri::WebviewWindow<R> {
    app.get_webview_window(SETTINGS_WINDOW_LABEL)
        .unwrap_or_else(|| {
            tauri::WebviewWindow::builder(
                app,
                SETTINGS_WINDOW_LABEL,
                tauri::WebviewUrl::App("/settings.html".into()),
            )
            .title("Settings")
            .center()
            .inner_size(800.0, 600.0)
            .build()
            .unwrap()
        })
}

fn get_crosshair_window<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> tauri::WebviewWindow<R> {
    app.get_webview_window(CROSSHAIR_WINDOW_LABEL)
        .unwrap_or_else(|| {
            let crosshair_window = tauri::WebviewWindow::builder(
                app,
                CROSSHAIR_WINDOW_LABEL,
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
            .unwrap();

            crosshair_window.set_ignore_cursor_events(false).unwrap();

            crosshair_window
        })
}
