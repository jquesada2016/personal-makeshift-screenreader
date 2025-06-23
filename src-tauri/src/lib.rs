#[macro_use]
extern crate serde;

use mouse_position::mouse_position;
use mouse_position::Mouse;
use tauri::Manager;

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
        .invoke_handler(tauri::generate_handler![get_mouse_position])
        .setup(|app| {
            let crosshair_window = app.get_webview_window("crosshair").expect("`main` window");

            crosshair_window.set_ignore_cursor_events(true).unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
