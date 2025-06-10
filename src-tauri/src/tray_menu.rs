use std::error::Error;

use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App, Manager,
};

use crate::{
    controllers::{quit, toggle_language},
    language::Language,
};

pub const TRAY_ICON_ID: &str = "10";

pub fn register_tray_menu(app: &App, start_language: Language) -> Result<(), Box<dyn Error>> {
    let quit_item = MenuItemBuilder::new("Quit").id("quit").build(app)?;
    let show_item = MenuItemBuilder::new("Show").id("show").build(app)?;
    let toggle_item = MenuItemBuilder::new("Toggle").id("toggle").build(app)?;
    let menu = MenuBuilder::new(app)
        .items(&[&quit_item, &show_item, &toggle_item])
        .build()?;

    TrayIconBuilder::with_id(TRAY_ICON_ID)
        .icon(get_icon_image(start_language))
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app_handle, menu_event| match menu_event.id().as_ref() {
            "quit" => {
                quit(app_handle);
            }
            "show" => {
                if let Some(window) = app_handle.get_webview_window("main") {
                    let _ = window.show();
                }
            }
            "toggle" => {
                toggle_language(app_handle);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray_icon, event| {
            if let TrayIconEvent::Click {
                button,
                button_state,
                ..
            } = event
            {
                if (button != MouseButton::Left) | (button_state != MouseButtonState::Up) {
                    return;
                }
                toggle_language(tray_icon.app_handle());
            }
        })
        .build(app)?;

    Ok(())
}

static mut UK_IMAGE: Option<Image<'static>> = None;
static mut FRANCE_IMAGE: Option<Image<'static>> = None;

pub fn get_icon_image(language: Language) -> Image<'static> {
    if language == Language::English {
        unsafe {
            if UK_IMAGE.is_none() {
                UK_IMAGE = Some(Image::from_path("resources/uk.png").expect("Missing uk icon"));
            }

            UK_IMAGE.clone().unwrap()
        }
    } else {
        unsafe {
            if FRANCE_IMAGE.is_none() {
                FRANCE_IMAGE =
                    Some(Image::from_path("resources/france.png").expect("Missing france icon"));
            }

            FRANCE_IMAGE.clone().unwrap()
        }
    }
}
