module Core = {
  
  external invoke: (string, 'a) : promise<'b> = "invoke" 
}
