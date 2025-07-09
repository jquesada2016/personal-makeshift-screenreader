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
    pub increase_line_thickness: Option<String>,
    pub decrease_line_thickness: Option<String>,
    pub increase_pointer_gap: Option<String>,
    pub decrease_pointer_gap: Option<String>,
}

impl Default for Shortcuts {
    fn default() -> Self {
        Self {
            show_settings: "Cmd + Ctrl + KeyC".into(),
            show_crosshair: "Cmd + Alt + KeyC".into(),
            increase_line_thickness: None,
            decrease_line_thickness: None,
            increase_pointer_gap: None,
            decrease_pointer_gap: None,
        }
    }
}
