import gleam/dynamic/decode
import gleam/result
import plinth/javascript/storage

const preferences_storage_key = "prefs"

pub type Preferences {
  Preferences(line_thickness: Float, cursor_gap: Float)
}

fn preferences_decoder() -> decode.Decoder(Preferences) {
  use line_thickness <- decode.field("line_thickness", decode.float)
  use cursor_gap <- decode.field("cursor_gap", decode.float)
  decode.success(Preferences(line_thickness:, cursor_gap:))
}

const default = Preferences(line_thickness: 1.0, cursor_gap: 8.0)

fn to_string(prefs: Preferences) -> String {
  todo
}

pub fn get() -> Preferences {
  let assert Ok(storage) = storage.local()

  let prefs =
    storage.get_item(storage, preferences_storage_key)
    |> result.lazy_unwrap(fn() { todo })
    |> result.lazy_unwrap(fn() {
      save(default)
      to_string(default)
    })

  todo
}

pub fn save(preferences: Preferences) {
  todo
}
