#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serial_port::start_serial;
use tauri::{Manager, Window};

mod serial_port;

#[tauri::command]
fn init_process(window: Window) {
    std::thread::spawn(move || {
        start_serial(window).expect("serial error");
    });
}
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            app.listen_global("click", |event| {
                println!("got event-name with payload {:?}", event.payload());
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![init_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
