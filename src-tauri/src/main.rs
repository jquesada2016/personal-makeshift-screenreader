#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[macro_use]
extern crate serde;
#[macro_use]
extern crate tracing;

#[macro_use]
mod error;
mod cmds;
mod color_filtering;

use color_filtering::ColorFilteringState;
use tauri::{
  generate_handler, CustomMenuItem, Manager, SystemTray, SystemTrayEvent,
  SystemTrayMenu,
};
use tracing_subscriber::prelude::*;


#[tokio::main]
async fn main() {
  let fmt_layer = tracing_subscriber::fmt::layer().pretty();

  tracing_subscriber::registry().with(fmt_layer).init();

  tauri::Builder::default()
    .invoke_handler(generate_handler![
      cmds::get_cursor_position,
      cmds::is_color_filter_on,
      cmds::get_text_at_cursor,
    ])
    .manage(ColorFilteringState::new().expect("key to be created successfully"))
    .system_tray(SystemTray::new().with_menu(
      SystemTrayMenu::new().add_item(CustomMenuItem::new("quit", "Quit")),
    ))
    .on_system_tray_event(|app, e| match e {
      SystemTrayEvent::MenuItemClick { id, .. } if id == "quit" => app.exit(0),
      _ => {}
    })
    .setup(|app| {
      app
        .state::<ColorFilteringState>()
        .watch_for_changes(app.handle());

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
