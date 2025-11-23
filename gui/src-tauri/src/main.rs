
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate tauri;

mod io_handler;

#[tauri::command]
async fn login(username: String, password: String) -> Result<String, String> {
    println!("Login attempt: '{}' / '{}'", username, password);

    if username.trim() == "admin" && password == "secret" {
        Ok("Login successful!".to_string())
    } else {
        Err("Invalid username or password".to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            io_handler::print_messages,
            io_handler::log_message,
            io_handler::clear_messages,
            login
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}

