type position = {x: int, y: int}

let getMousePosition: unit => promise<position> = () => Tauri.Core.invoke("get_mouse_position")
