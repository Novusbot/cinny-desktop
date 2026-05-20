#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// mod menu;

use tauri::{webview::{NewWindowResponse, WebviewWindowBuilder}, WebviewUrl};
use tauri_plugin_opener::OpenerExt;

pub fn run() {
    let port: u16 = 44548;
    let context = tauri::generate_context!();
    let builder = tauri::Builder::default();

    // #[cfg(target_os = "macos")]
    // {
    //     builder = builder.menu(menu::menu());
    // }

    builder
        // Базовые плагины для работы фронтенда
        .plugin(tauri_plugin_localhost::Builder::new(port).build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        
        // Плагины для работы с системой (нужны для скачивания и UI)
        .plugin(tauri_plugin_fs::init())             // Работа с файлами
        .plugin(tauri_plugin_dialog::init())         // Окна сохранения/открытия
        .plugin(tauri_plugin_notification::init())   // Пуши
        .plugin(tauri_plugin_clipboard_manager::init()) // Буфер обмена
        .plugin(tauri_plugin_shell::init())          // Запуск системных команд
        .plugin(tauri_plugin_http::init())           // HTTP-запросы из Rust
        .plugin(tauri_plugin_process::init())        // Управление процессами
        .plugin(tauri_plugin_os::init())             // Информация об ОС
        
        .setup(move |app| {
            // Dev: use devUrl from tauri.conf.json (http://localhost:8080) to support HMR
            #[cfg(debug_assertions)]
            let window_url = WebviewUrl::App(Default::default());

            // Release: tauri-plugin-localhost serves bundled frontend assets on this port
            #[cfg(not(debug_assertions))]
            let window_url = {
                let url = format!("http://localhost:{}", port).parse().unwrap();
                WebviewUrl::External(url)
            };

            let app_handle = app.handle().clone();
            WebviewWindowBuilder::new(app, "main".to_string(), window_url)
                .title("Cinny")
                .on_new_window(move |url, _features| {
                    let _ = app_handle.opener().open_url(url.as_str(), None::<&str>);
                    NewWindowResponse::Deny
                })
                .build()?;
            Ok(())
        })
        .run(context)
        .expect("error while building tauri application");
}
