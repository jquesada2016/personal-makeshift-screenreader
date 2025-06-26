import gleam/float
import gleam/result
import lustre
import lustre/attribute as attr
import lustre/effect
import lustre/element.{type Element}
import lustre/element/html
import lustre/event
import preferences/preferences.{type Preferences}

pub fn main() -> Nil {
  let app = lustre.application(init, update, view)

  let prefs = preferences.get()

  let assert Ok(_) = lustre.start(app, "#app", prefs)

  Nil
}

type Model {
  Model(line_thickness: Float, cursor_gap: Float)
}

type Message {
  UserChangedLineThickness(Float)
  UserChangedCursorGap(Float)
}

fn init(prefs: Preferences) -> #(Model, effect.Effect(Message)) {
  let preferences.Preferences(line_thickness:, cursor_gap:) = prefs

  #(Model(line_thickness:, cursor_gap:), effect.none())
}

fn update(model: Model, msg: Message) -> #(Model, effect.Effect(Message)) {
  #(model, effect.none())
}

fn view(model: Model) -> Element(Message) {
  html.main([attr.class("p-8")], [
    html.h1([attr.class("text-2xl font-bold mb-8")], [
      element.text("Preferences"),
    ]),
    html.fieldset([attr.class("fieldset")], [
      number_input(label: "Line Thickness", value: 0.0, on_change: fn(_) {
        todo
      }),
      number_input(label: "Cursor Gap", value: 0.0, on_change: fn(_) { todo }),
    ]),
  ])
}

fn number_input(
  label label: String,
  value value: Float,
  on_change on_change: fn(Float) -> msg,
) -> Element(msg) {
  let on_change = fn(num) {
    float.parse(num)
    |> result.unwrap(0.0)
    |> on_change
  }

  html.label([attr.class("input input-primary")], [
    html.span([attr.class("label")], [
      element.text(label),
      html.input([
        attr.type_("number"),
        attr.min("0"),
        attr.step("0.1"),
        attr.value(float.to_string(value)),
        event.on_change(on_change),
      ]),
    ]),
  ])
}
