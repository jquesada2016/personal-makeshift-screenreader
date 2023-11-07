use leptos::*;
use leptos_tea::Cmd;
use tauri_api_wasm::{
  clipboard::read_text,
  global_shortcut::{register, unregister_all, GlobalShortcutHandler},
  tauri::InvokeCommand,
};
use wasm_bindgen::JsValue;

const DEFAULT_RATE: f32 = 2.0;
const RATE_STEP: f32 = 0.2;

const GET_TEXT_AT_CURSOR_COMMAND: InvokeCommand<(), String, JsValue> =
  InvokeCommand::new("get_text_at_cursor");

#[derive(leptos_tea::Model, TypedBuilder)]
#[builder(field_defaults(default))]
struct Model {
  _english_shortcut: Option<GlobalShortcutHandler>,
  _spanish_shortcut: Option<GlobalShortcutHandler>,
  _increase_speed_shortcut: Option<GlobalShortcutHandler>,
  _decrease_speed_shortcut: Option<GlobalShortcutHandler>,
  _toggle_paush_shortcut: Option<GlobalShortcutHandler>,
  _english_at_cursor_shortchut: Option<GlobalShortcutHandler>,
  _spanish_at_cursor_shortchut: Option<GlobalShortcutHandler>,
  #[builder(!default)]
  speech_synthesis: web_sys::SpeechSynthesis,
  #[builder(!default)]
  utterance: web_sys::SpeechSynthesisUtterance,
  paused: bool,
}

#[derive(Debug, Default)]
enum Msg {
  #[default]
  Init,
  SpeakEnglish,
  SpeakEnglishText(String),
  SpeakSpanishText(String),
  SpeakSpanish,
  IncreaseRate,
  DecreaseRate,
  TogglePause,
  SpeakEnglishAtCursor,
  SpeakSpanishAtCursor,
}

