mod command;
mod core;
use command::wifi::get_wifi_passwords;
use core::tray::tray::{create_tray, register_tray_listeners};
use std::env;
use tauri::{AppHandle, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // 处理单实例程序的逻辑:如果已经存在一个实例，则显示该实例的窗口
            let _ = handle_single_instance_event(app);
        }))
        .plugin(tauri_plugin_opener::init())
        // 注册命令
        .invoke_handler(tauri::generate_handler![get_wifi_passwords])
        .setup(|app| {
            #[cfg(all(desktop))]
            {
                // 创建系统托盘，并注册语言/主题联动监听
                let handle = app.handle();
                create_tray(handle)?;
                register_tray_listeners(handle);
            }
            Ok(())
        })
        // 窗口关闭，不退出程序，隐藏窗口
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn handle_single_instance_event(app: &AppHandle) {
    // 处理单实例程序的逻辑:如果已经存在一个实例，则显示该实例的窗口
    // 这里只是一个示例，你可以根据自己的需求进行修改
    for (key, value) in app.webview_windows() {
        if key == "main" {
            // 显示主窗口
            value.show().unwrap();
            // 取消窗口最小化状态
            value.unminimize().unwrap();
            // 将焦点设置到主窗口上
            value.set_focus().unwrap();
        }
    }
}
