#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::collections::HashMap;
extern crate tauri;
use reqwest::Error;
use tauri::command;

mod io_handler;

#[tauri::command]
async fn login(user: String, pass: String) -> Result<i32, i32> {
    println!("Login attempt: '{}' / '{}'", user, pass);
    
    let resp_text = reqwest::get(format!("http://98.93.98.244/Authenticate/username/{}/password/{}", user, pass))
        .await
        .map_err(|_| 0)?          // Map reqwest error to your error type (0)
        .text()
        .await
        .map_err(|_| 0)?;         // Await text and map error too

    // Parse the response text safely as i32, default to 0 on error
    if resp_text.parse::<i32>().unwrap_or(0) == 1 {
        Ok(1)
    } else {
        Err(0)
    }
}

#[tauri::command]
async fn register(user: String, pass: String) -> Result<i32, i32> {
    println!("Register attempt: '{}' / '{}'", user, pass);

    let resp_text = reqwest::get(format!("http://98.93.98.244/createaccount/username/{}/password/{}", user, pass))
        .await
        .map_err(|_| 0)?
        .text()
        .await
        .map_err(|_| 0)?;

    if resp_text.parse::<i32>().unwrap_or(0) == 1 {
        Ok(1)
    } else {
        Err(0)
    }
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