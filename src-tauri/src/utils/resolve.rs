use super::{init, server};
use crate::{core::Profiles, states};
use tauri::{App, AppHandle, Manager};

/// handle something when start app
pub fn resolve_setup(app: &App) {
  resolve_window(app);

  // setup a simple http server for singleton
  server::embed_server(&app.handle());

  // init app config
  init::init_app(app.package_info());

  // init states
  let clash_state = app.state::<states::ClashState>();
  let verge_state = app.state::<states::VergeState>();
  let profiles_state = app.state::<states::ProfilesState>();

  let mut clash = clash_state.0.lock().unwrap();
  let mut verge = verge_state.0.lock().unwrap();
  let mut profiles = profiles_state.0.lock().unwrap();

  if let Err(err) = clash.run_sidecar() {
    log::error!("{}", err);
  }

  *profiles = Profiles::read_file();
  if let Err(err) = profiles.activate(&clash) {
    log::error!("{}", err);
  }

  verge.init_sysproxy(clash.info.port.clone());
  verge.init_launch();
  if let Err(err) = verge.sync_launch() {
    log::error!("{}", err);
  }
}

/// reset system proxy
pub fn resolve_reset(app_handle: &AppHandle) {
  let verge_state = app_handle.state::<states::VergeState>();
  let mut verge = verge_state.0.lock().unwrap();

  verge.reset_sysproxy();
}

/// customize the window theme
fn resolve_window(app: &App) {
  let window = app.get_window("main").unwrap();

  #[cfg(target_os = "windows")]
  {
    use tauri_plugin_shadows::Shadows;
    use tauri_plugin_vibrancy::Vibrancy;

    window.set_decorations(false).unwrap();
    window.set_shadow(true);
    window.apply_blur();
  }

  #[cfg(target_os = "macos")]
  {
    use tauri::LogicalSize;
    use tauri::Size::Logical;
    window.set_decorations(true).unwrap();
    window
      .set_size(Logical(LogicalSize {
        width: 800.0,
        height: 610.0,
      }))
      .unwrap();
    // use tauri_plugin_vibrancy::MacOSVibrancy;
    // #[allow(deprecated)]
    // window.apply_vibrancy(MacOSVibrancy::AppearanceBased);
  }
}
