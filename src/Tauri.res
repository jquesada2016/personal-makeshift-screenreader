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
}
