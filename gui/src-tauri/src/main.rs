#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod io_handler;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            io_handler::print_messages,
            io_handler::log_message,
            io_handler::clear_messages,
            commands::login,
            commands::register
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
