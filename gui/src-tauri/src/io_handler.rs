use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::fs::read_dir;
use std::fs::File;
use std::io::Read;
use std::io::Write;

// user stuff
#[derive(Clone, Hash)]
pub struct User {
    pub id: String,
    pub password: String,
}

#[derive(Clone)]
pub struct UserList<'a> {
    pub active_users: HashMap<&'a str, User>,
}

// message stuff
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub user: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chat {
    pub name: String,
    pub messages: Vec<Message>,
}

// sending a message and printing it out
pub fn display_message(message: &str) {
    // only prints it out if it's not empty
    if !message.is_empty() {
        println!("{}", message);
    }
}

fn read_messages_from_chat_json(path: &str) -> Vec<Message> {
    if let Ok(mut file) = File::open(path) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            return serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new());
        }
    }
    Vec::new()
}

fn read_chats_from_json(path: &str) -> Vec<Chat> {
    if let Ok(mut file) = File::open(path) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            return serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new());
        }
    }
    Vec::new()
}

// save chat
#[tauri::command]
pub fn log_message(users: Vec<String>, user: String, message: String) -> Result<(), String> {
    let chat_filename = chat_filename(&users);
    let dir_path = "../app_data/cache";
    let file_path = format!("{}/{}", dir_path, chat_filename);

    create_dir_all(dir_path).map_err(|e| format!("Failed to create directory: {}", e))?;

    let mut messages = read_messages_from_chat_json(&file_path);
    messages.push(Message {
        user,
        message,
        timestamp: Utc::now(),
    });

    let json = serde_json::to_string_pretty(&messages).map_err(|e| e.to_string())?;

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&file_path)
        .map_err(|e| e.to_string())?;

    file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_chat_messages(users: Vec<String>) -> Result<Vec<Message>, String> {
    let chat_filename = chat_filename(&users);
    let file_path = format!("../app_data/cache/{}", chat_filename);
    Ok(read_messages_from_chat_json(&file_path))
}

fn chat_filename(users: &[String]) -> String {
    let mut sorted = users.to_vec(); // Fixed
    sorted.sort();
    format!("{}.json", sorted.join("_"))
}

#[tauri::command]
pub fn print_messages(path: Option<String>) -> Result<Vec<Chat>, String> {
    let path = path.unwrap_or_else(|| "../app_data/cache/chat_history.json".to_string());

    let chats = read_chats_from_json(&path);

    Ok(chats)
}

#[tauri::command]
pub fn clear_messages(path: Option<String>) -> Result<(), String> {
    let path = path.unwrap_or_else(|| "../app_data/cache/chat_history.json".to_string()); // Uses default if path doesn't exist -- will be fixed later just for testing now
    let mut file = File::create(&path).map_err(|e| e.to_string())?;
    file.write_all(b"[]").map_err(|e| e.to_string())?;
    Ok(())
}

// pub fn hash_name(name: &str) -> Jas {}

impl<'a> UserList<'a> {
    pub fn handle_join_message(&mut self, user_id: &'a str) {
        let new_user = User {
            id: user_id.to_string(),
            password: " ".to_string(), // irrelevant here???
        };
        self.active_users.insert(user_id, new_user.clone());
        println!("User {} joined.", new_user.id);
    }

    pub fn handle_leave_message(&mut self, user_id: &'a str) {
        if self.active_users.remove(user_id).is_some() {
            println!("User {} left.", user_id);
        }
    }

    pub fn display_active_users(&self) {
        println!("--- Active Users ---");
        for user_id in self.active_users.keys() {
            println!("- {}", user_id);
        }
        println!("--------------------");
    }
}