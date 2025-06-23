let getEl = id =>
  switch Browser.Document.getElementById(id) {
  | Null.Null => panic(`element #${id} not found`)
  | Null.Value(el) => el
  }

let drawLeft = (~left, ~mousePos: Command.position, ~lineThickness, ~gap) => {
  let xMousePos = mousePos.x->Int.toString
  let yMousePos = mousePos.y->Int.toString
  let lineThicknessOffset = (lineThickness *. 2.0)->Float.toString
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
  ~scaleFactor,
) => {
  let yMousePos = mousePos.y->Int.toString
  let width = (((windowSize.width :> float) /. scaleFactor)->Float.toInt - mousePos.x)->Int.toString
  let lineThicknessOffset = (lineThickness *. 2.0)->Float.toString
  let lineThickness = lineThickness->Float.toString
  let gap = gap->Float.toString

  right->Browser.Element.setStyle("top", `calc(${yMousePos}px - ${lineThicknessOffset}rem)`)
  right->Browser.Element.setStyle("height", `${lineThickness}rem`)
  right->Browser.Element.setStyle("width", `calc(${width}px - ${gap}rem)`)
  right->Browser.Element.setStyle("right", "0")
}

let drawTop = (~top, ~mousePos: Command.position, ~lineThickness, ~gap) => {
  let xMousePos = mousePos.x->Int.toString
  let yMousePos = mousePos.y->Int.toString
  let lineThicknessOffset = (lineThickness /. 2.0)->Float.toString
  let lineThickness = lineThickness->Float.toString
  let gap = gap->Float.toString

  top->Browser.Element.setStyle("left", `calc(${xMousePos}px - ${lineThicknessOffset}rem)`)
  top->Browser.Element.setStyle("width", `${lineThickness}rem`)
  top->Browser.Element.setStyle("height", `calc(${yMousePos}px - ${gap}rem)`)
}

let drawBottom = (
  ~bottom,
  ~mousePos: Command.position,
  ~windowSize: Tauri.Window.physicalSize,
  ~lineThickness,
  ~gap,
  ~scaleFactor,
) => {
  let xMousePos = mousePos.x->Int.toString
  let height =
    (((windowSize.height :> float) /. scaleFactor)->Float.toInt - mousePos.y)->Int.toString
  let lineThicknessOffset = (lineThickness /. 2.0)->Float.toString
  let lineThickness = lineThickness->Float.toString
  let gap = gap->Float.toString

  bottom->Browser.Element.setStyle("left", `calc(${xMousePos}px - ${lineThicknessOffset}rem)`)
  bottom->Browser.Element.setStyle("width", `${lineThickness}rem`)
  bottom->Browser.Element.setStyle("height", `calc(${height}px - ${gap}rem)`)
  bottom->Browser.Element.setStyle("bottom", "0")
}

let main = async () => {
  let left = getEl("left")
  let right = getEl("right")
  let top = getEl("top")
  let bottom = getEl("bottom")

  let window = Tauri.Window.getCurrentWindow()

  while true {
    await Async.requestAnimationFrame()
    let scaleFactor = await window->Tauri.Window.scaleFactor
    let windowSize = await window->Tauri.Window.outerSize
    let mousePos = await Command.getMousePosition()
    let lineThickness = 1.5
    let gap = 8.0

    // let mousePos: Command.position = {
    //   x: ((mousePos.x :> float) *. scaleFactor)->Float.toInt,
    //   y: ((mousePos.y :> float) *. scaleFactor)->Float.toInt,
    // }

    drawLeft(~left, ~mousePos, ~gap, ~lineThickness)
    drawRight(~right, ~mousePos, ~windowSize, ~lineThickness, ~gap, ~scaleFactor)
    drawTop(~top, ~mousePos, ~gap, ~lineThickness)
    drawBottom(~bottom, ~mousePos, ~windowSize, ~lineThickness, ~gap, ~scaleFactor)
  }
}

let _ = main()
