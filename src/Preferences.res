module Preferences = {
  let usePreference = pref => {
    module Storage = Dom.Storage2

    let storage = Storage.localStorage

    let val = switch storage->Storage.getItem(pref) {
    | Some(val) => val
    | None => panic(`missing preference \`${pref}\``)
    }

    let (val, setVal) = React.useState(() => val)

    let setVal = val => {
      storage->Storage.setItem(pref, val)
      setVal(_ => val)
    }

    (val, setVal)
  }

  let useLineThicknessPreference = () => usePreference(Const.Preferences.lineThickness)
  let useGapPreference = () => usePreference(Const.Preferences.gap)

  @react.component
  let make = () => {
    let (lineThickness, setLineThickness) = useLineThicknessPreference()
    let (gap, setGap) = useGapPreference()

    <main className="p-8">
      <h1 className="text-2xl"> {"Preferences"->React.string} </h1>
      <label className="label-text">
        <span className="label-text"> {"Line Thickness"->React.string} </span>
        <input
          className="input input-primary" type_="number" min="0" step=0.1 value={lineThickness}
        />
      </label>
    </main>
  }
}

switch ReactDOM.querySelector("#app") {
| Some(el) => el
| None => panic("#app not found")
}
->ReactDOM.Client.createRoot
->ReactDOM.Client.Root.render(<Preferences />)
