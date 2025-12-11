#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use reqwest::Client;

pub struct HttpClient(pub Client);
mod commands;
mod io_handler;

fn main() {
    tauri::Builder::default()
        .manage(HttpClient(reqwest::Client::new())) // ← shared client
        // .route("/listchats", get(list_chats)) will need
        // to be added once list_chats is made in server
        .invoke_handler(tauri::generate_handler![
            io_handler::log_message,
            io_handler::print_messages,
            commands::login,
            commands::register,
            commands::list_chats,
            commands::handleNewChat
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
