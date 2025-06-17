module Element = {
  type t
}

module Document = {
  @scope("document") external getElementById: string => null<Element.t> = "getElementById"
}
