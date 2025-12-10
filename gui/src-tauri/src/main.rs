#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::collections::HashMap;
extern crate tauri;
use reqwest::Error;
use tauri::command;

mod io_handler;

#[command]
async fn login(user: String, pass: String) -> Result<i32, String> {
    let url = format!("http://98.93.98.244/Authenticate/username/{}/password/{}", user, pass);
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    let resp_text = resp.text()
        .await
        .map_err(|e| format!("Response failed: {}", e))?;
    
    match resp_text.trim().parse::<i32>() {
        Ok(1) => Ok(1),
        _ => Ok(0),
    }
}

#[command]
async fn register(user: String, pass: String) -> Result<i32, String> {
    let url = format!("http://98.93.98.244/createaccount/username/{}/password/{}", user, pass);
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    let resp_text = resp.text()
        .await
        .map_err(|e| format!("Response failed: {}", e))?;
    
    match resp_text.trim().parse::<i32>() {
        Ok(1) => Ok(1),
        _ => Ok(0),
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
