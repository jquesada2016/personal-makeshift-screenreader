use error_stack::{Result, ResultExt};
use tauri::{AppHandle, Manager};
use windows::core::w;
use windows::{core::PCWSTR, Win32::System::Registry as reg};

pub const COLOR_FILTERING_CHANGED_EVENT: &str = "COLOR_FILTERING_CHANGED_EVENT";

declare_error!(
  NewColorFilteringStateError,
  "failed to get HKEY for color filtering",
);
declare_error!(IsActiveError, "failed to get color filtering active state");
declare_error!(
  BlockUntilChangeError,
  "error while waiting for color filtering state to change",
);

#[derive(Clone, Debug)]
pub struct ColorFilteringState(reg::HKEY);

impl Drop for ColorFilteringState {
  fn drop(&mut self) {
    let _ = unsafe { reg::RegCloseKey(self.0) };
  }
}

impl ColorFilteringState {
  const HKEY: PCWSTR = w!("Software\\Microsoft\\ColorFiltering");
  const ACTIVE_VALUE_KEY: PCWSTR = w!("Active");

  pub fn new() -> Result<Self, NewColorFilteringStateError> {
    let mut hkey = reg::HKEY::default();

    unsafe {
      reg::RegOpenKeyExW(
        reg::HKEY_CURRENT_USER,
        Self::HKEY,
        0,
        reg::KEY_READ,
        &mut hkey,
      )
    }
    .change_context(NewColorFilteringStateError)?;

    Ok(Self(hkey))
  }

  pub fn is_active(&self) -> Result<bool, IsActiveError> {
    let mut data = 0u32;

    unsafe {
      reg::RegGetValueW(
        self.0,
        None,
        Self::ACTIVE_VALUE_KEY,
        reg::RRF_RT_DWORD,
        None,
        Some(&mut data as *mut u32 as *mut std::ffi::c_void),
        Some(&mut 4),
      )
    }
    .change_context(IsActiveError)?;

    Ok(data == 1)
  }

  fn block_until_change(&self) -> Result<bool, BlockUntilChangeError> {
    unsafe {
      reg::RegNotifyChangeKeyValue(
        self.0,
        false,
        reg::REG_NOTIFY_CHANGE_LAST_SET,
        None,
        false,
      )
    }
    .change_context(BlockUntilChangeError)
    .attach_printable("waiting for change notification")?;

    self.is_active().change_context(BlockUntilChangeError)
  }

  pub fn watch_for_changes(&self, app_handle: AppHandle) {
    let this = self.clone();

    let _ = std::thread::Builder::new()
      .name("color-filtering-registry-notification".into())
      .spawn(move || loop {
        let is_active = match this.block_until_change() {
          Ok(is_activ) => is_activ,
          Err(err) => {
            error!("failed to get notification:\n{err:#?}");

            return;
          }
        };

        info!(is_active, "color filtering state changed");

        if let Err(err) =
          app_handle.emit_all(COLOR_FILTERING_CHANGED_EVENT, is_active)
        {
          tracing::error!("failed to emit event:\n{err:#?}");
        }
      });
  }
}
