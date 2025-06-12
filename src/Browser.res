module Element = {
  type t
}

module Document = {
  @scope("document") external getElementById: string => nullable<Element.t> = "getElementById"
}
