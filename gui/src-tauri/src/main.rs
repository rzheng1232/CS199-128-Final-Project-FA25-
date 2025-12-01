
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
 use std::collections::HashMap;
extern crate tauri;
use reqwest::Error;

mod io_handler;

#[tauri::command]
async fn login(username: String, password: String) -> Result<String, String> {
    println!("Login attempt: '{}' / '{}'", username, password);
    
     //     query 

 return Ok("Login successful!".to_string());

}

#[tauri::command]
async fn register(username: String, password: String) -> Result<String, String> {
    println!("Register attempt: '{}' / '{}'", username, password);

    return Ok("Registration successful!".to_string());
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            io_handler::print_messages,
            io_handler::log_message,
            io_handler::clear_messages,
            login,
            register
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}

