use wasm_bindgen::prelude::Closure;

use crate::tauri;

pub const STORAGE_PATH: &str = "settings.json";
pub const STORAGE_KEY: &str = "settings";

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub line_thickness: f32,
    pub pointer_gap: f32,
    pub shortcuts: Shortcuts,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            line_thickness: 1.5,
            pointer_gap: 8.0,
            shortcuts: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Shortcuts {
    pub show_settings: String,
    pub show_crosshair: String,
}

impl Default for Shortcuts {
    fn default() -> Self {
        Self {
            show_settings: "Cmd + Ctrl + KeyC".into(),
            show_crosshair: "Cmd + Alt + KeyC".into(),
        }
    }
}

pub(crate) fn subscribe_to_settings_changes<S>(set_settings: S)
where
    S: leptos::reactive::traits::Set<Value = Option<Settings>> + 'static,
{
    use tauri::store::Store;

    leptos::task::spawn_local(async move {
        let settings_store = Store::get("settings.json").await.unwrap();

        let settings = settings_store.get_key("settings").await;
        let settings = serde_wasm_bindgen::from_value(settings).unwrap();

        set_settings.set(Some(settings));

        let f = Closure::new(move |settings| {
            let settings = serde_wasm_bindgen::from_value(settings).unwrap();

            set_settings.set(Some(settings));
        });

        settings_store.on_key_change("settings", &f).await.unwrap();

        f.forget();
    });
}
