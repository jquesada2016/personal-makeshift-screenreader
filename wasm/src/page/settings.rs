use crate::settings::{self, Settings, subscribe_to_settings_changes};
use leptos::{either::Either, prelude::*};
use wasm_bindgen::prelude::*;

#[instrument]
#[wasm_bindgen]
pub fn start_settings() {
    info!("starting settings page");

    mount_to_body(Settings);
}

#[component]
fn Settings() -> impl IntoView {
    let (settings, set_settings) = signal(None);

    subscribe_to_settings_changes(set_settings);

    let handle_change_line_thickness = move |e: web_sys::Event| {
        let settings = settings.get_untracked().unwrap();

        let line_thickness = e
            .current_target()
            .unwrap()
            .unchecked_into::<web_sys::HtmlInputElement>()
            .value()
            .parse::<f32>()
            .unwrap_or(1.5);

        update_settings(Settings {
            line_thickness,
            ..settings
        });
    };

    let handle_change_pointer_gap = move |e: web_sys::Event| {
        let settings = settings.get_untracked().unwrap();

        let pointer_gap = e
            .current_target()
            .unwrap()
            .unchecked_into::<web_sys::HtmlInputElement>()
            .value()
            .parse::<f32>()
            .unwrap_or(8.0);

        update_settings(Settings {
            pointer_gap,
            ..settings
        });
    };

    move || {
        if let Some(settings) = settings.get() {
            let view = view! {
              <main class="p-8">
                <h1 class="text-2xl font-bold mb-8">Settings</h1>

                <fieldset class="fieldset">
                  <label class="input input-primary w-full">
                    <span class="label-text">Line Thickness</span>
                    <input
                      type="number"
                      min="0"
                      step="0.1"
                      value=settings.line_thickness
                      on:input=handle_change_line_thickness

                    />
                  </label>
                  <label class="input input-primary w-full">
                    <span class="label-text">Pointer Gap</span>
                    <input
                      type="number"
                      min="0"
                      step="0.1"
                      value=settings.pointer_gap
                      on:input=handle_change_pointer_gap
                    />
                  </label>
                  <button
                    class="btn btn-warning"
                    on:click=|_| update_settings(Default::default())
                  >Reset</button>
                </fieldset>
              </main>
            };

            Either::Left(view)
        } else {
            Either::Right(())
        }
    }
}

fn update_settings(settings: Settings) {
    use crate::tauri::store::Store;

    let settings = serde_wasm_bindgen::to_value(&settings).unwrap();

    leptos::task::spawn_local(async move {
        Store::get(settings::STORAGE_PATH)
            .await
            .unwrap()
            .set_key(settings::STORAGE_KEY, settings)
            .await
            .unwrap();
    });
}
