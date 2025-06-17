let getEl = id =>
  switch Browser.Document.getElementById(id) {
  | Null.Null => panic(`element #${id} not found`)
  | Null.Value(el) => el
  }

let left = getEl("left")
let right = getEl("right")
let top = getEl("top")
let bottom = getEl("bottom")
