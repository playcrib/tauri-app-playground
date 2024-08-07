

use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{
    menu::{Menu, MenuItem, MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, Runtime, WebviewUrl, WebviewWindowBuilder,
};

pub fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let quit = MenuItemBuilder::new("Quit").id("quit").build(app).unwrap();
    let hide = MenuItemBuilder::new("Hide").id("hide").build(app).unwrap();
    let show = MenuItemBuilder::new("Show").id("show").build(app).unwrap();
    // we could opt handle an error case better than calling unwrap
    let menu = MenuBuilder::new(app)
      .items(&[&quit, &hide, &show])
      .build()
      .unwrap();

    let _ = TrayIconBuilder::new()
      .icon(app.default_window_icon().unwrap().clone())
      .menu(&menu)
      .on_menu_event(|app, event| match event.id().as_ref() {
        "quit" => app.exit(0),
        "hide" => {
          dbg!("menu item hide clicked");
          let window = app.get_webview_window("main").unwrap();
          window.hide().unwrap();
        }
        "show" => {
          dbg!("menu item show clicked");
          if let Some(window) = app.get_webview_window("main") {
            window.show().unwrap();
          } else {
            WebviewWindowBuilder::from_config(app, app.config().app.windows.get(0).unwrap())
              .unwrap()
              .build()
              .unwrap();
          }
        }
        _ => {}
      })
      .on_tray_icon_event(|tray, event| {
          if let TrayIconEvent::Click {
              button_state: MouseButtonState::Down,
              button: MouseButton::Left,
              ..
          } = event
          {
              let app = tray.app_handle();
              if let Some(window) = app.get_webview_window("main") {
                  let _ = window.show();
                  let _ = window.set_focus();
              }
          }
      })
      .build(app);

    Ok(())
}
