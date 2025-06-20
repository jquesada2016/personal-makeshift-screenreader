module Element = {
  type t

  type css

  @get external style: t => css = "style"

  @set_index external setStyle: (css, string, string) => unit = ""

  let setStyle = (el, name, val) => el->style->setStyle(name, val)
}

module Document = {
  @scope("document") external getElementById: string => null<Element.t> = "getElementById"
}
