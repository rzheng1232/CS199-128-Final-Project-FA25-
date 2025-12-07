#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod io_handler;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            io_handler::log_message,      // ← ADD
            io_handler::get_chat_messages, // ← ADD  
            io_handler::list_chats,       // ← ADD
            io_handler::print_messages,
            io_handler::clear_messages,
            commands::login,
            commands::register
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
