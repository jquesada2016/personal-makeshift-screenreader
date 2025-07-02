type shortcut = {
  meta: bool,
  ctrl: bool,
  alt: bool,
  shift: bool,
  key: string,
}

type shortcuts = {showSettings: shortcut}

type t = {
  lineThickness: float,
  pointerGap: float,
  shortcuts: shortcuts,
}

module Store = {
  module SettingsStore = Tauri.StoreBuilder({
    let path = "settings.json"
  })

  module SettingsKeyConfig = {
    type t = t
    let name = "settings"
  }

  module SettingsKey = SettingsStore.Key(SettingsKeyConfig)

  let getStore = SettingsStore.get
  let get = SettingsKey.get
  let set = SettingsKey.set
  let reset = SettingsStore.reset
}
