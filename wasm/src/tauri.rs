use wasm_bindgen::prelude::*;

pub mod store {
    use super::*;

    #[wasm_bindgen]
    extern "C" {
        pub type Store;

        #[wasm_bindgen(js_namespace = ["__TAURI__", "store"], static_method_of = Store, catch )]
        pub async fn get(path: &str) -> Result<Store, JsValue>;

        #[wasm_bindgen(method, js_name = get)]
        pub async fn get_key(this: &Store, key: &str) -> JsValue;

        #[wasm_bindgen(method, js_name = set, catch)]
        pub async fn set_key(this: &Store, key: &str, value: JsValue) -> Result<(), JsValue>;

        #[wasm_bindgen(method, js_name = onKeyChange, catch)]
        pub async fn on_key_change(
            this: &Store,
            key: &str,
            f: &Closure<dyn FnMut(JsValue)>,
        ) -> Result<(), JsValue>;
    }
}

pub mod window {
    use super::*;

    #[wasm_bindgen]
    extern "C" {
        pub type PhysicalPosition;

        #[wasm_bindgen(method, getter)]
        pub fn x(this: &PhysicalPosition) -> f32;

        #[wasm_bindgen(method, getter)]
        pub fn y(this: &PhysicalPosition) -> f32;

        #[wasm_bindgen(js_namespace = ["__TAURI__", "window"], js_name = cursorPosition)]
        pub async fn get_cursor_position() -> PhysicalPosition;

        pub type WebViewWindow;

        #[wasm_bindgen(js_namespace = ["__TAURI__", "window"], js_name = getCurrentWindow)]
        pub fn get_current_window() -> WebViewWindow;

        #[wasm_bindgen(method, js_name = scaleFactor)]
        async fn wb_scale_factor(this: &WebViewWindow) -> JsValue;

        #[wasm_bindgen(method, js_name = innerPosition)]
        pub async fn inner_position(this: &WebViewWindow) -> PhysicalPosition;
    }

    impl WebViewWindow {
        pub async fn scale_factor(&self) -> f32 {
            self.wb_scale_factor().await.as_f64().unwrap() as f32
        }
    }
}
