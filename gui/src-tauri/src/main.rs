
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
 use std::collections::HashMap;
extern crate tauri;

mod io_handler;

#[tauri::command]
async fn login(username: String, password: String) -> Result<String, String> {
    println!("Login attempt: '{}' / '{}'", username, password);
    let mut user_map: HashMap<String, String> = HashMap::new();

        // Insert key-value pairs
        user_map.insert("admin".to_string(), "secret".to_string());
        user_map.insert("len".to_string(), "password".to_string());
        user_map.insert("mia".to_string(), "password".to_string());
        user_map.insert("ryan".to_string(), "password".to_string());

        for (user, pass) in &user_map {
            if username == *user && password == *pass {
                return Ok("Login successful!".to_string());
            }
        }
        Err("Invalid username or password".to_string())
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

