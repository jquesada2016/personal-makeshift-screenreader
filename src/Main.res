let getEl = id =>
  switch Browser.Document.getElementById(id) {
  | Null.Null => panic(`element #${id} not found`)
  | Null.Value(el) => el
  }

let left = getEl("left")
let right = getEl("right")
let top = getEl("top")
let bottom = getEl("bottom")

let drawLeft = (~left, ~mousePos: Command.position, ~lineThickness, ~gap) => {
  let xMousePos = mousePos.x->Int.toString
  let yMousePos = mousePos.y->Int.toString
  let lineThicknessOffset = (lineThickness /. 2.0)->Float.toString
  let lineThickness = lineThickness->Float.toString
  let gap = gap->Float.toString

  left->Browser.Element.setStyle("top", `calc(${yMousePos}px - ${lineThicknessOffset}rem)`)
  left->Browser.Element.setStyle("height", `${lineThickness}rem`)
  left->Browser.Element.setStyle("width", `calc(${xMousePos}px - ${gap}rem)`)
}

let drawRight = (
  ~right,
  ~mousePos: Command.position,
  ~windowSize: Tauri.Window.physicalSize,
  ~lineThickness,
  ~gap,
) => {
  let xMousePos = mousePos.x->Int.toString
  let yMousePos = mousePos.y->Int.toString
  let width = (windowSize.width - mousePos.x)->Int.toString
  let lineThicknessOffset = (lineThickness /. 2.0)->Float.toString
  let lineThickness = lineThickness->Float.toString
  let gap = gap->Float.toString

  right->Browser.Element.setStyle("top", `calc(${yMousePos}px - ${lineThicknessOffset}rem)`)
  right->Browser.Element.setStyle("height", `${lineThickness}rem`)
  right->Browser.Element.setStyle("width", `calc(${width}px - ${gap}rem)`)
  right->Browser.Element.setStyle("right", "0")
}

let main = async () => {
  let window = Tauri.Window.getCurrentWindow()

  let size = await window->Tauri.Window.innerSize
  let pos = await window->Tauri.Window.innerPosition
  let mousePos = await Command.getMousePosition()

  while true {
    await Async.requestAnimationFrame()
    let windowSize = await window->Tauri.Window.innerSize
    let mousePos = await Command.getMousePosition()
    let lineThickness = 1.5
    let gap = 8.0

    drawLeft(~left, ~mousePos, ~gap, ~lineThickness)
    drawRight(~right, ~mousePos, ~windowSize, ~lineThickness, ~gap)
  }
}

let _ = main()
