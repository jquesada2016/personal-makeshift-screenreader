use crate::{color_filtering::ColorFilteringState, error::StringError};
use error_stack::{Result, ResultExt};
use tauri::State;
use uiautomation::UIAutomation;

type StringErrorResult<T> = std::result::Result<T, StringError>;

declare_error!(GetCursorPositionError, "failed to get cursor position");

pub fn get_cursor_pos(
) -> Result<uiautomation::types::Point, GetCursorPositionError> {
  use windows::Win32::{
    Foundation::POINT, UI::WindowsAndMessaging::GetCursorPos,
  };

  let mut point = POINT::default();

  unsafe { GetCursorPos(&mut point) }.change_context(GetCursorPositionError)?;

  Ok(point.into())
}

#[tauri::command]
pub fn get_cursor_position() -> StringErrorResult<(i32, i32)> {
  let point = get_cursor_pos()?;

  Ok((point.get_x(), point.get_y()))
}

declare_error!(IsColorFilterOnError, "failed to get color filter state");

#[tauri::command]
pub fn is_color_filter_on(
  color_filtering: State<ColorFilteringState>,
) -> StringErrorResult<bool> {
  color_filtering
    .is_active()
    .change_context(IsColorFilterOnError)
    .map_err(Into::into)
}

declare_error!(GetTextAtCursorError, "failed to get text at cursor");

#[tauri::command]
pub async fn get_text_at_cursor() -> StringErrorResult<String> {
  tokio::task::block_in_place(|| {
    std::thread::spawn(|| {
      let automation =
        UIAutomation::new().change_context(GetTextAtCursorError)?;

      let point = get_cursor_pos().change_context(GetTextAtCursorError)?;

      let element = automation
        .element_from_point(point)
        .change_context(GetTextAtCursorError)
        .attach_printable("getting element at point")?;

      Ok(
        element
          .get_name()
          .change_context(GetTextAtCursorError)
          .attach_printable("getting element name")?,
      )
    })
    .join()
    .expect("thread not to panic")
  })
}
