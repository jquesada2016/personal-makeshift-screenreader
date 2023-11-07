use crate::futs::request_animation_frame;
use leptos::*;
use leptos_tea::Cmd;
use tauri_api_wasm::{
  event::{listen_serde, Event, ListenUnsubscriber},
  global_shortcut::{register, GlobalShortcutHandler},
  tauri::InvokeCommand,
  window::APP_WINDOW,
};
use wasm_bindgen::JsValue;

const COLOR_FILTERING_CHANGED_EVENT: &str = "COLOR_FILTERING_CHANGED_EVENT";
const CURSOR_MARGIN: i32 = 48;
const GET_CURSOR_POSITION_COMMAND: InvokeCommand<(), (i32, i32), JsValue> =
  InvokeCommand::new("get_cursor_position");
const IS_INVERTED_COMMAND: InvokeCommand<(), bool, JsValue> =
  InvokeCommand::new("is_color_filter_on");

#[derive(Default, leptos_tea::Model)]
struct Model {
  show: bool,
  cursor_pos: (i32, i32),
  window_size: (i32, i32),
  is_inverted: bool,
  _show_shortcut: Option<GlobalShortcutHandler>,
  _is_inverted: Option<ListenUnsubscriber<dyn FnMut(Event)>>,
}

#[derive(Default, Debug)]
enum Msg {
  #[default]
  Init,
  UpdateCursorPosition((i32, i32)),
  UpdateWindowSize(i32, i32),
  ToggleShow,
}

fn update(model: UpdateModel, msg: Msg, mut cmd: Cmd<Msg>) {
  // debug!("msg: {msg:#?}");

  match msg {
    Msg::Init => {
      cmd.cmd(async {
        APP_WINDOW.set_ignore_cursor_events(true).await;

        None
      });

      cmd.cmd(async move {
        match IS_INVERTED_COMMAND.invoke_with_serde_js_err(&()).await {
          Ok(is_inverted) => model.is_inverted.set(is_inverted),
          Err(err) => {
            tracing::error!("failed to get initial inversion state:\n{err:#?}")
          }
        }

        None
      });

      cmd.cmd(async move {
        let listener = listen_serde::<bool>(
          COLOR_FILTERING_CHANGED_EVENT,
          move |payload, _| {
            let payload = payload.unwrap();

            model.is_inverted.set(payload);
          },
        )
        .await;

        model._is_inverted.set(Some(listener));

        None
      });

      cmd.cmd({
        let mut cmd = cmd.clone();

        async move {
          let shortcut = register("ctrl+super+alt+f", move |_| {
            cmd.msg(Msg::ToggleShow);
            cmd.perform();
          })
          .await;

          model._show_shortcut.set(Some(shortcut));

          None
        }
      });

      cmd.cmd::<_, Option<Msg>>({
        let mut cmd = cmd.clone();

        async move {
          loop {
            request_animation_frame().await;

            let width =
              window().inner_width().unwrap().as_f64().unwrap() as i32;
            let height =
              window().inner_height().unwrap().as_f64().unwrap() as i32;

            cmd.msg(Msg::UpdateWindowSize(width, height));

            let scale_factor =
              APP_WINDOW.scale_factor().await.as_f64().unwrap();

            match GET_CURSOR_POSITION_COMMAND
              .invoke_with_serde_js_err(&())
              .await
            {
              Ok((x, y)) => {
                cmd.msg(Msg::UpdateCursorPosition((
                  (x as f64 / scale_factor) as i32,
                  (y as f64 / scale_factor) as i32,
                )));
                cmd.perform();
              }
              Err(err) => {
                tracing::error!("error getting cursor position:\n{err:#?}")
              }
            }
          }
        }
      });
    }
    Msg::UpdateCursorPosition(coords) => model.cursor_pos.set(coords),
    Msg::UpdateWindowSize(width, height) => {
      model.window_size.set((width, height))
    }
    Msg::ToggleShow => model.show.update(|show| *show = !*show),
  }
}

#[component]
pub fn CursorLocator() -> impl IntoView {
  let (model, _msg) = Model::default().init(update);

  let top_line_style = move || {
    format!(
      "left: {}px;height: {}px",
      model.cursor_pos.get().0 - 8,
      model.cursor_pos.get().1 - CURSOR_MARGIN
    )
  };

  let bottom_line_style = move || {
    format!(
      "left: {}px;height: {}px",
      model.cursor_pos.get().0 - 8,
      model.window_size.get().1 - model.cursor_pos.get().1 - CURSOR_MARGIN
    )
  };

  let left_line_style = move || {
    format!(
      "top: {}px;width: {}px",
      model.cursor_pos.get().1 - 8,
      model.cursor_pos.get().0 - CURSOR_MARGIN
    )
  };

  let right_line_style = move || {
    format!(
      "top: {}px;width: {}px",
      model.cursor_pos.get().1 - 8,
      model.window_size.get().0 - model.cursor_pos.get().0 - CURSOR_MARGIN,
    )
  };

  // lazy white-listing
  let _ = ["hidden", "from-white", "from-black", "to-white", "to-black"];

  view! {
    <div
      class="w-full h-full text-5xl text-white flex flex-col justify-center items-center"
      class:hidden=move || !model.show.get()
    >
      <div
        class="bg-white w-4 absolute top-0 bg-gradient-to-b"
        class:from-black=move || !model.is_inverted.get()
        class:to-white=move || !model.is_inverted.get()
        class:from-white=move || model.is_inverted.get()
        class:to-black=move || model.is_inverted.get()
        style=top_line_style
      />
      <div
        class="bg-white w-4 absolute bottom-0 bg-gradient-to-t from-black to-white"
        class:from-black=move || !model.is_inverted.get()
        class:to-white=move || !model.is_inverted.get()
        class:from-white=move || model.is_inverted.get()
        class:to-black=move || model.is_inverted.get()
        style=bottom_line_style
      />
      <div
        class="bg-white h-4 absolute left-0 bg-gradient-to-r from-black to-white"
        class:from-black=move || !model.is_inverted.get()
        class:to-white=move || !model.is_inverted.get()
        class:from-white=move || model.is_inverted.get()
        class:to-black=move || model.is_inverted.get()
        style=left_line_style
      />
      <div
        class="bg-white h-4 absolute right-0 bg-gradient-to-l from-black to-white"
        class:from-black=move || !model.is_inverted.get()
        class:to-white=move || !model.is_inverted.get()
        class:from-white=move || model.is_inverted.get()
        class:to-black=move || model.is_inverted.get()
        style=right_line_style
      />
    </div>
  }
}