fn update(model: UpdateModel, msg: Msg, mut cmd: Cmd<Msg>) {
  debug!("msg: {msg:#?}");

  match msg {
    Msg::Init => {
      model.utterance.get().set_rate(DEFAULT_RATE);

      // Clear shortcuts
      cmd.cmd(async {
        unregister_all().await;

        None
      });

      // Register shortcuts
      cmd.cmd({
        let cmd = cmd.clone();

        async move {
          let english_shortcut = register("ctrl+super+alt+l", {
            {
              let mut cmd = cmd.clone();

              move |_| {
                cmd.msg(Msg::SpeakEnglish);
                cmd.perform();
              }
            }
          })
          .await;

          let spanish_shortcut = register("ctrl+super+alt+n", {
            {
              let mut cmd = cmd.clone();

              move |_| {
                cmd.msg(Msg::SpeakSpanish);
                cmd.perform();
              }
            }
          })
          .await;

          let increase_rate_shortcut = register("ctrl+super+i", {
            {
              let mut cmd = cmd.clone();

              move |_| {
                cmd.msg(Msg::IncreaseRate);
                cmd.perform();
              }
            }
          })
          .await;

          let decrease_rate_shortcut = register("ctrl+super+alt+i", {
            {
              let mut cmd = cmd.clone();

              move |_| {
                cmd.msg(Msg::DecreaseRate);
                cmd.perform();
              }
            }
          })
          .await;

          let toggle_pause_shortcut = register("ctrl+super+alt+p", {
            {
              let mut cmd = cmd.clone();

              move |_| {
                cmd.msg(Msg::TogglePause);
                cmd.perform();
              }
            }
          })
          .await;

          let english_at_cursor_shortcut = register("ctrl+super+alt+k", {
            {
              let mut cmd = cmd.clone();

              move |_| {
                cmd.msg(Msg::SpeakEnglishAtCursor);
                cmd.perform();
              }
            }
          })
          .await;

          let spanish_at_cursor_shortcut = register("ctrl+super+alt+b", {
            {
              let mut cmd = cmd.clone();

              move |_| {
                cmd.msg(Msg::SpeakSpanishAtCursor);
                cmd.perform();
              }
            }
          })
          .await;

          model._english_shortcut.set(Some(english_shortcut));
          model._spanish_shortcut.set(Some(spanish_shortcut));
          model
            ._increase_speed_shortcut
            .set(Some(increase_rate_shortcut));
          model
            ._decrease_speed_shortcut
            .set(Some(decrease_rate_shortcut));
          model
            ._toggle_paush_shortcut
            .set(Some(toggle_pause_shortcut));
          model
            ._english_at_cursor_shortchut
            .set(Some(english_at_cursor_shortcut));
          model
            ._spanish_at_cursor_shortchut
            .set(Some(spanish_at_cursor_shortcut));

          None
        }
      });
    }
    Msg::SpeakEnglish => cmd.cmd(async move {
      read_text()
        .await
        .as_deref()
        .map(sanitize)
        .map(Msg::SpeakEnglishText)
    }),
    Msg::SpeakEnglishText(text) => {
      let synthesis = model.speech_synthesis.get();
      let utterance = model.utterance.get();

      if synthesis.speaking() {
        synthesis.cancel();
      }

      utterance.set_lang("en-us");

      utterance.set_text(&text);
      synthesis.speak(&utterance);
    }
    Msg::SpeakSpanish => cmd.cmd(async move {
      read_text()
        .await
        .as_deref()
        .map(sanitize)
        .map(Msg::SpeakSpanishText)
    }),
    Msg::SpeakSpanishText(text) => {
      let synthesis = model.speech_synthesis.get();
      let utterance = model.utterance.get();

      if synthesis.speaking() {
        synthesis.cancel();
      }

      utterance.set_lang("es-cr");

      utterance.set_text(&text);

      synthesis.speak(&utterance);
    }
    Msg::IncreaseRate => {
      let utterance = model.utterance.get();

      let mut rate = utterance.rate() + RATE_STEP;

      rate = rate.clamp(0.0, 10.0);

      utterance.set_rate(rate);
    }
    Msg::DecreaseRate => {
      let utterance = model.utterance.get();

      let mut rate = utterance.rate() - RATE_STEP;

      rate = rate.clamp(0.0, 10.0);

      utterance.set_rate(rate);
    }
    Msg::TogglePause => {
      let synthesis = model.speech_synthesis.get();

      if synthesis.paused() {
        synthesis.resume();
      } else {
        synthesis.pause();
      }
    }
    Msg::SpeakEnglishAtCursor => {
      cmd.cmd(async {
        let text = match GET_TEXT_AT_CURSOR_COMMAND
          .invoke_with_serde_js_err(&())
          .await
        {
          Ok(res) => res,
          Err(err) => {
            tracing::error!("failed to get text at cursor:\n{err:#?}");

            return None;
          }
        };

        let text = if text.is_empty() {
          "There's nothing to read.".into()
        } else {
          text
        };

        Some(Msg::SpeakEnglishText(text))
      });
    }
    Msg::SpeakSpanishAtCursor => {
      cmd.cmd(async {
        let text = match GET_TEXT_AT_CURSOR_COMMAND
          .invoke_with_serde_js_err(&())
          .await
        {
          Ok(res) => res,
          Err(err) => {
            tracing::error!("failed to get text at cursor:\n{err:#?}");

            return None;
          }
        };

        let text = if text.is_empty() {
          "No hay nada que leer.".into()
        } else {
          text
        };

        Some(Msg::SpeakSpanishText(text))
      });
    }
  }
}

#[component]
pub fn ReadClipboardAloudProvider(
  #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
  let _ = Model::builder()
    .speech_synthesis(window().speech_synthesis().unwrap())
    .utterance(web_sys::SpeechSynthesisUtterance::new().unwrap())
    .build()
    .init(update);

  children.map(|children| children())
}

fn sanitize(text: &str) -> String {
  text.replace(['<'], " ")
}
