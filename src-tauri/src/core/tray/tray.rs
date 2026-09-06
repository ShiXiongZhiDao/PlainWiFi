use serde::Deserialize;
use tauri::{
    image::Image,
    menu::{Menu, MenuBuilder, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Listener, Manager, Runtime,
};

/// 托盘悬浮提示文案（跟随应用语言）
fn tray_tooltip(locale: &str) -> &'static str {
    if locale == "en" {
        "PlainWiFi"
    } else {
        "师兄明码WiFi"
    }
}

/// 按语言重建托盘菜单（菜单项 id 保持不变，事件路由不受影响）
fn localized_menu<R: Runtime>(
    app: &tauri::AppHandle<R>,
    locale: &str,
) -> tauri::Result<Menu<R>> {
    let (show_t, hide_t, quit_t) = if locale == "en" {
        ("Show", "Hide", "Quit")
    } else {
        ("显示", "隐藏", "退出")
    };
    let show_i = MenuItem::with_id(app, "show", show_t, true, None::<&str>)?;
    let hide_i = MenuItem::with_id(app, "hide", hide_t, true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", quit_t, true, None::<&str>)?;
    Ok(MenuBuilder::new(app)
        .item(&show_i)
        .item(&hide_i)
        .separator()
        .item(&quit_i)
        .build()?)
}

#[derive(Deserialize)]
pub struct TrayLocalePayload {
    pub locale: String,
}

#[derive(Deserialize)]
pub struct TrayThemePayload {
    pub dark: bool,
}

/// 前端 emit("tray:locale", { locale })：刷新菜单文案与悬浮提示
pub fn update_tray_locale<R: Runtime>(app: &tauri::AppHandle<R>, locale: &str) {
    if let Some(tray) = app.tray_by_id("tray") {
        if let Ok(menu) = localized_menu(app, locale) {
            let _ = tray.set_menu(Some(menu));
        }
        let _ = tray.set_title(Some(tray_tooltip(locale)));
    }
}

/// 前端 emit("tray:theme", { dark })：深色用白色剪影，浅色用默认彩色图标
pub fn update_tray_theme<R: Runtime>(app: &tauri::AppHandle<R>, dark: bool) {
    if let Some(tray) = app.tray_by_id("tray") {
        let icon = if dark {
            Image::from_bytes(include_bytes!("../../../icons/tray-white.png")).ok()
        } else {
            app.default_window_icon().cloned()
        };
        if let Some(i) = icon {
            let _ = tray.set_icon(Some(i));
        }
    }
}

/// 注册托盘语言/主题联动监听（在 setup 中 create_tray 之后调用一次）
pub fn register_tray_listeners<R: Runtime>(app: &tauri::AppHandle<R>) {
    let handle = app.clone();
    app.listen("tray:locale", move |event| {
        if let Ok(payload) = serde_json::from_str::<TrayLocalePayload>(event.payload()) {
            update_tray_locale(&handle, &payload.locale);
        }
    });
    let handle = app.clone();
    app.listen("tray:theme", move |event| {
        if let Ok(payload) = serde_json::from_str::<TrayThemePayload>(event.payload()) {
            update_tray_theme(&handle, payload.dark);
        }
    });
}

pub fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    // 默认中文菜单；前端启动后会依据持久化语言/主题 emit 初始状态刷新
    let menu = localized_menu(app, "zh")?;

    let _ = TrayIconBuilder::with_id("tray")
        .tooltip(tray_tooltip("zh"))
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "show" => {
                for (key, value) in app.webview_windows() {
                    if key == "main" {
                        value.show().unwrap();
                    }
                }
            }
            "hide" => {
                for (key, value) in app.webview_windows() {
                    if key == "main" {
                        value.hide().unwrap();
                        value.unminimize().unwrap();
                        value.set_focus().unwrap();
                    }
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            // 处理托盘图标事件
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let windows = tray.app_handle().webview_windows();
                for (key, value) in windows {
                    if key == "login" || key == "home" {
                        // 显示主窗口
                        value.show().unwrap();
                        // 取消窗口最小化状态
                        value.unminimize().unwrap();
                        // 将焦点设置到主窗口上
                        value.set_focus().unwrap();
                    }
                }
            }
            // 双击鼠标左键事件:显示/隐藏窗口
            if let TrayIconEvent::DoubleClick {
                button: MouseButton::Left,
                ..
            } = event
            {
                let windows = tray.app_handle().webview_windows();
                for (key, value) in windows {
                    if key == "main" || key == "home" {
                        if value.is_visible().unwrap() {
                            value.hide().unwrap();
                        } else {
                            // 显示主窗口
                            value.show().unwrap();
                            // 取消窗口最小化状态
                            value.unminimize().unwrap();
                            value.set_focus().unwrap();
                        }
                    }
                }
            }
        })
        .build(app);

    Ok(())
}
