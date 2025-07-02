module Core = {
  @module("@tauri-apps/api/core")
  external invoke: (string, ~args: 'a=?) => promise<'b> = "invoke"
}

module Window = {
  type t

  type physicalPosition = {x: int, y: int}
  type physicalSize = {width: int, height: int}

  @module("@tauri-apps/api/window")
  external getCurrentWindow: unit => t = "getCurrentWindow"

  @send external innerPosition: t => promise<physicalPosition> = "innerPosition"
  @send external innerSize: t => promise<physicalSize> = "innerSize"
  @send external outerSize: t => promise<physicalSize> = "outerSize"
  @send external scaleFactor: t => promise<float> = "scaleFactor"
}

module Dialog = {
  module Kind = {
    type t = | @as("warn") Warn | @as("error") Error
  }

  type dialogOptions = {kind?: Kind.t}

  @scope(("__TAURI__", "dialog"))
  external ask: (string, ~opts: dialogOptions=?) => promise<bool> = "ask"

  @scope(("__TAURI__", "dialog"))
  external message: (string, ~opts: dialogOptions=?) => promise<unit> = "message"
}

module Store = {
  type t

  @scope(("__TAURI__", "store", "Store"))
  external getStore: string => promise<t> = "get"

  @send
  external get: (t, string) => promise<'a> = "get"

  @send
  external set: (t, string, 'a) => promise<unit> = "set"

  @send
  external reset: t => promise<unit> = "reset"

  @send
  external onKeyChange: (t, string, 'a => unit) => promise<unit => unit> = "onKeyChange"
}

module StoreBuilder = (
  Config: {
    let path: string
  },
) => {
  let get = () => Store.getStore(Config.path)

  let reset = Store.reset

  module Key = (
    Config: {
      type t
      let name: string
    },
  ) => {
    let get = (t): promise<Config.t> => t->Store.get(Config.name)
    let set = (t, value: Config.t) => t->Store.set(Config.name, value)
    let onChange = (t, f: Config.t => _) => t->Store.onKeyChange(Config.name, f)
  }
}
