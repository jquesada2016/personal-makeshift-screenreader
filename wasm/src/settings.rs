use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[instrument]
#[wasm_bindgen]
pub fn start_settings() {
    mount_to_body(Settings);
}

#[component]
fn Settings() -> impl IntoView {
    view! {
      <main class="p-8">
        <h1 class="text-2xl font-bold mb-8">Settings</h1>
      </main>
    }
}
