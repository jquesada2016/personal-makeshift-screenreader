module Settings = {
  @react.component
  let make = () => {
    <main className="p-8">
      <h1 className="text-2xl font-bold"> {"Settings"->React.string} </h1>
    </main>
  }
}

let main = () =>
  switch ReactDOM.querySelector("#app") {
  | Some(el) => el
  | None => panic("#app not found")
  }
  ->ReactDOM.Client.createRoot
  ->ReactDOM.Client.Root.render(<Settings />)
