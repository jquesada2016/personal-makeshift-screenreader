#[macro_use]
extern crate tracing;
#[macro_use]
extern crate serde;

mod page;
pub mod settings;
mod tauri;

use tracing_subscriber::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    init_tracing();

    let _ = info_span!("start").entered();

    info!("wasm initialized successfully");
}

fn init_tracing() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .without_time()
        .pretty()
        .with_ansi(false)
        .with_writer(tracing_subscriber_wasm::MakeConsoleWriter::default());

    let filter_layer =
        tracing_subscriber::filter::Targets::new().with_default(tracing::Level::INFO);

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(filter_layer)
        .init();
}
